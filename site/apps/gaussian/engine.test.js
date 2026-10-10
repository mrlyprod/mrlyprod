import { expect, test } from 'bun:test';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { BAND, BANDS, FAINTLY, HALF_TONE, HOLD, HOLLOW, HOLLOW_HALF, REACH, SKIP, SPAN, STRONG, choose, hit, lay, name, palette, picture, product, reached, share, study, styleOf, table, verdict } from './engine.js';
import { SPEC } from './scene.js';

num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const BUDGET = 160801;

const value = (over) => ({ ...defaults(SPEC), seed: 1, roll: 0, ring: 'square', limit: 12, ...over });

const tally = (plan) => {
  const counts = [0, 0, 0, 0, 0, 0];
  for (let p = 0; p < plan.count; p++) counts[plan.fate[p]]++;
  return counts;
};

const mix = (a, b, t) => `mix(${a},${b},${t})`;

const veil = (a, t) => `veil(${a},${t})`;

test('the study lays every point of the window in order of norm, each classed as the census counts', () => {
  for (const ring of ['square', 'hex']) {
    const plan = study(num, ring, 12);
    const { facts } = plan;
    expect([ring, plan.count, facts.points]).toEqual([ring, num.gauss.Ring.count(plan.ring, 12), facts.points]);
    expect(tally(plan)).toEqual([facts.composites, facts.split, facts.inert, facts.ramified, facts.units, 1]);
    expect(facts.primes).toBe(facts.split + facts.inert + facts.ramified);
    for (let p = 1; p < plan.count; p++) expect(plan.norm[p] >= plan.norm[p - 1]).toBe(true);
    expect([plan.a[0], plan.b[0], plan.norm[0], plan.fate[0]]).toEqual([0, 0, 0, 5]);
    const seen = new Set();
    for (let p = 0; p < plan.count; p++) seen.add(`${plan.a[p]},${plan.b[p]}`);
    expect(seen.size).toBe(plan.count);
  }
});

test('the square window is the full square and the hex window leaves its corners out', () => {
  const square = study(num, 'square', 12);
  expect([square.word, square.facts.ring, square.side, square.top]).toEqual(['square', 'gaussian', 25, 288]);
  expect(square.grid.every((f) => f !== 255)).toBe(true);
  expect(square.frame).toEqual([-12.5, -12.5, 12.5, 12.5]);
  const hex = study(num, 'hex', 12);
  expect([hex.word, hex.facts.ring, hex.top, hex.facts.symmetry, hex.facts.units]).toEqual(['hex', 'eisenstein', 144, 12, 6]);
  expect([hit(num, hex, 12, -12), hit(num, hex, -12, 12), hit(num, hex, 12, 12)?.fate]).toEqual([null, null, 0]);
  expect(hex.frame[2] - hex.frame[0]).toBeCloseTo(25);
  expect(hex.frame[3] - hex.frame[1]).toBeCloseTo(12 * Math.sqrt(3) + 1);
});

test('the same window is studied once, and another ring or reach is studied afresh', () => {
  const plan = study(num, 'square', 12);
  expect(study(num, 'square', 12)).toBe(plan);
  expect(study(num, 'square', 13)).not.toBe(plan);
  expect(study(num, 'hex', 12)).not.toBe(plan);
});

test('the busiest norm counts every point of its shell, which the window holds whole', () => {
  for (const [ring, limit] of [['square', 12], ['square', 1], ['hex', 10], ['hex', 12], ['hex', 1]]) {
    const { facts, norm, count } = study(num, ring, limit);
    let held = 0;
    for (let p = 0; p < count; p++) if (norm[p] === facts.busy) held++;
    expect([ring, limit, facts.most]).toEqual([ring, limit, held]);
  }
});

