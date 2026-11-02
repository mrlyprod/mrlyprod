import { expect, test } from "bun:test";
import type { Act, State } from "./engine.ts";
import { boot, deal, dealt, home, moves, read, solve, step, turn } from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const whorl = (turns: string, arms = 3): State => ({
  rings: turns.length,
  arms,
  turns,
  taps: 0,
  par: 0,
  over: null,
  seed: 1,
});

test("a turn moves its ring and the ring just outside one notch, wraps at the arms' period, and step never changes its input", () => {
  const start = freeze(whorl("0123"));
  expect(step(start, turn(1)).turns).toBe("0233");
  expect(step(start, turn(3)).turns).toBe("0120");
  expect([start.turns, step(start, turn(0)).taps]).toEqual(["0123", 1]);
  for (const act of [turn(4), turn(-1), turn(1.5), { verb: "spin" }] as Act[]) expect(step(start, act)).toBe(start);
});

test("the whorl is whole once every ring is home, and a whole whorl takes no move", () => {
  const start = whorl("0002", 4);
  const done = step(start, turn(3));
  expect([home(start), done.over, home(done)]).toEqual([3, "done", 4]);
  expect([moves(done), step(done, turn(0))]).toEqual([[], done]);
  expect(moves(start)).toEqual([turn(0), turn(1), turn(2), turn(3)]);
});

test("solve sets the rings from the middle out", () => {
  expect(solve(whorl("320"))).toEqual([0, 1, 2, 2, 2]);
  expect(solve(whorl("000"))).toEqual([]);
});

test("a deal is unsolved, its par is what solve needs, and one seed deals one whorl", () => {
  for (const seed of [1, 2, 3, 4, 5]) {
    const state = boot({ rings: 5, arms: 3, seed });
    expect([state.over, state.par > 0, state.par]).toEqual([null, true, solve(state).length]);
    expect(boot({ rings: 5, arms: 3, seed })).toEqual(state);
  }
  expect(boot({ rings: 6, arms: 2, seed: 1 }).turns).not.toBe(boot({ rings: 6, arms: 2, seed: 2 }).turns);
});

test("boot refuses a whorl out of range", () => {
  for (const config of [
    { rings: 2, arms: 3 },
    { rings: 8, arms: 3 },
    { rings: 5, arms: 1 },
    { rings: 5, arms: 5 },
    { rings: 4.5, arms: 3 },
  ])
    expect(() => boot({ ...config, seed: 1 })).toThrow();
});

test("deal boots the round's seed, and read walks a path", () => {
  const state = deal({ rings: 4, arms: 2 }, { seed: 7, round: 2 });
  expect(state).toEqual(boot({ rings: 4, arms: 2, seed: dealt(7, 2) }));
  expect([read(state, "rings"), read(state, "par"), read(state, "turns/x")]).toEqual([4, state.par, undefined]);
});
