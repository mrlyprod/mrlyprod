import { expect, test } from "bun:test";
import type { State } from "./engine.ts";
import {
  boot,
  cost,
  DAYS,
  deal,
  dealt,
  MAX,
  moves,
  NONE,
  PAR,
  PATIENCE,
  PRICE,
  pair,
  pairs,
  read,
  report,
  run,
  step,
  TOWNS,
  tick,
} from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const world = (currencies: number, seed: number, fee = 3, traders = 48) =>
  boot({ currencies, fee, traders, length: DAYS, seed });

const days = (state: State) => {
  const out = [state];
  while (!out[out.length - 1].over) out.push(step(out[out.length - 1], tick));
  return out;
};

const held = (state: State) => state.wallet.reduce((a, b) => a + b, 0) + state.purse.reduce((a, b) => a + b, 0);

test("one seed draws one map: the same towns for every count of coins, split into that many regions", () => {
  const maps = [1, 2, 3, 4, 5].map((n) => world(n, 7));
  expect(maps.map((one) => new Set(one.region).size)).toEqual([1, 2, 3, 4, 5]);
  expect(maps.every((one) => JSON.stringify([one.tx, one.ty]) === JSON.stringify([maps[0].tx, maps[0].ty]))).toBe(true);
  expect([
    maps[4].tx.length,
    JSON.stringify(world(5, 7)),
    JSON.stringify(world(5, 8)) !== JSON.stringify(maps[4]),
  ]).toEqual([TOWNS, JSON.stringify(maps[4]), true]);
});

test("every capital lies in its own region, and every pair of regions has one changer on the second's land", () => {
  for (const seed of [0, 1, 2, 3])
    for (let n = 1; n <= MAX; n++) {
      const state = world(n, seed);
      expect(state.capital.slice(0, n).map((town) => state.region[town])).toEqual(
        Array.from({ length: n }, (_, c) => c),
      );
      expect(state.bx.filter((one) => one !== NONE).length).toBe((n * (n - 1)) / 2);
      expect(pairs(n).length).toBe((n * (n - 1)) / 2);
    }
});

test("step never touches its input, and the state is plain integers, strings and nulls", () => {
  const start = freeze(world(5, 3));
  const next = step(start, tick);
  const all = days(world(5, 3));
  const flat = JSON.stringify(all[all.length - 1]);
  expect([next.day, start.day, JSON.parse(flat)]).toEqual([1, 0, all[all.length - 1]]);
  expect(/\d\.\d|e[+-]/.test(flat)).toBe(false);
});

test("with one coin nothing is ever changed: no fee lost, no trip failed, and every coin stays in the world", () => {
  for (const seed of [0, 1, 2]) {
    const all = days(world(1, seed));
    const end = all[all.length - 1];
    expect([report(end).lost, end.failed, end.waited, report(end).booths, end.traded > 0]).toEqual([0, 0, 0, 0, true]);
    expect(all.every((one) => held(one) === one.traders * 100)).toBe(true);
  }
});

test("a change costs the price at the two rates, plus the fee", () => {
  const rate = [PAR, PAR * 2, PAR, PAR, PAR];
  expect([cost({ rate, fee: 3 }, 0, 2), cost({ rate, fee: 0 }, 0, 1), cost({ rate, fee: 10 }, 1, 0)]).toEqual([
    { base: PRICE, need: 21 },
    { base: PRICE * 2, need: PRICE * 2 },
    { base: PRICE / 2, need: 11 },
  ]);
});

test("with five coins a run changes money, loses its fees and never lets a coin go negative", () => {
  const all = days(world(5, 4, 5));
  const end = all[all.length - 1];
  expect([end.lost > 0, end.traded > 0, all.every((one) => [...one.wallet, ...one.till].every((n) => n >= 0))]).toEqual(
    [true, true, true],
  );
});

test("a changer with an empty side holds its traders, and after their patience they give up and walk home", () => {
  const start = world(2, 5);
  const p = pair(0, 1);
  const dry = { ...start, till: start.till.map((one, i) => (i === p * 2 || i === p * 2 + 1 ? 0 : one)) };
  const all = days(dry);
  const end = all[all.length - 1];
  expect([end.failed > 0, end.lost, all.every((one) => one.wait.every((w) => w <= PATIENCE))]).toEqual([true, 0, true]);
  expect(end.gave.every((trader) => end.goal[trader] === end.home[trader] && end.buy[trader] === 0)).toBe(true);
});

test("the rates drift inside their band, and one coin keeps its rate", () => {
  const five = run(world(5, 2), DAYS);
  const one = run(world(1, 2), DAYS);
  expect([
    five.rate.slice(0, 5).some((r) => r !== PAR),
    five.rate.every((r) => r >= 600 && r <= 1600),
    one.rate,
  ]).toEqual([true, true, Array(MAX).fill(PAR)]);
});

test("a run lasts its days, then offers no move and steps to itself", () => {
  const end = run(world(3, 1), DAYS + 5);
  expect([end.day, end.over, moves(end), step(end, tick) === end, moves(world(3, 1))]).toEqual([
    DAYS,
    "done",
    [],
    true,
    [tick],
  ]);
});

test("read follows a path into the state", () => {
  const state = world(3, 1);
  expect([read(state, "currencies"), read(state, "tx/2"), read(state, "nope/deeper"), read(state)]).toEqual([
    3,
    state.tx[2],
    undefined,
    state,
  ]);
});

test("deal boots the seed on round zero and a fresh seed each round after", () => {
  const knobs = { currencies: 4, fee: 3, traders: 48 };
  expect(deal(knobs, { seed: 9 })).toEqual(world(4, 9));
  expect(deal(knobs, { seed: 9, round: 2 })).toEqual(world(4, dealt(9, 2)));
  expect(dealt(9, 1)).not.toBe(dealt(9, 2));
});

test("boot refuses a count of coins or traders outside the toy", () => {
  expect(() => world(0, 1)).toThrow();
  expect(() => world(6, 1)).toThrow();
  expect(() => world(3, 1, 3, 1)).toThrow();
});