test('random picks the ring, the reach and the look from the seed, and a named window is kept', () => {
  const rolls = Array.from({ length: 60 }, (_, i) => choose(value({ roll: 1 }), rng(i)));
  expect(new Set(rolls.map((one) => one.word))).toEqual(new Set(['square', 'hex']));
  expect(new Set(rolls.map((one) => one.look))).toEqual(new Set(['fate', 'norm', 'plain']));
  expect(rolls.every((one) => one.limit >= SPAN[0] && one.limit <= SPAN[1])).toBe(true);
  expect(new Set(rolls.map((one) => one.limit)).size).toBeGreaterThan(20);
  expect(choose(value({ roll: 1 }), rng(5))).toEqual(choose(value({ roll: 1 }), rng(5)));
  expect(choose(value({ ring: 'hex', limit: 7, look: 'norm' }), rng(5))).toEqual({ word: 'hex', limit: 7, look: 'norm' });
});

test('the reach runs to the cap of its ring, which keeps the window under the point budget', () => {
  const limit = (over) => tidy(SPEC, { ...defaults(SPEC), limit: 999, ...over }).limit;
  expect([limit({ ring: 'square' }), limit({ ring: 'hex' })]).toEqual([REACH.square, REACH.hex]);
  expect(num.gauss.Ring.count('Gaussian', REACH.square) <= BUDGET).toBe(true);
  expect(num.gauss.Ring.count('Gaussian', REACH.square + 1) > BUDGET).toBe(true);
  expect(num.gauss.Ring.count('Eisenstein', REACH.hex) <= BUDGET).toBe(true);
  expect(num.gauss.Ring.count('Eisenstein', REACH.hex + 1) > BUDGET).toBe(true);
});

test('a hit reads the norm, the fate, the factors, the conjugate and the associates of a point, and nothing off the window', () => {
  const plan = study(num, 'square', 12);
  const split = hit(num, plan, 2, 3);
  expect(split).toMatchObject({ a: 2, b: 3, norm: 13, fate: 1, factors: [[13, 1]], conjugate: [2, -3] });
  expect(split.associates).toEqual([[2, 3], [-3, 2], [-2, -3], [3, -2]]);
  expect(hit(num, plan, 3, 0)).toMatchObject({ norm: 9, fate: 2, factors: [[3, 2]] });
  expect(hit(num, plan, 1, 1)).toMatchObject({ norm: 2, fate: 3 });
  expect(hit(num, plan, 0, 1)).toMatchObject({ norm: 1, fate: 4, factors: [] });
  expect(hit(num, plan, 0, 0)).toMatchObject({ norm: 0, fate: 5, factors: [] });
  expect(hit(num, plan, 4, 2)).toMatchObject({ norm: 20, fate: 0, factors: [[2, 2], [5, 1]] });
  expect(hit(num, plan, 13, 0)).toBe(null);
  const hex = study(num, 'hex', 12);
  expect(hit(num, hex, 2, 1)).toMatchObject({ norm: 3, fate: 3, conjugate: [1, -1] });
  expect(hit(num, hex, 2, 0)).toMatchObject({ norm: 4, fate: 2 });
});

test('a point is named with its letter and its verdict says what became of its norm', () => {
  expect([name('square', 3, 0), name('square', 0, 1), name('square', 0, -2), name('square', 2, -3), name('hex', 1, 1)]).toEqual(['3', 'i', '-2i', '2 - 3i', '1 + w']);
  expect(product([[2, 2], [5, 1]])).toBe('2^2 · 5');
  const plan = study(num, 'square', 12);
  expect(verdict('square', hit(num, plan, 2, 3))).toBe('norm 13 = (2 + 3i)(2 - 3i), split');
  expect(verdict('square', hit(num, plan, 3, 0))).toBe('norm 9 = 3^2, 3 stays prime');
  expect(verdict('square', hit(num, plan, 1, 1))).toBe('norm 2, 2 ramifies');
  expect(verdict('square', hit(num, plan, 0, 1))).toBe('norm 1, a unit');
  expect(verdict('square', hit(num, plan, 0, 0))).toBe('the origin');
  expect(verdict('square', hit(num, plan, 4, 2))).toBe('norm 20 = 2^2 · 5, composite');
});

test('the share grows over the span, holds, starts over, and is whole in a still or with no span', () => {
  expect([share(0, 12000, false), share(6000, 12000, false), share(12000, 12000, false)]).toEqual([0, 0.5, 1]);
  expect(share(12000 + HOLD / 2, 12000, false)).toBe(1);
  expect(share(12000 + HOLD + 3000, 12000, false)).toBe(0.25);
  expect([share(0, 12000, true), share(5000, 0, false)]).toEqual([1, 1]);
});

