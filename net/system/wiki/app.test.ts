import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese } from "../../test/gym.ts";
import type { Wiki } from "./app.ts";
import wiki, { REVISIONS, STUBS, TEXT, TITLE } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "wiki", apps: ["wiki"], actors: ACTORS };

const K = kernel([wiki]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.wiki as Wiki;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `wiki.${verb}`, args, who: as(id) }).world, world);

test(`a world boots with ${STUBS} stub articles drawn from its seed, each one revision by nobody`, () => {
  const one = state(K.boot({ ...CONFIG, seed: 1 }));
  const two = state(K.boot({ ...CONFIG, seed: 2 }));
  const stubs = Object.values(one.articles);
  expect([stubs.length, stubs.every((art) => art.n === 1 && art.revisions[0]!.by === null)]).toEqual([STUBS, true]);
  expect(Object.keys(one.articles)).not.toEqual(Object.keys(two.articles));
  expect(digest(K.boot({ ...CONFIG, seed: 1 }))).toBe(digest(K.boot({ ...CONFIG, seed: 1 })));
});

test("a write, an edit or a talk out of its schema is a bad act and changes nothing", () => {
  const world = K.boot(CONFIG);
  const slug = Object.keys(state(world).articles)[0]!;
  const bad: [string, Act["args"]][] = [
    ["write", { title: "x".repeat(TITLE + 1), text: "a" }],
    ["write", { title: "Salt", text: "x".repeat(TEXT + 1) }],
    ["write", { title: "Salt" }],
    ["write", { title: "Salt", text: "a", slug: "salt" }],
    ["edit", { slug: "nowhere", text: "a" }],
    ["talk", { slug, text: "" }],
    ["watch", { slug: "nowhere" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `wiki.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("a second write of a slug, an unchanged edit and an unwatch of an unwatched article are refused with plain notes, and a watcher may not write", () => {
  const world = K.boot(CONFIG);
  const slug = Object.keys(state(world).articles)[0]!;
  const notes = [
    ["write", { title: state(world).articles[slug]!.title, text: "again" }],
    ["edit", { slug, text: state(world).articles[slug]!.text }],
    ["unwatch", { slug }],
    ["write", { title: "???", text: "a" }],
  ].map(([verb, args]) => {
    const done = K.call(world, { verb: `wiki.${verb}`, args: args as Act["args"], who: as("test-1") });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });
  expect(notes).toEqual([
    `there is an article ${slug}; edit it`,
    `${slug} already reads so; change something`,
    `you do not watch ${slug}`,
    "the title needs a letter or a digit",
  ]);
  expect(() => K.call(world, { verb: "wiki.write", args: { title: "Salt", text: "a" }, who: as("guest") })).toThrow(
    "refused",
  );
});

test(`an article keeps its last ${REVISIONS} revisions, counts every one, and names who wrote each; talk and watching are per person`, () => {
  const written = play(K.boot(CONFIG), [["test-1", "write", { title: "Sea Salt!", text: "Salt from the sea." }]]);
  const edits = Array.from({ length: REVISIONS + 2 }, (_, i): [string, string, Act["args"]] => [
    `test-${(i % 3) + 1}`,
    "edit",
    { slug: "sea-salt", text: `Salt from the sea, take ${i}.` },
  ]);
  const world = play(written, [
    ...edits,
    ["test-2", "talk", { slug: "sea-salt", text: "Needs a source." }],
    ["test-2", "watch", { slug: "sea-salt" }],
  ]);
  const one = state(world).articles["sea-salt"]!;
  expect([one.n, one.revisions.length, one.revisions[0]!.n, one.revisions.at(-1)!.by, one.talk[0]!.by]).toEqual([
    REVISIONS + 3,
    REVISIONS,
    4,
    "test-3",
    "test-2",
  ]);
  expect(K.read(world, "wiki/@test-2").state).toEqual({
    watching: [{ slug: "sea-salt", title: "Sea Salt!", n: REVISIONS + 3 }],
    edited: ["sea-salt"],
  });
});

test("10,000 seeded worlds of wiki played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
