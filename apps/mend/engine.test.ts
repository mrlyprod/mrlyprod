import { expect, test } from "bun:test";
import * as math from "@mrlyprod/math";
import type { Act, State } from "./engine.ts";
import {
  blocks,
  boot,
  cap,
  coded,
  grown,
  HOME,
  LEFT,
  lay,
  moves,
  par,
  picture,
  plan,
  RIGHT,
  read,
  rotate,
  step,
  turn,
  under,
} from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const deal = (code: string, number: number, level: number, mess: number, seed: number) =>
  boot({
    number,
    level,
    motif: grown(math, number, 1, code),
    goal: grown(math, number, level, code),
    mess,
    seed,
  });

const flat = (state: State): State => ({ ...state, turns: state.turns.map(() => 0), over: null });

const solve = (state: State) => {
  let at = state;
  for (let i = 0; i < 1000 && !at.over; i++) {
    const want = plan(at);
    const block = want.findIndex((q) => q !== 0);
    at = step(at, turn(block, want[block] === 3 ? LEFT : RIGHT));
  }
  return at;
};

test("unturned blocks paint the door's own fractal", () => {
  for (const [code, number, level] of [
    [HOME, 3, 3],
    ["457", 3, 4],
    ["491", 5, 2],
    ["239", 7, 2],
  ] as const) {
    const state = flat(deal(code, number, level, 1, 1));
    expect([code, picture(state) === state.goal]).toEqual([code, true]);
  }
});

test("a right turn spins a block and everything inside it a quarter clockwise, and a left turn spins it back", () => {
  const start = flat(deal(HOME, 3, 2, 1, 1));
  const right = step(start, turn(0, RIGHT));
  expect(picture(right)).toBe(rotate(start.goal, 9, 1));
  expect(picture(step(right, turn(0, LEFT)))).toBe(start.goal);
  const small = step(start, turn(1, RIGHT));
  const [x, y, size] = lay(small).boxes.subarray(3, 6);
  const changed = [...picture(small)].flatMap((one, i) => (one === start.goal[i] ? [] : [i]));
  expect(changed.length).toBeGreaterThan(0);
  expect(
    changed.every((i) => i % 9 >= x && i % 9 < x + size && Math.floor(i / 9) >= y && Math.floor(i / 9) < y + size),
  ).toBe(true);
});

test("a deal of a design with no quarter turn symmetry is broken, with a par of at least one", () => {
  for (let seed = 0; seed < 20; seed++) {
    const state = deal(HOME, 3, 3, 1, seed);
    expect([seed, state.over, state.par > 0, picture(state) === state.goal]).toEqual([seed, null, true, false]);
  }
});

test("a design that every quarter turn keeps deals already mended at par 0", () => {
  const state = deal("495", 3, 3, 10, 3);
  expect([state.over, state.par, state.moves, moves(state)]).toEqual(["mended", 0, 0, []]);
});

test("par is the fewest turns: a breadth-first search over every turn agrees", () => {
  for (let seed = 0; seed < 6; seed++) {
    const start = deal(HOME, 3, 2, 4, seed);
    const seen = new Set([start.turns.join("")]);
    let ring = [start];
    let far = 0;
    while (!ring.some((one) => picture(one) === one.goal)) {
      ring = ring.flatMap((one) =>
        moves(one)
          .map((act) => step({ ...one, over: null }, act))
          .filter((next) => !seen.has(next.turns.join("")) && seen.add(next.turns.join(""))),
      );
      far++;
    }
    expect([seed, par(start)]).toEqual([seed, far]);
  }
});

test("following the plan mends the board in exactly par turns, and a mended board takes no more", () => {
  for (const [code, number, level, mess] of [
    [HOME, 3, 3, 10],
    ["122", 3, 3, 20],
    ["313", 3, 3, 12],
    ["491", 5, 2, 8],
  ] as const) {
    const start = deal(code, number, level, mess, 42);
    const end = solve(start);
    expect([code, end.over, end.moves]).toEqual([code, "mended", start.par]);
    expect(step(end, turn(0, RIGHT))).toBe(end);
  }
});

const GAME: Act[] = [turn(0, RIGHT), turn(3, LEFT), turn(1, RIGHT), turn(0, LEFT), turn(12, RIGHT)];

const replay = (seed: number) => GAME.reduce(step, deal(HOME, 3, 3, 10, seed));

test("the same seed and turns replay to the same state, byte for byte", () => {
  expect(JSON.stringify(replay(42))).toBe(JSON.stringify(replay(42)));
  expect(JSON.stringify(replay(42))).not.toBe(JSON.stringify(replay(43)));
});

test("step leaves its input untouched and its state holds only integers and strings", () => {
  let state = deal(HOME, 3, 3, 10, 42);
  for (const act of GAME) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), act);
    expect(JSON.stringify(state)).toBe(before);
    expect(JSON.parse(JSON.stringify(next))).toEqual(next);
    state = next;
  }
  expect(
    Object.values(state)
      .flat()
      .every((one) => one === null || typeof one === "string" || Number.isInteger(one)),
  ).toBe(true);
});

test("step refuses a block out of range, a broken block and a half turn", () => {
  const state = deal(HOME, 3, 2, 4, 1);
  for (const act of [turn(-1, RIGHT), turn(blocks(state), RIGHT), turn(1.5, RIGHT), turn(0, 2)])
    expect(step(state, act)).toBe(state);
});

test("moves lists a right and a left turn for every block", () => {
  const state = deal(HOME, 3, 2, 4, 1);
  expect(moves(state)).toHaveLength(2 * 7);
  expect(moves(state).slice(0, 2)).toEqual([turn(0, RIGHT), turn(0, LEFT)]);
});

test("under finds the block of a level that covers a cell, and none over a hole", () => {
  const state = flat(deal(HOME, 3, 2, 1, 1));
  const laid = lay(state);
  expect([
    under(state, laid, 0, 8, 0),
    under(state, laid, 1, 0, 0),
    under(state, laid, 1, 8, 8),
    under(state, laid, 1, 8, 0),
  ]).toEqual([0, 1, 6, -1]);
});

test("read walks a slash path into the state", () => {
  const state = deal(HOME, 3, 2, 4, 1);
  expect([read(state, "par"), read(state, "turns/0"), read(state, "nope/x")]).toEqual([
    state.par,
    state.turns[0],
    undefined,
  ]);
});

test("the level cap keeps the board within 128 cells a side, and a blank code is the triangle", () => {
  expect([cap(3), cap(5), cap(7), coded(math, ""), coded(math, "513")]).toEqual([4, 3, 2, HOME, "1"]);
});
