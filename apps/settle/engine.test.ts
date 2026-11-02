import { expect, test } from "bun:test";
import type { Borders, Own, State } from "./engine.ts";
import {
  ARRIVAL,
  arrives,
  boot,
  DAYS,
  deal,
  dealt,
  EMPTY,
  H,
  HOME,
  housed,
  MILLE,
  members,
  moves,
  NONE,
  passport,
  read,
  report,
  run,
  SEA,
  step,
  TENT,
  tick,
  W,
  xy,
} from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const world = (own: Own, seed: number, more: { bots?: number; city?: number; borders?: Borders } = {}) =>
  boot({ bots: 64, own, city: 16, borders: "open", length: DAYS, seed, ...more });

const done = (own: Own, seed: number, more: { bots?: number; city?: number; borders?: Borders } = {}) =>
  run(world(own, seed, more), DAYS);

const coast = (state: State, i: number) => {
  const [x, y] = xy(i);
  return [
    [1, 0],
    [-1, 0],
    [0, 1],
    [0, -1],
  ].some(([dx, dy]) => state.ground[(y + dy) * W + x + dx] === SEA);
};

test("the same seed and days replay to the same state, byte for byte, and another seed another", () => {
  const once = (seed: number) => JSON.stringify(run(world("none", seed), 50));
  expect(once(42)).toBe(once(42));
  expect(once(42)).not.toBe(once(43));
});

test("step leaves its input untouched and its state holds only integers, strings and nulls", () => {
  let state = world("private", 7);
  const flat = (x: unknown): unknown[] => (x && typeof x === "object" ? Object.values(x).flatMap(flat) : [x]);
  for (let i = 0; i < 30; i++) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), tick);
    expect(JSON.stringify(state)).toBe(before);
    state = next;
  }
  expect(flat(state).every((one) => one === null || typeof one === "string" || Number.isInteger(one))).toBe(true);
});

test("an island is born empty in the sea, its rim all water, the submarine docked beside its shore", () => {
  for (let seed = 0; seed < 10; seed++) {
    const state = world("shared", seed);
    const rim = state.ground.filter((_, i) => {
      const [x, y] = xy(i);
      return x === 0 || y === 0 || x === W - 1 || y === H - 1;
    });
    expect([
      rim.every((one) => one === SEA),
      state.use.every((one) => one === EMPTY),
      state.ground[state.dock],
      state.ground[state.shore] !== SEA && coast(state, state.shore),
      state.region.every((one, i) => (one === NONE) === (state.ground[i] === SEA)),
    ]).toEqual([true, true, SEA, true, true]);
  }
});

test("the bots come ashore over sixty days, every one by then, and the run ends on its last day", () => {
  const state = world("shared", 3);
  expect([arrives(state, 0), arrives(state, state.bots - 1) <= ARRIVAL]).toEqual([1, true]);
  const landed = Array.from({ length: DAYS }, (_, day) => run(state, day + 1).landed).flat();
  const end = run(state, DAYS);
  expect([landed.length, new Set(landed).size, end.over, moves(end), step(end, tick)]).toEqual([
    state.bots,
    state.bots,
    "done",
    [],
    end,
  ]);
});

test("first come fences plots: each bot owns its home, the rest own nothing", () => {
  const fenced = done("private", 5);
  const owners = fenced.home.filter((tile, bot) => housed(fenced, bot) && fenced.owner[tile] === bot).length;
  expect(owners).toBeGreaterThan(fenced.bots / 2);
  for (const own of ["shared", "none"] as const) expect(done(own, 5).owner.every((one) => one === NONE)).toBe(true);
});

test("everyone stacks homes into towers when the town grows; nobody never does", () => {
  const tall = (own: Own) => Math.max(...done(own, 9, { city: 128 }).level);
  expect([tall("shared") > 1, tall("none")]).toEqual([true, 1]);
});

test("a full town sends the next bot to found another, so no town holds more than its size", () => {
  const state = done("shared", 4, { city: 8 });
  const sizes = state.towns.map((_, town) => members(state, town).length);
  expect([state.towns.length >= 8, Math.max(...sizes)]).toEqual([true, 8]);
});

test("fenced borders keep every bot in the land its passport names", () => {
  const state = done("none", 6, { borders: "closed" });
  const homes = state.home.flatMap((tile, bot) => (tile === NONE ? [] : [[state.region[tile], passport(state, bot)]]));
  expect(homes.length).toBeGreaterThan(0);
  for (const [region, own] of homes) expect(region).toBe(own);
});

test("a crowd on fenced plots ends on the beach: tents only by the sea, and fewer housed than everyone housed", () => {
  const more = { bots: 128, borders: "closed" as const };
  const crowd = done("private", 1, more);
  const tents = crowd.use.flatMap((use, i) => (use === TENT ? [i] : []));
  expect([tents.length > 0, tents.every((i) => coast(crowd, i))]).toEqual([true, true]);
  expect(report(crowd).housed).toBeLessThan(report(done("shared", 1, more)).housed);
});

test("the report counts in thousandths of the bots ashore", () => {
  const state = done("shared", 2);
  const told = report(state);
  expect([told.ashore, told.housed, told.fed, told.towns, told.fenced]).toEqual([64, MILLE, MILLE, 4, 0]);
  for (const key of ["green", "reach"] as const) expect(told[key] >= 0 && told[key] <= MILLE).toBe(true);
  expect(state.use.filter((one) => one === HOME).length).toBeGreaterThan(0);
});

test("deal boots a hundred days from the seed, a round deals another island, and read walks a path", () => {
  const knobs = { bots: 32, own: "none" as const, city: 8, borders: "open" as const };
  expect(deal(knobs, { seed: 5 })).toEqual(boot({ ...knobs, length: DAYS, seed: 5 }));
  expect(deal(knobs, { seed: 5, round: 2 })).toEqual(boot({ ...knobs, length: DAYS, seed: dealt(5, 2) }));
  expect(deal(knobs, { seed: 5, round: 1 }).ground).not.toEqual(deal(knobs, { seed: 5 }).ground);
  const state = deal(knobs, { seed: 5 });
  expect([read(state, "bots"), read(state, "home/0"), read(state, "nope/deeper"), read(state)]).toEqual([
    32,
    NONE,
    undefined,
    state,
  ]);
});
