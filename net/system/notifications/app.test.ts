import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { app, below, notice, noticed } from "../../sdk/system.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { Inbox } from "./app.ts";
import inbox, { KEEP } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "notifications", apps: ["notifications"], actors: ACTORS };

const bell = app<number>({
  id: "bell",
  title: "Bell",
  line: "Rings someone.",
  boot: () => 0,
  verbs: {
    ring: {
      line: "Ring someone.",
      args: { type: "object", properties: { to: { enum: ACTORS } }, required: ["to"], additionalProperties: false },
      run: (state, args, ctx) => {
        ctx.wish(notice(String(args.to), `Ring ${state + 1}`));
        return state + 1;
      },
    },
  },
  bot: ({ actors, me, draw }) => ({ verb: "ring", args: { to: actors.filter((one) => one !== me)[below(draw, 7)]! } }),
});

const K = kernel([inbox, bell]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.notifications as Inbox;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `notifications.${verb}`, args, who: as(id) }).world, world);

const raised = (to: string, text: string): [string, string, Act["args"]] => [
  "test-1",
  "notice",
  { to, app: "mail", text },
];

test("a notice, read or dismiss out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [raised("test-2", "hi")]);
  const bad: [string, Act["args"]][] = [
    ["notice", { to: "test-9", app: "mail", text: "hi" }],
    ["notice", { to: "test-2", app: "mail", text: "x".repeat(141) }],
    ["notice", { to: "test-2", text: "hi" }],
    ["read", { id: 2 }],
    ["dismiss", {}],
    ["clear", { id: 1 }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `notifications.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("only a notice's owner reads or dismisses it, an app id is plain, and a watcher may not act", () => {
  const world = play(K.boot(CONFIG), [raised("test-2", "hi")]);
  const nosy = K.call(world, { verb: "notifications.dismiss", args: { id: 1 }, who: as("test-3") });
  const odd = K.call(world, {
    verb: "notifications.notice",
    args: { to: "test-2", app: "Mail!", text: "hi" },
    who: as("test-1"),
  });
  expect([nosy.envelope.last.note, odd.envelope.last.note]).toEqual([
    "notice 1 is test-2's",
    "an app id is lowercase letters and digits",
  ]);
  expect(digest(nosy.world.apps)).toBe(digest(world.apps));
  expect(() => K.call(world, { verb: "notifications.read", args: {}, who: as("guest") })).toThrow("refused");
});

test(`a user keeps the last ${KEEP} notices; read with no id reads them all and clear drops the read ones`, () => {
  const many = Array.from({ length: KEEP + 3 }, (_, i) => raised("test-2", `n${i}`));
  const full = play(K.boot(CONFIG), many);
  const read = play(full, [["test-2", "read", { id: KEEP + 3 }]]);
  const all = play(read, [["test-2", "read", {}]]);
  const cleared = play(read, [["test-2", "clear", {}]]);
  expect([state(full).notices["test-2"]!.length, state(full).notices["test-2"]![0]!.text]).toEqual([KEEP, "n3"]);
  expect(K.read(read, "notifications/@test-2", ["unread"]).state).toEqual({ unread: KEEP - 1 });
  expect(K.read(all, "notifications/@test-2", ["unread"]).state).toEqual({ unread: 0 });
  expect(state(cleared).notices["test-2"]!.length).toBe(KEEP - 1);
});

test("an app's notice wish becomes an inbox.notice act by the same user at the same time, stamped with the app", () => {
  const done = K.call(K.boot({ ...CONFIG, apps: ["notifications", "bell"] }), {
    verb: "bell.ring",
    args: { to: "test-4" },
    who: as("test-1"),
    at: 5000,
  });
  expect(noticed(done, [inbox, bell])).toEqual([
    { verb: "notifications.notice", args: { to: "test-4", text: "Ring 1", app: "bell" }, who: as("test-1"), at: 5000 },
  ]);
  expect(
    noticed(
      K.call(K.boot({ ...CONFIG, apps: ["bell"] }), { verb: "bell.ring", args: { to: "test-4" }, who: as("test-1") }),
      [inbox, bell],
    ),
  ).toEqual([]);
});

test("the scripted test users read and clear the notices other apps raise, with no refusal", () => {
  const run = scripted([inbox, bell], { ...CONFIG, apps: ["notifications", "bell"] }, 120);
  expect([run.refused, run.verbs["notifications.read"]! > 50, run.verbs["notifications.notice"]! > 300]).toEqual([
    {},
    true,
    true,
  ]);
});

test("10,000 seeded worlds of inbox played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
