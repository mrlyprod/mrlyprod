import { expect, test } from 'bun:test';
import * as num from '@mrlyprod/mrly/num';
import { BANDS, FADE, GHOST, HOLD, LISTED, PER, REACH, ROLLS, ZEROS, bands, fold, hop, ladder, passed, phase, picture, plane, reading, roll, snap, study, table } from './engine.js';

const line = new num.zeta.Line();

const inks = { accent: '#008cff', zero: '#ffd100', head: '#ff8f2c', guess: '#ff325a' };

test('the walk is studied once from zero to the reach, with the first hundred zeros to a billionth and a reach that only grows', () => {
  const plan = study(num);
  expect(plan.n).toBe(REACH * PER + 1);
  expect(plan.xs[0]).toBeCloseTo(-1.4603545, 6);
  expect(plan.zeros).toHaveLength(ZEROS);
  expect(Math.max(...Array.from(line.zeros(13), (t, i) => Math.abs(t - plan.zeros[i])))).toBeLessThan(1e-8);
  expect(passed(plan.zeros, REACH)).toBe(line.count(REACH));
  expect(plan.peak.every((r, i) => r >= 2 && (!i || r >= plan.peak[i - 1]))).toBe(true);
  expect(study(num)).toBe(plan);
});

test('the reading at the head is zeta, Z and theta from the line, the zeros below, the next zero and the first eight marked passed', () => {
  const [p, z] = line.point(30);
  const read = reading(num, study(num), 30);
  expect(read).toMatchObject({ t: 30, re: p.re, im: p.im, z, theta: line.theta(30), count: 3 });
  expect(read.next).toBeCloseTo(30.424876, 6);
  expect(read.list).toHaveLength(LISTED);
  expect(read.list.map(([, past]) => past)).toEqual([true, true, true, false, false, false, false, false]);
  expect(read.list[0][0]).toBeCloseTo(14.134725, 6);
});

test('passed counts the zeros at or below a t', () => {
  const zeros = Float64Array.from([14.1, 21, 25]);
  expect([0, 14.1, 20, 25, 100].map((t) => passed(zeros, t))).toEqual([0, 1, 1, 3, 3]);
  expect(passed(new Float64Array(0), 5)).toBe(0);
});

test('a hop lands on the next or the last zero, and on zero or the reach past the ends', () => {
  const zeros = Float64Array.from([14.1, 21, 25]);
  expect([hop(zeros, 0, 1), hop(zeros, 14.1, 1), hop(zeros, 25, 1)]).toEqual([14.1, 21, REACH]);
  expect([hop(zeros, 30, -1), hop(zeros, 21, -1), hop(zeros, 14.1, -1)]).toEqual([25, 14.1, 0]);
});

test('a t snaps to the slider step inside the reach', () => {
  expect([snap(14.134725), snap(30.424876), snap(-1), snap(500), snap(0.1 + 0.2)]).toEqual([14.15, 30.4, 0, REACH, 0.3]);
});

test('a roll draws t, the zeros and x from the seed inside their spans', () => {
  const rolls = Array.from({ length: 30 }, (_, seed) => roll(seed));
  for (const [key, [lo, hi]] of Object.entries(ROLLS)) expect(rolls.every((one) => Number.isInteger(one[key]) && one[key] >= lo && one[key] <= hi)).toBe(true);
  expect(new Set(rolls.map((one) => one.t)).size).toBeGreaterThan(10);
  expect(roll(7)).toEqual(roll(7));
});

test('the trail falls in bands from full ink at the head to the ghost, clipped at the start of the walk', () => {
  const rows = bands(1200, 600);
  expect(rows).toHaveLength(BANDS);
  expect(rows[0]).toEqual({ from: 600, to: 650, first: 600, alpha: GHOST + (1 - GHOST) * (1 / BANDS) });
  expect(rows.at(-1)).toEqual({ from: 1150, to: 1200, first: 1151, alpha: 1 });
  const early = bands(30, 600);
  expect(early[0]).toEqual({ from: 0, to: 0, first: 0, alpha: early[0].alpha });
  expect(early.at(-1)).toEqual({ from: 0, to: 30, first: 1, alpha: 1 });
});

