import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { rng } from '../../lib/scene.js';
import { CUT, FIELD, HEX, LEAD, ORDERS, OUT, PRESETS, SIDE, STEPS, WHEEL, cap, cells, degrees, disc, drawn, key, layout, leading, pace, paint, peak, petals, rings, rosette, seen, shares, source, spun, step, stops, study, title, turn } from './engine.js';
import { SOURCE } from './scene.js';
import { read } from './source.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = (over) => ({ ...read({}), seed: 1, ...over });

test('the level caps keep the flat raster under its budget and the slice under its own', () => {
  expect([cap(3), cap(5), cap(7), cap(9)]).toEqual([5, 3, 2, 2]);
  for (const n of [3, 5, 7, 9]) expect([n ** cap(n) <= SIDE, n ** (cap(n) + 1) > SIDE]).toEqual([true, true]);
  expect([cap(3, CUT), cap(5, CUT)]).toEqual([3, 2]);
});

test('the source opens on the carpet 495 of 3 at level 4, the demo still', () => {
  const plan = study(math, value({}), rng(1));
  expect([plan.kind, plan.code, plan.name, plan.side, plan.fills, plan.order, title(plan)]).toEqual(['flat', '495', 'carpet', 81, 4096, 4, 'carpet 495']);
});

test('the presets are the moire unit names, and a moire source is its field at its side with no fills', () => {
  expect(math.moire.all(1).map((one) => one.name)).toEqual(PRESETS);
  const made = source(math, value({ source: 'moire', preset: 'weave', limit: 9 }), rng(1));
  expect([made.kind, made.name, made.side, made.data.length, made.fills, made.binary]).toEqual(['moire', 'weave', FIELD, FIELD * FIELD, null, false]);
});

test('a hex slice is the cut of the cube design rastered on its square, its fills the cut triangles', () => {
  const made = source(math, value({ source: 'slice', scode: '23', tile: 3, slevel: 2 }), rng(1));
  expect([made.kind, made.code, made.name, made.side, made.data.length, made.fills]).toEqual(['slice', '23', 'carpet', HEX, HEX * HEX, 306]);
  expect(source(math, value({ source: 'slice', scode: '23', tile: 5, slevel: 9 }), rng(1)).fills).toBe(math.six.fills(math.six.cut_design('23', 5, 2, 2)));
});

test('a study is kept for its source keys, and the seed joins the key only where a junk code rolls from it', () => {
  const first = study(math, value({ level: 2 }), rng(1));
  expect(study(math, value({ level: 2, seed: 9 }), rng(9))).toBe(first);
  expect(study(math, value({ level: 3 }), rng(1))).not.toBe(first);
  expect(key(value({ code: 'x', seed: 3 }))).not.toBe(key(value({ code: 'x', seed: 4 })));
  expect(key(value({ source: 'moire', seed: 3 }))).toBe(key(value({ source: 'moire', seed: 4 })));
});

test('a roll draws a new design for the source in hand', () => {
  expect(Object.keys(drawn(math, value({}), rng(2)))).toEqual(['code']);
  expect(Object.keys(drawn(math, value({ source: 'slice' }), rng(2)))).toEqual(['scode']);
  const moire = drawn(math, value({ source: 'moire' }), rng(2));
  expect([PRESETS.includes(moire.preset), moire.limit % 2, read({ ...value({ source: 'moire' }), ...moire }).limit]).toEqual([true, 1, moire.limit]);
});

test('the carpet facts: rings of the mass of the fills, the dark disc, the edge and the leading orders', () => {
  const plan = study(math, value({ level: 2 }), rng(1));
  expect([plan.side, plan.fills, plan.profile.length, plan.power.length, plan.peak]).toEqual([9, 64, STEPS, ORDERS + 1, 1]);
  expect(Math.abs(plan.mass - plan.fills)).toBeLessThan(0.5);
  expect(plan.reach).toBeCloseTo(9 / Math.SQRT2, 9);
  expect(plan.disc).toBeCloseTo(1.5, 1);
  expect(plan.leading).toHaveLength(LEAD);
  expect(plan.leading.every((one) => one.order % 4 === 0)).toBe(true);
});

