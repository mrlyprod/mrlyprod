import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import inbox from "../notifications/app.ts";
import type { Clock } from "./app.ts";
import clock, { DAY, LABEL, SECONDS, SET } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "clock", apps: ["clock"], actors: ACTORS };

const K = kernel([clock]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.clock as Clock;

const play = (world: World, acts: [string, string, Act["args"], number][]) =>
  acts.reduce(
    (then, [id, verb, args, at]) => K.call(then, { verb: `clock.${verb}`, args, who: as(id), at }).world,
    world,
  );

const set = play(K.boot(CONFIG), [
  ["test-1", "alarm", { at: 7 * 3_600_000, label: "Wake up", daily: true }, 1000],
  ["test-1", "timer", { seconds: 10, label: "Tea" }, 1000],
]);

test("an alarm, timer, cancel or tick out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["alarm", { label: "x" }],
    ["alarm", { at: -1 }],
    ["alarm", { at: 5000, label: "x".repeat(LABEL + 1) }],
    ["timer", { seconds: 0 }],
    ["timer", { seconds: SECONDS + 1 }],
    ["cancel", { id: 3 }],
    ["tick", { now: 1 }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(set, { verb: `clock.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("only an alarm's owner cancels it, an alarm rings in the future, and a watcher may not act", () => {
  const notes = [
    K.call(set, { verb: "clock.cancel", args: { id: 1 }, who: as("test-2") }),
    K.call(set, { verb: "clock.alarm", args: { at: 500 }, who: as("test-2") }),
  ];
  expect(notes.map((one) => one.envelope.last.note)).toEqual([
    "alarm or timer 1 is test-1's",
    "an alarm rings in the future; the world's time is 1000",
  ]);
  expect(notes.map((one) => digest(one.world.apps))).toEqual(notes.map(() => digest(set.apps)));
  expect(() => K.call(set, { verb: "clock.tick", args: {}, who: as("guest") })).toThrow("refused");
});

test("the next act by anyone rings what is due: a timer once, a daily alarm again the next day, each with a notice", () => {
  const tea = K.call(set, { verb: "clock.tick", args: {}, who: as("test-2"), at: 11_000 });
  const wake = K.call(tea.world, { verb: "clock.tick", args: {}, who: as("test-3"), at: DAY + 8 * 3_600_000 });
  const dial = state(wake.world).dials["test-1"]!;
  expect(tea.envelope.wishes.map((wish) => wish.data)).toEqual([{ to: "test-1", text: "Timer done: Tea" }]);
  expect([dial.timers, dial.alarms.map((one) => one.at), dial.rung.map((one) => [one.kind, one.at])]).toEqual([
    [],
    [2 * DAY + 7 * 3_600_000],
    [
      ["timer", 11_000],
      ["alarm", 7 * 3_600_000],
    ],
  ]);
  expect(state(play(wake.world, [["test-1", "clear", {}, DAY + 9 * 3_600_000]])).dials["test-1"]!.rung).toEqual([]);
});

test(`a user keeps at most ${SET} alarms and runs at most ${SET} timers`, () => {
  const many = Array.from({ length: SET }, (_, i): [string, string, Act["args"], number] => [
    "test-1",
    "timer",
    { seconds: 100 + i },
    1000,
  ]);
  const full = play(set, many.slice(1));
  expect(
    K.call(full, { verb: "clock.timer", args: { seconds: 60 }, who: as("test-1"), at: 1000 }).envelope.last.note,
  ).toBe(`you run ${SET} timers; cancel one first`);
});

test("the scripted test users set and cancel alarms and timers that ring into their inboxes, with no refusal", () => {
  const run = scripted([clock, inbox], { ...CONFIG, apps: ["clock", "notifications"] }, 150);
  const counts = ["clock.timer", "clock.alarm", "clock.cancel", "notifications.notice"].map(
    (verb) => run.verbs[verb]! > 5,
  );
  expect([run.refused, counts]).toEqual([{}, [true, true, true, true]]);
});

test("10,000 seeded worlds of clock played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
