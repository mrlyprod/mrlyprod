import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import people from "../people/app.ts";
import type { Wallet } from "../wallet/app.ts";
import wallet from "../wallet/app.ts";
import type { Stocks } from "./app.ts";
import stocks, { advance, COMPANIES, EXCHANGE, HISTORY, LOT, MOVE, symbolOf } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "stocks", apps: ["people", "stocks"], actors: ACTORS };

const GRANT = 100_000;

const RESERVE = 10_000_000;

const MARKET = {
  name: "market",
  apps: [{ id: "wallet", config: { reserves: { [EXCHANGE]: RESERVE } } }, "stocks"],
  actors: ACTORS,
  rules: { grant: GRANT },
};

const K = kernel([people, stocks]);

const M = kernel([wallet, stocks]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.stocks as Stocks;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb, args, who: as(id) }).world, world);

test(`the board boots with ${COMPANIES} seeded companies at whole-cent prices, and the same seed gives the same board`, () => {
  const one = state(K.boot({ ...CONFIG, seed: 1 }));
  const prices = Object.values(one.board).map((stock) => stock.price);
  expect([prices.length, prices.every((price) => Number.isInteger(price) && price >= 500), one.tick]).toEqual([
    COMPANIES,
    true,
    0,
  ]);
  expect(Object.keys(one.board)).toEqual(Object.keys(state(K.boot({ ...CONFIG, seed: 1 })).board));
  expect(Object.keys(one.board)).not.toEqual(Object.keys(state(K.boot({ ...CONFIG, seed: 2 })).board));
  expect([symbolOf("Norvik Mills", []), symbolOf("Norvik Mills", ["NM"]), symbolOf("Acme Paper Works", [])]).toEqual([
    "NM",
    "NM2",
    "APW",
  ]);
});

test(`prices move every world tick by at most ${MOVE} per mille, in whole cents, the same for any path to a tick, keeping ${HISTORY} back`, () => {
  const booted = state(K.boot({ ...CONFIG, seed: 3 }));
  const straight = advance(booted, 40);
  const stepped = advance(advance(advance(booted, 5), 17), 40);
  expect(digest(straight)).toBe(digest(stepped));
  const one = Object.values(straight.board)[0]!;
  const was = Object.values(booted.board)[0]!.price;
  expect([straight.tick, one.history.length, Number.isInteger(one.price), one.price > 0]).toEqual([
    40,
    HISTORY,
    true,
    true,
  ]);
  expect(Math.abs(one.history[1]! - one.history[0]!) * 1000).toBeLessThanOrEqual(MOVE * one.history[0]! + 1000);
  expect(one.history[0]).not.toBe(was);
  expect(advance(booted, 0)).toBe(booted);
});

test("a watch, an unwatch or a listing out of its schema is a bad act and changes nothing", () => {
  const world = K.boot(CONFIG);
  const bad: [string, Act["args"]][] = [
    ["stocks.watch", { symbol: "NOPE" }],
    ["stocks.watch", {}],
    ["stocks.unwatch", { symbol: 7 }],
    ["stocks.list", { org: 0 }],
    ["stocks.list", { org: 1, symbol: "ME" }],
    ["stocks.buy", { symbol: "NOPE", qty: 1 }],
    ["stocks.buy", { symbol: Object.keys(state(world).board)[0]!, qty: 0 }],
    ["stocks.sell", { symbol: Object.keys(state(world).board)[0]!, qty: LOT + 1 }],
  ];
  for (const [verb, args] of bad) expect(() => K.call(world, { verb, args, who: as("test-1") })).toThrow("bad-act");
});

test("a watchlist is its owner's, only an owner lists a live org of people once, and a watcher may not act", () => {
  const world = play(K.boot(CONFIG), [
    ["test-1", "people.found", { name: "Norvik Mills", open: true }],
    ["test-2", "people.join", { org: 1 }],
    ["test-1", "stocks.list", { org: 1 }],
  ]);
  const symbol = Object.keys(state(world).board)[0]!;
  const notes = [
    ["test-1", "stocks.unwatch", { symbol }],
    ["test-2", "stocks.list", { org: 1 }],
    ["test-1", "stocks.list", { org: 1 }],
    ["test-1", "stocks.list", { org: 2 }],
  ].map(([id, verb, args]) => {
    const done = K.call(world, { verb: verb as string, args: args as Act["args"], who: as(id as string) });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });
  const mills = Object.values(state(world).board).find((one) => one.org === 1)!;
  expect(notes).toEqual([
    `you do not watch ${symbol}`,
    "only an owner of Norvik Mills lists it",
    `Norvik Mills is listed as ${mills.symbol} already`,
    "no org 2 in people",
  ]);
  expect([Object.keys(state(world).board).length, mills.symbol.startsWith("NM")]).toEqual([COMPANIES + 1, true]);
  const alone = kernel([stocks]).boot({ ...CONFIG, apps: ["stocks"] });
  expect(
    kernel([stocks]).call(alone, { verb: "stocks.list", args: { org: 1 }, who: as("test-1") }).envelope.last.note,
  ).toBe("this world has no people, so no orgs to list; the board holds the seeded companies");
  expect(() => K.call(world, { verb: "stocks.watch", args: { symbol }, who: as("guest") })).toThrow("refused");
});

