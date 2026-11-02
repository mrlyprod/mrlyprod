import { expect, test } from "bun:test";
import type { Act, State } from "./engine.ts";
import { boot, deal, fold, HIGH, LOW, moves, par, plan, punch, read, sheet, stack, step, X, Y } from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const blank = (size: number, target: string): State => ({
  size,
  target,
  holes: "0".repeat(size * size),
  folds: [],
  moves: 0,
  par: 0,
  over: null,
  seed: 1,
});

const solve = (state: State) => plan(state).reduce(step, state);

test("a fold lays the flap over the rest, mirrored across the crease, and the sheet shrinks to the rest", () => {
  const state = step(blank(6, "1".repeat(36)), fold(X, 2, LOW));
  const { xs, ys } = sheet(state);
  expect([Array.from(xs.at), xs.lo, xs.hi, ys.lo, ys.hi]).toEqual([[3, 2, 2, 3, 4, 5], 2, 6, 0, 6]);
  const twice = step(state, fold(Y, 3, HIGH));
  expect(stack(twice, sheet(twice), 2, 0)).toEqual([1, 2, 31, 32]);
});

test("a flap wider than the rest, a crease off the sheet and a broken side are refused", () => {
  const state = blank(6, "1".repeat(36));
  for (const act of [
    fold(X, 4, LOW),
    fold(X, 2, HIGH),
    fold(X, 0, LOW),
    fold(Y, 6, HIGH),
    fold(X, 3, 0),
    fold(2, 3, LOW),
  ])
    expect(step(state, act)).toBe(state);
});

test("a punch goes through every layer of its stack, and only where every layer wants a hole", () => {
  const target = "100001".padEnd(36, "0");
  const state = step(blank(6, target), fold(X, 3, HIGH));
  expect(step(state, punch(1, 0))).toBe(state);
  expect(step(state, punch(4, 0))).toBe(state);
  const done = step(state, punch(0, 0));
  expect([done.holes, done.over, done.moves]).toEqual([target, "done", 2]);
});

test("folding a ring still to punch onto plain paper leaves the sheet stuck", () => {
  const state = step(blank(4, "1000".padEnd(16, "0")), fold(X, 2, LOW));
  expect([state.over, moves(state)]).toEqual(["stuck", []]);
});

test("par is the fewest moves: a breadth-first search over every move agrees", () => {
  for (let seed = 1; seed <= 8; seed++) {
    const start = boot({ size: 4, folds: 2 + (seed % 3), seed });
    const name = (one: State) => `${one.holes}:${one.folds.join()}`;
    const seen = new Set([name(start)]);
    let ring = [start];
    let far = 0;
    while (!ring.some((one) => one.over === "done") && ring.length) {
      ring = ring.flatMap((one) =>
        moves(one)
          .map((act) => step(one, act))
          .filter((next) => !seen.has(name(next)) && seen.add(name(next))),
      );
      far++;
    }
    expect([seed, par(start)]).toEqual([seed, far]);
  }
});

test("following the plan makes the lace in exactly par, and a finished sheet takes no more", () => {
  for (const [size, folds, seed] of [
    [4, 1, 3],
    [6, 2, 5],
    [8, 3, 7],
    [9, 6, 42],
  ] as const) {
    const start = boot({ size, folds, seed });
    const end = solve(start);
    expect([size, end.over, end.moves, end.holes === start.target]).toEqual([size, "done", start.par, true]);
    expect(step(end, fold(X, 1, LOW))).toBe(end);
  }
});

test("every deal is a lace to make: some holes, some paper, and a par of at least one", () => {
  for (let seed = 0; seed < 30; seed++) {
    const state = boot({ size: 4 + (seed % 6), folds: 1 + (seed % 6), seed });
    expect([seed, state.over, state.target.includes("1"), state.target.includes("0"), state.par > 0]).toEqual([
      seed,
      null,
      true,
      true,
      true,
    ]);
  }
});

const GAME: Act[] = [fold(X, 4, HIGH), fold(Y, 3, LOW), fold(X, 2, LOW), punch(5, 5), fold(Y, 6, HIGH)];

const replay = (seed: number) => GAME.reduce(step, deal({ size: 8, folds: 3 }, { seed }));

test("the same seed and moves replay to the same state, byte for byte", () => {
  expect(JSON.stringify(replay(42))).toBe(JSON.stringify(replay(42)));
  expect(JSON.stringify(replay(42))).not.toBe(JSON.stringify(replay(43)));
  expect(deal({ size: 8, folds: 3 }, { seed: 42, round: 1 }).target).not.toBe(replay(42).target);
});

test("step leaves its input untouched and its state holds only integers and strings", () => {
  let state = deal({ size: 8, folds: 3 }, { seed: 42 });
  for (const act of plan(state)) {
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

test("moves lists every legal fold and punch, and each one moves the game", () => {
  const state = blank(4, "1001".padEnd(16, "0"));
  const all = moves(state);
  expect(all.filter((one) => one.verb === "fold")).toHaveLength(8);
  expect(all.filter((one) => one.verb === "punch")).toEqual<Act[]>([punch(0, 0), punch(3, 0)]);
  for (const act of all) expect(step(state, act)).not.toBe(state);
});

test("read walks a slash path into the state, and a sheet out of range is refused", () => {
  const state = deal({ size: 6, folds: 2 }, { seed: 1 });
  expect([read(state, "par"), read(state, "folds/0"), read(state, "nope/x")]).toEqual([
    state.par,
    undefined,
    undefined,
  ]);
  expect(() => boot({ size: 3, folds: 2, seed: 1 })).toThrow();
  expect(() => boot({ size: 10, folds: 2, seed: 1 })).toThrow();
});
