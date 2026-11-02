import { expect, test } from "bun:test";
import type { Act, Obj, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { round } from "../../sim/bots.ts";
import { geese } from "../../test/gym.ts";
import type { Wallet } from "./app.ts";
import wallet, { MOST, NOTE } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const GRANT = 1000;

const ON = { payments: "on", gifts: "on" };

const OFF = { payments: "off", gifts: "off" };

const configOf = (rules: Obj) => ({
  name: "wallet",
  apps: [{ id: "wallet", config: { causes: { commons: "The commons" } } }],
  actors: ACTORS,
  rules: { grant: GRANT, ...rules },
});

const K = kernel([wallet]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.wallet as Wallet;

const call = (world: World, id: string, verb: string, args: Act["args"]) =>
  K.call(world, { verb: `wallet.${verb}`, args, who: as(id) });

const notes = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.map(([id, verb, args]) => {
    const done = call(world, id, verb, args);
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });

test("boot grants each test user the grant rule once, logged as a grant, a pick of grants settles to one, an open world grants nobody, and a bad rule stops the boot", () => {
  const booted = state(K.boot(configOf(OFF)));
  const picked = state(K.boot({ ...configOf(OFF), seed: 5, rules: { grant: { pick: [10, 20, 30] } } }));
  const open = state(K.boot({ name: "wallet", apps: ["wallet"], rules: { grant: GRANT } }));
  expect([booted.balances["test-3"], booted.history["test-3"]![0]!.kind, booted.supply, booted.n]).toEqual([
    GRANT,
    "grant",
    GRANT * 8,
    8,
  ]);
  expect([10, 20, 30].includes(picked.balances["test-1"]!)).toBe(true);
  expect([open.balances, open.supply]).toEqual([{}, 0]);
  expect(() => K.boot({ ...configOf(OFF), rules: { grant: 1.5 } })).toThrow("whole number");
  expect(() => K.boot({ ...configOf(OFF), rules: { payments: true } })).toThrow('"on" or "off"');
});

test("a reserve is an account the config funds at boot for an app's counterparty, counted in the supply, and a bad reserve stops the boot", () => {
  const config = {
    ...configOf(OFF),
    apps: [{ id: "wallet", config: { causes: { commons: "The commons" }, reserves: { exchange: 500 } } }],
  };
  const booted = state(K.boot(config));
  expect([booted.balances.exchange, booted.supply, "exchange" in booted.history]).toEqual([
    500,
    GRANT * 8 + 500,
    false,
  ]);
  const bad: Obj[] = [{ exchange: 1.5 }, { "test-1": 5 }, { commons: 5 }, { Exchange: 5 }];
  for (const reserves of bad)
    expect(() =>
      K.boot({ ...config, apps: [{ id: "wallet", config: { causes: { commons: "The commons" }, reserves } }] }),
    ).toThrow("wallet: reserve");
});

test("a payment or a gift out of its schema is a bad act and changes nothing", () => {
  const world = K.boot(configOf(ON));
  const bad: [string, Act["args"]][] = [
    ["send", { to: "test-2", amount: 0 }],
    ["send", { to: "test-2", amount: 2.5 }],
    ["send", { to: "test-2", amount: MOST + 1 }],
    ["send", { to: "commons", amount: 5 }],
    ["give", { to: "test-9", amount: 5 }],
    ["give", { to: "test-2", amount: 5, note: "x".repeat(NOTE + 1) }],
    ["give", { to: "test-2", amount: 5, from: "test-3" }],
  ];
  for (const [verb, args] of bad) expect(() => call(world, "test-1", verb, args)).toThrow("bad-act");
});

test("with payments off, a payment is refused with a note naming the rule, while a gift to a person still goes through", () => {
  const world = K.boot(configOf({ payments: "off", gifts: "on" }));
  expect(notes(world, [["test-1", "send", { to: "test-2", amount: 5 }]])).toEqual([
    "payments between people are off in this world; give to a cause: commons",
  ]);
  const given = state(call(world, "test-1", "give", { to: "test-2", amount: 30 }).world);
  expect([given.balances["test-1"], given.balances["test-2"], given.gifts.length]).toEqual([GRANT - 30, GRANT + 30, 1]);
});

test("with gifts off, a gift to a person is refused with a note naming the rule, a gift to a cause goes through, and a payment still does", () => {
  const world = K.boot(configOf({ payments: "on", gifts: "off" }));
  expect(notes(world, [["test-1", "give", { to: "test-2", amount: 5 }]])).toEqual([
    "gifts between people are off in this world; give to a cause: commons",
  ]);
  const given = state(call(world, "test-1", "give", { to: "commons", amount: 30 }).world);
  const paid = state(call(world, "test-1", "send", { to: "test-2", amount: 20 }).world);
  expect([given.balances["test-1"], given.raised.commons, given.gifts.length]).toEqual([GRANT - 30, 30, 1]);
  expect([paid.balances["test-1"], paid.balances["test-2"]]).toEqual([GRANT - 20, GRANT + 20]);
});

test("nobody pays or gives to themselves, spends more than they hold, or acts as a watcher", () => {
  const world = K.boot(configOf(ON));
  expect(
    notes(world, [
      ["test-1", "send", { to: "test-1", amount: 5 }],
      ["test-1", "give", { to: "test-1", amount: 5 }],
      ["test-1", "give", { to: "test-2", amount: GRANT + 1 }],
    ]),
  ).toEqual([
    "you cannot pay yourself",
    "a gift goes to someone else; nobody gives to themselves",
    `you hold ${GRANT}, less than ${GRANT + 1}`,
  ]);
  expect(() => call(world, "guest", "give", { to: "commons", amount: 1 })).toThrow("refused");
});

test("with payments and gifts on, a payment and a gift move the exact amount, show in both histories, only the gift is public, and the payee is told", () => {
  const paid = call(K.boot(configOf(ON)), "test-1", "send", { to: "test-2", amount: 120, note: "rent" });
  let world = paid.world;
  world = call(world, "test-2", "give", { to: "test-3", amount: 45 }).world;
  const after = state(world);
  expect(ACTORS.slice(0, 4).map((id) => after.balances[id])).toEqual([GRANT - 120, GRANT + 75, GRANT + 45, GRANT]);
  expect(after.history["test-2"]!.map((one) => [one.kind, one.from, one.to, one.amount])).toEqual([
    ["grant", null, "test-2", GRANT],
    ["pay", "test-1", "test-2", 120],
    ["gift", "test-2", "test-3", 45],
  ]);
  expect(after.gifts.map((one) => one.n)).toEqual([10]);
  expect(paid.envelope.wishes).toEqual([{ kind: "notice", data: { to: "test-2", text: "test-1 paid you 120: rent" } }]);
  expect(K.read(world, "wallet/@test-3/balance").state).toBe(GRANT + 45);
});

test("the test users' policy plays 300 rounds under every setting of the switches and money is only moved: the supply holds, no balance falls below zero", () => {
  for (const payments of ["on", "off"])
    for (const gifts of ["on", "off"]) {
      let world = K.boot({ ...configOf({ payments, gifts }), seed: 7 });
      const table = {
        world: () => world,
        list: () => K.list(world),
        call: (act: Act) => {
          world = K.call(world, act).world;
        },
      };
      for (let i = 1; i <= 300; i++) round(table, ACTORS.map(as), i * 1000, [wallet]);
      const after = state(world);
      const held = Object.values(after.balances).reduce((a, b) => a + b, 0);
      const raised = Object.values(after.raised).reduce((a, b) => a + b, 0);
      expect([held + raised, Object.values(after.balances).every((one) => one >= 0), after.gifts.length > 0]).toEqual([
        after.supply,
        true,
        true,
      ]);
    }
});

test("10,000 seeded worlds of wallet played by the goose through the gym replay to their digests", async () => {
  const played = await geese(configOf(ON), 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
