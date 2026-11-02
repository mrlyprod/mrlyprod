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
    let state: State = boot({ levels: 4, seed });
    while (!state.over && state.taps < 2 * state.par) state = step(state, pick(state, mode, rand)!);
    if (state.over) {
      solved++;
      taps += state.taps - state.par;
    }
  }
  return [solved, taps];
};

test("over 30 trees smart blooms each in exactly par spins, and random within twice par blooms fewer than one in ten", () => {
  expect(run("smart", 30)).toEqual([30, 0]);
  expect(run("random", 30)[0]).toBeLessThan(3);
});

test("random picks a listed move, and no one picks in a bloomed tree", () => {
  const rand = rng(3);
  const state = boot({ levels: 4, seed: 3 });
  for (let i = 0; i < 30; i++) expect(moves(state)).toContainEqual(pick(state, "random", rand)!);
  expect(pick({ ...state, over: "done" }, "smart", rand)).toBe(null);
});
