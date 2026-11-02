import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { Files } from "./app.ts";
import files, { ENTRIES, PATH, TEXT } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "files", apps: ["files"], actors: ACTORS };

const K = kernel([files]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.files as Files;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `files.${verb}`, args, who: as(id) }).world, world);

const stored = play(K.boot(CONFIG), [
  ["test-1", "folder", { path: "/notes" }],
  ["test-1", "write", { path: "/notes/todo.txt", text: "milk" }],
]);

test("a write, folder, move or delete out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["write", { path: "/a.txt" }],
    ["write", { path: "/a.txt", text: "x".repeat(TEXT + 1) }],
    ["write", { path: `/${"a".repeat(PATH)}`, text: "x" }],
    ["folder", { path: "" }],
    ["move", { from: "/notes" }],
    ["delete", { path: "/notes", user: "test-2" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(stored, { verb: `files.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("a path names the caller's own store and never leaves it, and a watcher may not act", () => {
  const notes = [
    K.call(stored, { verb: "files.write", args: { path: "/../test-1/notes/todo.txt", text: "x" }, who: as("test-2") }),
    K.call(stored, { verb: "files.delete", args: { path: "/notes/todo.txt" }, who: as("test-2") }),
    K.call(stored, { verb: "files.write", args: { path: "notes/a.txt", text: "x" }, who: as("test-2") }),
    K.call(stored, { verb: "files.write", args: { path: "/work/a.txt", text: "x" }, who: as("test-1") }),
  ];
  expect(notes.map((one) => one.envelope.last.note)).toEqual([
    "a path stays inside your store; . and .. are not names",
    "no /notes/todo.txt in your store",
    "a path starts with / and names something in your store, like /notes/todo.txt, not notes/a.txt",
    "no folder /work; make it first",
  ]);
  expect(notes.map((one) => digest(one.world.apps))).toEqual(notes.map(() => digest(stored.apps)));
  expect(() => K.call(stored, { verb: "files.folder", args: { path: "/a" }, who: as("guest") })).toThrow("refused");
});

test("a store reads as a tree; a move carries a folder's contents and a delete removes them", () => {
  const moved = play(stored, [
    ["test-1", "folder", { path: "/notes/old" }],
    ["test-1", "write", { path: "/notes/old/a.txt", text: "a" }],
    ["test-1", "move", { from: "/notes", to: "/archive" }],
  ]);
  const inside = K.call(moved, { verb: "files.move", args: { from: "/archive", to: "/archive/x" }, who: as("test-1") });
  expect(K.read(moved, "files/@test-1").state).toEqual({ archive: { old: { "a.txt": "a" }, "todo.txt": "milk" } });
  expect([K.read(moved, "files/@test-1/archive/todo.txt").state, inside.envelope.last.note]).toEqual([
    "milk",
    "/archive cannot move inside itself",
  ]);
  expect(K.read(play(moved, [["test-1", "delete", { path: "/archive/old" }]]), "files/@test-1").state).toEqual({
    archive: { "todo.txt": "milk" },
  });
});

test(`a store holds at most ${ENTRIES} files and folders`, () => {
  const many = Array.from({ length: ENTRIES }, (_, i): [string, string, Act["args"]] => [
    "test-1",
    "write",
    { path: `/f${i}`, text: "" },
  ]);
  const full = play(K.boot(CONFIG), many);
  const over = K.call(full, { verb: "files.folder", args: { path: "/more" }, who: as("test-1") });
  const replaced = K.call(full, { verb: "files.write", args: { path: "/f0", text: "new" }, who: as("test-1") });
  expect([over.envelope.last.note, state(replaced.world).stores["test-1"]!["/f0"]!.text]).toEqual([
    `your store holds at most ${ENTRIES} files and folders`,
    "new",
  ]);
});

test("the scripted test users make folders and write, edit, move and delete files, with no refusal", () => {
  const run = scripted([files], CONFIG, 150);
  const counts = ["files.folder", "files.write", "files.move", "files.delete"].map((verb) => run.verbs[verb]! > 5);
  expect([run.refused, counts]).toEqual([{}, [true, true, true, true]]);
});

test("10,000 seeded worlds of files played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
