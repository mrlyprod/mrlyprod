import { expect, test } from 'bun:test';
import * as num from '@mrlyprod/mrly/num';
import { defaults, tidy } from '../../lib/app/spec.js';
import { rng } from '../../lib/rand.js';
import { CAPS, HOLD, OCTAVE, ORDER_CAP, PALE, ROLLED, ROOTS, SIX, STRIDE, clock, count, glyph, hit, layout, nib, reach, roll, said, shade, shadow, stack, step, study } from './engine.js';
import { SPEC } from './scene.js';

const a = num.apollonian;

test('the roots, the top cap, the depth cap and the stride are the crate\'s own', () => {
  expect(ROOTS).toEqual(a.ROOTS());
  expect(CAPS.at(-1)).toBe(Number(a.CURVATURE_CAP()));
  expect(ORDER_CAP).toBe(a.ORDER_CAP());
  expect(STRIDE).toBe(a.STRIDE());
  expect(study(num, 'strip', 2048).data.length).toBe(2450 * STRIDE);
  expect(CAPS.every((cap, i) => cap >= 2 && (i === 0 || cap === 2 * CAPS[i - 1]))).toBe(true);
});

test('a roll is the same for one seed, keeps every key the spec allows and reaches every root', () => {
  expect(roll(rng(7))).toEqual(roll(rng(7)));
  const rolls = Array.from({ length: 60 }, (_, i) => roll(rng(i)));
  for (const one of rolls) {
    expect(tidy(SPEC, { ...defaults(SPEC), ...one })).toMatchObject(one);
    expect(ROLLED).toContain(one.cap);
  }
  expect(new Set(rolls.map((one) => one.root)).size).toBe(ROOTS.length);
});

test('the cap steps through the powers of two and stays at the ends', () => {
  expect([step(2048, 1), step(2048, -1), step(8192, 1), step(8, -1), step(999, 1)]).toEqual([4096, 1024, 8192, 8, 4096]);
});

test('the strip to 2048 holds 2448 circles after its two root circles, every one of its 323 line circles a Ford circle', () => {
  const plan = study(num, 'strip', 2048);
  expect(plan.facts).toMatchObject({ name: 'strip', title: 'strip (0, 0, 2, 2)', cap: 2048, circles: 2448, drawn: 2450, quads: 2449, broken: 0, strayed: 0, line: 323, ford: 323 });
  expect([plan.strip, plan.roots, plan.total, plan.k0, plan.octaves, plan.span, plan.frame, plan.hull]).toEqual([true, 2, 2450, 8, 9, 8 * OCTAVE, [0, 0, 1, 1], null]);
  expect(plan.quad).toEqual([[0, 0, -1], [0, 0, 1], [2, 0, 1], [2, 2, 1]]);
  expect([plan.ints[6], plan.ints[7], plan.ints[8]]).toEqual([8, 4, 1]);
  expect([plan.bands[0], plan.bands[2], plan.bands[plan.total - 1], Math.max(...plan.bands)]).toEqual([0, 0, 8, 8]);
  expect(plan.at.get(0.5)).toEqual({ num: 1, den: 2, k: 8 });
});

test('a bounded root keeps its hull as the container, three root circles ahead of the grown ones, and no line', () => {
  const plan = study(num, '-1,2,2,3', 32);
  expect([plan.strip, plan.roots, plan.total, plan.k0, plan.frame, plan.hull]).toEqual([false, 3, 38, 3, [-1, -1, 1, 1], { cx: 0, cy: 0, r: 1 }]);
  expect(plan.facts).toMatchObject({ title: '(-1, 2, 2, 3)', circles: 35, drawn: 39, broken: 0, line: null, ford: null });
  expect([plan.facts.shadow, plan.at.size]).toEqual([null, 0]);
  expect(study(num, '-3,4,12,13', 8).span).toBe(0);
});

test('the Ford flags mark the circles resting on the line, the root pair included', () => {
  const plan = study(num, 'strip', 2048);
  const marked = (flags) => flags.reduce((n, v) => n + v, 0);
  expect(marked(plan.fords)).toBe(plan.facts.ford + plan.roots);
  expect([plan.fords[0], plan.fords[1], plan.fords[2], plan.fords[3]]).toEqual([1, 1, 1, 0]);
  expect(marked(study(num, '-1,2,2,3', 32).fords)).toBe(0);
});

test('the hue of a circle cycles through six with each octave of its curvature', () => {
  const plan = study(num, 'strip', 2048);
  expect(SIX).toHaveLength(6);
  for (let i = 0; i < plan.total; i += 97) expect(plan.hues[i]).toBe(Math.floor(Math.log2(plan.data[i * STRIDE + 3])) % 6);
  expect([plan.hues[0], plan.hues[2]]).toEqual([1, 3]);
});

test('the strip reads the Farey stack at the deepest depth its cap covers, every node touched and none missed', () => {
  expect(study(num, 'strip', 2048).facts.shadow).toEqual({ order: 32, reach: 2048, covered: true, nodes: 323, touched: 323, missed: 0, offford: 0, bright: 528, want: 528 });
  expect(study(num, 'strip', 8).facts.shadow).toMatchObject({ order: 2, covered: true, missed: 0 });
});