test('every point of the trail belongs to one band, so a bead at a joint is laid once', () => {
  for (const [head, kept] of [[1200, 600], [30, 600], [5000, 8000]]) {
    const owned = bands(head, kept).flatMap(({ first, to }) => Array.from({ length: Math.max(0, to - first + 1) }, (_, i) => first + i));
    expect(new Set(owned).size).toBe(owned.length);
    expect(Math.max(...owned)).toBe(head);
  }
});

test('the phase of a looping fold runs, holds, fades at the end of the hold and is the end under reduced motion', () => {
  expect(phase(1500, 3000, false)).toEqual({ at: 0.5, veil: 1 });
  expect(phase(3000 + HOLD - FADE / 2, 3000, false).veil).toBeCloseTo(0.5, 6);
  expect(phase(3000 + HOLD + 30, 3000, false).at).toBeCloseTo(0.01, 9);
  expect(phase(123, 3000, true)).toEqual({ at: 1, veil: 1 });
});

test('the plane keeps the origin at the centre of its stage and the reach inside the padding, and a stage smaller than its padding scales to zero', () => {
  const box = plane({ x: 100, y: 0, w: 400, h: 300 }, 3, 24);
  expect([box.ox, box.oy, box.k]).toEqual([300, 150, (150 - 24) / 3]);
  expect(plane({ x: 0, y: 0, w: 30, h: 30 }, 3, 24).k).toBe(0);
});

test('the stairs box fills its stage inside the pad at any aspect, one x a column and the top at the roof', () => {
  for (const [w, h] of [[800, 600], [390, 844]]) {
    const box = ladder({ x: 0, y: 0, w, h }, 100, 50, 24);
    expect([box.left, box.right, box.roof, box.floor]).toEqual([24, w - 24, 24, h - 24]);
    expect([box.left + 99 * box.kx, box.floor - 50 * box.ky]).toEqual([w - 24, 24]);
  }
});

test('the stairs are planned once an x with the curve cached a count of zeros, and the facts land closer with more zeros', () => {
  const steps = fold(num, 100);
  expect(steps.stair).toHaveLength(100);
  expect(steps.stair[99]).toBeCloseTo(94.0453112, 6);
  expect(steps.ladder.xs).toHaveLength(199);
  expect(steps.curve(10)).toBe(steps.curve(10));
  expect(steps.facts(10)).toMatchObject({ x: 100, zeros: 10, psi: steps.stair[99] });
  expect(Math.abs(steps.facts(100).gap)).toBeLessThan(Math.abs(steps.facts(0).gap));
  expect(steps.top(10)).toBeGreaterThan(steps.stair[99]);
  fold(num, 200);
  expect(fold(num, 100)).toBe(steps);
});

test('the svg is the one view shown: the walk with its banded trail, or the clipped stairs', () => {
  const plan = study(num);
  const at = (view) => picture({ plan, steps: fold(num, 100), view, head: 30, k: 10, zeros: 10, kept: 4 * PER, look: 'both', w: 800, h: 600 }, inks);
  const walk = at('walk');
  const stairs = at('stairs');
  expect(walk.startsWith('<svg')).toBe(true);
  expect(walk).toContain('fill="#ff8f2c"');
  expect(walk.match(/fill-opacity/g)).toHaveLength(BANDS);
  expect(walk).not.toContain('<clipPath');
  expect(walk).not.toContain('#ff325a');
  expect(stairs).toContain('<clipPath');
  expect(stairs).toContain('stroke="#ff325a"');
  expect(stairs).not.toContain('#ff8f2c');
});

test('the csv lists the zeros numbered from one to six decimals', () => {
  const rows = table(study(num).zeros).trim().split('\n');
  expect(rows).toHaveLength(ZEROS + 1);
  expect([rows[0], rows[1]]).toEqual(['zero,t', '1,14.134725']);
  expect(rows[13].startsWith('13,59.347044')).toBe(true);
});
