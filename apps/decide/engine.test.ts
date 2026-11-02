import { expect, test } from "bun:test";
import type { Mode, State, Wish } from "./engine.ts";
import { boot, median, moves, read, report, run, step, tick, WEEK } from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const village = (decide: Mode, seed: number) => boot({ bots: 5, decide, eager: 30, patience: 6, length: WEEK, seed });

const wish = (id: number, by: number, hour: number, bots: number, more: Partial<Wish> = {}): Wish => ({
  id,
  by,
  hour,
  at: 0,
  judge: (by + 1) % bots,
  votes: Array.from({ length: bots }, (_, i) => (i === by ? "1" : "0")).join(""),
  end: -1,
  how: "open",
  ...more,
});

const world = (decide: Mode, hours: number[], wishes: Wish[], ruler = 0): State => ({
  bots: hours.length,
  decide,
  eager: 0,
  patience: 50,
  length: WEEK,
  tick: 0,
  hours,
  ruler,
  wishes,
  over: null,
  seed: 1,
});

const fate = (state: State) => state.wishes.map((one) => `${one.how}@${one.end}`);

test("the same seed and ticks replay to the same state, byte for byte", () => {
  const once = (seed: number) => JSON.stringify(run(village("vote", seed), 60));
  expect(once(42)).toBe(once(42));
  expect(once(42)).not.toBe(once(43));
});

test("step leaves its input untouched and its state holds only integers, strings and nulls", () => {
  let state = village("lottery", 7);
  const flat = (x: unknown): unknown[] => (x && typeof x === "object" ? Object.values(x).flatMap(flat) : [x]);
  for (let i = 0; i < 30; i++) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), tick);
    expect(JSON.stringify(state)).toBe(before);
    state = next;
  }
  expect(flat(state).every((one) => one === null || typeof one === "string" || Number.isInteger(one))).toBe(true);
});

test("vote: each bot casts one vote a tick on the oldest wish it has not voted on, and a majority settles it", () => {
  const start = world("vote", [0, 0, 0, 6, 6], [wish(0, 3, 6, 5), wish(1, 0, 0, 5)]);
  const one = step(start, tick);
  expect([fate(one), one.wishes[0].votes, one.wishes[1].votes]).toEqual([["refused@1", "open@-1"], "22210", "10022"]);
  expect(fate(step(one, tick))).toEqual(["refused@1", "granted@2"]);
});

test("ruler: the ruler alone rules, one wish a tick, oldest first, by its own taste, and grants its own", () => {
  const start = world("ruler", [0, 6, 0], [wish(0, 0, 0, 3), wish(1, 1, 0, 3), wish(2, 2, 6, 3)], 1);
  expect([1, 2, 3].map((n) => fate(run(start, n)))).toEqual([
    ["refused@1", "open@-1", "open@-1"],
    ["refused@1", "granted@2", "open@-1"],
    ["refused@1", "granted@2", "granted@3"],
  ]);
});

test("lottery: each wish's drawn judge rules on one wish a tick, and a judge is never the wish's own bot", () => {
  const start = world("lottery", [0, 6, 0], [wish(0, 0, 0, 3), wish(1, 2, 0, 3, { judge: 1 }), wish(2, 1, 6, 3)]);
  expect([fate(step(start, tick)), fate(run(start, 2))]).toEqual([
    ["refused@1", "open@-1", "refused@1"],
    ["refused@1", "refused@2", "refused@1"],
  ]);
  for (let seed = 0; seed < 20; seed++)
    expect(run(village("lottery", seed), 40).wishes.every((one) => one.judge !== one.by)).toBe(true);
});

test("a wish left open for patience ticks is taken back", () => {
  const start = { ...world("ruler", [0, 6, 0], [wish(0, 0, 0, 3), wish(1, 2, 6, 3)], 1), patience: 1 };
  expect(fate(step(start, tick))).toEqual(["refused@1", "dropped@1"]);
});

test("the readouts are exact: granted in thousandths of wishes posted, wait the median ticks to a decision, reach the bots granted", () => {
  const state = world(
    "vote",
    [0, 0, 0],
    [
      wish(0, 0, 0, 3, { how: "granted", at: 0, end: 1 }),
      wish(1, 0, 0, 3, { how: "granted", at: 1, end: 3 }),
      wish(2, 2, 0, 3, { how: "granted", at: 0, end: 4 }),
      wish(3, 1, 0, 3, { how: "refused", at: 2, end: 7 }),
      wish(4, 1, 0, 3, { how: "dropped", at: 0, end: 9 }),
      wish(5, 1, 0, 3),
      wish(6, 0, 0, 3),
    ],
  );
  expect(report(state)).toEqual({ posted: 7, granted: 428, wait: 3000, reach: 2, dropped: 1 });
  expect([median([3, 1, 2]), median([]), median([1, 2])]).toEqual([2000, 0, 1500]);
});

test("a week of five bots at seeds 1 to 3 reports these readouts in each mode", () => {
  const week = (decide: Mode) =>
    [1, 2, 3].map((seed) => {
      const { granted, wait, reach } = report(run(village(decide, seed), WEEK));
      return [granted, wait, reach];
    });
  expect({ vote: week("vote"), ruler: week("ruler"), lottery: week("lottery") }).toEqual({
    vote: [
      [416, 2000, 5],
      [570, 2000, 5],
      [553, 2000, 5],
    ],
    ruler: [
      [530, 3000, 5],
      [566, 3000, 5],
      [562, 3000, 4],
    ],
    lottery: [
      [371, 1000, 5],
      [422, 1000, 5],
      [368, 1000, 5],
    ],
  });
});

test("a run ends after its length with no moves left, and a tick past it changes nothing", () => {
  const end = run(village("vote", 3), WEEK + 5);
  expect([end.tick, end.over, moves(end), step(end, tick) === end, moves(village("vote", 3))]).toEqual([
    WEEK,
    "done",
    [],
    true,
    [tick],
  ]);
});

test("read walks a slash path into the state", () => {
  const state = village("ruler", 1);
  expect([read(state, "ruler"), read(state, "hours/0"), read(state, "nope/x")]).toEqual([
    state.ruler,
    state.hours[0],
    undefined,
  ]);
});