test('the points reached grow with the square of the share, so the front moves at one speed', () => {
  expect([reached(0, 101), reached(0.5, 101), reached(1, 101)]).toEqual([0, 25, 100]);
});

test('the layout fits the frame inside the padding or pins a cell size, centred either way', () => {
  const fit = lay([-7, -7, 7, 7], 400, 300, 10);
  expect([fit.k, fit.ox, fit.oy]).toEqual([20, 200, 150]);
  const pinned = lay([-7, -7, 7, 7], 400, 300, 10, 3);
  expect([pinned.k, pinned.ox, pinned.oy]).toEqual([3, 200, 150]);
});

test('the style of a point follows the look and its fate, bands by the root of the norm', () => {
  expect([styleOf('fate', 1, 5, 0, 100), styleOf('fate', 1, 0, 4, 100), styleOf('fate', 0, 0, 4, 100)]).toEqual([SKIP, FAINTLY, SKIP]);
  expect([styleOf('fate', 1, 3, 2, 100), styleOf('fate', 1, 4, 1, 100), styleOf('plain', 1, 4, 1, 100)]).toEqual([HOLLOW, HOLLOW_HALF, HOLLOW]);
  expect([styleOf('fate', 1, 1, 5, 100), styleOf('fate', 1, 2, 9, 100), styleOf('plain', 1, 2, 9, 100)]).toEqual([STRONG, HALF_TONE, STRONG]);
  expect([styleOf('norm', 1, 1, 0, 100), styleOf('norm', 1, 1, 25, 100), styleOf('norm', 1, 1, 100, 100)]).toEqual([BAND, BAND + BANDS / 2, BAND + BANDS - 1]);
  const tones = palette(mix, veil, 'A', 'P');
  expect(tones.length).toBe(BAND + BANDS);
  expect([tones[0], tones[FAINTLY], tones[HOLLOW], tones[BAND]]).toEqual([null, { color: 'veil(A,0.12)', hollow: false }, { color: 'A', hollow: true }, { color: 'mix(A,P,0)', hollow: false }]);
});

test('the svg lays the square window as rects per style, a hollow mark to its own rect, and the hex window as circles, and the csv lists the primes', () => {
  const square = study(num, 'square', 12);
  const style = Uint8Array.from(square.fate, (f, p) => styleOf('fate', 1, f, square.norm[p], square.top));
  const tones = palette(mix, veil, 'A', 'P');
  const text = picture(square, style, tones);
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="250" height="250" viewBox="0 0 250 250">')).toBe(true);
  expect(text.match(/<g fill="A">/g)).toHaveLength(1);
  expect(text.match(/<g fill="none" stroke="A"/g)).toHaveLength(1);
  expect(text.match(/<rect /g).length).toBeGreaterThanOrEqual(square.facts.primes);
  const plain = picture(square, Uint8Array.from(square.fate, (f, p) => styleOf('plain', 1, f, square.norm[p], square.top)), tones);
  const hollow = plain.match(/<g fill="none" stroke="A"[^>]*>(.*?)<\/g>/)[1];
  expect(hollow.match(/<rect /g)).toHaveLength(square.facts.ramified + square.facts.units);
  expect(hollow.match(/width="8"/g)).toHaveLength(square.facts.ramified + square.facts.units);
  const hex = study(num, 'hex', 12);
  const round = picture(hex, Uint8Array.from(hex.fate, (f, p) => styleOf('plain', 0, f, hex.norm[p], hex.top)), tones);
  expect(round.match(/<circle /g)).toHaveLength(hex.facts.primes + hex.facts.units);
  expect(round).not.toContain('<rect');
  const rows = table(square).trim().split('\n');
  expect(rows[0]).toBe('a,b,norm,fate');
  expect(rows.length - 1).toBe(square.facts.primes);
  expect(rows).toContain('1,1,2,ramified');
  expect(rows).toContain('2,3,13,split');
});