test("a watch moves the board to the act's tick, and a slice is the watcher's symbols with price and change", () => {
  const booted = K.boot(CONFIG);
  const symbol = Object.keys(state(booted).board).sort()[0]!;
  const world = play(booted, [
    ["test-1", "people.found", { name: "Idle", open: true }],
    ["test-1", "people.found", { name: "Idle Too", open: true }],
    ["test-2", "stocks.watch", { symbol }],
  ]);
  const one = state(world).board[symbol]!;
  expect([state(world).tick, one.history.length, digest(state(world).board)]).toEqual([
    3,
    3,
    digest(advance(state(booted), 3).board),
  ]);
  expect(K.read(world, "stocks/@test-2").state).toEqual({
    tick: 3,
    watching: [{ symbol, name: one.name, price: one.price, change: one.price - one.history[0]! }],
    holdings: [],
  });
});

test("a buy pays the exchange at the price of the act's tick and adds the shares, a sell is paid by it, and the envelope names the payment", () => {
  const booted = M.boot({ ...MARKET, seed: 4 });
  const symbol = Object.keys(state(booted).board).sort()[0]!;
  const bought = M.call(booted, { verb: "stocks.buy", args: { symbol, qty: 3 }, who: as("test-1") });
  const price = state(bought.world).board[symbol]!.price;
  const cash = (world: World, id: string) => (world.apps.wallet as Wallet).balances[id];
  expect([bought.envelope.paid, cash(bought.world, "test-1"), cash(bought.world, EXCHANGE)]).toEqual([
    { from: "test-1", to: EXCHANGE, amount: price * 3 },
    GRANT - price * 3,
    RESERVE + price * 3,
  ]);
  expect(M.read(bought.world, "stocks/@test-1").state).toEqual({
    tick: 1,
    watching: [],
    holdings: [{ symbol, name: state(bought.world).board[symbol]!.name, qty: 3, price, value: price * 3 }],
  });
  expect((bought.world.apps.wallet as Wallet).history["test-1"]!.at(-1)).toEqual({
    n: 9,
    at: 0,
    kind: "pay",
    from: "test-1",
    to: EXCHANGE,
    amount: price * 3,
    note: "stocks.buy",
  });
  const sold = M.call(bought.world, { verb: "stocks.sell", args: { symbol, qty: 2 }, who: as("test-1") });
  const now = state(sold.world).board[symbol]!.price;
  expect([sold.envelope.paid, cash(sold.world, "test-1"), state(sold.world).holdings["test-1"]]).toEqual([
    { from: EXCHANGE, to: "test-1", amount: now * 2 },
    GRANT - price * 3 + now * 2,
    { [symbol]: 1 },
  ]);
});

test("a buy beyond your balance, a sale of shares you lack, or a trade in a world without a wallet is refused and moves nothing", () => {
  const world = M.boot(MARKET);
  const symbol = Object.keys(state(world).board).sort()[0]!;
  const price = advance(state(world), 1).board[symbol]!.price;
  const refused = [
    M.call(world, { verb: "stocks.buy", args: { symbol, qty: LOT }, who: as("test-1") }),
    M.call(world, { verb: "stocks.sell", args: { symbol, qty: 1 }, who: as("test-1") }),
    K.call(K.boot(CONFIG), { verb: "stocks.buy", args: { symbol, qty: 1 }, who: as("test-1") }),
  ];
  expect(refused.map((done) => [done.envelope.last.note, done.envelope.paid])).toEqual([
    [`test-1 holds ${GRANT}, less than ${price * LOT}`, undefined],
    [`you hold 0 ${symbol}, not 1`, undefined],
    ["this world has no wallet, so nothing can be paid", undefined],
  ]);
  expect(digest(refused[0]!.world.apps)).toBe(digest(world.apps));
  expect(digest(refused[1]!.world.apps)).toBe(digest(world.apps));
});

test("the test users' policy trades 150 rounds: buys and sells both happen, the supply holds, and no stocks act is refused", () => {
  const run = scripted([wallet, stocks], { ...MARKET, seed: 9 }, 150);
  const after = run.world.apps.wallet as Wallet;
  const held = Object.values(after.balances).reduce((a, b) => a + b, 0);
  const stocksRefused = Object.entries(run.refused).filter(
    ([note]) => !note.includes("gifts") && !note.includes("payments"),
  );
  expect([run.verbs["stocks.buy"]! > 0, run.verbs["stocks.sell"]! > 0, held, stocksRefused]).toEqual([
    true,
    true,
    after.supply,
    [],
  ]);
});

test("10,000 seeded worlds of wallet and stocks played by the goose through the gym replay to their digests", async () => {
  const played = await geese(MARKET, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);

test("10,000 seeded worlds of people and stocks played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
