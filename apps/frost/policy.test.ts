import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { State } from "./engine.ts";
import { boot, moves, saved, step } from "./engine.ts";
import { pick } from "./policy.ts";

const run = (mode: string, games: number) => {
  let kept = 0;
  let of = 0;
  for (let seed = 1; seed <= games; seed++) {
    const rand = rng(seed);
    let state: State = boot({ size: 15, frost: 2, flowers: 5, seed });
    while (!state.over) state = step(state, pick(state, mode, rand)!);
    kept += saved(state);
    of += state.flowers;
  }
  return kept / of;
};

test("over 30 fields smart saves more than four flowers in five, and random fewer than one in three", () => {
  expect(run("smart", 30)).toBeGreaterThan(0.8);
  expect(run("random", 30)).toBeLessThan(0.34);
});

test("random picks a listed move, and no one picks on a finished field", () => {
  const rand = rng(3);
  const state = boot({ size: 11, frost: 1, flowers: 3, seed: 3 });
  for (let i = 0; i < 30; i++) expect(moves(state)).toContainEqual(pick(state, "random", rand)!);
  expect(pick({ ...state, over: "done" }, "smart", rand)).toBe(null);
});
