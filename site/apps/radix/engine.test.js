import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { BUDGET, FADE, HOLD, KINDS, MOST, STEP, bands, bits, cap, choose, corners, deepest, ease, lattice, lay, phase, picture, reach, resolve, roll, spell, steps, study } from './engine.js';
import { SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });
num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const value = (over) => ({ ...defaults(SPEC), seed: 1, code: '', ...over });

test('a rolled code has five digits or more and never the whole box, the same for one seed', () => {
  const codes = Array.from({ length: 3000 }, (_, i) => Number(roll(rng(i))));
  expect(codes.every((code) => code >= 1 && code <= 510 && bits(code) >= 5)).toBe(true);
  expect(Array.from({ length: 20 }, (_, i) => roll(rng(i)))).toEqual(Array.from({ length: 20 }, (_, i) => roll(rng(i))));
});

test('a code is read modulo the box, and an empty, unreadable or digitless one takes the roll', () => {
  expect([resolve('495', rng(1)), resolve('1000', rng(1)), resolve('-1', rng(1))]).toEqual(['495', '488', '511']);
  expect(resolve('', rng(1))).toBe(roll(rng(1)));
  expect(resolve('carpet', rng(1))).toBe(roll(rng(1)));
  expect(resolve('0', rng(1))).toBe(roll(rng(1)));
  expect(resolve('512', rng(1))).toBe(roll(rng(1)));
});

test('the deepest level keeps the fill inside the budget and a lone digit at the ceiling', () => {
  expect([deepest(2), deepest(3), deepest(4), deepest(7), deepest(8), deepest(9), deepest(1), deepest(0)]).toEqual([17, 10, 8, 6, 5, 5, MOST, MOST]);
  expect(2 ** 17 <= BUDGET && 2 ** 18 > BUDGET).toBe(true);
});

test('the cap follows the shape, the tile its code or the code its seed rolls, and a random shape takes the ceiling', () => {
  expect([cap(value({ shape: 'twindragon' })), cap(value({ shape: 'terdragon' })), cap(value({ shape: 'flowsnake' })), cap(value({ shape: 'koch' })), cap(value({ shape: 'gasket' }))]).toEqual([17, 10, 6, 8, 10]);
  expect([cap(value({ shape: 'tile', code: '495' })), cap(value({ shape: 'tile', code: '3' })), cap(value({ shape: '' }))]).toEqual([5, 17, MOST]);
  const seeds = Array.from({ length: 30 }, (_, seed) => seed);
  const caps = seeds.map((seed) => cap(value({ shape: 'tile', code: '', seed })));
  expect(caps).toEqual(seeds.map((seed) => study(num, math, value({ shape: 'tile', code: '', seed, level: MOST }), rng(seed)).top));
  expect(new Set(caps).size).toBeGreaterThan(1);
});

test('a random shape is rolled from the seed with a turn of the ring, a named shape keeps its code out and a tile rolls one', () => {
  const rolled = Array.from({ length: 40 }, (_, i) => choose(value({ shape: '' }), rng(i)));
  expect(new Set(rolled.map((one) => one.kind)).size).toBe(KINDS.length);
  expect(rolled.every((one) => KINDS.includes(one.kind) && (one.kind !== 'tile' || bits(Number(one.code)) >= 5))).toBe(true);
  expect(choose(value({ shape: '' }), rng(7))).toEqual(choose(value({ shape: '' }), rng(7)));
  expect(choose(value({ shape: 'koch', code: '495' }), rng(1))).toMatchObject({ kind: 'koch', code: '' });
  expect(choose(value({ shape: 'tile', code: '495' }), rng(1))).toEqual({ kind: 'tile', code: '495', turn: 0 });
  const turns = new Set(Array.from({ length: 40 }, (_, i) => choose(value({ shape: 'twindragon' }), rng(i)).turn));
  expect([...turns].sort((a, b) => a - b)).toEqual([0, Math.PI / 2, Math.PI, (3 * Math.PI) / 2]);
});

test('fixing the shape a random seed rolled keeps its turn and its code, written or left blank', () => {
  for (let seed = 0; seed < 200; seed++) {
    const rolled = choose(value({ shape: '' }), rng(seed));
    expect(choose(value({ shape: rolled.kind, code: rolled.code }), rng(seed))).toEqual(rolled);
    expect(choose(value({ shape: rolled.kind, code: '' }), rng(seed))).toEqual(rolled);
  }
});

