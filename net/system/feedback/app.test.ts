import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import notes from "../notes/app.ts";
import type { Feedback } from "./app.ts";
import feedback, { KEEP, TEXT } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "feedback", apps: ["notes", "feedback"], actors: ACTORS };

const K = kernel([notes, feedback]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.feedback as Feedback;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `feedback.${verb}`, args, who: as(id) }).world, world);

const refusals = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.map(([id, verb, args]) => {
    const done = K.call(world, { verb: `feedback.${verb}`, args, who: as(id) });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });

const BUG = { app: "notes", verb: "write", happened: "The note vanished", severity: "bug" };

const sent = play(K.boot(CONFIG), [
  ["test-1", "send", BUG],
  ["test-2", "send", { app: "feedback", happened: "No way to attach a screenshot", severity: "note" }],
]);

test("a send or a same out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["send", { app: "notes", happened: "x" }],
    ["send", { app: "notes", happened: "x", severity: "meh" }],
    ["send", { app: "notes", happened: "", severity: "bug" }],
    ["send", { app: "notes", happened: "x".repeat(TEXT + 1), severity: "bug" }],
    ["send", { app: "", happened: "x", severity: "bug" }],
    ["send", { ...BUG, by: "test-1" }],
    ["same", { id: 0 }],
    ["same", { id: 3 }],
    ["same", { id: "1" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(sent, { verb: `feedback.${verb}`, args, who: as("test-3") })).toThrow("bad-act");
});

test("feedback names an app of this world and a verb it has, nobody seconds their own item or one twice, and a watcher may not send", () => {
  expect(
    refusals(sent, [
      ["test-3", "send", { app: "snake", happened: "x", severity: "bug" }],
      ["test-3", "send", { app: "notes", verb: "fly", happened: "x", severity: "bug" }],
      ["test-3", "send", { app: "notes", happened: "   ", severity: "bug" }],
      ["test-1", "same", { id: 1 }],
    ]),
  ).toEqual([
    "no app snake in this world; one of: feedback, notes",
    "notes has no verb fly; one of: write, edit, mark, delete",
    "happened is only spaces; say what happened",
    "you sent item 1; others say the same",
  ]);
  const seconded = play(sent, [["test-2", "same", { id: 1 }]]);
  expect(refusals(seconded, [["test-2", "same", { id: 1 }]])).toEqual(["you already said the same of item 1"]);
  expect(() => K.call(sent, { verb: "feedback.send", args: BUG, who: as("guest") })).toThrow("refused");
});

test("every item is read newest last, by app and by sender; a user sends at most 100 and the makers keep them all", () => {
  const seconded = play(sent, [["test-3", "same", { id: 1 }]]);
  const ids = (value: unknown) => (value as { id: number }[]).map((one) => one.id);
  expect([
    ids((K.read(seconded, "feedback").state as { items: unknown }).items),
    ids(K.read(seconded, "feedback/notes").state),
    K.read(seconded, "feedback/@test-3").state,
    K.read(seconded, "feedback/@test-1/sent/0/same").state,
    state(seconded).items["1"]!.verb,
    state(seconded).items["2"]!.expected,
  ]).toEqual([[1, 2], [1], { sent: [], same: [1] }, ["test-3"], "write", null]);
  const full = play(
    K.boot(CONFIG),
    Array.from({ length: KEEP }, (): [string, string, Act["args"]] => ["test-1", "send", BUG]),
  );
  expect(refusals(full, [["test-1", "send", BUG]])).toEqual([
    `you sent ${KEEP} items of feedback; the makers have them`,
  ]);
});

test("the scripted test users send feedback now and then and second each other, with no refusal", () => {
  const run = scripted([notes, feedback], CONFIG, 150);
  expect([run.refused, run.verbs["feedback.send"]! > 5, run.verbs["feedback.same"]! > 5]).toEqual([{}, true, true]);
});

test("10,000 seeded worlds of notes and feedback played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
