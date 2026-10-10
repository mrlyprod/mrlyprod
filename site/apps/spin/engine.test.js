import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { LEAD, NEEDLES, ORDERS, OUT, SIDE, STEPS, WHEEL, angle, cap, design, disc, layout, leading, pace, peak, petals, pixels, rings, rosette, seen, shares, step, study, turns } from './engine.js';
import { DESIGN } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = (over) => ({ ...defaults(DESIGN), seed: 1, ...over });

test('the level cap keeps the raster side under the budget for every number', () => {
  expect([cap(3), cap(5), cap(7), cap(9)]).toEqual([5, 3, 2, 2]);
  for (const n of [3, 5, 7, 9]) expect([n ** cap(n) <= SIDE, n ** (cap(n) + 1) > SIDE]).toEqual([true, true]);
});

test('a blank or junk code rolls one from the seed, never the empty design, and a number wraps into its base', () => {
  const rolled = design(math, value({ code: '' }), rng(5));
  expect(design(math, value({ code: 'carpet' }), rng(5))).toBe(rolled);
  expect(Number(rolled) >= 1 && Number(rolled) < 512).toBe(true);
  expect(new Set(Array.from({ length: 64 }, (_, seed) => design(math, value({ base: 2 }), rng(seed)))).has('0')).toBe(false);
  expect(design(math, value({ code: '495' }), rng(1))).toBe('495');
  expect(design(math, value({ base: 2, code: '23' }), rng(1))).toBe('7');
});

test('the carpet of 3 at level 2 reads 64 fills, rings of the same mass, rotation order 4 and 12 petals over 6 copies', () => {
  const plan = study(math, value({ code: '495', level: 2 }), rng(1));
  expect([plan.code, plan.level, plan.side, plan.fills, plan.order, plan.profile.length, plan.power.length]).toEqual(['495', 2, 9, 64, 4, STEPS, ORDERS + 1]);
  expect(Math.abs(plan.mass - plan.fills)).toBeLessThan(0.5);
  expect(plan.reach).toBeCloseTo(9 / Math.SQRT2, 9);
  expect([plan.inner, plan.peak]).toEqual([4.5, 1]);
  expect(plan.disc).toBeCloseTo(1.5, 1);
  expect(plan.share.reduce((a, b) => a + b, 0)).toBeCloseTo(1, 9);
  expect(plan.leading).toHaveLength(LEAD);
  expect(plan.leading.every((one) => one.order % 4 === 0 && one.share > 0)).toBe(true);
  expect(plan.leading[0].order).toBe(4);
  expect(petals(math, plan, 6)).toBe(12);
  expect(petals(math, plan, 4)).toBe(4);
  expect(petals(math, { order: 0 }, 6)).toBe(0);
});

test('a study is kept for its design keys and read again when one changes or the seed rolls another code', () => {
  const first = study(math, value({ code: '495', level: 2 }), rng(1));
  expect(study(math, value({ code: '495', level: 2 }), rng(2))).toBe(first);
  expect(study(math, value({ code: '495', level: 3 }), rng(1))).not.toBe(first);
  const blank = study(math, value({ code: '', seed: 3 }), rng(3));
  expect(study(math, value({ code: '', seed: 4 }), rng(4))).not.toBe(blank);
});

test('a level over the cap is grown at the cap', () => {
  const plan = study(math, value({ code: '495', number: 9, level: 5 }), rng(1));
  expect([plan.level, plan.side]).toEqual([2, 81]);
});

test('the rosette stacks the copies by the blend on the output raster, its top the peak of the stack', () => {
  const plan = study(math, value({ code: '495', level: 2 }), rng(1));
  const mean = rosette(math, plan, 4, 'mean');
  expect([mean.values.length, mean.side, mean.top]).toEqual([OUT * OUT, OUT, 1]);
  expect(rosette(math, plan, 4, 'mean')).toBe(mean);
  const sum = rosette(math, plan, 4, 'sum');
  expect(sum).not.toBe(mean);
  expect(sum.top).toBeGreaterThan(1);
  expect(sum.top).toBeLessThanOrEqual(4);
  expect(rosette(math, plan, 1, 'union').top).toBe(1);
  const dot = study(math, value({ code: '1', level: 2 }), rng(1));
  expect(rosette(math, dot, 4, 'mean').top).toBeCloseTo(0.25, 2);
});

