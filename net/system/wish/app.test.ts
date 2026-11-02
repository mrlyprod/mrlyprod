import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese } from "../../test/gym.ts";
import type { Wishes } from "./app.ts";
import wish, { OPEN, TEXT } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "wish", apps: ["wish"], actors: ACTORS };

const K = kernel([wish]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.wish as Wishes;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `wish.${verb}`, args, who: as(id) }).world, world);

const notes = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.map(([id, verb, args]) => {
    const done = K.call(world, { verb: `wish.${verb}`, args, who: as(id) });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });

test("a post, a back or a withdrawal out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [["test-1", "post", { text: "Build a notes app" }]]);
  const bad: [string, Act["args"]][] = [
    ["post", { text: "" }],
    ["post", { text: "x".repeat(TEXT + 1) }],
    ["back", { wish: 0 }],
    ["back", { wish: 2 }],
    ["back", { wish: "1" }],
    ["withdraw", { wish: 1, by: "test-1" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `wish.${verb}`, args, who: as("test-2") })).toThrow("bad-act");
});

test("nobody backs their own wish or one twice, only its author withdraws it, and a watcher may not post", () => {
  const world = play(K.boot(CONFIG), [
    ["test-1", "post", { text: "Build a notes app" }],
    ["test-2", "back", { wish: 1 }],
  ]);
  expect(
    notes(world, [
      ["test-1", "back", { wish: 1 }],
      ["test-2", "back", { wish: 1 }],
      ["test-2", "withdraw", { wish: 1 }],
    ]),
  ).toEqual(["you posted wish 1; others back it", "you already back wish 1", "only test-1 withdraws wish 1"]);
  expect(() => K.call(world, { verb: "wish.post", args: { text: "x" }, who: as("guest") })).toThrow("refused");
});

test(`a wish asks once: a repeat is refused, ${OPEN} open each, and a withdrawn wish takes no backers and frees a place`, () => {
  const full = play(
    K.boot(CONFIG),
    Array.from({ length: OPEN }, (_, i): [string, string, Act["args"]] => ["test-1", "post", { text: `ask ${i}` }]),
  );
  const freed = play(full, [["test-1", "withdraw", { wish: 2 }]]);
  expect(
    notes(full, [
      ["test-2", "post", { text: "ASK 0" }],
      ["test-1", "post", { text: "one more" }],
    ]),
  ).toEqual(["wish 1 already asks this; back it instead", `you have ${OPEN} open wishes; withdraw one first`]);
  expect(notes(freed, [["test-3", "back", { wish: 2 }]])).toEqual(["wish 2 was withdrawn"]);
  const after = play(freed, [
    ["test-1", "post", { text: "one more" }],
    ["test-3", "back", { wish: 6 }],
  ]);
  expect([state(after).wishes["6"]!.backers, K.read(after, "wish/@test-3").state]).toEqual([
    ["test-3"],
    { posted: [], backing: [6] },
  ]);
});

test("10,000 seeded worlds of wish played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
