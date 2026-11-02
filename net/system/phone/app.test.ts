import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import inbox from "../notifications/app.ts";
import type { Calls } from "./app.ts";
import calls, { LOG, RING } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "phone", apps: ["phone"], actors: ACTORS };

const K = kernel([calls]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.phone as Calls;

const play = (world: World, acts: [string, string, Act["args"], number?][]) =>
  acts.reduce(
    (then, [id, verb, args, at]) => K.call(then, { verb: `phone.${verb}`, args, who: as(id), at }).world,
    world,
  );

const rung = play(K.boot(CONFIG), [["test-1", "ring", { to: "test-2" }, 1000]]);

test("a ring, accept, decline or end out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["ring", { to: "test-9" }],
    ["ring", {}],
    ["accept", { id: 2 }],
    ["decline", { id: "1" }],
    ["end", { id: 1, reason: "busy" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(rung, { verb: `phone.${verb}`, args, who: as("test-2") })).toThrow("bad-act");
});

test("only the callee answers, only the two people end a call, the busy are refused, and a watcher may not ring", () => {
  const notes = [
    K.call(rung, { verb: "phone.accept", args: { id: 1 }, who: as("test-1") }),
    K.call(rung, { verb: "phone.end", args: { id: 1 }, who: as("test-3") }),
    K.call(rung, { verb: "phone.ring", args: { to: "test-1" }, who: as("test-3") }),
    K.call(rung, { verb: "phone.ring", args: { to: "test-3" }, who: as("test-1") }),
    K.call(rung, { verb: "phone.ring", args: { to: "test-3" }, who: as("test-3") }),
  ];
  expect(notes.map((one) => one.envelope.last.note)).toEqual([
    "only test-2 answers call 1",
    "call 1 is between test-1 and test-2",
    "test-1 is on another call",
    "you are on call 1; end it first",
    "a call is between two people; call someone else",
  ]);
  expect(notes.map((one) => digest(one.world.apps))).toEqual(notes.map(() => digest(rung.apps)));
  expect(() => K.call(rung, { verb: "phone.ring", args: { to: "test-1" }, who: as("guest") })).toThrow("refused");
});

test("a call rings, is accepted and ends with its times; one left ringing is missed on the next act, with a notice", () => {
  const talked = play(rung, [
    ["test-2", "accept", { id: 1 }, 3000],
    ["test-1", "end", { id: 1 }, 63_000],
  ]);
  expect(state(talked).calls["1"]).toMatchObject({ status: "ended", answered: 3000, ended: 63_000 });
  const late = K.call(rung, { verb: "phone.ring", args: { to: "test-4" }, who: as("test-3"), at: 1000 + RING });
  expect(state(late.world).calls["1"]).toMatchObject({ status: "missed", ended: 1000 + RING });
  expect([
    late.envelope.wishes.map((wish) => wish.data),
    (K.read(late.world, "phone/@test-2").state as { missed: number }).missed,
  ]).toEqual([[{ to: "test-2", text: "Missed call from test-1" }], 1]);
  const declined = play(rung, [["test-2", "decline", { id: 1 }, 2000]]);
  expect(K.call(declined, { verb: "phone.end", args: { id: 1 }, who: as("test-1"), at: 2000 }).envelope.last.note).toBe(
    "call 1 is already declined",
  );
});

test(`each log keeps its last ${LOG} calls; a call neither log holds is dropped`, () => {
  const many = Array.from({ length: LOG + 3 }, (_, i): [string, string, Act["args"], number][] => [
    ["test-1", "ring", { to: "test-2" }, 1000 + i],
    ["test-1", "end", { id: i + 1 }, 1000 + i],
  ]).flat();
  const full = state(play(K.boot(CONFIG), many));
  expect([full.logs["test-1"]!.length, full.logs["test-1"]![0], Object.keys(full.calls).length]).toEqual([LOG, 4, LOG]);
});

test("the scripted test users ring, answer and hang up, with no refusal", () => {
  const run = scripted([calls, inbox], { ...CONFIG, apps: ["phone", "notifications"] }, 150);
  const counts = ["phone.ring", "phone.accept", "phone.decline", "phone.end"].map((verb) => run.verbs[verb]! > 5);
  expect([run.refused, counts]).toEqual([{}, [true, true, true, true]]);
});

test("10,000 seeded worlds of calls played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
