import { expect, test } from "bun:test";
import * as math from "@mrlyprod/math";
import type { State } from "./engine.ts";
import {
  boot,
  cap,
  claim,
  coded,
  digits,
  echoes,
  fills,
  grown,
  HOME,
  join,
  moves,
  ONE,
  read,
  spot,
  step,
  TWO,
  tally,
} from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const deal = (code: string, level: number, seed: number, number = 3) =>
  boot({ number, level, motif: grown(math, number, 1, code), seed });

const play = (state: State, cells: number[]) => cells.reduce((at, cell) => step(at, claim(cell)), state);

const first = (state: State) => moves(state)[0].args.cell;

test("the cells sit one to one on the filled cells of the door's own fractal", () => {
  for (const [code, number, level] of [
    [HOME, 3, 3],
    ["495", 3, 3],
    ["186", 3, 4],
    ["491", 5, 2],
  ] as const) {
    const state = deal(code, level, 1, number);
    const side = number ** level;
    const goal = grown(math, number, level, code);
    const seen = new Set(Array.from({ length: state.board.length }, (_, cell) => spot(state, cell).join()));
    const filled = [...goal].flatMap((one, i) => (one === "1" ? [`${i % side},${Math.floor(i / side)}`] : []));
    expect([code, seen.size, new Set(filled)]).toEqual([code, state.board.length, seen]);
  }
});

test("a claim takes the cell and its echoes: its path read from every scale", () => {
  const state = deal(HOME, 3, 1);
  const f = fills(state.motif).length;
  const cell = join([0, 2, 5], f);
  const taken = play(state, [cell]);
  const mine = [...taken.board].flatMap((one, i) => (one === "0" ? [] : [i]));
  expect(mine.sort((a, b) => a - b)).toEqual(
    [join([0, 2, 5], f), join([2, 5, 0], f), join([5, 0, 2], f)].sort((a, b) => a - b),
  );
  expect(echoes(state, join([4, 4, 4], f))).toEqual([join([4, 4, 4], f)]);
  expect(echoes(deal(HOME, 2, 1), join([1, 3], f))).toEqual([join([1, 3], f), join([3, 1], f)]);
  expect(digits(cell, f, 3)).toEqual([0, 2, 5]);
});

test("the seed's coin picks who opens, and the players then take turns", () => {
  const openers = new Set(Array.from({ length: 12 }, (_, seed) => deal(HOME, 3, seed).turn));
  expect(openers).toEqual(new Set([ONE, TWO]));
  const state = deal(HOME, 3, 4);
  const after = step(state, claim(first(state)));
  expect([after.turn, step(after, claim(first(after))).turn]).toEqual([state.turn === ONE ? TWO : ONE, state.turn]);
});

test("a block goes to whoever holds more of its parts, and a tie to the part sitting where the block sits", () => {
  const state = deal(HOME, 2, 1);
  const board = (marks: Record<number, number>) =>
    tally({ ...state, board: Array.from(state.board, (_, i) => String(marks[i] ?? 0)).join("") }).owners;
  const leaf = (a: number, b: number) => 7 + a * 6 + b;
  const plain = board({ [join([2, 0], 6)]: ONE, [join([2, 1], 6)]: ONE, [join([2, 3], 6)]: TWO });
  expect(plain[1 + 2]).toBe(ONE);
  const tied = board({ [join([2, 0], 6)]: ONE, [join([2, 2], 6)]: TWO });
  const flip = board({ [join([2, 2], 6)]: ONE, [join([2, 0], 6)]: TWO });
  expect([tied[1 + 2], flip[1 + 2], tied[leaf(2, 2)]]).toEqual([TWO, ONE, TWO]);
});

test("a block is worth its cells, so a board held whole scores every scale alike", () => {
  const state = deal(HOME, 3, 1);
  const full = tally({ ...state, board: "1".repeat(state.board.length) });
  expect(full.points).toEqual([4 * 216, 0]);
});

test("moves lists one claim per free echo, and claiming them all fills the board exactly once", () => {
  const state = deal(HOME, 3, 1);
  const all = moves(state);
  expect(all).toHaveLength(6 + (216 - 6) / 3);
  let at = state;
  for (const act of all) {
    const free = [...at.board].filter((one) => one === "0").length;
    const next = step(at, act);
    expect(free - [...next.board].filter((one) => one === "0").length).toBe(echoes(at, act.args.cell).length);
    at = next;
  }
  expect([at.board.includes("0"), at.over === null, moves(at)]).toEqual([false, false, []]);
});

test("a full game ends in a result that matches the score, and takes no move after", () => {
  let state = deal(HOME, 3, 9);
  while (!state.over) state = step(state, claim(first(state)));
  const [one, two] = state.points;
  expect(state.over).toBe(one > two ? "one" : two > one ? "two" : "draw");
  expect(state.points).toEqual(tally(state).points);
  expect(step(state, claim(0))).toBe(state);
});

test("step refuses a taken cell, a cell out of range and a broken cell", () => {
  const state = play(deal(HOME, 3, 1), [0]);
  for (const act of [claim(0), claim(-1), claim(216), claim(1.5), { verb: "turn", args: { cell: 1 } } as never])
    expect(step(state, act)).toBe(state);
});

const GAME = [0, 17, 40, 101, 7, 215];

const replay = (seed: number) => play(deal(HOME, 3, seed), GAME);

test("the same seed and claims replay to the same state, byte for byte", () => {
  expect(JSON.stringify(replay(42))).toBe(JSON.stringify(replay(42)));
  expect(JSON.stringify(replay(42))).not.toBe(JSON.stringify(replay(43)));
});

test("step leaves its input untouched and its state holds only integers, strings and nulls", () => {
  let state = deal(HOME, 3, 42);
  for (const cell of GAME) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), claim(cell));
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

test("an empty design deals a finished draw", () => {
  const state = deal("0", 3, 1);
  expect([state.board, state.over, moves(state)]).toEqual(["", "draw", []]);
});

test("read walks a slash path into the state", () => {
  const state = play(deal(HOME, 3, 1), [0]);
  expect([read(state, "points/0"), read(state, "turn"), read(state, "nope/x")]).toEqual([
    state.points[0],
    state.turn,
    undefined,
  ]);
});

test("the level cap keeps the board within 81 cells a side, and a blank code is the triangle", () => {
  expect([cap(3), cap(5), cap(7), coded(math, ""), coded(math, "513")]).toEqual([4, 2, 2, HOME, "1"]);
});
