import { expect, test } from "bun:test";
import type { Act, Obj, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { BLOCKS } from "../../sdk/blocks.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { Docs } from "./app.ts";
import docs, { DOCS, TITLE } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "docs", apps: ["docs"], actors: ACTORS };

const K = kernel([docs]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.docs as Docs;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `docs.${verb}`, args, who: as(id) }).world, world);

const TREE: Obj[] = [
  { kind: "h1", text: "Soup" },
  { kind: "li", text: "lentils", kids: [{ kind: "li", text: "red" }] },
];

const made = play(K.boot(CONFIG), [
  ["test-1", "create", { title: "Soup" }],
  ["test-1", "write", { doc: "doc-1", blocks: TREE }],
  ["test-1", "share", { doc: "doc-1", with: "test-2" }],
]);

test("a create, write, edit, share or delete out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["create", { title: "" }],
    ["create", { title: "x".repeat(TITLE + 1) }],
    ["write", { doc: "doc-9", blocks: [] }],
    ["write", { doc: "doc-1", blocks: [{ kind: "table", text: "" }] }],
    ["edit", { doc: "doc-1", at: [], text: "x" }],
    ["share", { doc: "doc-1", with: "test-9" }],
    ["delete", { doc: "doc-1", owner: "test-1" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(made, { verb: `docs.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("an editor writes but never shares or deletes, a stranger does nothing, and a watcher may not act", () => {
  const edit = K.call(made, {
    verb: "docs.edit",
    args: { doc: "doc-1", at: [1, 0], text: "green" },
    who: as("test-2"),
  });
  const refused = [
    K.call(made, { verb: "docs.share", args: { doc: "doc-1", with: "test-3" }, who: as("test-2") }),
    K.call(made, { verb: "docs.delete", args: { doc: "doc-1" }, who: as("test-2") }),
    K.call(made, { verb: "docs.write", args: { doc: "doc-1", blocks: [] }, who: as("test-3") }),
    K.call(made, { verb: "docs.edit", args: { doc: "doc-1", at: [5], text: "x" }, who: as("test-1") }),
  ];
  expect(state(edit.world).docs["doc-1"]!.blocks[1]!.kids![0]!.text).toBe("green");
  expect(refused.map((one) => one.envelope.last.note)).toEqual([
    "doc-1 is test-1's; only its owner may",
    "doc-1 is test-1's; only its owner may",
    "doc-1 is test-1's; ask to be an editor",
    "doc-1 has no block at 5",
  ]);
  expect(refused.map((one) => digest(one.world.apps))).toEqual(refused.map(() => digest(made.apps)));
  expect(() => K.call(made, { verb: "docs.create", args: { title: "x" }, who: as("guest") })).toThrow("refused");
});

test("a slice holds the documents a user owns or edits; a tree over the block cap is refused", () => {
  expect(Object.keys((K.read(made, "docs/@test-2").state as Docs).docs)).toEqual(["doc-1"]);
  expect(K.read(made, "docs/@test-3/docs").state).toEqual({});
  const many = Array.from({ length: BLOCKS / 2 + 1 }, () => ({ kind: "p", text: "", kids: [{ kind: "p", text: "" }] }));
  const over = K.call(made, { verb: "docs.write", args: { doc: "doc-1", blocks: many }, who: as("test-1") });
  expect(over.envelope.last.note).toBe(`a tree holds at most ${BLOCKS} blocks`);
});

test(`a user holds at most ${DOCS} documents`, () => {
  const many = Array.from({ length: DOCS }, (): [string, string, Act["args"]] => ["test-1", "create", { title: "d" }]);
  const full = K.call(play(K.boot(CONFIG), many), {
    verb: "docs.create",
    args: { title: "one more" },
    who: as("test-1"),
  });
  expect(full.envelope.last.note).toBe(`you hold ${DOCS} documents already; delete one first`);
});

test("the scripted test users create, write, edit, share and delete documents with no refusal", () => {
  const run = scripted([docs], CONFIG, 150);
  const counts = ["docs.create", "docs.write", "docs.edit", "docs.share"].map((verb) => run.verbs[verb]! > 5);
  expect([run.refused, counts]).toEqual([{}, [true, true, true, true]]);
});

test("10,000 seeded worlds of docs played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
