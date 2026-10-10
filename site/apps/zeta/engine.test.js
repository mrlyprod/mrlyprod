import { expect, test } from 'bun:test';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { rng } from '../../lib/scene.js';
import { BANDS, FADE, FLASH, GHOST, HOLD, KEEP, LEAST, MOST, ROLL, bands, choose, density, fit, flares, fold, index, layout, longest, passed, phase, picture, staircase, study, table } from './engine.js';

num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const line = new num.zeta.Line();

test('a from of zero is rolled from the seed below the roll and a typed from stays', () => {
  const rolled = Array.from({ length: 20 }, (_, seed) => choose({ from: 0 }, rng(seed)));
  expect(rolled.every((from) => Number.isInteger(from) && from >= 0 && from < ROLL)).toBe(true);
  expect(new Set(rolled).size).toBeGreaterThan(10);
  expect(choose({ from: 0 }, rng(7))).toBe(choose({ from: 0 }, rng(7)));
  expect(choose({ from: 14.5 }, rng(7))).toBe(14.5);
});

test('the sample density rises with the height of the walk and stays inside its bounds', () => {
  const tops = [0, 60, 1000, 10000, 100000];
  const seen = tops.map(density);
  expect(seen).toEqual([...seen].sort((a, b) => a - b));
  expect(seen[0]).toBe(LEAST);
  expect(seen[2]).toBeGreaterThan(seen[1]);
  expect(seen.every((d) => d >= LEAST && d <= MOST)).toBe(true);
});

test('the walk from zero finds the first thirteen zeros to a billionth, numbers them from one, and is kept for the same value', () => {
  const plan = study(num, 0, 60);
  const known = line.zeros(13);
  expect(plan.zeros.length).toBe(13);
  expect(Math.max(...Array.from(plan.zeros, (t, i) => Math.abs(t - known[i])))).toBeLessThan(1e-8);
  expect(plan.first).toBe(1);
  expect(plan.n).toBe(60 * plan.per + 1);
  expect(plan.xs[0]).toBeCloseTo(-1.4603545, 6);
  expect(plan.reach).toBeGreaterThan(3);
  expect(plan.frame[0]).toBeLessThanOrEqual(-1);
  expect(plan.frame[2]).toBeGreaterThanOrEqual(1);
  expect(study(num, 0, 60)).toBe(plan);
  expect(study(num, 0, 70)).not.toBe(plan);
});

test('a walk high on the line counts its zeros as the line does and numbers the first from the count below it', () => {
  const plan = study(num, 1000, 60);
  expect(plan.first).toBe(line.count(1000) + 1);
  expect(plan.zeros.length).toBe(line.count(1060) - line.count(1000));
  expect(plan.zeros.every((t, i) => t > 1000 && t < 1060 && (!i || t > plan.zeros[i - 1]))).toBe(true);
  expect(Math.abs(line.z(plan.zeros[0]))).toBeLessThan(1e-6);
});

test('the phase walks the span, holds, fades at the end of the hold and is the end under reduced motion', () => {
  const length = 30000;
  expect(phase(0, length, false)).toEqual({ at: 0, veil: 1, play: 0 });
  expect(phase(15000, length, false)).toEqual({ at: 0.5, veil: 1, play: 15000 });
  expect(phase(length + 100, length, false)).toEqual({ at: 1, veil: 1, play: length + 100 });
  const late = phase(length + HOLD - FADE / 2, length, false);
  expect(late.at).toBe(1);
  expect(late.veil).toBeCloseTo(0.5, 6);
  expect(phase(length + HOLD + 10, length, false).at).toBeCloseTo(10 / length, 9);
  expect(phase(-100, length, false).at).toBe(1);
  expect(phase(123, length, true)).toEqual({ at: 1, veil: 1, play: 0 });
  expect(index(0.5, 11)).toBe(5);
});

