import { expect, test } from "bun:test";
import type { Choose } from "./engine.ts";
import {
  better,
  boot,
  CHOICES,
  deal,
  GRANTED,
  HOURS,
  LET,
  MOON,
  moves,
  NONE,
  OPEN,
  PATIENCE,
  read,
  report,
  run,
  step,
  tick,
} from "./engine.ts";

const done = (choose: Choose, seed: number, bots = 24) => run(deal({ choose, bots }, { seed }), HOURS);

test("one seed deals one field, the deal boots two hundred and forty hours, and another seed another field", () => {
  const one = deal({ choose: "first", bots: 24 }, { seed: 7 });
  expect([one, one.length]).toEqual([boot({ choose: "first", bots: 24, length: HOURS, seed: 7 }), HOURS]);
  expect(deal({ choose: "first", bots: 24 }, { seed: 8 }).x).not.toEqual(one.x);
});

test("a run ends at its last hour, then takes no move and no step", () => {
  const end = done("near", 3);
  expect([end.hour, end.over, moves(end), step(end, tick) === end, read(end, "hour")]).toEqual([
    HOURS,
    "done",
    [],
    true,
    HOURS,
  ]);
});

test("oldest picks the older wish, nearest the closer wisher, most the wish with more hearts", () => {
  const field = deal({ choose: "first", bots: 8 }, { seed: 1 });
  const at = (choose: Choose) => ({
    ...field,
    choose,
    x: [0, 900, 100, 0, 0, 0, 0, 0],
    y: [0, 900, 100, 0, 0, 0, 0, 0],
    kind: [0, 0],
    by: [1, 2],
    born: [3, 5],
    hearts: [1, 4],
    taker: [NONE, NONE],
    end: [OPEN, OPEN],
  });
  expect(CHOICES.map((choose) => better(at(choose), 0, 1, 0))).toEqual([false, true, true]);
});

test("a wish is granted only by a bot that grants its kind, and nobody grants the moon", () => {
  for (const choose of CHOICES)
    for (const seed of [1, 2, 3]) {
      const end = done(choose, seed);
      const given = end.end.flatMap((one, w) => (one === GRANTED ? [w] : []));
      expect(given.length).toBeGreaterThan(0);
      expect(given.every((w) => end.kind[w] !== MOON && end.can.includes(end.kind[w]))).toBe(true);
    }
});

test("only a wish nobody took is let go, after sixty hours on the board", () => {
  const end = done("most", 4);
  const freed = end.end.flatMap((one, w) => (one === LET ? [w] : []));
  const told = report(end);
  expect([freed.length > 0, freed.every((w) => end.taker[w] === NONE), told.gone, told.oldest <= PATIENCE]).toEqual([
    true,
    true,
    freed.length,
    true,
  ]);
});

test("nearest grants wishes sooner on average than oldest first", () => {
  const mean = (choose: Choose) => [1, 2, 3, 4, 5, 6].reduce((sum, seed) => sum + report(done(choose, seed)).wait, 0);
  expect(mean("near")).toBeLessThan(mean("first"));
});
