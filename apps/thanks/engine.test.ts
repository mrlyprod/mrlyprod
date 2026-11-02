import { expect, test } from "bun:test";
import type { Credit, State } from "./engine.ts";
import {
  alive,
  BUILD,
  boot,
  CREDITS,
  DAYS,
  deal,
  dealt,
  HAND,
  IDLE,
  moves,
  OUT,
  PRICE,
  read,
  report,
  run,
  step,
  tick,
} from "./engine.ts";

const freeze = <T>(x: T): T => {
  if (x && typeof x === "object") {
    Object.values(x).forEach(freeze);
    Object.freeze(x);
  }
  return x;
};

const town = (credit: Credit, seed: number, bots = 32) => boot({ credit, bots, length: DAYS, seed });

const days = (state: State) => {
  const out = [state];
  while (!out[out.length - 1].over) out.push(step(out[out.length - 1], tick));
  return out;
};

test("one seed draws one town for every rule, and another seed another", () => {
  const towns = CREDITS.map((credit) => town(credit, 7));
  expect(
    towns.every((one) => JSON.stringify([one.skill, one.rank]) === JSON.stringify([towns[0].skill, towns[0].rank])),
  ).toBe(true);
  expect([
    JSON.stringify(town("runner", 7)) === JSON.stringify(towns[0]),
    JSON.stringify(town("runner", 8)) === JSON.stringify(towns[0]),
    [...towns[0].rank].sort((a, b) => a - b),
    towns[0].need,
  ]).toEqual([true, false, Array.from({ length: 32 }, (_, k) => k), 24]);
});

test("a step never touches the state it was handed, and a run ends after its days with no moves left", () => {
  const start = freeze(town("all", 3));
  const end = run(start, DAYS * 2);
  expect([end.day, end.over, moves(end), moves(start), step(end, tick) === end]).toEqual([
    DAYS,
    "done",
    [],
    [tick],
    true,
  ]);
});

test("every day the town gets exactly the goods it needs, robots first and hands for the rest", () => {
  for (const credit of CREDITS)
    for (const state of days(town(credit, 5)).slice(1)) {
      const told = report(state);
      expect([credit, state.day, told.robots + told.hand]).toEqual([credit, state.day, state.need]);
      expect(state.work.every((n, robot) => n <= OUT && (n === 0 || alive(state, robot)))).toBe(true);
    }
});

test("thanks are only made by goods: a sale moves twelve from the buyer to the maker and makes none", () => {
  for (const credit of CREDITS) {
    const all = days(town(credit, 2));
    const end = all[all.length - 1];
    const sum = end.thanks.reduce((a, b) => a + b, 0);
    const sales = all.reduce((n, one) => n + one.sold.length, 0);
    expect([credit, sum, end.paid]).toEqual([credit, end.byRobot + end.byHand, sales * PRICE]);
  }
});

test("each rule thanks its own: the runner, the maker, or every bot in turn", () => {
  for (const credit of CREDITS) {
    const ok = days(town(credit, 4)).every((state) => {
      for (let i = 0; i < state.flows.length; i += 3) {
        const [robot, to] = [state.flows[i], state.flows[i + 1]];
        if (credit === "runner" && to !== state.runner[robot]) return false;
        if (credit === "maker" && to !== state.maker[robot]) return false;
      }
      return true;
    });
    expect([credit, ok]).toEqual([credit, true]);
  }
  const end = run(town("maker", 4), DAYS);
  expect(end.toMakers).toBe(end.byRobot);
  const got = new Set(days(town("all", 4)).flatMap((state) => state.flows.filter((_, i) => i % 3 === 1)));
  expect(got.size).toBe(32);
});

test("bots build only while robots fall short, and the hand work keeps to the same bots", () => {
  for (const state of days(town("runner", 6))) {
    const live = state.ends.filter((_, robot) => alive(state, robot)).length - state.fresh.length;
    const building = state.job.filter((one) => one === BUILD).length;
    expect([state.day, building > 0 && live * OUT >= state.need]).toEqual([state.day, false]);
  }
  const [a, b] = [run(town("runner", 6), 3), run(town("runner", 6), 4)];
  const kept = a.job.every((job, bot) => job !== HAND || b.job[bot] === HAND || report(b).hand < report(a).hand);
  expect([kept, report(a).idle + report(a).hand + report(a).building]).toEqual([true, 32]);
});

test("robots take over the work, and the runner rule leaves more thanks with the top fifth than everyone", () => {
  for (const seed of [0, 1, 2, 3]) {
    const end = Object.fromEntries(CREDITS.map((credit) => [credit, report(run(town(credit, seed), DAYS))]));
    expect([seed, end.runner.top > end.all.top, end.maker.top > end.all.top, end.runner.built > 0]).toEqual([
      seed,
      true,
      true,
      true,
    ]);
    expect(report(run(town("runner", seed), 80)).share).toBeGreaterThan(800);
  }
});

test("bad rules and crowds are refused, read walks a path, and deal draws the next round's town", () => {
  expect(() => town("boss" as Credit, 1)).toThrow();
  expect(() => town("runner", 1, 4)).toThrow();
  const state = town("runner", 1);
  expect([read(state, "need"), read(state, "skill/0"), read(state, "need/deep"), state.job[0]]).toEqual([
    24,
    state.skill[0],
    undefined,
    IDLE,
  ]);
  expect([
    deal({ credit: "all", bots: 32 }, { seed: 1 }),
    deal({ credit: "all", bots: 32 }, { seed: 1, round: 2 }).seed,
  ]).toEqual([town("all", 1), town("all", dealt(1, 2)).seed]);
});