test('the steps move the kind and the level from now and keep a tile code', () => {
  const told = [];
  const fresh = (facts) => steps({ facts }, (patch) => told.push(patch));
  const tile = fresh({ kind: 'tile', code: '495', level: 4 });
  tile.deeper(-1);
  tile.turn(1);
  const dragon = fresh({ kind: 'twindragon', code: '3', level: 6 });
  dragon.deeper(1);
  dragon.turn(-1);
  expect(told).toEqual([{ shape: 'tile', level: 3, code: '495' }, { shape: 'twindragon' }, { shape: 'twindragon', level: 7 }, { shape: 'tile' }]);
});

test('a base is spelled with its letter', () => {
  expect([spell('i', 1, 1), spell('w', 2, 1), spell('w', 3, 0), spell('i', 0, -1), spell('w', -1, -2), spell('i', 0, 0)]).toEqual(['1 + i', '2 + w', '3', '-i', '-1 - 2w', '0']);
});

test('the twindragon study reads base 1 + i of norm 2 and lays every level up to the top with a frame round the last', () => {
  const plan = study(num, math, value({ shape: 'twindragon', level: 6 }), rng(1));
  expect(plan.facts).toMatchObject({ kind: 'twindragon', name: 'Twindragon', code: '3', ring: 'Z[i]', base: '1 + i', norm: 2, digits: 2, canonical: true, dimension: 2, level: 6, fill: 64, distinct: 64 });
  expect([plan.sides, plan.top, plan.levels.length, plan.sign]).toEqual([4, 6, 6, -1]);
  expect(plan.levels.map((pts) => pts.length / 2)).toEqual([2, 4, 8, 16, 32, 64]);
  expect(plan.angle).toBeCloseTo(Math.PI / 4);
  const [x0, y0, x1, y1] = plan.frame;
  const last = plan.levels[5];
  for (let i = 0; i < last.length; i += 2) expect(last[i] >= x0 && last[i] <= x1 && last[i + 1] >= y0 && last[i + 1] <= y1).toBe(true);
  expect([plan.fill(3), plan.distinct(3)]).toEqual([8, 8]);
});

test('the terdragon glues words onto one point, so its distinct count falls under its fill', () => {
  const plan = study(num, math, value({ shape: 'terdragon', level: 4 }), rng(1));
  expect(plan.facts).toMatchObject({ ring: 'Z[w]', base: '2 + w', norm: 3, digits: 3, fill: 81, distinct: 43 });
  expect(plan.angle).toBeCloseTo(Math.PI / 6);
});

test('a tile study keeps the box code, names the carpet, draws y down and caps the level by its digits', () => {
  const plan = study(num, math, value({ shape: 'tile', code: '495', level: 9 }), rng(1));
  expect(plan.facts).toMatchObject({ kind: 'tile', name: 'carpet', code: '495', ring: 'Z[i]', base: '3', norm: 9, digits: 8, level: 5, fill: 32768, distinct: 32768 });
  expect(plan.facts.dimension).toBeCloseTo(Math.log(8) / Math.log(3));
  expect([plan.sign, plan.turn, plan.angle]).toEqual([1, 0, 0]);
  const one = plan.levels[0];
  expect([...one].map((v) => Math.round(v * 3))).toEqual([0, 0, 1, 0, 2, 0, 0, 1, 2, 1, 0, 2, 1, 2, 2, 2]);
});

test('a random shape takes its deepest level whatever the level knob says, and a blank tile code is rolled after the kind and the turn', () => {
  const plan = study(num, math, value({ shape: '', level: 2 }), rng(3));
  expect(plan.top).toBe(deepest(plan.facts.digits));
  const tile = study(num, math, value({ shape: 'tile', code: '' }), rng(2));
  const rand = rng(2);
  rand();
  rand();
  expect(tile.facts.code).toBe(roll(rand));
});

test('the lattice of a level is the base scale to the level, the cell a square or a hexagon through its neighbours', () => {
  expect(lattice(2, 4, 2)).toEqual({ spacing: 0.5, radius: 0.5 / Math.SQRT2 });
  expect(lattice(4, 6, 1)).toEqual({ spacing: 0.5, radius: 0.5 / Math.sqrt(3) });
  const square = corners(4, Math.PI / 4, 1, 0, 1);
  expect([...square].map((v) => Math.round(v * 1e6) / 1e6)).toEqual([1, 0, 0, 1, -1, 0, -0, -1]);
  const hex = corners(6, 0, 3, 0, 2);
  expect([hex.length, hex[0], hex[1]]).toEqual([12, 2 * Math.cos(Math.PI / 6), 2 * Math.sin(Math.PI / 6)]);
});

