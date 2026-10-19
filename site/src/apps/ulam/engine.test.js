import { expect, test } from 'bun:test';
import * as num from '@mrlyprod/mrly/num';
import { palette as hues } from '../../lib/theme/palette.gen.js';
import { defaults, tidy } from '../../lib/app/spec.js';
import { blend } from '../../lib/color.js';
import { rng } from '../../lib/rand.js';
import { DIM, GROUND, HIT, LEAD, LEAST, LIT, MISS, MOST, SIDES, SOIL, factors, fitted, formula, ink, landed, lay, palette, picture, read, share, shuffle, span, step, study, table, wind } from './engine.js';
import { SPEC } from './scene.js';

const value = (over) => ({ ...defaults(SPEC), ...over });

const L = num.spiral.Lattice;

const count = (tone, which) => tone.filter((t) => t === which).length;

test('a cell found under its own centre is itself on both lattices, and the centre of one is the stage middle', () => {
  for (const lattice of ['square', 'hex']) {
    const geo = lay(num, lattice, 7, 300, 200);
    expect(geo.at(0, 0)).toEqual([150, 100]);
    for (let x = -6; x <= 6; x++) for (let y = -6; y <= 6; y++) expect([lattice, x, y, geo.cell(...geo.at(x, y))]).toEqual([lattice, x, y, [x, y]]);
  }
  expect(lay(num, 'square', 10, 300, 200).at(2, -1)).toEqual([170, 110]);
  expect(lay(num, 'hex', 10, 300, 200).at(0, 2)[1]).toBeCloseTo(100 + 3 * (10 / Math.sqrt(3)));
});

test('a fitted sheet fills the stage inside its margin on both lattices, touching the margin on the tighter axis', () => {
  const pad = 56;
  const rings = 20;
  const across = (lattice, px) => (2 * rings + 1) * px;
  const down = (lattice, px) => (lattice === 'hex' ? (px / Math.sqrt(3)) * (3 * rings + 2) : across(lattice, px));
  for (const lattice of ['square', 'hex']) {
    for (const [w, h] of [[800, 600], [300, 900], [1000, 1000]]) {
      const px = fitted(lattice, rings, w, h, pad);
      expect(Math.max(across(lattice, px) / (w - 2 * pad), down(lattice, px) / (h - 2 * pad))).toBeCloseTo(1);
    }
  }
  expect(fitted('square', 100, 800, 600, pad)).toBeCloseTo((600 - 2 * pad) / 201);
  expect(fitted('square', 100, 800, 600)).toBeCloseTo(600 / 201);
});

test('the winding lists the cell of every number from one, agreeing with the unit', () => {
  const { xs, ys } = wind(num, 'Square', 25);
  expect([xs[1], ys[1], xs[2], ys[2], xs[10], ys[10], xs[25], ys[25]]).toEqual([0, 0, 1, 0, 2, -1, 2, -2]);
  const hex = wind(num, 'Hex', 19);
  expect([hex.xs[8], hex.ys[8], hex.xs[19], hex.ys[19]]).toEqual([1, 1, 0, 2]);
});

test('the winding is kept: a smaller sheet reads the kept cells and a larger one extends them', () => {
  const big = wind(num, 'Square', 400);
  const small = wind(num, 'Square', 100);
  expect([small.xs.buffer === big.xs.buffer, small.xs.length, small.ys.length]).toEqual([true, 101, 101]);
  const more = wind(num, 'Square', 30000);
  const [x, y] = L.xy('Square', 30000);
  expect([more.xs.length, more.xs[30000], more.ys[30000]]).toEqual([30001, Number(x), Number(y)]);
  expect([...more.xs.subarray(0, 401)]).toEqual([...big.xs]);
});

test('the numbers landed grow from one to the top as the share runs, ring by ring', () => {
  expect(landed(num, 'Square', 10, 0)).toBe(1);
  expect(landed(num, 'Square', 10, 1)).toBe(441);
  expect(landed(num, 'Square', 10, 0.5)).toBe(121);
  expect(landed(num, 'Square', 10, 0.55)).toBe(Math.floor(121 + 0.5 * (169 - 121)));
  expect(landed(num, 'Hex', 10, 1)).toBe(331);
  let last = 0;
  for (let k = 0; k <= 100; k++) {
    const n = landed(num, 'Hex', 10, k / 100);
    expect(n).toBeGreaterThanOrEqual(last);
    last = n;
  }
});

test('the wind runs twelve and a half rings a second between its least and most', () => {
  expect([span(10), span(100), span(400)]).toEqual([LEAST, 8000, MOST]);
});

test('the share rises to one over the span once and stays there, and a still is whole', () => {
  const length = span(100);
  expect([share(0, length, false), share(length / 2, length, false), share(length, length, false), share(5 * length, length, false), share(0, length, true)]).toEqual([0, 0.5, 1, 1, 1]);
});

test('a shuffle rolls a lattice, an odd side in range, an a, a b of its parity and an odd prime c, the same for one seed, and the knobs keep it as rolled', () => {
  const rolls = Array.from({ length: 200 }, (_, i) => shuffle(num, rng(i)));
  expect(new Set(rolls.map((one) => one.lattice))).toEqual(new Set(['square', 'hex']));
  expect(new Set(rolls.map((one) => one.a))).toEqual(new Set(Array.from({ length: LEAD }, (_, i) => i + 1)));
  expect(new Set(rolls.map((one) => one.side)).size).toBeGreaterThan(50);
  expect(rolls.every(({ side, a, b, c }) => side % 2 === 1 && side >= SIDES[0] && side <= SIDES[1] && (a + b) % 2 === 0 && c % 2 === 1 && num.prime.is_prime(c))).toBe(true);
  for (const one of rolls) expect(tidy(SPEC, value(one))).toEqual(value(one));
  expect(shuffle(num, rng(5))).toEqual(shuffle(num, rng(5)));
});

