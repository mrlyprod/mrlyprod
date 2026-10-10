import { expect, test } from 'bun:test';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { ACROSS, CAPS, HOLD, OCTAVE, ORDER_CAP, PALE, ROOTS, STRIDE, choose, clock, count, depth, glyph, hit, layout, reach, said, shade, stack, step, study, thick } from './engine.js';
import { SPEC } from './scene.js';

num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const a = num.apollonian;

test('the roots, the top cap and the depth cap are the crate\'s own', () => {
  expect(ROOTS).toEqual(a.ROOTS());
  expect(CAPS.at(-1)).toBe(Number(a.CURVATURE_CAP()));
  expect(ORDER_CAP).toBe(a.ORDER_CAP());
  expect(CAPS.every((cap, i) => cap >= 2 && (i === 0 || cap === 2 * CAPS[i - 1]))).toBe(true);
});

test('a blank root draws one from the seed, the same for one seed, and a named root is kept', () => {
  expect(ROOTS).toContain(choose('', rng(1)));
  expect(choose('', rng(7))).toBe(choose('', rng(7)));
  expect(new Set(Array.from({ length: 40 }, (_, i) => choose('', rng(i)))).size).toBe(ROOTS.length);
  expect(choose('strip', rng(1))).toBe('strip');
});

test('the cap steps through the powers of two and stays at the ends', () => {
  expect([step(2048, 1), step(2048, -1), step(8192, 1), step(8, -1), step(999, 1)]).toEqual([4096, 1024, 8192, 8, 4096]);
});

test('the strip to 2048 holds 2448 circles after its two root circles, in curvature order, every line circle a Ford circle', () => {
  const plan = study(num, 'strip', 2048);
  expect(plan.facts).toEqual({ name: 'strip', title: 'strip (0, 0, 2, 2)', cap: 2048, circles: 2448, ford: 323, broken: 0 });
  expect([plan.strip, plan.roots, plan.total, plan.k0, plan.octaves, plan.span, plan.frame, plan.hull]).toEqual([true, 2, 2450, 8, 9, 8 * OCTAVE, [0, 0, 1, 1], null]);
  expect(plan.quad).toEqual([[0, 0, -1], [0, 0, 1], [2, 0, 1], [2, 2, 1]]);
  for (let i = 1; i < plan.total; i++) expect(plan.data[i * STRIDE + 3]).toBeGreaterThanOrEqual(plan.data[(i - 1) * STRIDE + 3]);
  expect([plan.data[0], plan.data[1], plan.data[2], plan.data[3]]).toEqual([0, 0.5, 0.5, 2]);
  expect([plan.data[2 * STRIDE], plan.data[2 * STRIDE + 1], plan.data[2 * STRIDE + 2], plan.data[2 * STRIDE + 3]]).toEqual([0.5, 0.125, 0.125, 8]);
  expect([plan.ints[6], plan.ints[7], plan.ints[8]]).toEqual([8, 4, 1]);
  expect([plan.bands[0], plan.bands[2], plan.bands[plan.total - 1], Math.max(...plan.bands)]).toEqual([0, 0, 8, 8]);
  expect(plan.at.get(0.5)).toEqual({ num: 1, den: 2, k: 8 });
});

test('a bounded root keeps its hull as the container, three root circles ahead of the grown ones, and no line', () => {
  const plan = study(num, '-1,2,2,3', 32);
  expect([plan.strip, plan.roots, plan.total, plan.k0, plan.frame, plan.hull]).toEqual([false, 3, 38, 3, [-1, -1, 1, 1], { cx: 0, cy: 0, r: 1 }]);
  expect(plan.facts).toMatchObject({ title: '(-1, 2, 2, 3)', circles: 35, ford: null, broken: 0 });
  expect([depth(num, plan, 8), plan.at.size]).toEqual([{ bars: [], nodes: null, bright: null, want: null }, 0]);
  expect(study(num, '-3,4,12,13', 8).span).toBe(0);
});

test('the Ford flags mark the circles resting on the line, the root pair included, and nothing off the strip', () => {
  const plan = study(num, 'strip', 2048);
  const marked = (flags) => flags.reduce((n, v) => n + v, 0);
  expect(marked(plan.fords)).toBe(plan.facts.ford + plan.roots);
  expect([plan.fords[0], plan.fords[1], plan.fords[2], plan.fords[3]]).toEqual([1, 1, 1, 0]);
  expect(marked(study(num, '-1,2,2,3', 32).fords)).toBe(0);
});

test('the stack stands one bar per Farey node at the curvature of the circle resting there, in the order the circles land', () => {
  const stood = depth(num, study(num, 'strip', 2048), 8);
  expect(stood.bars).toHaveLength(23);
  expect(stood.bars.slice(0, 3)).toEqual([{ x: 0, h: 1, k: 2 }, { x: 1, h: 1, k: 2 }, { x: 0.5, h: 0.5, k: 8 }]);
  expect(stood.bars.at(-1)).toMatchObject({ h: 0.125, k: 128 });
  for (let i = 1; i < stood.bars.length; i++) expect(stood.bars[i].k).toBeGreaterThanOrEqual(stood.bars[i - 1].k);
  expect(stood).toMatchObject({ nodes: 21, bright: 36, want: 36 });
});

test('the depth never asks for more than the cap carries, and the crate agrees it is the deepest the cap covers', () => {
  expect([stack(8), stack(16), stack(32), stack(2048), stack(8192)]).toEqual([2, 2, 4, 32, 64]);
  for (const cap of CAPS) {
    const packing = a.grow('strip', cap);
    expect(a.shadow(packing, stack(cap)).covered).toBe(true);
    if (stack(cap) < ORDER_CAP) expect(a.shadow(packing, stack(cap) + 1).covered).toBe(false);
  }
  const order = (cap) => tidy(SPEC, { ...defaults(SPEC), cap, order: 64 }).order;
  expect([order(32), order(2048), order(8192)]).toEqual([4, 32, 64]);
});

