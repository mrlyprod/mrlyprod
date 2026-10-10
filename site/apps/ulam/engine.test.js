import { expect, test } from 'bun:test';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { DIM, FAINT, GROUND, HIT, HOLD, LEAD, LEAST, LIT, MISS, MOST, RINGS, SOIL, choose, factors, fitted, formula, landed, lay, mix, palette, picture, reach, segments, share, span, step, study, table, wind } from './engine.js';
import { SPEC } from './scene.js';

num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

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

test('a fitted sheet sits inside the stage on both lattices', () => {
  for (const lattice of ['square', 'hex']) {
    for (const [w, h] of [[800, 600], [300, 900], [1000, 1000]]) {
      const rings = 20;
      const geo = lay(num, lattice, fitted(lattice, rings, w, h), w, h);
      for (let n = 1; n <= L.count(lattice === 'hex' ? 'Hex' : 'Square', 2 * rings + 1); n++) {
        const [x, y] = L.xy(lattice === 'hex' ? 'Hex' : 'Square', n);
        const [px, py] = geo.at(Number(x), Number(y));
        expect(px > geo.px / 2 && px < w - geo.px / 2 && py > geo.px / 2 && py < h - geo.px / 2).toBe(true);
      }
    }
  }
  expect(fitted('square', 100, 800, 600)).toBeCloseTo((0.94 * 600) / 201);
});