test('a small cell is as wide as the bounding box of the cell, so a turned square lattice leaves no holes', () => {
  for (const level of [1, 2, 3, 4, 5]) {
    for (const turn of [0, Math.PI / 2, Math.PI]) {
      const { spacing, radius } = lattice(2, 4, level);
      const ring = corners(4, Math.PI / 4, level, turn, radius);
      const xs = [ring[0], ring[2], ring[4], ring[6]];
      expect(Math.max(...xs) - Math.min(...xs)).toBeCloseTo(spacing * reach(4, Math.PI / 4, level, turn));
    }
  }
  expect([reach(4, Math.PI / 4, 16, 0), reach(4, Math.PI / 4, 17, 0)]).toEqual([1, Math.SQRT2].map((v) => expect.closeTo(v)));
  expect([reach(6, Math.PI / 6, 5, 0), reach(4, 0, 9, 0)]).toEqual([1, 1]);
});

test('the clock fades each level in over its step, holds the top, then starts over from the top', () => {
  expect(phase(0, 5, false)).toEqual({ level: 1, k: 0, from: 0 });
  expect(phase(FADE / 2, 5, false)).toEqual({ level: 1, k: 0.5, from: 0 });
  expect(phase(STEP + FADE / 4, 5, false)).toEqual({ level: 2, k: 0.25, from: 1 });
  expect(phase(3 * STEP + FADE, 5, false)).toEqual({ level: 4, k: 1, from: 3 });
  expect(phase(5 * STEP + HOLD / 2, 5, false)).toEqual({ level: 5, k: 1, from: 4 });
  expect(phase(5 * STEP + HOLD + FADE / 2, 5, false)).toEqual({ level: 1, k: 0.5, from: 5 });
  expect(phase(1234, 5, true)).toEqual({ level: 5, k: 1, from: 0 });
  expect([ease(0), ease(0.5), ease(1)]).toEqual([0, 0.5, 1]);
});

test('the fit centres the frame inside the padding, turns a wide picture onto a tall stage, and a pinned scale wins', () => {
  const flat = lay([0, 0, 4, 1], 800, 600, 10);
  expect(flat).toEqual({ k: 195, cx: 2, cy: 0.5, ox: 400, oy: 300, quarter: false });
  const tall = lay([0, 0, 4, 1], 600, 800, 10);
  expect([tall.k, tall.quarter]).toEqual([195, true]);
  expect(lay([0, 0, 1, 1], 600, 800, 10).quarter).toBe(false);
  expect(lay([0, 0, 4, 1], 800, 600, 10, 50).k).toBe(50);
});

test('the svg of a tile is one rect per run, that of a dragon one use of the cell per distinct point', () => {
  const tile = study(num, math, value({ shape: 'tile', code: '495', level: 2 }), rng(1));
  const sheet = picture(tile, '#008cff');
  expect(sheet).toContain('width="90" height="90" viewBox="0 0 9 9"');
  expect(sheet.match(/<rect /g).length).toBeLessThan(64);
  expect([...sheet.matchAll(/width="(\d+)" height="1"/g)].reduce((sum, [, w]) => sum + Number(w), 0)).toBe(64);
  expect(sheet).toContain('fill="#008cff"');
  const dragon = picture(study(num, math, value({ shape: 'terdragon', level: 4 }), rng(1)), '#008cff');
  expect(dragon.match(/<use /g)).toHaveLength(43);
  expect(dragon).toContain('<path id="c" d="M');
  const [, w, h] = dragon.match(/^<svg xmlns="http:\/\/www\.w3\.org\/2000\/svg" width="([\d.]+)" height="([\d.]+)"/);
  expect(Math.max(Number(w), Number(h))).toBe(2048);
});

test('the runs of a set of words are read row by row, with no raster of the plane', () => {
  expect(bands([[1n, 0n], [0n, 2n], [3n, 0n], [0n, 0n], [1n, 2n]])).toEqual([[0, 0, 2], [0, 3, 1], [2, 0, 2]]);
  expect(bands([])).toEqual([]);
});

test('the svg of a tile at the level cap is a handful of rects drawn in a moment, whatever the side of the plane', () => {
  const start = performance.now();
  for (const code of ['448', '1']) {
    const plan = study(num, math, value({ shape: 'tile', code, level: MOST }), rng(1));
    const sheet = picture(plan, '#008cff');
    const side = 3 ** plan.top;
    expect(sheet.match(/<rect /g)).toHaveLength(1);
    expect(sheet).toContain(`width="2048" height="2048" viewBox="0 0 ${side} ${side}"`);
  }
  expect(performance.now() - start).toBeLessThan(2000);
});