test('a shuffle mostly rolls the a that runs straight, 4 on the square and 3 on the hexagon, and still rolls the others', () => {
  const rolls = Array.from({ length: 400 }, (_, i) => shuffle(num, rng(i)));
  const joined = rolls.filter(({ lattice, a }) => a === (lattice === 'hex' ? 3 : 4)).length;
  expect(joined / rolls.length).toBeGreaterThan(0.7);
  expect(joined).toBeLessThan(rolls.length);
});

test('c steps to the next prime up or down and stays at the ends', () => {
  expect([step(num, 41, 1, 200), step(num, 41, -1, 200), step(num, 2, -1, 200), step(num, 199, 1, 200), step(num, 0, 1, 200), step(num, -9, 1, 200), step(num, -9, -1, 200)]).toEqual([43, 37, 2, 199, 2, 2, -9]);
});

test('the formula and the factors read as a person writes them', () => {
  expect(formula(4, -2, 41)).toBe('4k² - 2k + 41');
  expect(formula(1, 0, 0)).toBe('1k²');
  expect(formula(3, 3, -1)).toBe('3k² + 3k - 1');
  expect(factors([[41, 1], [43, 1]])).toBe('41 · 43');
  expect(factors([[2, 3], [5, 1]])).toBe('2^3 · 5');
});

test("the opening sheet of side 201 holds 40401 numbers, and Euler's line opens with 21 primes then 1763", () => {
  const plan = study(num, value(), 800, 600);
  const facts = read(num, plan, value());
  expect(facts).toMatchObject({ lattice: 'square', rings: 100, side: 201, top: 40401, primes: num.prime.prime_count(40401), lit: num.prime.prime_count(40401), a: 4, b: -2, c: 41 });
  expect(facts.line).toMatchObject({ count: 101, hits: 80, streak: 21, values: [41, 43, 53, 71, 97, 131, 173, 223], more: true, next: { n: 1763, factors: [[41, 1], [43, 1]] } });
  expect(facts.line.share).toBeCloseTo(80 / 101);
  expect([plan.tone.length, plan.xs.length]).toEqual([40402, 40402]);
});

test('the tones are the marks alone: composites ground only while faint, a Mobius minus one dim, and no line in them', () => {
  const on = study(num, value({ side: 21 }), 800, 600);
  const off = study(num, value({ side: 21, faint: 0 }), 800, 600);
  expect([count(on.tone, LIT), count(on.tone, SOIL), count(on.tone, HIT) + count(on.tone, MISS)]).toEqual([on.primes, on.top - on.primes, 0]);
  expect([count(off.tone, SOIL), count(off.tone, LIT)]).toEqual([0, off.primes]);
  const mobius = study(num, value({ side: 21, mark: 'mobius' }), 800, 600);
  expect(count(mobius.tone, DIM)).toBeGreaterThan(0);
  expect(mobius.lit).toBe(count(mobius.tone, LIT));
});

test('a at zero draws no line and reads no line', () => {
  const plan = study(num, value({ a: 0 }), 400, 300);
  expect([plan.line, read(num, plan, value({ a: 0 })).line]).toEqual([null, null]);
});

test('the colours are the tint for the mark, pink, orange for a hit, blue for a miss, and a neutral ground', () => {
  expect([ink('#000000'), ink('#ffffff')]).toEqual(['#ffffff', '#000000']);
  const tones = palette('#ffd100', '#000000');
  expect([tones[LIT], tones[DIM], tones[HIT], tones[MISS], tones[SOIL]]).toEqual(['#ffd100', hues.pink, hues.orange, hues.blue, blend('#000000', '#ffffff', GROUND)]);
});

test('the svg holds every tone as rects on the square and as hexagons used on the hexagon, the line in orange and blue over them', () => {
  const tones = palette('#ffd100', '#000000');
  const plan = study(num, value({ side: 21, a: 0 }), 800, 600);
  const text = picture(plan, tones);
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="210" height="210" viewBox="0 0 210 210"')).toBe(true);
  expect(text.match(/<rect /g).length).toBeGreaterThan(count(plan.tone, LIT) / 3);
  expect(text).toContain('fill="#ffd100"');
  expect(text).not.toContain(`fill="${hues.orange}"`);
  const euler = picture(study(num, value({ side: 61 }), 800, 600), tones);
  expect([euler.includes(`<g fill="${hues.orange}"`), euler.includes(`<g fill="${hues.blue}"`)]).toEqual([true, true]);
  const hex = study(num, value({ side: 11, lattice: 'hex', a: 0, faint: 0 }), 800, 600);
  const used = picture(hex, tones);
  expect(used).toContain('<defs><path id="h"');
  expect(used.match(/<use /g)).toHaveLength(count(hex.tone, LIT));
});

test('the csv lists k, n and the hit of every landing', () => {
  const line = num.spiral.diagonal('Square', 21, 4, -2, 1);
  const rows = table(line).split('\n');
  expect(rows[0]).toBe('k,n,prime');
  expect(rows).toHaveLength(line.values.length + 1);
  expect(rows[2]).toBe('1,3,1');
});
