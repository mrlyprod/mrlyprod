import { expect, test } from "bun:test";
import type { Answers, State } from "./engine.ts";
import {
  BRANCH,
  boot,
  CAP,
  DAYS,
  deal,
  dealt,
  knows,
  moves,
  read,
  report,
  run,
  step,
  TOLD,
  tick,
  UNDERSTOOD,
  WORLDS,
} from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const school = (answers: Answers, seed: number, pupils = 16) => boot({ pupils, answers, length: DAYS, seed });

const days = (state: State) => {
  const out = [state];
  while (!out[out.length - 1].over) out.push(step(out[out.length - 1], tick));
  return out;
};

const mean = (answers: Answers, key: "skill" | "asked" | "taught") => {
  const seeds = [0, 1, 2, 3, 4, 5];
  return seeds.reduce((sum, seed) => sum + report(run(school(answers, seed), DAYS))[key], 0) / seeds.length;
};

test("the same seed and days replay to the same state, byte for byte, and another seed another", () => {
  const once = (seed: number) => JSON.stringify(run(school("wiki", seed), 60));
  expect(once(42)).toBe(once(42));
  expect(once(42)).not.toBe(once(43));
});

test("step leaves its input untouched and its state holds only integers, strings and nulls", () => {
  let state = school("oracle", 7);
  const flat = (x: unknown): unknown[] => (x && typeof x === "object" ? Object.values(x).flatMap(flat) : [x]);
  for (let i = 0; i < 60; i++) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), tick);
    expect(JSON.stringify(state)).toBe(before);
    state = next;
  }
  expect(flat(state).every((one) => one === null || typeof one === "string" || Number.isInteger(one))).toBe(true);
});

test("a school opens knowing only the root and runs a hundred and twenty days to its end", () => {
  const start = school("off", 3);
  const end = run(start, DAYS);
  expect([report(start).understood, report(start).open, end.day, end.over, moves(end), step(end, tick)]).toEqual([
    1,
    start.parent.length - 1,
    DAYS,
    "done",
    [],
    end,
  ]);
});

test("the tree stays a tree: each puzzle one deeper than its parent, at most three children, never past the cap", () => {
  for (const answers of WORLDS) {
    const end = run(school(answers, 5, 32), DAYS);
    const kids = end.parent.map((_, p) => end.parent.filter((one, c) => c > 0 && one === p).length);
    expect([
      end.parent.length <= CAP,
      end.parent.every((one, p) => !p || (one < p && end.depth[p] === end.depth[one] + 1)),
      kids.every((n) => n <= BRANCH),
      end.know.length,
    ]).toEqual([true, true, true, end.parent.length * end.pupils]);
  }
});

test("what a pupil knows it keeps, and an invented puzzle grows on one its inventor understands", () => {
  for (const answers of WORLDS) {
    const all = days(school(answers, 9));
    for (let i = 1; i < all.length; i++) {
      const [was, now] = [all[i - 1], all[i]];
      expect(was.know.every((one, k) => now.know[k] >= one)).toBe(true);
      for (const p of now.posed) expect(knows(was, now.parent[p], now.by[p])).toBe(UNDERSTOOD);
    }
  }
});

test("with no answers nobody asks; a wiki answers only what some pupil has worked out; an oracle answers what nobody has", () => {
  const told = (answers: Answers) => days(school(answers, 4)).some((state) => report(state).told > 0);
  const off = run(school("off", 4), DAYS);
  expect([off.asked, off.know.includes(TOLD), told("wiki"), told("oracle")]).toEqual([0, false, false, true]);
});

test("in this toy, answers at hand trade lessons and skill for questions: more asked, fewer taught, less understood", () => {
  expect(mean("oracle", "asked")).toBeGreaterThan(mean("off", "asked"));
  expect(mean("off", "taught")).toBeGreaterThan(Math.max(mean("wiki", "taught"), mean("oracle", "taught")));
  expect(mean("off", "skill")).toBeGreaterThan(Math.max(mean("wiki", "skill"), mean("oracle", "skill")));
});

test("boot refuses an unknown world or too few pupils", () => {
  expect(() => school("magic" as Answers, 1)).toThrow("school: no world magic");
  expect(() => school("off", 1, 1)).toThrow("school: pupils run from 2");
});

test("deal deals round zero from the seed itself and each later round another school", () => {
  const first = deal({ pupils: 16, answers: "wiki" }, { seed: 4 });
  const later = deal({ pupils: 16, answers: "wiki" }, { seed: 4, round: 1 });
  expect([first, dealt(4, 0), JSON.stringify(later) === JSON.stringify(first)]).toEqual([school("wiki", 4), 4, false]);
});

test("read walks a path into the state", () => {
  const state = school("wiki", 2);
  expect([read(state, "answers"), read(state, "parent/0"), read(state, "nope/x"), read(state)]).toEqual([
    "wiki",
    0,
    undefined,
    state,
  ]);
});
