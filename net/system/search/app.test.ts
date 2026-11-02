import { expect, test } from "bun:test";
import type { Act, Obj, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import docs from "../docs/app.ts";
import chat from "../messages/app.ts";
import type { Found } from "./app.ts";
import search, { find, HITS, leaves, QUERY, words } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "search", apps: ["messages", "docs", "search"], actors: ACTORS };

const K = kernel([chat, docs, search]);

const as = (id: string): Who => ({ id, kind: "human" });

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb, args, who: as(id) }).world, world);

const filled = play(K.boot(CONFIG), [
  ["test-1", "messages.send", { to: "test-2", text: "Lentil soup tonight?" }],
  ["test-2", "docs.create", { title: "Soup" }],
  [
    "test-2",
    "docs.write",
    { doc: "doc-1", blocks: [{ kind: "li", text: "red lentils", kids: [{ kind: "li", text: "onion" }] }] },
  ],
]);

const found = (world: World, id: string) => K.read(world, `search/@${id}`).state as Found;

test("a find out of its schema is a bad act, and one of no word is refused", () => {
  const bad: Obj[] = [{}, { words: "" }, { words: "x".repeat(QUERY + 1) }, { words: "soup", app: "messages" }];
  for (const args of bad)
    expect(() => K.call(filled, { verb: "search.find", args, who: as("test-1") })).toThrow("bad-act");
  const bare = K.call(filled, { verb: "search.find", args: { words: "a - !" }, who: as("test-1") });
  expect([bare.envelope.last.note, digest(bare.world.apps) === digest(filled.apps)]).toEqual([
    "say a word of two letters or digits at least",
    true,
  ]);
  expect(() => K.call(filled, { verb: "search.find", args: { words: "soup" }, who: as("guest") })).toThrow("refused");
});

test("a search is the caller's own: it keeps their last words and hits and touches nobody else's", () => {
  const two = play(filled, [
    ["test-1", "search.find", { words: "lentil" }],
    ["test-2", "search.find", { words: "onion" }],
  ]);
  expect([
    found(two, "test-1").words,
    Object.keys(found(two, "test-1").hits),
    Object.keys(found(two, "test-2").hits),
  ]).toEqual([["lentil"], ["docs", "messages"], ["docs"]]);
  expect(found(two, "test-3")).toEqual({ words: [], hits: {}, at: 0 });
});

test("the index walks every string of every app but search, matches all the words in a path or a text, and caps hits per app", () => {
  const world = {
    a: { list: ["Lentil soup", "bread"], deep: { note: "lentils and rice" } },
    b: ["Soup"],
    search: { last: {} },
  };
  expect(leaves(world.a, "a").map((one) => one.path)).toEqual(["a/deep/note", "a/list/0", "a/list/1"]);
  expect(find(world, ["lentil", "soup"], ["search"])).toEqual({ a: [{ path: "a/list/0", text: "Lentil soup" }] });
  expect(find(world, ["note"], ["search"])).toEqual({ a: [{ path: "a/deep/note", text: "lentils and rice" }] });
  const many = { c: Array.from({ length: HITS + 5 }, () => "soup") };
  expect([find(many, ["soup"]).c!.length, words("Soup, SOUP; red-lentils 2x!")]).toEqual([
    HITS,
    ["soup", "red", "lentils", "2x"],
  ]);
});

test("the scripted test users search while others chat and write, with no refusal", () => {
  const run = scripted([chat, docs, search], CONFIG, 120);
  expect([run.refused, run.verbs["search.find"]! > 5]).toEqual([{}, true]);
});

test("10,000 seeded worlds of chat, docs and search played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts]).toEqual([10_000, 60_000]);
}, 30_000);
