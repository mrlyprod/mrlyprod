import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese } from "../../test/gym.ts";
import people from "../people/app.ts";
import type { Archive } from "./app.ts";
import archive, { KEEP, PATH, SIZE } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "archive", apps: ["people", "archive"], actors: ACTORS };

const K = kernel([people, archive]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.archive as Archive;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `archive.${verb}`, args, who: as(id) }).world, world);

test("a save or a delete out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [["test-1", "save", { path: "people/test-2" }]]);
  const bad: [string, Act["args"]][] = [
    ["save", { path: "" }],
    ["save", { path: "x".repeat(PATH + 1) }],
    ["save", { path: "people/test-2", page: { name: "Eve" } }],
    ["save", {}],
    ["delete", { id: 2 }],
    ["delete", {}],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `archive.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("only the saver deletes a page; a path that answers nothing, the archive itself or a blank path is refused with a plain note; a watcher may not save", () => {
  const world = play(K.boot(CONFIG), [["test-1", "save", { path: "people/test-2" }]]);
  const notes = [
    ["test-2", "delete", { id: 1 }],
    ["test-1", "save", { path: "///" }],
    ["test-1", "save", { path: "people/test-9" }],
    ["test-1", "save", { path: "archive/pages" }],
  ].map(([id, verb, args]) => {
    const done = K.call(world, { verb: `archive.${verb}`, args: args as Act["args"], who: as(id as string) });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });
  expect(notes).toEqual([
    "page 1 is test-1's; only they delete it",
    "a path names an app, then a key inside it: people/test-1",
    "nothing answers at people/test-9",
    "the archive does not save itself",
  ]);
  expect(() => K.call(world, { verb: "archive.save", args: { path: "people" }, who: as("guest") })).toThrow("refused");
});

test(`a saved page is what the path answered at that tick, kept after the source changes, and one person keeps ${KEEP}`, () => {
  const saved = play(K.boot(CONFIG), [["test-1", "save", { path: "/people/test-2/" }]]);
  const was = K.read(saved, "people/test-2").state;
  const renamed = K.call(saved, { verb: "people.rename", args: { name: "Ada" }, who: as("test-2") }).world;
  const one = state(renamed).pages[1]!;
  expect([one.path, one.tick, one.by, one.page]).toEqual(["people/test-2", 1, "test-1", was]);
  expect(K.read(renamed, "people/test-2/name").state).toBe("Ada");
  expect(K.read(renamed, "archive/@test-1/pages", ["id", "path"]).state).toEqual([{ id: 1, path: "people/test-2" }]);
  expect(Object.keys(state(play(renamed, [["test-1", "delete", { id: 1 }]])).pages)).toEqual([]);
  const full = play(
    K.boot(CONFIG),
    Array.from({ length: KEEP }, (): [string, string, Act["args"]] => ["test-3", "save", { path: "people" }]),
  );
  expect(K.call(full, { verb: "archive.save", args: { path: "people" }, who: as("test-3") }).envelope.last.note).toBe(
    `you keep ${KEEP} pages; delete one first`,
  );
  expect(SIZE).toBeGreaterThan(JSON.stringify(K.read(full, "people").state).length);
});

test("10,000 seeded worlds of archive played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
