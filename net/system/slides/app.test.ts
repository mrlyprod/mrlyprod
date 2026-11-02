import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { Slides } from "./app.ts";
import slides, { SLIDES, TITLE } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "slides", apps: ["slides"], actors: ACTORS };

const K = kernel([slides]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.slides as Slides;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `slides.${verb}`, args, who: as(id) }).world, world);

const made = play(K.boot(CONFIG), [
  ["test-1", "create", { title: "Soup night" }],
  ["test-1", "add", { deck: "deck-1", title: "Agenda" }],
  ["test-1", "add", { deck: "deck-1", title: "Thanks" }],
  ["test-1", "write", { deck: "deck-1", slide: 0, blocks: [{ kind: "li", text: "lentils" }] }],
  ["test-1", "share", { deck: "deck-1", with: "test-2" }],
]);

test("a create, add, write, drop, share or delete out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["create", { title: "x".repeat(TITLE + 1) }],
    ["add", { deck: "deck-9", title: "x" }],
    ["write", { deck: "deck-1", slide: SLIDES, blocks: [] }],
    ["write", { deck: "deck-1", slide: 0, blocks: [{ kind: "p" }] }],
    ["drop", { deck: "deck-1", slide: -1 }],
    ["share", { deck: "deck-1", with: "test-9" }],
    ["delete", { deck: "deck-1", slide: 0 }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(made, { verb: `slides.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("an editor adds and drops slides but never shares or deletes, a stranger does nothing, and a watcher may not act", () => {
  const dropped = K.call(made, { verb: "slides.drop", args: { deck: "deck-1", slide: 1 }, who: as("test-2") });
  const refused = [
    K.call(made, { verb: "slides.share", args: { deck: "deck-1", with: "test-3" }, who: as("test-2") }),
    K.call(made, { verb: "slides.delete", args: { deck: "deck-1" }, who: as("test-2") }),
    K.call(made, { verb: "slides.add", args: { deck: "deck-1", title: "x" }, who: as("test-3") }),
    K.call(made, { verb: "slides.write", args: { deck: "deck-1", slide: 2, blocks: [] }, who: as("test-1") }),
  ];
  expect(state(dropped.world).decks["deck-1"]!.slides.map((one) => one.title)).toEqual(["Agenda"]);
  expect(refused.map((one) => one.envelope.last.note)).toEqual([
    "deck-1 is test-1's; only its owner may",
    "deck-1 is test-1's; only its owner may",
    "deck-1 is test-1's; ask to be an editor",
    "deck-1 has 2 slides, numbered from 0; no slide 2",
  ]);
  expect(refused.map((one) => digest(one.world.apps))).toEqual(refused.map(() => digest(made.apps)));
  expect(() => K.call(made, { verb: "slides.create", args: { title: "x" }, who: as("guest") })).toThrow("refused");
});

test(`a slice holds the decks a user owns or edits, and a deck holds at most ${SLIDES} slides`, () => {
  expect(K.read(made, "slides/@test-2/decks/deck-1/slides/0/blocks/0/text").state).toBe("lentils");
  expect(K.read(made, "slides/@test-3/decks").state).toEqual({});
  const many = Array.from({ length: SLIDES - 2 }, (): [string, string, Act["args"]] => [
    "test-1",
    "add",
    { deck: "deck-1", title: "s" },
  ]);
  const full = K.call(play(made, many), {
    verb: "slides.add",
    args: { deck: "deck-1", title: "one more" },
    who: as("test-1"),
  });
  expect(full.envelope.last.note).toBe(`deck-1 holds ${SLIDES} slides already; drop one first`);
});

test("the scripted test users create decks, add, write, drop and share slides with no refusal", () => {
  const run = scripted([slides], CONFIG, 150);
  const counts = ["slides.create", "slides.add", "slides.write", "slides.share"].map((verb) => run.verbs[verb]! > 5);
  expect([run.refused, counts]).toEqual([{}, [true, true, true, true]]);
});

test("10,000 seeded worlds of slides played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
