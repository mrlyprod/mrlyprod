import { expect, test } from "bun:test";
import type { Act, State } from "./engine.ts";
import { boot, fell, moves, read, sizing, step, TICK } from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const rain = (over?: Partial<State>): State =>
  freeze({ cols: 3, rows: 4, pool: 10, hues: 0, drops: [1, 4, 5], tints: [-1, -1, -1], lit: [], seed: 1, ...over });

const GAME: Act[] = [sizing(6, 5, 1), ...Array(40).fill(TICK), sizing(9, 7, 1), ...Array(400).fill(TICK)];

const replay = (seed: number, hues = 8) => GAME.reduce(step, boot({ pool: 36, hues, seed }));

test("a column resets past the floor on a rare roll", () => {
  expect([fell(101, 100, 0.98), fell(101, 100, 0.97), fell(99, 100, 0.99)]).toEqual([true, false, false]);
});

test("a size opens new columns on row one unless scattered, keeps the old ones and opens only once", () => {
  const open = step(boot({ pool: 10, hues: 0, seed: 5 }), sizing(4, 6, 1));
  expect([open.drops, open.tints, open.seed]).toEqual([[1, 1, 1, 1], [-1, -1, -1, -1], 5]);
  const wider = step(open, sizing(6, 6, 1));
  expect(wider.drops.slice(0, 4)).toEqual([1, 1, 1, 1]);
  expect(wider.drops.slice(4).every((row) => row >= 1 && row <= 6)).toBe(true);
  const scattered = step(boot({ pool: 10, hues: 3, seed: 5 }), sizing(40, 6, 0));
  expect(new Set(scattered.drops).size).toBeGreaterThan(1);
  expect(scattered.tints.every((k) => k >= 0 && k < 3)).toBe(true);
});

test("a tick prints a glyph in every column above the floor and moves each drop down a row", () => {
  const next = step(rain(), TICK);
  expect([next.lit[0] >= 0, next.lit[1] >= 0, next.lit[2]]).toEqual([true, true, -1]);
  expect(next.lit.slice(0, 2).every((k) => k < 10)).toBe(true);
  expect(next.drops.map((row, i) => row === rain().drops[i] + 1 || row === 1)).toEqual([true, true, true]);
});

test("a fall below the floor restarts the column on row one with a new tint", () => {
  let state = rain({ hues: 8, drops: [5, 5, 5] });
  while (state.drops.every((row) => row > 1)) state = step(state, TICK);
  const i = state.drops.indexOf(1);
  expect([state.drops[i], state.tints[i] >= 0]).toEqual([1, true]);
});

test("the same seed and acts replay to the same state, byte for byte", () => {
  expect(JSON.stringify(replay(42))).toBe(JSON.stringify(replay(42)));
  expect(JSON.stringify(replay(42))).not.toBe(JSON.stringify(replay(43)));
});

test("step leaves its input untouched and its output is integers and survives a JSON round trip", () => {
  let state = boot({ pool: 36, hues: 8, seed: 42 });
  for (const act of GAME) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), act);
    expect(JSON.stringify(state)).toBe(before);
    expect(JSON.parse(JSON.stringify(next))).toEqual(next);
    state = next;
  }
  expect(Object.values(state).flat().every(Number.isInteger)).toBe(true);
});

test("moves is always the tick, and read walks a slash path into the state", () => {
  const state = rain();
  expect(moves(state)).toEqual([TICK]);
  expect([read(state), read(state, "rows"), read(state, "drops/1"), read(state, "nope/x")]).toEqual([
    state,
    4,
    4,
    undefined,
  ]);
});
