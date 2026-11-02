import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { Papers } from "./app.ts";
import papers, { REFERENCES, SECTIONS, TITLE } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "papers", apps: ["papers"], actors: ACTORS };

const K = kernel([papers]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.papers as Papers;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `papers.${verb}`, args, who: as(id) }).world, world);

const drafted = play(K.boot(CONFIG), [
  ["test-1", "create", { title: "On lentils" }],
  ["test-1", "write", { paper: "paper-1", sections: [{ heading: "Abstract", text: "Soup." }] }],
  ["test-1", "cite", { paper: "paper-1", title: "The Soup Book", by: "A. Cook" }],
]);

test("a create, write, cite or publish out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["create", { title: "x".repeat(TITLE + 1) }],
    ["write", { paper: "paper-9", sections: [] }],
    ["write", { paper: "paper-1", sections: [{ heading: "", text: "x" }] }],
    ["write", { paper: "paper-1", sections: Array.from({ length: SECTIONS + 1 }, () => ({ heading: "h", text: "" })) }],
    ["cite", { paper: "paper-1", title: "x" }],
    ["publish", { paper: "paper-1", status: "published" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(drafted, { verb: `papers.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("only the author writes, cites or publishes, a published paper is frozen, and a watcher may not act", () => {
  const published = play(drafted, [["test-1", "publish", { paper: "paper-1" }]]);
  const refused = [
    K.call(drafted, { verb: "papers.write", args: { paper: "paper-1", sections: [] }, who: as("test-2") }),
    K.call(drafted, { verb: "papers.publish", args: { paper: "paper-1" }, who: as("test-2") }),
    K.call(published, { verb: "papers.cite", args: { paper: "paper-1", title: "x", by: "y" }, who: as("test-1") }),
    K.call(drafted, {
      verb: "papers.cite",
      args: { paper: "paper-1", title: "The Soup Book", by: "A. Cook" },
      who: as("test-1"),
    }),
  ];
  expect(state(published).papers["paper-1"]!.status).toBe("published");
  expect(refused.map((one) => one.envelope.last.note)).toEqual([
    "paper-1 is test-1's; only its author may",
    "paper-1 is test-1's; only its author may",
    "paper-1 is published and frozen",
    "paper-1 cites The Soup Book already",
  ]);
  expect(refused.map((one, i) => digest(one.world.apps) === digest((i === 2 ? published : drafted).apps))).toEqual([
    true,
    true,
    true,
    true,
  ]);
  expect(() => K.call(drafted, { verb: "papers.create", args: { title: "x" }, who: as("guest") })).toThrow("refused");
});

test(`an empty draft cannot be published, a paper cites at most ${REFERENCES} works, and a slice holds the author's papers`, () => {
  const empty = play(K.boot(CONFIG), [["test-2", "create", { title: "Blank" }]]);
  const bare = K.call(empty, { verb: "papers.publish", args: { paper: "paper-1" }, who: as("test-2") });
  const many = Array.from({ length: REFERENCES - 1 }, (_, i): [string, string, Act["args"]] => [
    "test-1",
    "cite",
    { paper: "paper-1", title: `w${i}`, by: "x" },
  ]);
  const full = K.call(play(drafted, many), {
    verb: "papers.cite",
    args: { paper: "paper-1", title: "more", by: "x" },
    who: as("test-1"),
  });
  expect([bare.envelope.last.note, full.envelope.last.note]).toEqual([
    "paper-1 has no section yet; write one first",
    `paper-1 cites ${REFERENCES} works already`,
  ]);
  expect(Object.keys((K.read(drafted, "papers/@test-1").state as Papers).papers)).toEqual(["paper-1"]);
  expect(K.read(drafted, "papers/@test-2/papers").state).toEqual({});
});

test("the scripted test users draft, write, cite and publish papers with no refusal", () => {
  const run = scripted([papers], CONFIG, 150);
  const counts = ["papers.create", "papers.write", "papers.cite", "papers.publish"].map((verb) => run.verbs[verb]! > 5);
  expect([run.refused, counts]).toEqual([{}, [true, true, true, true]]);
});

test("10,000 seeded worlds of papers played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
