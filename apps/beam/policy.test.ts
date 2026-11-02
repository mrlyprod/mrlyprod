import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { State } from "./engine.ts";
import { boot, moves, step } from "./engine.ts";
import { pick } from "./policy.ts";

const run = (mode: string, games: number) => {
  let solved = 0;
  let taps = 0;
  for (let seed = 1; seed <= games; seed++) {
    const rand = rng(seed);
    let state: State = boot({ size: 9, mirrors: 16, stars: 5, seed });
    while (!state.over && state.taps < 2 * state.par) state = step(state, pick(state, mode, rand)!);
    if (state.over) {
      solved++;
      taps += state.taps - state.par;
    }
  }
  return [solved, taps];
};

test("over 30 rooms smart lights every star in exactly par taps, and random within twice par lights fewer than one in five", () => {
  expect(run("smart", 30)).toEqual([30, 0]);
  expect(run("random", 30)[0]).toBeLessThan(6);
});

test("random picks a listed move, and no one picks in a finished room", () => {
  const rand = rng(3);
  const state = boot({ size: 8, mirrors: 12, stars: 4, seed: 3 });
  for (let i = 0; i < 30; i++) expect(moves(state)).toContainEqual(pick(state, "random", rand)!);
  expect(pick({ ...state, over: "done" }, "smart", rand)).toBe(null);
});