test('the rings that fill a stage cover every corner cell and stop at the cap', () => {
  for (const lattice of ['square', 'hex']) {
    const geo = lay(num, lattice, 6, 500, 300);
    const rings = reach(num, geo, 500, 300);
    const name = lattice === 'hex' ? 'Hex' : 'Square';
    for (const [x, y] of [[0, 0], [500, 0], [0, 300], [500, 300], [250, 0], [0, 150]]) expect(Number(L.ring_of(name, ...geo.cell(x, y)))).toBeLessThanOrEqual(rings);
    expect(rings).toBeLessThan(60);
  }
  expect(reach(num, lay(num, 'square', 2, 4000, 4000), 4000, 4000)).toBe(RINGS);
  expect(reach(num, lay(num, 'square', 6, 5120, 2880), 5120, 2880)).toBeLessThan(RINGS);
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

test('the span runs twelve and a half rings a second between three and twenty seconds', () => {
  expect([span(10), span(100), span(400)]).toEqual([LEAST, 8000, MOST]);
});

test('the share drawn rises to one over the span, holds, then starts over, and a still is whole', () => {
  const length = span(100);
  expect([share(0, length, false), share(length / 2, length, false), share(length, length, false)]).toEqual([0, 0.5, 1]);
  expect(share(length + HOLD / 2, length, false)).toBe(1);
  expect(share(length + HOLD + length / 4, length, false)).toBe(0.25);
  expect(share(0, length, true)).toBe(1);
});

test('a random sheet rolls the lattice, an a, a b of its parity and an odd prime c the knobs can hold, the same for one seed, and a named one is its own', () => {
  const rolls = Array.from({ length: 200 }, (_, i) => choose(num, value({ roll: 1 }), rng(i)));
  expect(new Set(rolls.map((one) => one.lattice))).toEqual(new Set(['square', 'hex']));
  expect(new Set(rolls.map((one) => one.a))).toEqual(new Set(Array.from({ length: LEAD }, (_, i) => i + 1)));
  expect(new Set(rolls.map((one) => one.b)).size).toBeGreaterThan(10);
  expect(rolls.every(({ a, b, c }) => (a + b) % 2 === 0 && c % 2 === 1 && num.prime.is_prime(c))).toBe(true);
  for (const one of rolls) expect(tidy(SPEC, value({ ...one, roll: 0 }))).toEqual(value({ ...one, roll: 0 }));
  expect(choose(num, value({ roll: 1 }), rng(5))).toEqual(choose(num, value({ roll: 1 }), rng(5)));
  expect(choose(num, value({ roll: 0, lattice: 'hex', a: 2, b: 7, c: 9 }), rng(5))).toEqual({ lattice: 'hex', a: 2, b: 7, c: 9 });
});

test('a random sheet mostly rolls the a that joins into a rail, 4 on the square and 3 on the hexagon, and still rolls the others', () => {
  const rolls = Array.from({ length: 400 }, (_, i) => choose(num, value({ roll: 1 }), rng(i)));
  const joined = rolls.filter(({ lattice, a }) => a === (lattice === 'hex' ? 3 : 4)).length;
  expect(joined / rolls.length).toBeGreaterThan(0.7);
  expect(joined).toBeLessThan(rolls.length);
});

test('the line steps to the next prime up or down and stays at the ends', () => {
  expect([step(num, 41, 1, 200), step(num, 41, -1, 200), step(num, 2, -1, 200), step(num, 199, 1, 200), step(num, 0, 1, 200)]).toEqual([43, 37, 2, 199, 2]);
});

test("Euler's diagonal is one run of neighbours on the square, a spoke one run on the hexagon, and a curve stays in pieces", () => {
  const euler = num.spiral.diagonal('Square', 201, 4, -2, 41);
  const runs = segments(num, 'square', euler.cells);
  expect(runs.at(-1)).toEqual(Array.from({ length: 81 }, (_, i) => i + 20));
  expect(runs.flat().filter((i) => i < 20).length).toBeLessThan(10);
  const spoke = num.spiral.diagonal('Hex', 41, 3, 3, 1);
  expect(segments(num, 'hex', spoke.cells)).toEqual([Array.from({ length: 21 }, (_, i) => i)]);
  const curve = num.spiral.diagonal('Square', 41, 1, 1, 41);
  expect(segments(num, 'square', curve.cells).flat().length).toBeLessThan(curve.cells.length / 2);
});

test('the formula and the factors read as a person writes them', () => {
  expect(formula(4, -2, 41)).toBe('4k² - 2k + 41');
  expect(formula(1, 0, 0)).toBe('1k²');
  expect(formula(3, 3, 1)).toBe('3k² + 3k + 1');
  expect(factors([[41, 1], [43, 1]])).toBe('41 · 43');
  expect(factors([[2, 3], [5, 1]])).toBe('2^3 · 5');
});

test("the whole sheet of 100 rings holds 40401 numbers, and Euler's line opens with 21 primes then 1763", () => {
  const plan = study(num, value({ fit: 1, rings: 100 }), 800, 600, 2);
  expect(plan.facts).toMatchObject({ lattice: 'square', rings: 100, side: 201, top: 40401, primes: num.prime.prime_count(40401), lit: num.prime.prime_count(40401), a: 4, b: -2, c: 41, rolled: true });
  expect(plan.facts.line).toMatchObject({ count: 101, hits: 80, streak: 21, next: { n: 1763, factors: [[41, 1], [43, 1]] } });
  expect(plan.facts.line.share).toBeCloseTo(80 / 101);
  expect(plan.tone.length).toBe(40402);
  expect(plan.runs).toHaveLength(1);
  expect(plan.xs.length).toBe(40402);
});

test('every landing of the quadratic is a cell of its own tone, a hit when prime and a miss when not, even when no two touch', () => {
  const sparse = study(num, value({ fit: 1, rings: 100, a: 1, b: 1, roll: 0, c: 41 }), 800, 600, 2);
  expect(sparse.runs.flat()).toEqual([]);
  expect([count(sparse.tone, HIT), count(sparse.tone, MISS)]).toEqual([sparse.facts.line.hits, sparse.facts.line.count - sparse.facts.line.hits]);
  expect(sparse.facts.line.count).toBe(201);
  const line = num.spiral.diagonal('Square', 201, 1, 1, 41);
  line.values.forEach((n, i) => expect(sparse.tone[n]).toBe(line.hit[i] ? HIT : MISS));
  const bare = study(num, value({ fit: 1, rings: 100, a: 0 }), 800, 600, 2);
  expect([count(bare.tone, HIT), count(bare.tone, MISS)]).toEqual([0, 0]);
});

test('the composites are ground only while faint is on, and the marks keep their own tones', () => {
  const on = study(num, value({ fit: 1, rings: 10, a: 0, faint: 1 }), 800, 600, 2);
  const off = study(num, value({ fit: 1, rings: 10, a: 0, faint: 0 }), 800, 600, 2);
  expect([count(on.tone, LIT), count(on.tone, SOIL), count(on.tone, LIT) + count(on.tone, SOIL)]).toEqual([on.facts.primes, on.facts.top - on.facts.primes, on.facts.top]);
  expect([count(off.tone, SOIL), count(off.tone, LIT)]).toEqual([0, off.facts.primes]);
  const mobius = study(num, value({ fit: 1, rings: 10, a: 0, mark: 'mobius' }), 800, 600, 2);
  expect(count(mobius.tone, DIM)).toBeGreaterThan(0);
  expect(mobius.facts.lit).toBe(count(mobius.tone, LIT));
});

test('a filled stage takes its rings from the stage and the cell, no line when a is zero, and a typed c is kept', () => {
  const plan = study(num, value({ a: 0 }), 400, 300, 2);
  expect([plan.lattice, plan.geo.px, plan.rings, plan.line, plan.runs, plan.facts.line]).toEqual(['square', 6, reach(num, plan.geo, 400, 300), null, [], null]);
  expect(plan.facts.top).toBe(L.count('Square', 2 * plan.rings + 1));
  const typed = study(num, value({ c: 17, roll: 0, mark: 'mobius', lattice: 'hex' }), 400, 300, 2);
  expect([typed.facts.c, typed.facts.rolled, typed.name]).toEqual([17, false, 'Hex']);
});

test('the colours are the ink, the ink faint, a half shade for the line solid for a hit and hollow for a miss, and a ground wash', () => {
  expect(mix('#000000', '#ffffff', 0.5)).toBe('rgb(128, 128, 128)');
  const tones = palette('#008cff', '#000000');
  expect(tones[LIT]).toEqual({ color: '#008cff', hollow: false });
  expect(tones[DIM].color).toBe(`rgba(0, 140, 255, ${FAINT})`);
  expect(tones[SOIL].color).toBe(`rgba(0, 140, 255, ${GROUND})`);
  expect([tones[HIT].color === tones[MISS].color, tones[HIT].hollow, tones[MISS].hollow]).toEqual([true, false, true]);
  expect(tones[HIT].color).toBe('rgb(0, 70, 128)');
});

test('the svg holds every tone as rects on the square and as hexagons used on the hexagon, the line as a path over them', () => {
  const tones = palette('#008cff', '#000000');
  const plan = study(num, value({ fit: 1, rings: 5, a: 0 }), 800, 600, 2);
  const text = picture(plan, () => true, tones);
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="110" height="110" viewBox="0 0 110 110"')).toBe(true);
  const lit = count(plan.tone, LIT);
  expect(text.match(/<rect /g).length).toBeGreaterThan(lit / 3);
  expect(text).toContain('fill="#008cff"');
  expect(text).toContain(`fill="rgba(0, 140, 255, ${GROUND})"`);
  expect(text).not.toContain('<path d="M');
  const euler = study(num, value({ fit: 1, rings: 30, roll: 0 }), 800, 600, 2);
  const lined = picture(euler, () => true, tones);
  expect(lined).toContain('<path d="M');
  expect(lined).toContain('stroke="#008cff" stroke-opacity=');
  expect(lined).toContain('fill="none" stroke="rgb(0, 70, 128)"');
  expect(lined.match(/<rect /g).length).toBeGreaterThan(count(euler.tone, MISS));
  const hex = study(num, value({ fit: 1, rings: 5, lattice: 'hex', a: 0, faint: 0 }), 800, 600, 2);
  const used = picture(hex, () => true, tones);
  expect(used).toContain('<defs><path id="h"');
  expect(used.match(/<use /g)).toHaveLength(count(hex.tone, LIT));
  const some = picture(plan, (n) => n <= 9, tones);
  expect(some).toContain('width="30" height="30"');
});

test('the csv lists k, n and the hit of every landing', () => {
  const line = num.spiral.diagonal('Square', 21, 4, -2, 1);
  const rows = table(line).split('\n');
  expect(rows[0]).toBe('k,n,prime');
  expect(rows).toHaveLength(line.values.length + 1);
  expect(rows[2]).toBe('1,3,1');
});
