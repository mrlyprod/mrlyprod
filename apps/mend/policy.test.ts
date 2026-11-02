import { expect, test } from "bun:test";
import * as math from "@mrlyprod/math";
import { rng } from "../../net/sdk/rand.ts";
import type { State } from "./engine.ts";
import { boot, grown, HOME, moves, par, step } from "./engine.ts";
import { pick } from "./policy.ts";

const deal = (code: string, level: number, mess: number, seed: number) =>
  boot({ number: 3, level, motif: grown(math, 3, 1, code), goal: grown(math, 3, level, code), mess, seed });

test("smart mends a game left half played by silly in exactly the par of where it took over", () => {
  for (let seed = 1; seed <= 12; seed++) {
    const rand = rng(seed);
    let state: State = deal(seed % 2 ? HOME : "94", 3, 12, seed);
    for (let i = 0; i < 6 && !state.over; i++) state = step(state, pick(state, "silly", rand)!);
    const owed = par({ ...state, over: null });
    const from = state.moves;
    for (let i = 0; i < 500 && !state.over; i++) state = step(state, pick(state, "smart", rand)!);
    expect([seed, state.over, state.moves - from]).toEqual([seed, "mended", owed]);
  }
});

test("silly picks a legal turn, and no one picks on a mended board", () => {
  const rand = rng(3);
  const state = deal(HOME, 3, 8, 3);
  for (let i = 0; i < 50; i++) expect(moves(state)).toContainEqual(pick(state, "silly", rand)!);
  expect(pick(deal("495", 3, 8, 3), "smart", rand)).toBe(null);
});