test('passed counts the zeros at or below a t', () => {
  const zeros = Float64Array.from([14.1, 21, 25]);
  expect([0, 14.1, 20, 25, 100].map((t) => passed(zeros, t))).toEqual([0, 1, 1, 3, 3]);
  expect(passed(new Float64Array(0), 5)).toBe(0);
});

test('the trail falls in bands from full ink at the head to the ghost, clipped at the start of the walk', () => {
  const rows = bands(1200, 600);
  expect(rows).toHaveLength(BANDS);
  expect(rows[0]).toEqual({ from: 600, to: 650, first: 600, alpha: GHOST + (1 - GHOST) * (1 / BANDS) });
  expect(rows.at(-1)).toEqual({ from: 1150, to: 1200, first: 1151, alpha: 1 });
  expect(rows.every((row, i) => !i || row.alpha > rows[i - 1].alpha)).toBe(true);
  const early = bands(30, 600);
  expect(early[0]).toEqual({ from: 0, to: 0, first: 0, alpha: early[0].alpha });
  expect(early.at(-1)).toEqual({ from: 0, to: 30, first: 1, alpha: 1 });
});

test('every point of the trail belongs to one band, so a bead at a joint is laid once', () => {
  for (const [head, kept] of [[1200, 600], [30, 600], [5000, 8000]]) {
    const owned = bands(head, kept).flatMap(({ first, to }) => Array.from({ length: Math.max(0, to - first + 1) }, (_, i) => first + i));
    expect(new Set(owned).size).toBe(owned.length);
    expect(Math.min(...owned)).toBe(bands(head, kept)[0].first);
    expect(Math.max(...owned)).toBe(head);
  }
});

test('a flash is timed in play ms from the zero, so a ring rings out in the hold while the head stands still', () => {
  const zeros = Float64Array.from([14, 21, 25]);
  const at = (play) => flares(zeros, 0, 2, play);
  expect(at(6999)).toEqual([]);
  expect(at(7000)).toEqual([0]);
  expect(at(7000 + FLASH / 2)).toEqual([0.5]);
  expect(at(7000 + FLASH)).toEqual([1]);
  expect(at(7000 + FLASH + 1)).toEqual([]);
  expect(at(10500 + 70)).toEqual([0.1]);
  expect(flares(Float64Array.from([10, 10.2]), 0, 1, 10300)).toEqual([100 / FLASH, 300 / FLASH]);
});

test('the trail stays under the point budget wherever the walk starts, rolled or typed', () => {
  for (const [from, span] of [[0, 300], [0, 10], [14.5, 60], [1999, 60], [10000, 300]]) {
    const top = (from || ROLL) + span;
    expect(longest(from, span) * density(top)).toBeLessThanOrEqual(KEEP);
    expect(longest(from, span)).toBeGreaterThan(10);
  }
  expect(longest(10000, 300)).toBeLessThan(longest(0, 300));
});

test('fit centres the frame of the walk inside the padded canvas', () => {
  const box = fit([-2, -1, 4, 3], 800, 600, 20);
  expect(box.k).toBeCloseTo(760 / 6, 9);
  expect(box.ox + 1 * box.k).toBe(400);
  expect(box.oy - 1 * box.k).toBe(300);
});

test('a canvas narrower than its padding fits at scale zero, never a negative radius', () => {
  for (const [w, h] of [[1, 600], [800, 1], [1, 1], [47, 47]]) expect(fit([-2, -1, 4, 3], w, h, 24).k).toBe(0);
});