test('the deepest stack a cap covers is the crate\'s own', () => {
  expect([stack(8), stack(16), stack(32), stack(2048), stack(8192)]).toEqual([2, 2, 4, 32, 64]);
  for (const cap of CAPS) {
    const packing = a.grow('strip', cap);
    expect(a.shadow(packing, stack(cap)).covered).toBe(true);
    if (stack(cap) < ORDER_CAP) expect(a.shadow(packing, stack(cap) + 1).covered).toBe(false);
  }
});

test('the stack is read against the circles below its reach alone, and the crate returns the shadow of the whole packing', () => {
  const seen = [];
  const spied = { ...num, apollonian: { ...a, shadow: (packing, order) => (seen.push([packing, a.shadow(packing, order)]), seen.at(-1)[1]) } };
  for (const cap of [2048, 8192]) {
    const packing = a.grow('strip', cap);
    const data = a.flat('strip', cap);
    for (const order of [1, 2, 8, 16, stack(cap)]) {
      seen.length = 0;
      shadow(spied, packing, data, 2, order);
      const [sent, read] = seen[0];
      expect(sent.circles).toEqual(packing.circles.filter((c) => c.k <= 2 * order * order));
      expect(read).toEqual(a.shadow(packing, order));
    }
  }
});

test('a root and cap studied once are kept, so flipping between caps grows nothing again', () => {
  const calls = [];
  const counted = { ...num, apollonian: { ...a, grow: (...args) => (calls.push(args), a.grow(...args)) } };
  const one = study(counted, '-3,4,12,13', 16);
  const two = study(counted, '-3,4,12,13', 32);
  expect([study(counted, '-3,4,12,13', 16), study(counted, '-3,4,12,13', 32)]).toEqual([one, two]);
  expect(calls).toEqual([['-3,4,12,13', 16], ['-3,4,12,13', 32]]);
});

test('the clock runs the span once, holds, then starts over, and a still scene is at the end', () => {
  expect([clock(0, 8000, HOLD, false), clock(4000, 8000, HOLD, false), clock(8000, 8000, HOLD, false)]).toEqual([{ loop: 0, share: 0 }, { loop: 0, share: 0.5 }, { loop: 0, share: 1 }]);
  expect([clock(8000 + HOLD / 2, 8000, HOLD, false), clock(8000 + HOLD, 8000, HOLD, false)]).toEqual([{ loop: 0, share: 1 }, { loop: 1, share: 0 }]);
  expect([clock(0, 8000, HOLD, true), clock(100, 0, HOLD, false)]).toEqual([{ loop: 0, share: 1 }, { loop: 0, share: 1 }]);
});

test('the reach climbs an octave a second from the first grown curvature to the cap', () => {
  expect([reach(8, 2048, 0), reach(8, 2048, 0.5), reach(8, 2048, 1), reach(13, 8, 1)]).toEqual([8, 128, 2048, 8]);
});

test('count finds the index past the last circle at or under a curvature, searching after the roots', () => {
  const data = new Float64Array(7 * STRIDE);
  [2, 2, 3, 3, 8, 8, 18].forEach((k, i) => (data[i * STRIDE + 3] = k));
  expect([count(data, 2, 7, 3), count(data, 2, 7, 8), count(data, 2, 7, 1), count(data, 2, 7, 100)]).toEqual([4, 6, 2, 7]);
});

test('the layout fits the frame whole and centres it on the stage at any aspect, one period of the strip as a square', () => {
  expect(layout([-1, -1, 1, 1], 800, 600, 24)).toEqual({ k: 276, ox: 400, oy: 300 });
  expect(layout([0, 0, 1, 1], 900, 600, 24)).toEqual({ k: 552, ox: 174, oy: 576 });
  expect(layout([0, 0, 1, 1], 300, 900, 24)).toEqual({ k: 252, ox: 24, oy: 576 });
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

test('shades run pale to full by octave, and a pen runs from 0.4 px on dust to 1.3 px on a big circle', () => {
  expect([shade(0, 9), shade(8, 9), shade(0, 1)]).toEqual([PALE, 1, 1]);
  expect([nib(1), nib(6), nib(12), nib(100)]).toEqual([0.4, 0.5, 1, 1.3]);
});

test('a read says the triple, the Ford fraction when there is one, the line otherwise', () => {
  expect(said({ k: 8, x: 4, y: 1, line: true, ford: true, num: 1, den: 2 })).toBe('(8, 4, 1) Ford at 1/2');
  expect(said({ k: 2, x: 0, y: 1, line: true, ford: true })).toBe('(2, 0, 1) Ford');
  expect(said({ k: 3, x: 0, y: -2, line: false, ford: false })).toBe('(3, 0, -2)');
  expect(said({ k: 5, x: 1, y: 1, line: true, ford: false })).toBe('(5, 1, 1) on the line');
});
