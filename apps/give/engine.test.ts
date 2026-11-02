import { expect, test } from "bun:test";
import type { Economy, State } from "./engine.ts";
import { boot, CAP, deal, KINDS, moves, NONE, PATIENCE, read, report, run, step, tick, WEEK } from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const village = (economy: Economy, seed: number) =>
  boot({ bots: 8, economy, plenty: 40, generous: 70, purse: 3, length: WEEK, seed });

const world = (economy: Economy, crafts: number[], more: Partial<State> = {}): State => ({
  bots: crafts.length,
  economy,
  plenty: 0,
  generous: 100,
  purse: 3,
  length: 3,
  tick: 2,
  crafts,
  stock: crafts.map(() => 0),
  coins: crafts.map(() => 3),
  store: Array(KINDS).fill(0),
  needs: Array(crafts.length * KINDS).fill(NONE),
  fed: crafts.map(() => 0),
  missed: crafts.map(() => 0),
  made: 0,
  posted: 0,
  spilled: 0,
  flows: [],
  over: null,
  seed: 1,
  ...more,
});

const needing = (bots: number, wants: [number, number, number][]) => {
  const needs = Array(bots * KINDS).fill(NONE);
  for (const [bot, kind, since] of wants) needs[bot * KINDS + kind] = since;
  return needs;
};

const ring = (economy: Economy, more: Partial<State> = {}) =>
  world(economy, [1, 2, 2, 3, 0, 1, 3], {
    stock: [0, 0, 0, 0, 1, 0, 0],
    needs: needing(7, [
      [0, 0, 0],
      [3, 0, 1],
    ]),
    ...more,
  });

const open = (state: State) => state.needs.flatMap((since, i) => (since === NONE ? [] : [i]));

test("the same seed and ticks replay to the same state, byte for byte", () => {
  const once = (seed: number) => JSON.stringify(run(village("gift", seed), 60));
  expect(once(42)).toBe(once(42));
  expect(once(42)).not.toBe(once(43));
});

test("step leaves its input untouched and its state holds only integers, strings and nulls", () => {
  let state = village("pool", 7);
  const flat = (x: unknown): unknown[] => (x && typeof x === "object" ? Object.values(x).flatMap(flat) : [x]);
  for (let i = 0; i < 30; i++) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), tick);
    expect(JSON.stringify(state)).toBe(before);
    state = next;
  }
  expect(flat(state).every((one) => one === null || typeof one === "string" || Number.isInteger(one))).toBe(true);
});

test("a village is born with every kind made, one thing in each basket and a full purse", () => {
  for (let seed = 0; seed < 20; seed++) {
    const state = boot({ bots: 5, economy: "money", plenty: 40, generous: 70, purse: 4, length: WEEK, seed });
    expect([new Set(state.crafts).size, state.stock, state.coins]).toEqual([KINDS, [1, 1, 1, 1, 1], [4, 4, 4, 4, 4]]);
  }
});

test("money: one coin buys from the nearest maker anywhere in the ring, oldest need first, and no coin no thing", () => {
  const paid = step(ring("money"), tick);
  expect([paid.flows, paid.coins, paid.fed, open(paid)]).toEqual([
    [{ how: "buy", from: 4, to: 0, kind: 0, coin: 1 }],
    [2, 3, 3, 3, 4, 3, 3],
    [1, 0, 0, 0, 0, 0, 0],
    [12],
  ]);
  const broke = step(ring("money", { coins: [0, 3, 3, 3, 3, 3, 3] }), tick);
  expect(broke.flows).toEqual([{ how: "buy", from: 4, to: 3, kind: 0, coin: 1 }]);
});

test("gift: a maker gives only within two bots, a coin of thanks when there is one, and a stingy one never", () => {
  const given = step(ring("gift"), tick);
  expect([given.flows, given.coins, open(given)]).toEqual([
    [{ how: "give", from: 4, to: 3, kind: 0, coin: 1 }],
    [3, 3, 3, 2, 4, 3, 3],
    [0],
  ]);
  const free = step(ring("gift", { coins: [3, 3, 3, 0, 3, 3, 3] }), tick);
  expect(free.flows).toEqual([{ how: "give", from: 4, to: 3, kind: 0, coin: 0 }]);
  expect(step(ring("gift", { generous: 0 }), tick).flows).toEqual([]);
});