test('a full turn shares the copies over one turn and shows lcm petals; a partial orbit shows none', () => {
  const plan = study(math, value({ level: 2 }), rng(1));
  const whole = { copies: 6, full: 1, step: 10 };
  const part = { copies: 6, full: 0, step: 45 };
  expect([turn(whole), degrees(whole), turn(part), degrees(part)]).toEqual([1 / 6, 60, 0.125, 45]);
  expect([petals(math, plan, whole), petals(math, plan, { ...whole, copies: 4 }), petals(math, plan, part), petals(math, { order: 0 }, whole)]).toEqual([12, 4, 'partial orbit', 'none']);
});

test('the rosette is kept per plan and stack, its step in turns, its top the peak of the stack', () => {
  const plan = study(math, value({ level: 2 }), rng(1));
  const stack = { copies: 4, full: 1, step: 60, blend: 'mean', samples: 2 };
  const mean = rosette(math, plan, stack);
  expect([mean.values.length, mean.side, mean.top]).toEqual([OUT * OUT, OUT, 1]);
  expect(rosette(math, plan, stack)).toBe(mean);
  expect(rosette(math, plan, { ...stack, full: 0, step: 45 })).not.toBe(mean);
  expect(rosette(math, plan, { ...stack, blend: 'sum' }).top).toBeGreaterThan(1);
});

test('the wheel raster is kept per plan and painted to the profile peak', () => {
  const dot = study(math, value({ code: '1', level: 2 }), rng(1));
  const bull = rings(math, dot);
  expect([bull.values.length, bull.side, bull.top]).toEqual([WHEEL * WHEEL, WHEEL, dot.peak]);
  expect(rings(math, dot)).toBe(bull);
});

test('the peak, the dark disc and the shares', () => {
  expect(peak([0, 0.5, 0.2])).toBe(0.5);
  expect([disc(Float32Array.from([0, 0, 1, 0]), 3), disc(Float32Array.from([1, 0]), 3), disc(Float32Array.from([0, 0]), 3)]).toEqual([2, 0, 3]);
  expect(shares([2, 1, 1])).toEqual([0.5, 0.25, 0.25]);
  expect(leading([10, 1, 0, 3, 2], 2)).toEqual([{ order: 3, share: 3 / 16 }, { order: 4, share: 2 / 16 }]);
});

test('a ramp runs from the paper through its stops, quantised into levels, inverted on request', () => {
  const look = { paper: '#000000', accent: '#0000ff' };
  expect(stops('heat', look)).toEqual([[0, 0, 0], [0, 0, 255], [255, 255, 255]]);
  expect(stops('tint', look)).toEqual([[0, 0, 0], [0, 0, 255]]);
  expect(stops('fire', { ...look, paper: '#ffffff' }).at(-1)).toEqual([0, 0, 0]);
  const two = Array.from(paint(Float32Array.from([0, 0.4, 1, NaN]), 1, stops('tint', look), 2, 0));
  expect(two).toEqual([0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 255, 255, 0, 0, 0, 255]);
  expect(Array.from(paint(Float32Array.from([1]), 1, stops('tint', look), 2, 1))).toEqual([0, 0, 0, 255]);
  expect(Array.from(cells(Float32Array.from([0, 1]), [1, 2, 3], 36))).toEqual([1, 2, 3, 36, 1, 2, 3, 255]);
});

test('the phase adds six degrees an rpm a second, so a live rpm change keeps the angle it reached', () => {
  expect(step(900, 1000 / 60)).toBeCloseTo(90, 9);
  expect(spun(350, 60, 100)).toBeCloseTo(26, 9);
  expect(spun(spun(0, 60, 500), 120, 100)).toBeCloseTo(252, 9);
});

test('what the eye sees is the step reduced by the rotation order, eased over frames', () => {
  expect([seen(90, 4), seen(50, 0)]).toEqual([0, 0]);
  expect(seen(89, 4)).toBeCloseTo(-1, 9);
  expect([pace(16, 0), pace(26, 16)]).toEqual([16, 17]);
});

test('the stage is one disc on the whole stage at any aspect, phone included', () => {
  expect(layout(800, 600, 32)).toEqual({ x: 400, y: 300, r: 268 });
  expect(layout(780, 1688, 32)).toEqual({ x: 390, y: 844, r: 358 });
  expect(layout(10, 10, 32).r).toBe(1);
});

test('every source row tidies to its default', () => {
  expect(read({})).toEqual(Object.fromEntries(SOURCE.map((row) => [row.key, row.def])));
});
