import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese } from "../../test/gym.ts";
import type { Books } from "./app.ts";
import books, { CHAPTERS, OWN, TEXT, TITLE } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "books", apps: ["books"], actors: ACTORS };

const K = kernel([books]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.books as Books;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `books.${verb}`, args, who: as(id) }).world, world);

const CHAPTER = { book: 1, title: "Morning", text: "The day began early." };

test("a write, a chapter, a shelve or a read out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [["test-1", "write", { title: "The Well" }]]);
  const bad: [string, Act["args"]][] = [
    ["write", { title: "x".repeat(TITLE + 1) }],
    ["write", { title: "Two", by: "test-2" }],
    ["chapter", { ...CHAPTER, text: "x".repeat(TEXT + 1) }],
    ["chapter", { book: 2, title: "Morning", text: "a" }],
    ["chapter", { book: 1, text: "a" }],
    ["shelve", {}],
    ["read", { book: 1, chapter: 0 }],
    ["read", { book: 1, chapter: CHAPTERS + 1 }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `books.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("only the author adds chapters, a shelf is its owner's, a missing chapter is refused with a plain note, and a watcher may not write", () => {
  const world = play(K.boot(CONFIG), [
    ["test-1", "write", { title: "The Well" }],
    ["test-1", "chapter", CHAPTER],
    ["test-2", "shelve", { book: 1 }],
  ]);
  const notes = [
    ["test-2", "chapter", { ...CHAPTER, title: "Mine" }],
    ["test-2", "shelve", { book: 1 }],
    ["test-3", "unshelve", { book: 1 }],
    ["test-3", "read", { book: 1, chapter: 2 }],
  ].map(([id, verb, args]) => {
    const done = K.call(world, { verb: `books.${verb}`, args: args as Act["args"], who: as(id as string) });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });
  expect(notes).toEqual([
    "book 1 is test-1's; only its author adds chapters",
    "book 1 is on your shelf already",
    "book 1 is not on your shelf",
    "The Well has 1 chapters, no chapter 2",
  ]);
  expect(() => K.call(world, { verb: "books.write", args: { title: "x" }, who: as("guest") })).toThrow("refused");
});

test(`chapters number from 1, a read moves your place and shelves the book, an unshelve forgets it, and one person writes ${OWN}`, () => {
  const world = play(K.boot(CONFIG), [
    ["test-1", "write", { title: "The Well" }],
    ["test-1", "chapter", CHAPTER],
    ["test-1", "chapter", { ...CHAPTER, title: "Rain" }],
    ["test-2", "read", { book: 1, chapter: 2 }],
    ["test-3", "shelve", { book: 1 }],
  ]);
  const one = state(world).books[1]!;
  expect([one.chapters.map((chapter) => [chapter.n, chapter.title]), state(world).shelves]).toEqual([
    [
      [1, "Morning"],
      [2, "Rain"],
    ],
    { "test-2": { 1: 2 }, "test-3": { 1: 0 } },
  ]);
  expect(K.read(world, "books/@test-2").state).toEqual({
    wrote: [],
    shelf: [{ id: 1, title: "The Well", by: "test-1", chapters: 2, at: 2 }],
  });
  expect(K.read(world, "books/@test-1/wrote").state).toEqual([{ id: 1, title: "The Well", chapters: 2 }]);
  expect(state(play(world, [["test-2", "unshelve", { book: 1 }]])).shelves["test-2"]).toEqual({});
  const full = play(
    K.boot(CONFIG),
    Array.from({ length: OWN }, (_, i): [string, string, Act["args"]] => ["test-4", "write", { title: `Book ${i}` }]),
  );
  expect(K.call(full, { verb: "books.write", args: { title: "One more" }, who: as("test-4") }).envelope.last.note).toBe(
    `you wrote ${OWN} books, the most one person may`,
  );
});

test("10,000 seeded worlds of books played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
