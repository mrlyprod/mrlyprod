import { expect, test } from "bun:test";
import * as life from "@mrlyprod/life";
import { bits } from "./engine.ts";
import type { State } from "./run.ts";
import { boot, moves, read, step, TICK } from "./run.ts";

const ISLAND = life.recipe.parse("hline.3.1 x1 island mask:code7.3.1 birth:3 survive:2,3 x:64 cap:31");

const KEYS = [
  "hline.3.1 x1 island mask:code7.3.1 birth:3 survive:2,3 x:64 cap:31",
  "code1.3.1 x5 island mask:code1.3.1 birth:evens survive:evens x:256 cap:151",
  "carpet.3.2 x3 island mask:copy birth:odds survive:evens x:64 cap:41",
  "point.3.1 x1 island mask:code7.3.1 birth:0 survive:5,6,7,8 x:200 cap:31",
  "star.5.1 x1 island mask:code7.5.1 birth:3 survive:2,3 x:12 cap:31",
  "code7.3.1 x1 island mask:carpet.9.1 birth:odds survive:evens x:16 cap:61",
  "code7.3.1 x1 wallpaper mask:code7.3.1 birth:0 survive:8 x:64",
  "htree.5.1 x1 wallpaper mask:copyinv birth:primes survive:binary x:64",
  "code15.3.1 x1 wallpaper mask:code7.3.1 birth:3 survive:8 x:64",
  "carpet.7.2 x1 wallpaper mask:carpet.9.2 birth:odds survive:evens x:40",
];

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const plain = (x: unknown): boolean =>
  x === null ||
  typeof x === "string" ||
  Number.isInteger(x) ||
  (typeof x === "object" && Object.values(x as object).every(plain));

const ticks = (state: State, n: number) => {
  for (let i = 0; i < n; i++) state = step(state, TICK);
  return state;
};

test("the reducer replays the wasm run frame for frame and stop for stop, on islands, torus wallpapers and flipping planes", () => {
  const kinds = new Set<string>();
  for (const name of KEYS) {
    const recipe = life.recipe.parse(name);
    const run = new life.recipe.Run(recipe);
    let state = boot(recipe);
    for (;;) {
      const frame = run.frame();
      expect([state.gen, state.rows, state.cols, state.cells, state.origin, state.background, state.stop]).toEqual([
        run.generation(),
        frame.shape[0],
        frame.shape[1],
        bits(frame.types),
        Array.from(run.origin(), Number),
        run.background(),
        run.stop() ?? null,
      ]);
      if (state.stop) break;
      run.step();
      state = step(state, TICK);
    }
    kinds.add(state.stop.kind);
    run.free();
  }
  expect([...kinds].sort()).toEqual(["dead", "limit", "loop", "still", "wall"]);
});

test("boot holds the recipe as integers and 0/1 strings, the seed's top-left corner as the origin", () => {
  const state = boot(ISLAND);
  expect([state.rows, state.cols, state.cells, state.origin, state.gen, state.pop, state.stop]).toEqual([
    1,
    3,
    "111",
    [1, 0],
    0,
    3,
    null,
  ]);
  expect([state.span, state.mask, state.budget, state.reach, state.cap, state.side]).toEqual([
    [3, 3],
    "111101111",
    8,
    1,
    31,
    3,
  ]);
});

test("a glider set by hand, which no symmetric recipe seeds, stops as a mover with its period, shift and the generation it meets the wall", () => {
  const glider = {
    ...boot({ ...ISLAND, x: 100, cap: 21 }),
    rows: 3,
    cols: 3,
    cells: "010001111",
    origin: [0, 0],
    seen: {},
  };
  expect(ticks(glider, 8).stop).toEqual({ kind: "mover", generation: 5, period: 4, dr: 1, dc: 1, wall_by: 33 });
  expect(ticks({ ...glider, x: 30 }, 8).stop?.wall_by).toBe(0);
});

test("the same recipe and ticks replay to the same state, byte for byte", () => {
  const other = life.recipe.parse(KEYS[3]);
  expect(JSON.stringify(ticks(boot(ISLAND), 5))).toBe(JSON.stringify(ticks(boot(ISLAND), 5)));
  expect(JSON.stringify(ticks(boot(ISLAND), 5))).not.toBe(JSON.stringify(ticks(boot(other), 5)));
});

test("step leaves its input untouched, its output is plain and survives a JSON round trip, and a stopped run ticks to itself", () => {
  let state = boot(life.recipe.parse(KEYS[3]));
  while (!state.stop) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), TICK);
    expect(JSON.stringify(state)).toBe(before);
    expect(JSON.parse(JSON.stringify(next))).toEqual(next);
    expect(plain(next)).toBe(true);
    state = next;
  }
  expect(step(state, TICK)).toBe(state);
});

test("moves lists the tick until the run stops, then nothing", () => {
  expect([moves(boot(ISLAND)), moves(ticks(boot(ISLAND), 2))]).toEqual([[TICK], []]);
});

test("read walks a slash path into the state", () => {
  const state = boot(ISLAND);
  expect([read(state), read(state, "pop"), read(state, "origin/0"), read(state, "nope/x")]).toEqual([
    state,
    3,
    1,
    undefined,
  ]);
});
