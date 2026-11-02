import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese } from "../../test/gym.ts";
import type { Votes } from "./app.ts";
import vote, { LONGEST, OPENED, QUESTION, SHORTEST } from "./app.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "vote", apps: ["vote"], actors: ACTORS };

const K = kernel([vote]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.vote as Votes;

type Step = [string, string, Act["args"], number?];

const play = (world: World, acts: Step[]) =>
  acts.reduce((now, [id, verb, args, at]) => K.call(now, { verb: `vote.${verb}`, args, who: as(id), at }).world, world);

const notes = (world: World, acts: Step[]) =>
  acts.map(([id, verb, args, at]) => {
    const done = K.call(world, { verb: `vote.${verb}`, args, who: as(id), at });
    expect(digest(done.world.apps)).toBe(digest(world.apps));
    return done.envelope.last.note;
  });

const OPEN: Step = ["test-1", "open", { question: "Build a maps app", wish: 4, seconds: 60 }, 1000];

test("an open, a cast or a close out of its schema is a bad act and changes nothing", () => {
  const world = play(K.boot(CONFIG), [OPEN]);
  const bad: [string, Act["args"]][] = [
    ["open", { question: "", seconds: 60 }],
    ["open", { question: "x".repeat(QUESTION + 1), seconds: 60 }],
    ["open", { question: "x", seconds: SHORTEST - 1 }],
    ["open", { question: "x", seconds: LONGEST + 1 }],
    ["cast", { ballot: 2, choice: "yes" }],
    ["cast", { ballot: 1, choice: "maybe" }],
    ["cast", { ballot: 1, choice: "yes", as: "test-3" }],
    ["close", {}],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(world, { verb: `vote.${verb}`, args, who: as("test-2") })).toThrow("bad-act");
});

test("one vote each and final, one open ballot per wish, a few open ballots each, and a watcher may not vote", () => {
  const world = play(K.boot(CONFIG), [OPEN, ["test-2", "cast", { ballot: 1, choice: "no" }, 2000]]);
  const busy = play(
    world,
    Array.from({ length: OPENED - 1 }, (_, i): Step => ["test-1", "open", { question: `q${i}`, seconds: 60 }, 3000]),
  );
  expect(
    notes(busy, [
      ["test-2", "cast", { ballot: 1, choice: "yes" }, 4000],
      ["test-3", "open", { question: "Maps, please", wish: 4, seconds: 60 }, 4000],
      ["test-1", "open", { question: "one more", seconds: 60 }, 4000],
    ]),
  ).toEqual([
    "you already voted no on ballot 1; one vote each",
    "ballot 1 already decides wish 4",
    `you have ${OPENED} ballots open; close one first`,
  ]);
  expect(() => K.call(world, { verb: "vote.cast", args: { ballot: 1, choice: "yes" }, who: as("guest") })).toThrow(
    "refused",
  );
});

test("a ballot closes on world time: votes stop at its closing, anyone closes it then, and the tally decides", () => {
  const voted = play(K.boot(CONFIG), [
    OPEN,
    ["test-2", "cast", { ballot: 1, choice: "yes" }, 30_000],
    ["test-3", "cast", { ballot: 1, choice: "yes" }, 40_000],
    ["test-4", "cast", { ballot: 1, choice: "no" }, 50_000],
  ]);
  expect(
    notes(voted, [
      ["test-5", "close", { ballot: 1 }, 52_000],
      ["test-5", "cast", { ballot: 1, choice: "no" }, 61_000],
    ]),
  ).toEqual(["ballot 1 is open for 9 seconds more", "ballot 1 has closed; its tally is final"]);
  const closed = play(voted, [["test-6", "close", { ballot: 1 }, 61_000]]);
  expect([
    K.read(closed, "vote/ballots/1", ["tally", "result"]).state,
    notes(closed, [["test-6", "close", { ballot: 1 }]]),
  ]).toEqual([{ tally: { yes: 2, no: 1 }, result: "carried" }, ["ballot 1 is already closed: carried"]]);
  const tie = play(K.boot(CONFIG), [
    OPEN,
    ["test-2", "cast", { ballot: 1, choice: "yes" }],
    ["test-3", "cast", { ballot: 1, choice: "no" }],
    ["test-4", "close", { ballot: 1 }, 61_000],
  ]);
  expect([state(tie).ballots["1"]!.result, K.read(tie, "vote/@test-3").state]).toEqual([
    "failed",
    { votes: { 1: "no" }, opened: [] },
  ]);
});

test("10,000 seeded worlds of vote played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
