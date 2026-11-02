import { expect, test } from "bun:test";
import type { State, Work } from "./engine.ts";
import {
  BASICS,
  boot,
  CRAFTS,
  DAYS,
  deal,
  dealt,
  ideas,
  MILLE,
  moves,
  NONE,
  OFFICE,
  REST,
  read,
  report,
  run,
  skill,
  step,
  TOP,
  tick,
} from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const world = (work: Work, seed: number, more: { bots?: number; teams?: number } = {}) =>
  boot({ bots: 48, work, teams: 1, length: DAYS, seed, ...more });

const days = (state: State) => Array.from({ length: DAYS }, (_, day) => run(state, day + 1));

test("the same seed and days replay to the same state, byte for byte, and another seed another", () => {
  const once = (seed: number) => JSON.stringify(run(world("free", seed), 30));
  expect(once(42)).toBe(once(42));
  expect(once(42)).not.toBe(once(43));
});

test("step leaves its input untouched and its state holds only integers, strings and nulls", () => {
  let state = world("earn", 7, { teams: 3 });
  const flat = (x: unknown): unknown[] => (x && typeof x === "object" ? Object.values(x).flatMap(flat) : [x]);
  for (let i = 0; i < 20; i++) {
    const before = JSON.stringify(state);
    const next = step(freeze(state), tick);
    expect(JSON.stringify(state)).toBe(before);
    state = next;
  }
  expect(flat(state).every((one) => one === null || typeof one === "string" || Number.isInteger(one))).toBe(true);
});

test("a town opens with every bot on the plaza and runs sixty days to its end", () => {
  const start = world("earn", 3);
  const end = run(start, DAYS);
  expect([start.at.every((one) => one === REST), end.tick, end.over, moves(end), step(end, tick)]).toEqual([
    true,
    DAYS,
    "done",
    [],
    end,
  ]);
});

test("work to eat: the basics hire enough hands, the office holds a third at most, and only a bot who worked eats", () => {
  for (const state of days(world("earn", 5))) {
    const office = state.at.filter((one) => one === OFFICE).length;
    expect(state.made.slice(0, BASICS).every((made) => made >= state.need)).toBe(true);
    expect(office).toBeLessThanOrEqual(Math.ceil(state.bots / 3));
    expect(state.ate.every((one, bot) => !one || state.at[bot] !== REST)).toBe(true);
  }
});

test("basics free: nobody takes a desk, a bot at work is at the craft it loves or answers a basic's call, and a full bakery feeds all", () => {
  for (const state of days(world("free", 5))) {
    expect(state.at.includes(OFFICE)).toBe(false);
    expect(state.at.every((at, bot) => at === REST || at === state.love[bot] || at < BASICS)).toBe(true);
    if (state.made[0] >= state.need) expect(state.ate.every((one) => one === 1)).toBe(true);
  }
});

test("on the same town, basics free gives more work to what a bot loves and none to busywork", () => {
  const earn = report(run(world("earn", 8), DAYS));
  const free = report(run(world("free", 8), DAYS));
  expect([free.loved > earn.loved, earn.busy > 0, free.busy]).toEqual([true, true, 0]);
});

test("one team never drops work; racing teams ship fewer ideas, drop the losers' work, and start each idea afresh", () => {
  const one = run(world("free", 4), DAYS);
  const four = run(world("free", 4, { teams: 4 }), DAYS);
  expect([report(one).dropped, report(four).dropped > 0, report(four).shipped < report(one).shipped]).toEqual([
    0,
    true,
    true,
  ]);
  for (const state of days(world("free", 4, { teams: 4 })))
    for (let j = 0; j < state.ships.length; j += 3) {
      const craft = state.ships[j];
      expect(ideas(craft)).toBe(true);
      expect(state.progress.slice(craft * 4, craft * 4 + 4)).toEqual([0, 0, 0, 0]);
    }
});

test("a bot gets better at the craft it practises, never past the top", () => {
  const start = world("free", 2);
  const end = run(start, DAYS);
  const grown = Array.from({ length: start.bots }, (_, bot) => bot).filter(
    (bot) => skill(end, bot, start.love[bot]) > skill(start, bot, start.love[bot]),
  );
  expect([grown.length > start.bots / 2, end.skill.every((one) => one >= 1 && one <= TOP)]).toEqual([true, true]);
});

test("the report counts in thousandths of bot-days, and shipped is every craft's ideas", () => {
  const state = run(world("earn", 6, { teams: 2 }), DAYS);
  const told = report(state);
  for (const key of ["fed", "loved", "idle", "busy", "basics", "best", "dropped"] as const)
    expect(told[key] >= 0 && told[key] <= MILLE).toBe(true);
  expect([told.shipped, told.working + told.office + told.resting, state.days]).toEqual([
    state.shipped.reduce((a, b) => a + b, 0),
    state.bots,
    state.bots * DAYS,
  ]);
  expect(state.shipped.slice(0, BASICS).every((one) => one === 0) && state.shipped.length === CRAFTS).toBe(true);
});

test("deal boots sixty days from the seed, a round deals another town, and read walks a path", () => {
  const knobs = { bots: 30, work: "free" as const, teams: 2 };
  expect(deal(knobs, { seed: 5 })).toEqual(boot({ ...knobs, length: DAYS, seed: 5 }));
  expect(deal(knobs, { seed: 5, round: 2 })).toEqual(boot({ ...knobs, length: DAYS, seed: dealt(5, 2) }));
  expect(deal(knobs, { seed: 5, round: 1 }).love).not.toEqual(deal(knobs, { seed: 5 }).love);
  const state = deal(knobs, { seed: 5 });
  expect([read(state, "bots"), read(state, "team/0"), read(state, "nope/deeper"), read(state)]).toEqual([
    30,
    NONE,
    undefined,
    state,
  ]);
});
