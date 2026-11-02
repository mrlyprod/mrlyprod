import { expect, test } from "bun:test";
import type { Act, Who, World } from "../../kernel/index.ts";
import { digest, kernel } from "../../kernel/index.ts";
import { geese, scripted } from "../../test/gym.ts";
import type { Calc } from "./app.ts";
import calc, { EXPR, KEEP } from "./app.ts";
import { DIGITS } from "./exact.ts";

const ACTORS = Array.from({ length: 8 }, (_, i) => `test-${i + 1}`);

const CONFIG = { name: "calc", apps: ["calc"], actors: ACTORS };

const K = kernel([calc]);

const as = (id: string): Who => ({ id, kind: "human" });

const state = (world: World) => world.apps.calc as Calc;

const play = (world: World, acts: [string, string, Act["args"]][]) =>
  acts.reduce((at, [id, verb, args]) => K.call(at, { verb: `calc.${verb}`, args, who: as(id) }).world, world);

const results = (world: World, id: string) => state(world).history[id]!.map((line) => line.result);

test("an eval or clear out of its schema is a bad act and changes nothing", () => {
  const bad: [string, Act["args"]][] = [
    ["eval", { expr: "" }],
    ["eval", { expr: "1".repeat(EXPR + 1) }],
    ["eval", { expr: 2 }],
    ["clear", { who: "test-2" }],
  ];
  for (const [verb, args] of bad)
    expect(() => K.call(K.boot(CONFIG), { verb: `calc.${verb}`, args, who: as("test-1") })).toThrow("bad-act");
});

test("a bad expression is refused with where it broke, each history is its owner's, and a watcher may not act", () => {
  const world = play(K.boot(CONFIG), [["test-1", "eval", { expr: "2 + 2" }]]);
  const tries = ["1 / 0", "2 + * 3", "(1 + 2", "ans", "10 ^ 1000", "2 ^ 0.5", "7 % 2"].map((expr) =>
    K.call(world, { verb: "calc.eval", args: { expr }, who: as("test-2") }),
  );
  expect(tries.map((one) => digest(one.world.apps))).toEqual(tries.map(() => digest(world.apps)));
  expect(tries.map((one) => one.envelope.last.note)).toEqual([
    "division by zero",
    'unexpected "*" at 5',
    "a ( at 1 is never closed",
    "no answer yet for ans",
    `the result passes ${DIGITS} digits`,
    "an exponent is a whole number from -1000 to 1000",
    'unexpected "%" at 3',
  ]);
  expect(results(play(world, [["test-2", "clear", {}]]), "test-1")).toEqual(["4"]);
  expect(() => K.call(world, { verb: "calc.eval", args: { expr: "1" }, who: as("guest") })).toThrow("refused");
});

test("arithmetic is exact: decimals stay decimals, thirds stay fractions, powers and precedence hold, ans is the last result", () => {
  const world = play(K.boot(CONFIG), [
    ["test-1", "eval", { expr: "0.1 + 0.2" }],
    ["test-1", "eval", { expr: "1 / 3" }],
    ["test-1", "eval", { expr: "ans * 3" }],
    ["test-1", "eval", { expr: "-2 ^ 2 + 2 ^ 3 ^ 2" }],
    ["test-1", "eval", { expr: "(1.5 - 4) * 2 ^ -2" }],
    ["test-1", "eval", { expr: "2 ^ 100" }],
  ]);
  expect(results(world, "test-1")).toEqual(["0.3", "1/3", "1", "508", "-0.625", "1267650600228229401496703205376"]);
});

test(`a history keeps its last ${KEEP} lines and clear empties it`, () => {
  const many = Array.from({ length: KEEP + 2 }, (_, i): [string, string, Act["args"]] => [
    "test-1",
    "eval",
    { expr: `${i}` },
  ]);
  const full = play(K.boot(CONFIG), many);
  expect([
    results(full, "test-1").length,
    results(full, "test-1")[0],
    results(play(full, [["test-1", "clear", {}]]), "test-1"),
  ]).toEqual([KEEP, "2", []]);
});

test("the scripted test users work out sums, products and powers, with no refusal", () => {
  const run = scripted([calc], CONFIG, 150);
  expect([run.refused, run.verbs["calc.eval"]! > 300, run.verbs["calc.clear"]! > 5]).toEqual([{}, true, true]);
});

test("10,000 seeded worlds of calc played by the goose through the gym replay to their digests", async () => {
  const played = await geese(CONFIG, 10_000, 6);
  expect([played.same, played.acts, played.refused > 0]).toEqual([10_000, 60_000, true]);
}, 30_000);
