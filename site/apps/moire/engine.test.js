import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { rng } from '../../lib/scene.js';
import { CROPS, TOP, bins, climb, colorizer, crop, design, drawn, field, heat, law, matrix, odd, paint, palette, presetOf, presets, stops, study } from './engine.js';

const LOOK = { paper: '#ffffff', accent: '#008cff' };

const NIGHT = { paper: '#000000', accent: '#008cff' };

const value = (over) => ({ base: 3, code: '495', level: 1, limit: 9, combine: 'sum', seed: 1, ...over });

const nans = (data) => data.reduce((sum, v) => sum + (Number.isNaN(v) ? 1 : 0), 0);

test('the odd scales run from one to the limit, an even limit rounded up', () => {
  expect(odd(9)).toEqual([1, 3, 5, 7, 9]);
  expect(odd(8)).toEqual([1, 3, 5, 7, 9]);
  expect(odd(1)).toEqual([1]);
});

test('the climb steps two scales a beat and wraps from the top back to one', () => {
  expect([climb(9, 1), climb(9, 2), climb(TOP - 2, 1), climb(TOP, 1), climb(TOP, 2)]).toEqual([11, 13, TOP, 1, 3]);
});

test('a ramp runs paper to accent, through the contrast, in bands, or through the fire and diverge ramps of the theme', () => {
  expect(bins('tint', 3, LOOK)).toEqual(['#ffffff', '#80c6ff', '#008cff']);
  expect(bins('ink', 3, LOOK)).toEqual(['#ffffff', '#008cff', '#000000']);
  expect(bins('ink', 2, NIGHT).at(-1)).toBe('#ffffff');
  expect(bins('bands', 4, LOOK)).toEqual(['#ffffff', '#008cff', '#ffffff', '#008cff']);
  const ink = palette(LOOK);
  expect(stops('fire', LOOK)).toEqual(ink.Ramp.fire().stops.map((c) => c.slice(0, 3)));
  expect(stops('diverge', NIGHT)[1]).toEqual([0, 0, 0]);
  expect(colorizer('tint', 2, LOOK)).toEqual({ Bins: { background: '#ffffff', ramp: ['#ffffff', '#008cff'] } });
});

test('the r-matrix paints zero in the foreground and a full correlation deep', () => {
  const ink = palette(NIGHT);
  expect(heat(ink, 0)).toEqual(ink.fg);
  expect(heat(ink, 1)).toEqual(ink.mix(ink.blue, ink.ground, 0.6));
});

test('a typed code is kept modulo the space, a blank one rolls from the seed, the level is clamped', () => {
  expect(design(math, { base: 3, code: '495', level: 1 }, rng(1))).toEqual({ base: 3, code: '495', level: 1, name: 'carpet' });
  expect(design(math, { base: 2, code: '23', level: 9 }, rng(1))).toMatchObject({ base: 2, code: '7', level: 5, name: 'carpet' });
  expect(design(math, { base: 3, code: '', level: 1 }, rng(1)).code).not.toBe(design(math, { base: 3, code: '', level: 1 }, rng(2)).code);
});

test('the presets come from the door as design keys, and a seed draws one of them with an odd limit', () => {
  const list = presets(math);
  expect(list.map((one) => one.name)).toEqual(['heatmap', 'weave', 'hive', 'carpet']);
  expect(list.find((one) => one.name === 'hive')).toEqual({ name: 'hive', base: 2, code: '1', level: 1, combine: 'hive' });
  expect([presetOf(list, value()), presetOf(list, value({ code: '7' }))]).toEqual(['carpet', '']);
  const one = drawn(math, 5);
  expect(one).toEqual(drawn(math, 5));
  expect([one.seed, one.limit % 2, one.limit <= TOP, presetOf(list, one) !== '']).toEqual([5, 1, true, true]);
});

test('the r-matrix is zero exactly where two scales are coprime', () => {
  const { scales, grid } = matrix(math, 15);
  expect(scales).toEqual([3, 5, 7, 9, 11, 13, 15]);
  expect([grid[3][0] > 0, grid[2][1], grid[6][1] > 0, grid[0][0]]).toEqual([true, 0, true, 1]);
});

test('the law reads the row at the largest scale under the pick and snaps the mate onto its earlier scales', () => {
  expect(law(math, 9, 9, 3)).toMatchObject({ row: 9, mate: 3, k: 4 });
  expect(law(math, 9, 9, 3).r).toBeCloseTo(0.2192645, 6);
  expect(law(math, 9, 41, 99)).toMatchObject({ row: 9, mate: 7, r: 0 });
  expect(law(math, 15, 13, 5)).toMatchObject({ row: 13, mate: 5, r: 0 });
  expect(law(math, 1, 9, 3)).toMatchObject({ row: 0, mate: 0, r: null, k: 0 });
});

test('a stack layers the odd scales on a square, on the hexagonal lattice for the hive', () => {
  const plan = study(math, value(), rng(1));
  expect([plan.name, plan.numbers, plan.merge, plan.lattice]).toEqual(['carpet', [1, 3, 5, 7, 9], 'Sum', 'Square']);
  expect(study(math, value({ combine: 'hive' }), rng(1)).lattice).toBe('Hex');
  const made = field(math, plan, 64);
  expect([made.size, made.data.length, made.field.max(), nans(made.data)]).toEqual([64, 4096, 5, 0]);
  made.field.free();
});

test('a crop keeps the inside of the shape, an anti crop the outside, the rest NaN', () => {
  expect(CROPS.slice(1).map(([one]) => one)).toEqual(math.shape.shapes(2));
  expect(crop(math, '', 16, 0, 64)).toBe(null);
  const plan = study(math, value(), rng(1));
  const inside = field(math, plan, 64, crop(math, 'ball', 16, 0, 64));
  const outside = field(math, plan, 64, crop(math, 'ball', 16, 1, 64));
  expect([Number.isNaN(inside.data[0]), Number.isNaN(inside.data[32 * 64 + 32]), Number.isNaN(outside.data[0]), Number.isNaN(outside.data[32 * 64 + 32])]).toEqual([true, false, false, true]);
  expect(nans(inside.data) + nans(outside.data)).toBeGreaterThanOrEqual(4096);
  inside.field.free();
  outside.field.free();
});

test('the paint is one colour a level with a clear alpha wherever the field is NaN', () => {
  const plan = study(math, value(), rng(1));
  const made = field(math, plan, 32, crop(math, 'ball', 16, 0, 32));
  const image = paint(math, made, colorizer('tint', 4, LOOK), 4, 0);
  expect([image.shape, image.colors[3], image.colors[(16 * 32 + 16) * 4 + 3]]).toEqual([[32, 32], 0, 255]);
  made.field.free();
});
