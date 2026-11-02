import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { Notes } from "./app.ts";
import notes, { KEEP, TEXT } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "notes", apps: ["notes"], actors: ACTORS };

const K = kernel([notes]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.notes as Notes;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `notes.${verb}`, args, who: as(id) }).world, world);

const written = play(K.boot(CONFIG), [
  ["test-1", "write", { text: "Groceries" }],
  ["test-1", "write", { text: "Diary", private: true }],
]);

test("a write, edit, mark or delete out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["write", { text: "" }],
    ["write", { text: "x".repeat(TEXT + 1) }],
    ["write", { text: "a", private: "yes" }],
    ["edit", { id: 3, text: "b" }],
    ["mark", { id: 1 }],
    ["delete", { id: 1, owner: "test-1" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(written, { verb: `notes.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("only a note's owner edits, marks or deletes it, and a watcher may not write", () => {
  const tries: [string, Act["args"]][] = [
    ["edit", { id: 1, text: "mine" }],
    ["mark", { id: 1, private: true }],
    ["delete", { id: 1 }],
  ];
  const notes = tries.map(([verb, args]) => K.call(written, { verb: `notes.${verb}`, args, who: as("test-2") }));
  expect(notes.map((one) => one.envelope.last.note)).toEqual([
    "note 1 is test-1's",
    "note 1 is test-1's",
    "note 1 is test-1's",
  ]);
  expect(notes.map((one) => digest(one.world.apps))).toEqual(notes.map(() => digest(written.apps)));
  expect(() => K.call(written, { verb: "notes.write", args: { text: "x" }, who: as("guest") })).toThrow("refused");
});

test("a note is public by default; a private note shows only to its owner, in every read", () => {
  const texts = (value: unknown) => (value as { text: string }[]).map((note) => note.text);
  const slice = (world: World, who: Who | null) =>
    texts((K.read(world, "notes/@test-1", undefined, who).state as { notes: unknown }).notes);
  const marked = play(written, [["test-1", "mark", { id: 2, private: false }]]);
  expect([
    texts(K.read(written, "notes/notes/test-1").state),
    texts(K.read(written, "notes/notes/test-1", undefined, as("test-1")).state),
    slice(written, as("test-2")),
    slice(written, as("test-1")),
    texts(K.read(marked, "notes/notes/test-1", undefined, as("test-2")).state),
  ]).toEqual([["Groceries"], ["Groceries", "Diary"], ["Groceries"], ["Diary", "Groceries"], ["Groceries", "Diary"]]);
});

test(`a user keeps at most ${KEEP} notes; an edit replaces the text and stamps the time`, () => {
  const full = play(
    K.boot(CONFIG),
    Array.from({ length: KEEP }, (): [string, string, Act["args"]] => ["test-1", "write", { text: "n" }]),
  );
  const over = K.call(full, { verb: "notes.write", args: { text: "one more" }, who: as("test-1") });
  const edited = K.call(written, {
    verb: "notes.edit",
    args: { id: 1, text: "Groceries: milk" },
    who: as("test-1"),
    at: 9000,
  });
  expect([over.envelope.last.note, state(edited.world).notes["test-1"]![0]]).toEqual([
    `you keep ${KEEP} notes; delete one first`,
    { id: 1, text: "Groceries: milk", private: false, at: 9000 },
  ]);
});

test("the scripted test users write, edit, mark and delete notes, with no refusal", () => {
  const run = scripted([notes], CONFIG, 150);
  const counts = ["notes.write", "notes.edit", "notes.mark", "notes.delete"].map((verb) => run.verbs[verb]! > 5);
  expect([run.refused, counts]).toEqual([{}, [true, true, true, true]]);
});

test("10,000 seeded worlds of notes played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
