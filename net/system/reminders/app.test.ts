import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import inbox from "../notifications/app.ts";
import type { Todo } from "./app.ts";
import todo, { ITEMS, LISTS, TEXT } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "reminders", apps: ["reminders"], actors: ACTORS };

const K = kernel([todo]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.reminders as Todo;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `reminders.${verb}`, args, who: as(id) }).world, world);

const made = play(K.boot(CONFIG), [
  ["test-1", "make", { title: "Groceries" }],
  ["test-1", "add", { list: 1, text: "Milk" }],
]);

test("a make, add, tick or share out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["make", { title: "" }],
    ["add", { list: 1, text: "x".repeat(TEXT + 1) }],
    ["add", { list: 3, text: "Eggs" }],
    ["tick", { item: 2 }],
    ["share", { list: 1, with: "test-9" }],
    ["delete", { list: 1, force: true }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(made, { verb: `reminders.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("only a list's owner shares or deletes it, only its people touch its items, and a watcher may not act", () => {
  const notes = [
    K.call(made, { verb: "reminders.tick", args: { item: 2, done: true }, who: as("test-2") }),
    K.call(made, { verb: "reminders.share", args: { list: 1, with: "test-3" }, who: as("test-2") }),
    K.call(made, { verb: "reminders.unshare", args: { list: 1, with: "test-3" }, who: as("test-2") }),
  ];
  expect(notes.map((one) => one.envelope.last.note)).toEqual([
    "item 2 is on test-1's list 1; ask them to share it",
    "list 1 is test-1's; only its owner shares or deletes it",
    "list 1 is test-1's; you may only leave it yourself",
  ]);
  expect(notes.map((one) => digest(one.world.apps))).toEqual(notes.map(() => digest(made.apps)));
  expect(() => K.call(made, { verb: "reminders.make", args: { title: "x" }, who: as("guest") })).toThrow("refused");
});

test("a shared list is the other's to tick and add to until they leave it or the owner deletes it", () => {
  const shared = play(made, [
    ["test-1", "share", { list: 1, with: "test-2" }],
    ["test-2", "tick", { item: 2, done: true }],
    ["test-2", "add", { list: 1, text: "Eggs" }],
  ]);
  const lists = (world: World, id: string) =>
    (K.read(world, `reminders/@${id}`).state as { lists: { open: number }[] }).lists;
  expect(state(shared).lists["1"]!.items.map((item) => [item.text, item.done, item.by])).toEqual([
    ["Milk", true, "test-1"],
    ["Eggs", false, "test-2"],
  ]);
  expect([
    lists(shared, "test-2").map((one) => one.open),
    lists(play(shared, [["test-2", "unshare", { list: 1, with: "test-2" }]]), "test-2"),
  ]).toEqual([[1], []]);
  expect(Object.keys(state(play(shared, [["test-1", "delete", { list: 1 }]])).lists)).toEqual([]);
});

test(`an owner keeps at most ${LISTS} lists and a list at most ${ITEMS} items`, () => {
  const lists = play(
    K.boot(CONFIG),
    Array.from({ length: LISTS }, (_, i) => ["test-1", "make", { title: `l${i}` }]),
  );
  const items = play(
    made,
    Array.from({ length: ITEMS - 1 }, () => ["test-1", "add", { list: 1, text: "x" }]),
  );
  expect([
    K.call(lists, { verb: "reminders.make", args: { title: "one more" }, who: as("test-1") }).envelope.last.note,
    K.call(items, { verb: "reminders.add", args: { list: 1, text: "one more" }, who: as("test-1") }).envelope.last.note,
  ]).toEqual([`you own ${LISTS} lists; delete one first`, `list 1 holds ${ITEMS} items; remove one first`]);
});

test("the scripted test users make lists, add and tick items and share lists, with no refusal", () => {
  const run = scripted([todo, inbox], { ...CONFIG, apps: ["reminders", "notifications"] }, 150);
  const counts = ["reminders.make", "reminders.add", "reminders.tick", "reminders.share"].map(
    (verb) => run.verbs[verb]! > 5,
  );
  expect([run.refused, counts]).toEqual([{}, [true, true, true, true]]);
});

test("10,000 seeded worlds of todo played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