test('the stack is read against the circles below its reach alone, and the crate returns the shadow of the whole packing', () => {
  const seen = [];
  const spied = { ...num, apollonian: { ...a, shadow: (packing, order) => (seen.push([packing, a.shadow(packing, order)]), seen.at(-1)[1]) } };
  for (const cap of [2048, 8192]) {
    const plan = study(num, 'strip', cap);
    plan.depths.clear();
    for (const order of [1, 2, 8, 16, stack(cap)]) {
      seen.length = 0;
      depth(spied, plan, order);
      const [sent, shadow] = seen[0];
      expect(sent.circles).toEqual(plan.packing.circles.filter((c) => c.k <= 2 * order * order));
      expect(shadow).toEqual(a.shadow(plan.packing, order));
    }
    seen.length = 0;
    plan.depths.clear();
    depth(spied, plan, 2);
    expect(seen[0][0].circles.length).toBeLessThan(plan.packing.circles.length / 8);
  }
});

test('the clock runs the span once, holds, then starts over, and a still scene is at the end', () => {
  expect([clock(0, 8000, HOLD, false), clock(4000, 8000, HOLD, false), clock(8000, 8000, HOLD, false)]).toEqual([{ loop: 0, share: 0 }, { loop: 0, share: 0.5 }, { loop: 0, share: 1 }]);
  expect([clock(8000 + HOLD / 2, 8000, HOLD, false), clock(8000 + HOLD, 8000, HOLD, false)]).toEqual([{ loop: 0, share: 1 }, { loop: 1, share: 0 }]);
  expect([clock(0, 8000, HOLD, true), clock(100, 0, HOLD, false)]).toEqual([{ loop: 0, share: 1 }, { loop: 0, share: 1 }]);
  expect([clock(8000 + HOLD, 8000, HOLD, false, true), clock(3 * (8000 + HOLD), 8000, HOLD, false, true), clock(4000, 8000, HOLD, false, true)]).toEqual([{ loop: 0, share: 1 }, { loop: 0, share: 1 }, { loop: 0, share: 0.5 }]);
});

test('the reach climbs an octave a second from the first grown curvature to the cap', () => {
  expect([reach(8, 2048, 0), reach(8, 2048, 0.5), reach(8, 2048, 1), reach(13, 8, 1)]).toEqual([8, 128, 2048, 8]);
});

test('count finds the index past the last circle at or under a curvature, searching after the roots', () => {
  const data = new Float64Array(7 * STRIDE);
  [2, 2, 3, 3, 8, 8, 18].forEach((k, i) => (data[i * STRIDE + 3] = k));
  expect([count(data, 2, 7, 3), count(data, 2, 7, 8), count(data, 2, 7, 1), count(data, 2, 7, 100)]).toEqual([4, 6, 2, 7]);
});

test('the layout centres a bounded frame and tiles the strip across the width above its band', () => {
  expect(layout([-1, -1, 1, 1], false, 800, 600, 24, 0)).toEqual({ k: 276, ox: 400, oy: 300, periods: [0, 0], line: 576 });
  expect(layout([0, 0, 1, 1], true, 3000, 300, 24, 0)).toEqual({ k: 252, ox: 1374, oy: 276, periods: [-6, 6], line: 276 });
});

test('the strip is never zoomed past three periods across, whatever the height allows', () => {
  expect(ACROSS).toBe(3);
  expect(layout([0, 0, 1, 1], true, 900, 600, 24, 100)).toEqual({ k: 300, ox: 300, oy: 400, periods: [-1, 1], line: 400 });
  const tall = layout([0, 0, 1, 1], true, 300, 900, 24, 0);
  expect([tall.k, tall.periods, tall.oy]).toEqual([100, [-1, 1], 500]);
});

test('a tap lands on the one disc holding its point, none in a gap and none on a disc not yet drawn', () => {
  const data = Float64Array.from([0, 0, 1, 1, 3, 0, 0.5, 2]);
  expect([hit(data, 2, 0.5, 0.5), hit(data, 2, 3.2, 0), hit(data, 2, 2, 2), hit(data, 1, 3.2, 0)]).toEqual([0, 1, -1, -1]);
  const plan = study(num, 'strip', 64);
  expect(hit(plan.data, plan.total, 0.5, 0.125)).toBe(2);
  expect(hit(plan.data, plan.total, 0.5, 1.5)).toBe(-1);
});

test('a label fits its number inside the circle, by height for a short one and by width for a long one', () => {
  expect([glyph(20, 1), glyph(20, 3), glyph(4, 1)]).toEqual([18, 16, 3.6]);
});

test('the shades and the widths run from pale and thick on the first octave to full and thin on the last', () => {
  expect([shade(0, 9), shade(8, 9), shade(0, 1)]).toEqual([PALE, 1, 1]);
  expect([thick(0, 9), thick(0, 1)]).toEqual([1.4, 1.4]);
  expect(thick(8, 9)).toBeCloseTo(0.5);
});

test('a read says the triple, the Ford fraction when there is one, and the line otherwise', () => {
  expect(said({ k: 8, x: 4, y: 1, line: true, ford: true, num: 1, den: 2 })).toBe('(8, 4, 1) Ford at 1/2');
  expect(said({ k: 2, x: 0, y: 1, line: true, ford: true })).toBe('(2, 0, 1) Ford');
  expect(said({ k: 3, x: 0, y: -2, line: false, ford: false })).toBe('(3, 0, -2)');
  expect(said({ k: 5, x: 1, y: 1, line: true, ford: false })).toBe('(5, 1, 1) on the line');
});
