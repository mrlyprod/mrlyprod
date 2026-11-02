import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import inbox from "../notifications/app.ts";
import type { Calendar } from "./app.ts";
import calendar, { KEPT, OWN, TITLE } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "calendar", apps: ["calendar"], actors: ACTORS };

const K = kernel([calendar]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.calendar as Calendar;

const play = (world: World, acts: [string, string, Act["args"], number?][]) =>
  acts.reduce(
    (at, [id, verb, args, when]) => K.call(at, { verb: `calendar.${verb}`, args, who: as(id), at: when }).world,
    world,
  );

const HOUR = 3_600_000;

const lunch = (guests: string[]): Act["args"] => ({ title: "Lunch", start: 12 * HOUR, minutes: 60, guests });

const added = play(K.boot(CONFIG), [["test-1", "add", lunch(["test-2", "test-3"])]]);

test("an add, move or answer out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["add", { title: "Lunch", start: 0 }],
    ["add", { title: "Lunch", start: -1, minutes: 30 }],
    ["add", { title: "Lunch", start: 0, minutes: 0 }],
    ["add", { title: "x".repeat(TITLE + 1), start: 0, minutes: 30 }],
    ["add", { title: "Lunch", start: 0, minutes: 30, guests: ["test-9"] }],
    ["move", { id: 1 }],
    ["accept", { id: 2 }],
    ["decline", {}],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(added, { verb: `calendar.${verb}`, args, who: as("test-2") })).toThrow("bad-act");
});

test("only an event's owner moves or cancels it, only its guests answer, and a watcher may not act", () => {
  const notes = [
    K.call(added, { verb: "calendar.cancel", args: { id: 1 }, who: as("test-2") }),
    K.call(added, { verb: "calendar.accept", args: { id: 1 }, who: as("test-4") }),
    K.call(added, { verb: "calendar.add", args: lunch(["test-5"]), who: as("test-5") }),
  ];
  expect(notes.map((one) => one.envelope.last.note)).toEqual([
    "event 1 is test-1's; only its owner changes it",
    "you are not invited to event 1",
    "you own the event; invite others",
  ]);
  expect(notes.map((one) => digest(one.world.apps))).toEqual(notes.map(() => digest(added.apps)));
  expect(() => K.call(added, { verb: "calendar.accept", args: { id: 1 }, who: as("guest") })).toThrow("refused");
});

test("guests accept or decline once each, a moved event keeps its answers, and a cancelled one leaves every calendar", () => {
  const answered = play(added, [
    ["test-2", "accept", { id: 1 }],
    ["test-3", "decline", { id: 1 }],
    ["test-1", "move", { id: 1, start: 13 * HOUR }],
  ]);
  const twice = K.call(answered, { verb: "calendar.accept", args: { id: 1 }, who: as("test-2") });
  const slice = (world: World, id: string) =>
    K.read(world, `calendar/@${id}`).state as { invites: unknown[]; events: unknown[] };
  expect(state(answered).events["1"]).toMatchObject({
    start: 13 * HOUR,
    guests: { "test-2": "accepted", "test-3": "declined" },
  });
  expect([
    slice(added, "test-2").invites.length,
    slice(answered, "test-2").invites.length,
    twice.envelope.last.note,
  ]).toEqual([1, 0, "you already accepted event 1"]);
  const cancelled = play(answered, [["test-1", "cancel", { id: 1 }]]);
  expect([slice(cancelled, "test-1").events, slice(cancelled, "test-3").events]).toEqual([[], []]);
});

test(`an owner holds at most ${OWN} events, and one ended more than 30 days ago leaves on the next act`, () => {
  const many = Array.from({ length: OWN }, (): [string, string, Act["args"]] => ["test-1", "add", lunch([])]);
  const full = play(K.boot(CONFIG), many);
  const over = K.call(full, { verb: "calendar.add", args: lunch([]), who: as("test-1") });
  const later = play(full, [["test-2", "add", lunch([]), 13 * HOUR + KEPT + 1]]);
  expect([over.envelope.last.note, Object.keys(state(later).events)]).toEqual([
    `you hold ${OWN} events; cancel one first`,
    [`${OWN + 1}`],
  ]);
});

test("the scripted test users add events, invite friends and answer, with no refusal", () => {
  const run = scripted([calendar, inbox], { ...CONFIG, apps: ["calendar", "notifications"] }, 150);
  const counts = ["calendar.add", "calendar.accept", "notifications.notice"].map((verb) => run.verbs[verb]! > 10);
  expect([run.refused, counts]).toEqual([{}, [true, true, true]]);
});

test("10,000 seeded worlds of calendar played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
