import { expect, test } from "bun:test";
import type { Act, State } from "./engine.ts";
import {
  boot,
  deal,
  depths,
  doomed,
  drop,
  EMPTY,
  FLOWER,
  frosty,
  future,
  grow,
  moves,
  NEVER,
  read,
  SEED,
  STONE,
  saved,
  step,
  WAIT,
} from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const field = (size: number, marks: [number, number, string][]): State => {
  const board = EMPTY.repeat(size * size).split("");
  for (const [x, y, one] of marks) board[y * size + x] = one;
  const text = board.join("");
  return {
    size,
    board: text,
    turn: 0,
    stones: 0,
    flowers: [...text].filter((one) => one === FLOWER).length,
    over: null,
    seed: 1,
  };
};

const at = (state: State, x: number, y: number) => state.board[y * state.size + x];

test("frost spreads into each square that touches exactly one frost, pointing back at it", () => {
  const board = grow(9, field(9, [[4, 4, SEED]]).board);
  expect([board[3 * 9 + 4], board[4 * 9 + 5], board[5 * 9 + 4], board[4 * 9 + 3], board[3 * 9 + 3]]).toEqual([
    "s",
    "w",
    "n",
    "e",
    EMPTY,
  ]);
  const twice = grow(9, board);
  expect([twice[3 * 9 + 3], twice[2 * 9 + 4], twice[2 * 9 + 3]]).toEqual([EMPTY, "s", EMPTY]);
});

test("a square two frosts touch at once stays clear forever", () => {
  let board = field(9, [
    [2, 4, SEED],
    [6, 4, SEED],
  ]).board;
  for (let i = 0; i < 12; i++) board = grow(9, board);
  expect(board[4 * 9 + 4]).toBe(EMPTY);
});

test("a stone blocks the frost, lands only on an empty square the frost reaches next, and every move grows the frost once", () => {
  const start = field(9, [
    [4, 4, SEED],
    [8, 8, FLOWER],
  ]);
  const stoned = step(start, drop(5, 4));
  expect([at(stoned, 5, 4), at(stoned, 4, 3), stoned.stones, stoned.turn]).toEqual([STONE, "s", 1, 1]);
  for (const act of [drop(4, 4), drop(5, 4), drop(0, 0), drop(9, 0), drop(-1, 2), drop(1.5, 2)] as Act[])
    expect(step(stoned, act)).toBe(stoned);
  const waited = step(stoned, WAIT);
  expect([waited.turn, waited.stones, at(waited, 4, 2)]).toEqual([2, 1, "s"]);
});

test("a flower the frost reaches is lost, and one the frost cannot reach ends the round saved", () => {
  const lost = step(
    field(9, [
      [4, 4, SEED],
      [4, 2, FLOWER],
      [0, 8, FLOWER],
    ]),
    WAIT,
  );
  const end = step(lost, WAIT);
  expect(at(end, 4, 2)).toBe("S");
  const walled = field(5, [
    [0, 0, SEED],
    [4, 4, FLOWER],
    [3, 4, STONE],
    [4, 3, STONE],
  ]);
  expect([doomed(walled), step(walled, WAIT).over, saved(step(walled, WAIT))]).toEqual([false, "done", 1]);
});

test("future names the turn each square freezes, as growing turn by turn finds", () => {
  const state = deal({ size: 15, frost: 3, flowers: 6 }, { seed: 9 });
  const when = future(state.size, state.board);
  let board = state.board;
  for (let t = 1; t < 40; t++) {
    const next = grow(state.size, board);
    for (let p = 0; p < board.length; p++)
      if (!frosty(board[p]) && frosty(next[p])) expect([p, when[p]]).toEqual([p, t]);
    board = next;
  }
  for (let p = 0; p < board.length; p++) if (!frosty(board[p]) && board[p] !== SEED) expect(when[p]).toBe(NEVER);
});

test("a square's depth in the frost tree is the turn it froze", () => {
  let state = deal({ size: 13, frost: 2, flowers: 4 }, { seed: 3 });
  const when = future(state.size, state.board);
  while (!state.over) state = step(state, WAIT);
  const deep = depths(state);
  for (let p = 0; p < state.board.length; p++)
    if (frosty(state.board[p])) expect(deep[p]).toBe(state.board[p] === SEED ? 0 : when[p]);
});

test("the round ends once no flower is left to lose, and the frost then grows out to the end", () => {
  let state = deal({ size: 15, frost: 2, flowers: 5 }, { seed: 4 });
  while (!state.over) state = step(state, WAIT);
  expect([saved(state), grow(state.size, state.board) === state.board, moves(state)]).toEqual([0, true, []]);
});

test("every deal is a field to save: its flowers, each one lost if no stone is dropped", () => {
  for (let seed = 0; seed < 30; seed++) {
    const flowers = 1 + (seed % 12);
    const state = boot({ size: 9 + (seed % 17), frost: 1 + (seed % 4), flowers, seed });
    const when = future(state.size, state.board);
    const blooms = [...state.board].flatMap((one, p) => (one === FLOWER ? [when[p]] : []));
    expect([seed, state.over, blooms.length, blooms.every((t) => t >= 3 && t < NEVER)]).toEqual([
      seed,
      null,
      flowers,
      true,
    ]);
  }
});

const GAME: Act[] = [drop(1, 1), WAIT, drop(7, 7), drop(3, 9), WAIT];

const replay = (seed: number) => GAME.reduce(step, deal({ size: 15, frost: 2, flowers: 5 }, { seed }));

test("the same seed and moves replay to the same state, byte for byte", () => {
  expect(JSON.stringify(replay(42))).toBe(JSON.stringify(replay(42)));
  expect(JSON.stringify(replay(42))).not.toBe(JSON.stringify(replay(43)));
  expect(deal({ size: 15, frost: 2, flowers: 5 }, { seed: 42, round: 1 }).board).not.toBe(
    deal({ size: 15, frost: 2, flowers: 5 }, { seed: 42 }).board,
  );
});

test("step leaves its input untouched and its state holds only integers and strings", () => {
  let state = deal({ size: 15, frost: 2, flowers: 5 }, { seed: 42 });
  for (const act of GAME) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), act);
    expect(JSON.stringify(state)).toBe(before);
    expect(JSON.parse(JSON.stringify(next))).toEqual(next);
    state = next;
  }
  expect(Object.values(state).every((one) => one === null || typeof one === "string" || Number.isInteger(one))).toBe(
    true,
  );
});

test("moves lists the wait and a stone on every empty square the frost reaches next, and each one moves the game", () => {
  const state = field(5, [
    [2, 2, SEED],
    [0, 0, FLOWER],
    [4, 4, STONE],
  ]);
  const all = moves(state);
  expect([all[0], all.length]).toEqual([WAIT, 5]);
  for (const act of all) expect(step(state, act)).not.toBe(state);
});

test("read walks a slash path into the state, and knobs out of range are refused", () => {
  const state = deal({ size: 15, frost: 2, flowers: 5 }, { seed: 1 });
  expect([read(state, "flowers"), read(state, "board/x"), read(state, "nope/x")]).toEqual([5, undefined, undefined]);
  expect(() => boot({ size: 8, frost: 2, flowers: 5, seed: 1 })).toThrow();
  expect(() => boot({ size: 15, frost: 5, flowers: 5, seed: 1 })).toThrow();
  expect(() => boot({ size: 15, frost: 2, flowers: 13, seed: 1 })).toThrow();
});
