import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { CARPET, LEAST, MOST, cutter, deepest, field, law, more, rungs } from './engine.js';

const nans = (data) => data.reduce((sum, v) => sum + (Number.isNaN(v) ? 1 : 0), 0);

test('L layers reach the odd side 2L - 1, and a tap adds a cut until the top wraps to the least', () => {
  expect([deepest(28), more(28), more(MOST - 2), more(MOST)]).toEqual([55, 30, MOST, LEAST]);
});

test('the stack is the mean ink of the cuts inside the hexagon, NaN outside, and keeps each cut for the next L', () => {
  const cuts = cutter(math);
  const made = field(math, cuts, 4, 64);
  const inside = 4096 - nans(made.data);
  expect([made.size, Number.isNaN(made.data[0]), inside > 4096 * 0.4, inside < 4096 * 0.6, cuts.count()]).toEqual([64, true, true, true, 4]);
  expect([made.low >= 0, made.high <= 1, made.low < made.high]).toEqual([true, true, true]);
  field(math, cuts, 6, 64).field.free();
  expect(cuts.count()).toBe(6);
  field(math, cuts, 4, 32).field.free();
  expect(cuts.count()).toBe(4);
  made.field.free();
  cuts.free();
});

test('a cut is the hexagon of the carpet cube: its mean ink is the exact hexagon share', () => {
  const cuts = cutter(math);
  const star = new math.six.star.Star(CARPET);
  const one = cuts.cut(41, 384);
  const inside = cuts.mask();
  let inked = 0;
  let read = 0;
  for (let i = 0; i < one.length; i++) if (inside[i]) (read++, (inked += one[i]));
  expect(inked / read).toBeCloseTo(star.hexagon(41).value(), 2);
  star.free();
  cuts.free();
});

test('the ladder halves L while it stays divisible by four and no rung goes under four', () => {
  expect([rungs(28), rungs(32), rungs(30), rungs(4)]).toEqual([[14, 28], [4, 8, 16, 32], [30], [4]]);
});

test('at W = 0 the arm is exact at every layer and the decay settles on -1/4 at L = 0 mod 4', () => {
  const star = new math.six.star.Star(CARPET);
  const read = law(math, star, 28, 0);
  expect(read).toMatchObject({ layers: 28, half: 0, deepest: 55, exact: 28, chi: 1, target: -0.25, branch: '0 mod 4' });
  expect(read.ribbon.slice(0, 4)).toEqual([{ n: 1, arm: '1/1', law: '1/1', exact: true }, { n: 3, arm: '1/3', law: '1/3', exact: true }, { n: 5, arm: '2/5', law: '2/5', exact: true }, { n: 7, arm: '4/7', law: '4/7', exact: true }]);
  expect([read.ribbon.length, read.ladder.map((one) => one.count)]).toEqual([12, [14, 28]]);
  expect(read.slope).toBeCloseTo(-0.25, 2);
  expect(read.excess).toBeCloseTo(read.arm - read.hexagon, 12);
  expect(read.constant).toBeCloseTo(-0.2937606, 6);
  expect(read.predicted).toBeCloseTo(-23 / 192, 12);
  star.free();
});

test('a wider band leaves the law, walks the width law, and L = 2 mod 4 refuses the slope', () => {
  const star = new math.six.star.Star(CARPET);
  const read = law(math, star, 30, 2);
  expect([read.exact < 30, read.ribbon[1].exact, read.slope, read.branch, read.target]).toEqual([true, false, null, '2 mod 4', -1 / 6]);
  star.free();
});