test('the wheel raster is kept per plan at one side and its top is the profile peak, so a lone cell still lights its ring', () => {
  const dot = study(math, value({ code: '1', level: 2 }), rng(1));
  const bull = rings(math, dot);
  expect([bull.values.length, bull.side]).toEqual([WHEEL * WHEEL, WHEEL]);
  expect(bull.top).toBe(dot.peak);
  expect(dot.peak).toBeLessThan(0.05);
  expect(dot.disc).toBeGreaterThan(dot.inner);
  expect(rings(math, dot)).toBe(bull);
  expect(rings(math, study(math, value({ code: '495', level: 2 }), rng(1)))).not.toBe(bull);
});

test('the peak is the top of a profile and the dark disc ends at its first lit sample, the reach when none is', () => {
  expect(peak([0, 0.5, 0.2])).toBe(0.5);
  expect(disc(Float32Array.from([0, 0, 1, 0]), 3)).toBe(2);
  expect(disc(Float32Array.from([1, 0]), 3)).toBe(0);
  expect(disc(Float32Array.from([0, 0]), 3)).toBe(3);
  expect(NEEDLES).toEqual([33, 45, 78, 899, 900]);
});

test('the shares split the power and the leading orders come strongest first past order zero', () => {
  expect(shares([2, 1, 1])).toEqual([0.5, 0.25, 0.25]);
  expect(shares([0, 0])).toEqual([0, 0]);
  expect(leading([10, 1, 0, 3, 2], 2)).toEqual([{ order: 3, share: 3 / 16 }, { order: 4, share: 2 / 16 }]);
});

test('pixels carry the accent at an alpha of the value over the top', () => {
  const bytes = pixels(Float32Array.from([0, 0.5, 2]), 2, [1, 2, 3]);
  expect(Array.from(bytes)).toEqual([0, 0, 0, 0, 1, 2, 3, 64, 1, 2, 3, 255]);
});

test('the turntable turns rpm over sixty seconds and a frame steps six degrees an rpm a second', () => {
  expect(turns(33, 60000)).toBe(33);
  expect(angle(60, 500)).toBeCloseTo(Math.PI, 9);
  expect(angle(60, 1000)).toBeCloseTo(0, 9);
  expect(step(900, 1000 / 60)).toBeCloseTo(90, 9);
  expect(step(33, 1000 / 60)).toBeCloseTo(3.3, 9);
});

test('what the eye sees is the step reduced by the rotation order: a quarter turn a frame freezes a fourfold design', () => {
  expect(seen(90, 4)).toBe(0);
  expect(seen(89, 4)).toBeCloseTo(-1, 9);
  expect(seen(198, 1)).toBeCloseTo(-162, 9);
  expect(seen(3.3, 4)).toBeCloseTo(3.3, 9);
  expect(seen(50, 0)).toBe(0);
});

test('the pace eases toward each new frame time', () => {
  expect(pace(16, 0)).toBe(16);
  expect(pace(26, 16)).toBe(17);
});

test('a single disc fills the stage, two sit side by side on a wide stage and stacked on a tall one', () => {
  expect(layout(800, 600, 'turn', 20)).toEqual({ turn: { x: 400, y: 300, r: 280 } });
  expect(layout(800, 600, 'radial', 20)).toEqual({ radial: { x: 400, y: 300, r: 280 } });
  const wide = layout(800, 600, 'both', 32);
  expect(wide).toEqual({ turn: { x: 208, y: 300, r: 176 }, wheel: { x: 592, y: 300, r: 176 } });
  const tall = layout(600, 800, 'both', 32);
  expect(tall).toEqual({ turn: { x: 300, y: 208, r: 176 }, wheel: { x: 300, y: 592, r: 176 } });
  expect(layout(4000, 600, 'both', 32).turn.r).toBe(268);
  expect(layout(10, 10, 'both', 32).turn.r).toBe(1);
});
