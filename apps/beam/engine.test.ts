import { expect, test } from "bun:test";
import type { Act, State } from "./engine.ts";
import { boot, deal, dealt, EMPTY, FALL, lit, moves, RISE, read, STAR, solve, step, tilt, trace } from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const room = (size: number, marks: [number, number, string][], lamp = 0, way = 1): State => {
  const board = EMPTY.repeat(size * size).split("");
  for (const [x, y, one] of marks) board[y * size + x] = one;
  const text = board.join("");
  return {
    size,
    board: text,
    lamp,
    way,
    stars: [...text].filter((one) => one === STAR).length,
    taps: 0,
    par: 0,
    over: null,
    seed: 1,
  };
};

test("the beam turns up off a rising mirror, down off a falling one, and leaves the room", () => {
  const start = room(5, [
    [2, 0, FALL],
    [2, 3, RISE],
  ]);
  expect(trace(start).map((hop) => hop.p)).toEqual([0, 1, 2, 7, 12, 17, 16, 15]);
  expect(trace(start).at(-1)!.way).toBe(3);
  expect(trace({ ...start, board: start.board.replace(FALL, RISE) }).map((hop) => hop.p)).toEqual([0, 1, 2]);
});

test("a tap tilts a mirror and counts, a tap elsewhere does nothing, and step never changes its input", () => {
  const start = freeze(room(5, [[2, 0, FALL]]));
  const after = step(start, tilt(2, 0));
  expect([after.board[2], after.taps, start.board[2]]).toEqual([RISE, 1, FALL]);
  for (const act of [tilt(1, 0), tilt(5, 0), tilt(-1, 0), tilt(1.5, 0), { verb: "wait" }] as Act[])
    expect(step(start, act)).toBe(start);
  expect(moves(start)).toEqual([tilt(2, 0)]);
});

test("the room is done once the beam lights every star, and a done room takes no move", () => {
  const start = room(5, [
    [2, 0, RISE],
    [2, 1, STAR],
    [2, 2, FALL],
    [4, 2, STAR],
  ]);
  const half = step(step(start, tilt(2, 2)), tilt(2, 0));
  const done = step(half, tilt(2, 2));
  expect([start, half, done].map((one) => [lit(one).size, one.over, one.taps])).toEqual([
    [0, null, 0],
    [1, null, 2],
    [2, "done", 3],
  ]);
  expect([moves(done), step(done, tilt(2, 0))]).toEqual([[], done]);
});

test("solve names the fewest mirrors to tilt, in the beam's order", () => {
  const start = room(5, [
    [2, 0, RISE],
    [2, 2, RISE],
    [3, 2, STAR],
    [4, 4, RISE],
  ]);
  expect(solve(start)).toEqual([2, 12]);
  expect(solve(room(5, [[2, 2, STAR]]))).toEqual(null);
});

test("a deal is unsolved, its par is what solve needs, and one seed deals one room", () => {
  for (const seed of [1, 2, 3, 4, 5]) {
    const state = boot({ size: 9, mirrors: 16, stars: 5, seed });
    expect([state.over, state.par, lit(state).size < state.stars]).toEqual([null, solve(state)!.length, true]);
    expect(boot({ size: 9, mirrors: 16, stars: 5, seed })).toEqual(state);
  }
  expect(boot({ size: 9, mirrors: 16, stars: 5, seed: 1 }).board).not.toBe(
    boot({ size: 9, mirrors: 16, stars: 5, seed: 2 }).board,
  );
});

test("boot refuses a room out of range", () => {
  for (const config of [
    { size: 5, mirrors: 12, stars: 4 },
    { size: 9, mirrors: 3, stars: 4 },
    { size: 9, mirrors: 12, stars: 0 },
    { size: 9.5, mirrors: 12, stars: 4 },
  ])
    expect(() => boot({ ...config, seed: 1 })).toThrow();
});

test("deal boots the round's seed, and read walks a path", () => {
  const state = deal({ size: 8, mirrors: 12, stars: 4 }, { seed: 7, round: 2 });
  expect(state).toEqual(boot({ size: 8, mirrors: 12, stars: 4, seed: dealt(7, 2) }));
  expect([read(state, "size"), read(state, "par"), read(state, "board/x")]).toEqual([8, state.par, undefined]);
});
