import { expect, test } from "bun:test";
import type { Inherit, State } from "./engine.ts";
import {
  ADULT,
  boot,
  COMMONS,
  deal,
  dealt,
  LAND,
  MILLE,
  moves,
  NONE,
  RULES,
  read,
  report,
  run,
  step,
  tick,
  wealth,
  YEARS,
} from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const world = (inherit: Inherit, seed: number, families = 16) => boot({ families, inherit, length: YEARS, seed });

const years = (state: State) => {
  const out = [state];
  while (!out[out.length - 1].over) out.push(step(out[out.length - 1], tick));
  return out;
};

const mean = (inherit: Inherit, key: "top" | "mobility") => {
  const seeds = [0, 1, 2, 3, 4, 5, 6, 7];
  return seeds.reduce((sum, seed) => sum + report(run(world(inherit, seed), YEARS))[key], 0) / seeds.length;
};

test("the same seed and years replay to the same state, byte for byte, and another seed another", () => {
  const once = (seed: number) => JSON.stringify(run(world("heir", seed), 80));
  expect(once(42)).toBe(once(42));
  expect(once(42)).not.toBe(once(43));
});

test("step leaves its input untouched and its state holds only integers, strings and nulls", () => {
  let state = world("commons", 7);
  const flat = (x: unknown): unknown[] => (x && typeof x === "object" ? Object.values(x).flatMap(flat) : [x]);
  for (let i = 0; i < 90; i++) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), tick);
    expect(JSON.stringify(state)).toBe(before);
    state = next;
  }
  expect(flat(state).every((one) => one === null || typeof one === "string" || Number.isInteger(one))).toBe(true);
});

test("a world opens with no coins and no land and runs two hundred years to its end", () => {
  const start = world("heir", 3);
  const end = run(start, YEARS);
  expect([
    wealth(start, 0),
    start.land.every((one) => one === NONE),
    end.year,
    end.over,
    moves(end),
    step(end, tick),
  ]).toEqual([0, true, YEARS, "done", [], end]);
});

test("every year the land adds up: each family holds the plots it owns, and no coins go below zero", () => {
  for (const inherit of RULES)
    for (const state of years(world(inherit, 5))) {
      const owned = state.tiles.map((_, family) => state.land.filter((one) => one === family).length);
      expect([state.land.length, owned, state.coins.every((one) => one >= 0)]).toEqual([
        state.families * LAND,
        state.tiles,
        true,
      ]);
    }
});

test("to the heir a child keeps its parent's coins and land; to nobody it starts with nothing and the land is freed", () => {
  for (const inherit of ["heir", "none"] as Inherit[]) {
    const all = years(world(inherit, 5));
    for (let i = 1; i < all.length; i++)
      for (const family of all[i].faded) {
        const was = all[i - 1];
        const kept = inherit === "heir" ? was.tiles[family] : 0;
        expect([all[i].age[family], all[i].tiles[family]]).toEqual([0, kept]);
        if (inherit === "none") expect(all[i].coins[family]).toBe(0);
      }
    const end = all[all.length - 1];
    expect(report(end).nothing).toBe(inherit === "heir" ? 0 : MILLE);
  }
});

test("to the commons the fading bots' land becomes everyone's for good, and its rent is shared", () => {
  const all = years(world("commons", 5));
  const commons = all.map((state) => state.land.filter((one) => one === COMMONS).length);
  const end = all[all.length - 1];
  expect([commons.every((n, i) => !i || n >= commons[i - 1]), commons[commons.length - 1] > 0, end.shared > 0]).toEqual(
    [true, true, true],
  );
});

test("a child grows up before it works: no bot under age makes coins of its own in a world with nothing passed on", () => {
  const all = years(world("none", 9));
  for (let i = 1; i < all.length; i++)
    for (let family = 0; family < all[i].families; family++)
      if (all[i].age[family] && all[i].age[family] < ADULT) expect(all[i].coins[family]).toBe(0);
});

test("in this toy, inheritance grows dynasties: the richest fifth holds more and fewer children change fifth than with the commons or nobody", () => {
  expect(mean("heir", "top")).toBeGreaterThan(Math.max(mean("commons", "top"), mean("none", "top")));
  expect(mean("heir", "mobility")).toBeLessThan(Math.min(mean("commons", "mobility"), mean("none", "mobility")));
});

test("boot refuses an unknown rule or too few families", () => {
  expect(() => world("gift" as Inherit, 1)).toThrow("roots: no rule gift");
  expect(() => world("heir", 1, 1)).toThrow("roots: families run from 2");
});

test("deal deals round zero from the seed itself and each later round another world", () => {
  const first = deal({ families: 16, inherit: "heir" }, { seed: 4 });
  const later = deal({ families: 16, inherit: "heir" }, { seed: 4, round: 1 });
  expect([first, dealt(4, 0), JSON.stringify(later) === JSON.stringify(first)]).toEqual([world("heir", 4), 4, false]);
});

test("read walks a path into the state", () => {
  const state = world("heir", 2);
  expect([read(state, "inherit"), read(state, "age/0"), read(state, "nope/x"), read(state)]).toEqual([
    "heir",
    state.age[0],
    undefined,
    state,
  ]);
});
