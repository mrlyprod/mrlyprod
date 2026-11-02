import { expect, test } from "bun:test";
import { rng } from "../../net/sdk/rand.ts";
import type { State } from "./engine.ts";
import { boot, moves, par, step } from "./engine.ts";
import { pick } from "./policy.ts";

const run = (mode: string, games: number) => {
  const out = { done: 0, stuck: 0, moves: 0 };
  for (let seed = 1; seed <= games; seed++) {
    const rand = rng(seed);
    let state: State = boot({ size: 8, folds: 3, seed });
    while (!state.over) state = step(state, pick(state, mode, rand)!);
    out[state.over]++;
    if (state.over === "done") out.moves += state.moves - state.par;
  }
  return out;
};

test("smart makes every one of 30 laces in exactly par, and random makes fewer than a third", () => {
  const smart = run("smart", 30);
  const random = run("random", 30);
  expect(smart).toEqual({ done: 30, stuck: 0, moves: 0 });
  expect(random.done).toBeLessThan(10);
});

test("smart finishes a game left half played by random in the par of where it took over", () => {
  let checked = 0;
  for (let seed = 1; seed <= 20; seed++) {
    const rand = rng(seed);
    let state: State = boot({ size: 6, folds: 2, seed });
    for (let i = 0; i < 2 && !state.over; i++) state = step(state, pick(state, "random", rand)!);
    if (state.over) continue;
    const owed = par(state);
    const from = state.moves;
    while (!state.over) state = step(state, pick(state, "smart", rand)!);
    expect([seed, state.over, state.moves - from]).toEqual([seed, "done", owed]);
    checked++;
  }
  expect(checked).toBeGreaterThan(5);
});

test("random picks a listed move, and no one picks on a finished sheet", () => {
  const rand = rng(3);
  const state = boot({ size: 6, folds: 2, seed: 3 });
  for (let i = 0; i < 30; i++) expect(moves(state)).toContainEqual(pick(state, "random", rand)!);
  expect(pick({ ...state, over: "done" }, "smart", rand)).toBe(null);
});