test('the svg of a walk writes the axes, the ghost, the trail bands, the dots of a dotted look and the head in the accent', () => {
  const plan = study(num, 0, 60);
  const lined = picture(plan, { w: 400, h: 300, pad: 24 }, plan.n - 1, 10 * plan.per, 'line', '#008cff');
  expect(lined.startsWith('<svg')).toBe(true);
  expect(lined.match(/<path /g)).toHaveLength(1 + 1 + BANDS);
  expect(lined.match(/stroke-linecap="butt"/g)).toHaveLength(BANDS);
  expect(lined).toContain('stroke-opacity="0.22"');
  expect(lined).toContain('stroke-opacity="1"');
  expect(lined.match(/<circle /g)).toHaveLength(2);
  expect(lined).not.toContain('fill-opacity');
  const dotted = picture(plan, { w: 400, h: 300, pad: 24 }, plan.n - 1, 10 * plan.per, 'dots', '#008cff');
  expect(dotted.match(/<path /g)).toHaveLength(1);
  expect(dotted.match(/<circle /g).length).toBeGreaterThan(100);
  const both = picture(plan, { w: 400, h: 300, pad: 24 }, plan.n - 1, 10 * plan.per, 'both', '#008cff');
  expect(both.match(/<path /g)).toHaveLength(1 + 1 + BANDS);
  expect(both.match(/fill-opacity/g)).toHaveLength(BANDS);
});

test('the csv lists the zeros of the walk numbered on the line to six decimals', () => {
  const rows = table(study(num, 0, 60)).trim().split('\n');
  expect(rows[0]).toBe('zero,t');
  expect(rows).toHaveLength(14);
  expect(rows[1]).toBe('1,14.134725');
  expect(rows[13].startsWith('13,59.347044')).toBe(true);
});

test('the staircase is the sieve to x, folded with the first zeros it matches the steps better and better, and the plan is kept', () => {
  const plan = fold(num, 100, 100);
  expect(plan.stair).toHaveLength(100);
  expect(plan.stair[99]).toBeCloseTo(94.0453112, 6);
  expect(plan.gammas).toHaveLength(100);
  expect(plan.gammas[0]).toBeCloseTo(14.134725, 6);
  expect(plan.ladder.xs).toHaveLength(199);
  expect(plan.ladder.ys.at(-1)).toBe(plan.stair[99]);
  const miss = (k) => {
    const ys = plan.curve(k);
    let worst = 0;
    plan.grid.forEach((u, i) => {
      if (u < 20 || Math.abs(u - Math.round(u)) < 0.25) return;
      worst = Math.max(worst, Math.abs(ys[i] - plan.stair[Math.floor(u) - 1]));
    });
    return worst;
  };
  expect(miss(100)).toBeLessThan(miss(10));
  expect(miss(10)).toBeLessThan(miss(0));
  expect(plan.curve(10)).toBe(plan.curve(10));
  expect(plan.facts).toMatchObject({ face: 'stairs', x: 100, zeros: 100, first: 1 });
  expect(plan.facts.last).toBe(plan.gammas[99]);
  expect(plan.facts.gap).toBeCloseTo(Math.abs(plan.facts.formula - plan.facts.psi), 12);
  expect(plan.top).toBeGreaterThan(plan.facts.psi);
  expect(fold(num, 100, 100)).toBe(plan);
  expect(fold(num, 100, 0).gammas).toHaveLength(0);
  expect(fold(num, 100, 0).facts.last).toBeNull();
});

test('the layout maps one to x across the padded width and zero to the top across the padded height', () => {
  const box = layout({ x: 101, top: 200 }, 800, 600, 20);
  expect(box.left + (101 - 1) * box.kx).toBeCloseTo(box.right, 9);
  expect(box.base - 200 * box.ky).toBeCloseTo(box.roof, 9);
});

test('the svg of the stairs holds the axes, the filled steps, the guess dashed and the formula at the zeros folded', () => {
  const plan = fold(num, 100, 10);
  const text = staircase(plan, { w: 400, h: 300, pad: 24 }, 10, '#008cff');
  expect(text.startsWith('<svg')).toBe(true);
  expect(text).toContain('<clipPath id="zeta-frame">');
  expect(text).toContain('stroke-dasharray="4 5"');
  expect(text.match(/<path /g)).toHaveLength(5);
  expect(text.match(/stroke="#008cff"/g).length).toBeGreaterThan(3);
  expect(staircase(plan, { w: 400, h: 300, pad: 24 }, 0, '#008cff')).not.toBe(text);
});
