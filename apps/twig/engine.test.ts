import { expect, test } from "bun:test";
import type { Act, State } from "./engine.ts";
import { block, boot, deal, dealt, moves, neat, read, solve, sorted, spin, step } from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const tree = (leaves: string): State => ({
  levels: Math.log2(leaves.length),
  leaves,
  taps: 0,
  par: 0,
  over: null,
  seed: 1,
});

test("a fork's crown is the run of leaves above it, and a sorted tree counts in base 36", () => {
  expect([block(3, 0), block(3, 2), block(3, 5)]).toEqual([
    [0, 8],
    [4, 8],
    [4, 6],
  ]);
  expect(sorted(5)).toBe("0123456789abcdefghijklmnopqrstuv");
});

test("a spin turns its whole crown around, and step never changes its input", () => {
  const start = freeze(tree("01234567"));
  expect(step(start, spin(0)).leaves).toBe("76543210");
  expect(step(start, spin(2)).leaves).toBe("01237654");
  expect(step(start, spin(6)).leaves).toBe("01234576");
  expect([start.leaves, step(start, spin(1)).taps]).toEqual(["01234567", 1]);
  for (const act of [spin(7), spin(-1), spin(1.5), { verb: "turn" }] as Act[]) expect(step(start, act)).toBe(start);
});

test("the tree blooms once every leaf is in order, and a bloomed tree takes no move", () => {
  const start = tree("1023");
  const done = step(start, spin(1));
  expect([neat(start, 0), neat(start, 2), done.over]).toEqual([false, true, "done"]);
  expect([moves(done), step(done, spin(0))]).toEqual([[], done]);
  expect(moves(start)).toEqual([spin(0), spin(1), spin(2)]);
});

test("solve spins from the trunk up", () => {
  expect(solve(tree("45670123"))).toEqual([0, 1, 2]);
  expect(solve(tree("01234567"))).toEqual([]);
});

test("a deal is unbloomed, its par is what solve needs, and one seed deals one tree", () => {
  for (const seed of [1, 2, 3, 4, 5]) {
    const state = boot({ levels: 4, seed });
    expect([state.over, state.par > 0, state.par]).toEqual([null, true, solve(state).length]);
    expect(boot({ levels: 4, seed })).toEqual(state);
  }
  expect(boot({ levels: 5, seed: 1 }).leaves).not.toBe(boot({ levels: 5, seed: 2 }).leaves);
});

test("boot refuses a tree out of range", () => {
  for (const levels of [1, 6, 3.5]) expect(() => boot({ levels, seed: 1 })).toThrow();
});

test("deal boots the round's seed, and read walks a path", () => {
  const state = deal({ levels: 3 }, { seed: 7, round: 2 });
  expect(state).toEqual(boot({ levels: 3, seed: dealt(7, 2) }));
  expect([read(state, "levels"), read(state, "par"), read(state, "leaves/x")]).toEqual([3, state.par, undefined]);
});
