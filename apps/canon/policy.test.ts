import { expect, test } from "bun:test";
import * as math from "@mrlyprod/math";
import { rng } from "../../net/sdk/rand.ts";
import type { State } from "./engine.ts";
import { boot, grown, HOME, moves, ONE, step } from "./engine.ts";
import { pick } from "./policy.ts";

const deal = (code: string, seed: number) => boot({ number: 3, level: 3, motif: grown(math, 3, 1, code), seed });

const duel = (one: string, two: string, games: number) => {
  const won = { one: 0, two: 0, draw: 0 };
  for (let seed = 1; seed <= games; seed++) {
    const rand = rng(seed);
    let state: State = deal(HOME, seed);
    while (!state.over) state = step(state, pick(state, state.turn === ONE ? one : two, rand)!);
    won[state.over]++;
  }
  return won;
};

test("smart beats random in every one of 30 seeded games, whoever the coin lets open", () => {
  expect(duel("smart", "random", 30)).toEqual({ one: 30, two: 0, draw: 0 });
});

test("greedy beats random most of the time, and smart beats greedy more often than it loses", () => {
  const easy = duel("greedy", "random", 30);
  const hard = duel("smart", "greedy", 30);
  expect(easy.one).toBeGreaterThanOrEqual(25);
  expect(hard.one).toBeGreaterThan(hard.two);
});

test("every bot picks a listed claim, and none picks on a finished board", () => {
  const rand = rng(3);
  const state = deal(HOME, 3);
  for (const mode of ["smart", "greedy", "random"]) expect(moves(state)).toContainEqual(pick(state, mode, rand)!);
  expect(pick(deal("0", 3), "smart", rand)).toBe(null);
});
