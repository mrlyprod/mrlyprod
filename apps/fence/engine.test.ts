import { expect, test } from "bun:test";
import type { Prison, State } from "./engine.ts";
import {
  AWAY,
  BEDS,
  boot,
  deal,
  dealt,
  FENCE,
  GATE,
  HOURS,
  inside,
  LONG,
  moves,
  NONE,
  PRISONS,
  QUIETLY,
  read,
  report,
  ripe,
  roofless,
  run,
  step,
  tick,
  via,
} from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const village = (prison: Prison, seed: number, scarcity = 2, bots = 24) =>
  boot({ prison, scarcity, bots, length: HOURS, seed });

const hours = (state: State) => {
  const out = [state];
  while (!out[out.length - 1].over) out.push(step(out[out.length - 1], tick));
  return out;
};

test("one seed draws one village, the same for every rule, and another seed another", () => {
  const all = PRISONS.map((prison) => village(prison, 7));
  expect(
    all.every(
      (one) =>
        JSON.stringify([one.hx, one.hy, one.rx, one.grow]) ===
        JSON.stringify([all[0].hx, all[0].hy, all[0].rx, all[0].grow]),
    ),
  ).toBe(true);
  expect([
    JSON.stringify(village("repair", 7)) === JSON.stringify(all[2]),
    JSON.stringify(village("repair", 8)) !== JSON.stringify(all[2]),
  ]).toEqual([true, true]);
});

test("scarcity leaves some bots without a home, from none at all to half the village", () => {
  expect([0, 1, 2, 3, 4].map((scarcity) => village("none", 1, scarcity).home.filter((h) => h === NONE).length)).toEqual(
    [0, 1, 2, 3, 4].map((scarcity) => roofless(24, scarcity)),
  );
  expect([roofless(24, 0), roofless(24, 4)]).toEqual([0, 12]);
});

test("step never touches its input, and the state is plain integers, strings and nulls", () => {
  const start = freeze(village("repair", 3));
  const next = step(start, tick);
  const all = hours(village("removal", 3, 4));
  const flat = JSON.stringify(all[all.length - 1]);
  expect([next.hour, start.hour, JSON.parse(flat)]).toEqual([1, 0, all[all.length - 1]]);
  expect(/\d\.\d|e[+-]/.test(flat)).toBe(false);
});

test("with nobody desperate breaks are rare whatever the rule, and scarcity multiplies them", () => {
  for (const prison of PRISONS) {
    const breaks = (scarcity: number) =>
      [0, 1, 2, 3, 4, 5, 6, 7].reduce((sum, seed) => sum + run(village(prison, seed, scarcity), HOURS).breaks, 0);
    expect([prison, breaks(0) * 2 < breaks(4)]).toEqual([prison, true]);
  }
});

test("nothing follows a break under none: nobody is removed and nobody mends, and the village patches the fence after its hours", () => {
  const all = hours(village("none", 2, 4));
  const end = all[all.length - 1];
  expect([end.breaks > 0, end.lost, end.repaired, end.patched > 0]).toEqual([true, 0, 0, true]);
  expect(all.every((one) => one.since.every((s) => s < LONG))).toBe(true);
});

test("removal sends the breaker to the quiet house for its hours and hands an empty home to a bot without one", () => {
  const all = hours(village("removal", 2, 4));
  const end = all[all.length - 1];
  const sent = all.flatMap((one, i) => one.sent.map((bot) => [i, bot]));
  const [i, bot] = sent[0];
  expect([sent.length, end.breaks, end.repaired]).toEqual([end.breaks, end.breaks, 0]);
  expect([all[i].kind[bot], all[i].away[bot], all[i + AWAY - 1].away[bot], all[i + AWAY].away[bot]]).toEqual([
    QUIETLY,
    AWAY,
    1,
    0,
  ]);
  expect(
    all.every((one) => one.home.filter((h) => h !== NONE).length === all[0].home.filter((h) => h !== NONE).length),
  ).toBe(true);
});

test("repair has the breaker and its neighbours mend every break, and nobody is removed", () => {
  const end = run(village("repair", 2, 4), HOURS);
  const open = FENCE.filter((_, seg) => !end.whole[seg]).length;
  expect([end.breaks > 0, end.repaired, end.patched, end.lost]).toEqual([true, end.breaks - open, 0, 0]);
});

test("trust falls at a break and is back higher under repair than under none", () => {
  const trust = (prison: Prison) =>
    [0, 1, 2, 3].reduce((sum, seed) => sum + report(run(village(prison, seed, 3), HOURS)).trust, 0);
  expect(trust("repair") > trust("none")).toBe(true);
});

test("a bot only walks into or out of the garden through the gate", () => {
  for (const prison of PRISONS) {
    const all = hours(village(prison, 5, 3));
    for (let i = 1; i < all.length; i++)
      for (let bot = 0; bot < all[i].bots; bot++) {
        const [a, b] = [all[i - 1], all[i]];
        if (inside(a.x[bot], a.y[bot]) === inside(b.x[bot], b.y[bot])) continue;
        const gate = (s: State) => s.x[bot] === GATE.x && s.y[bot] === GATE.y;
        expect([prison, i, bot, gate(a) || gate(b)]).toEqual([prison, i, bot, true]);
      }
  }
});

test("the way round the garden goes by its corners and never through it", () => {
  const [x, y] = via(500, 100, 500, 900);
  expect([x === 500, inside(x, y)]).toEqual([false, false]);
  expect(via(500, 900, GATE.x, GATE.y)).toEqual([GATE.x, GATE.y]);
});

test("the garden grows back and every bed stays inside its ripeness", () => {
  const all = hours(village("repair", 1, 0));
  expect(all.every((one) => one.grow.length === BEDS && one.grow.every((g) => g >= 0 && g <= ripe(0)))).toBe(true);
  expect(all[all.length - 1].food.every((f) => f >= 0)).toBe(true);
});

test("a run lasts its hours, then offers no move and steps to itself", () => {
  const end = run(village("none", 1), HOURS + 5);
  expect([end.hour, end.over, moves(end), step(end, tick) === end, moves(village("none", 1))]).toEqual([
    HOURS,
    "done",
    [],
    true,
    [tick],
  ]);
});

test("read follows a path into the state", () => {
  const state = village("repair", 1);
  expect([read(state, "prison"), read(state, "hx/2"), read(state, "nope/deeper"), read(state)]).toEqual([
    "repair",
    state.hx[2],
    undefined,
    state,
  ]);
});

test("deal boots the seed on round zero and a fresh seed each round after", () => {
  const knobs = { prison: "repair" as const, scarcity: 2, bots: 24 };
  expect(deal(knobs, { seed: 9 })).toEqual(village("repair", 9));
  expect(deal(knobs, { seed: 9, round: 2 })).toEqual(village("repair", dealt(9, 2)));
  expect(dealt(9, 1)).not.toBe(dealt(9, 2));
});

test("boot refuses a rule, a crowd or a scarcity outside the toy", () => {
  expect(() => boot({ prison: "jail" as Prison, scarcity: 2, bots: 24, length: HOURS, seed: 1 })).toThrow();
  expect(() => village("none", 1, 2, 8)).toThrow();
  expect(() => village("none", 1, 5)).toThrow();
});