test("pool: what is made goes to the store as generosity says, and the oldest need draws first", () => {
  const shared = step(world("pool", [0, 1, 2, 3], { plenty: 100 }), tick);
  expect([shared.flows.map((one) => one.how), shared.store, shared.stock]).toEqual([
    ["share", "share", "share", "share"],
    [1, 1, 1, 1],
    [0, 0, 0, 0],
  ]);
  const kept = step(world("pool", [0, 1, 2, 3], { plenty: 100, generous: 0 }), tick);
  expect([kept.store, kept.stock]).toEqual([
    [0, 0, 0, 0],
    [1, 1, 1, 1],
  ]);
  const drawn = step(
    world("pool", [0, 1, 2, 3], {
      store: [1, 0, 0, 0],
      needs: needing(4, [
        [2, 0, 1],
        [1, 0, 0],
      ]),
    }),
    tick,
  );
  expect([drawn.flows, open(drawn)]).toEqual([[{ how: "draw", from: -1, to: 1, kind: 0, coin: 0 }], [8]]);
});

test("a maker meets its own need from its own basket first, coin or no coin", () => {
  const state = world("money", [0, 1, 2, 3], {
    stock: [1, 0, 0, 0],
    coins: [0, 0, 0, 0],
    needs: needing(4, [[0, 0, 0]]),
  });
  expect(step(state, tick).flows).toEqual([{ how: "use", from: 0, to: 0, kind: 0, coin: 0 }]);
});

test("a thing made into a full basket or a full store spills", () => {
  const full = step(world("gift", [0, 1, 2, 3], { plenty: 100, stock: [CAP, 0, 0, 0] }), tick);
  expect([full.flows[0], full.spilled, full.made]).toEqual([{ how: "spill", from: 0, to: 0, kind: 0, coin: 0 }, 1, 4]);
  const store = step(world("pool", [0, 1, 2, 3], { plenty: 100, store: [CAP * 4, 0, 0, 0] }), tick);
  expect([store.flows[0], store.store]).toEqual([
    { how: "spill", from: -1, to: -1, kind: 0, coin: 0 },
    [CAP * 4, 1, 1, 1],
  ]);
});

test("a need left open for patience ticks is missed", () => {
  const state = world("money", [0, 1, 2, 3], {
    tick: PATIENCE - 1,
    length: PATIENCE,
    needs: needing(4, [[1, 0, 0]]),
  });
  const after = step(state, tick);
  expect([after.flows, after.missed, open(after)]).toEqual([
    [{ how: "miss", from: 1, to: 1, kind: 0, coin: 0 }],
    [0, 1, 0, 0],
    [],
  ]);
});

test("the readouts are exact: fed of posted, idle of made, top the best-fed fifth's share, hungry the bots missing more than they got", () => {
  const state = world("gift", [0, 1, 2, 3, 0, 1, 2, 3, 0, 1], {
    fed: [9, 1, 0, 4, 4, 2, 0, 0, 0, 0],
    missed: [0, 2, 1, 0, 0, 2, 0, 0, 0, 0],
    posted: 30,
    made: 40,
    spilled: 6,
  });
  expect(report(state)).toEqual({
    fed: 666,
    idle: 150,
    top: 650,
    fair: 200,
    made: 40,
    used: 20,
    spilled: 6,
    posted: 30,
    hungry: 2,
  });
});

test("a week of eight bots at seeds 1 to 3 reports these readouts in each economy", () => {
  const week = (economy: Economy) =>
    [1, 2, 3].map((seed) => {
      const { fed, idle, top } = report(run(village(economy, seed), WEEK));
      return [fed, idle, top];
    });
  expect({ money: week("money"), gift: week("gift"), pool: week("pool") }).toEqual({
    money: [
      [851, 281, 298],
      [835, 228, 285],
      [804, 288, 323],
    ],
    gift: [
      [813, 327, 304],
      [810, 273, 296],
      [759, 337, 330],
    ],
    pool: [
      [887, 197, 286],
      [853, 128, 293],
      [818, 216, 305],
    ],
  });
});

test("deal boots a week from the knobs, and each round of the table deals a new village", () => {
  const knobs = { bots: 6, economy: "gift" as const, plenty: 50, generous: 60, purse: 2 };
  expect(deal(knobs, { seed: 4 })).toEqual(boot({ ...knobs, length: WEEK, seed: 4 }));
  expect(JSON.stringify(deal(knobs, { seed: 4, round: 1 }))).not.toBe(
    JSON.stringify(deal(knobs, { seed: 4, round: 2 })),
  );
});

test("a run ends after its length with no moves left, and a tick past it changes nothing", () => {
  const end = run(village("money", 3), WEEK + 5);
  expect([end.tick, end.over, moves(end), step(end, tick) === end, moves(village("money", 3))]).toEqual([
    WEEK,
    "done",
    [],
    true,
    [tick],
  ]);
});

test("read walks a slash path into the state", () => {
  const state = village("pool", 1);
  expect([read(state, "crafts/0"), read(state, "economy"), read(state, "nope/x")]).toEqual([
    state.crafts[0],
    "pool",
    undefined,
  ]);
});
