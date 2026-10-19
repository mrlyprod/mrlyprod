import { expect, test } from 'bun:test';
import { fixed, float, nucleus, orbit, period, size, slice } from './orbit.js';

const NUCLEUS = -1.7548776662466927;

const reference = (re, im = 0) => {
  const one = orbit({ c: [fixed(re), fixed(im)] });
  one.step();
  return one;
};

test('fixed and float round-trip through BigInt fixed point', () => {
  const values = [0.3, NUCLEUS, 2.75, 1.234567e-27, -3.5e-29];
  expect(values.map((v) => float(fixed(v)))).toEqual(values);
  expect(fixed(1)).toBe(1n << 164n);
});

test('the BigInt orbit at -0.75 + 0.1i equals float64 until it escapes', () => {
  const one = orbit({ c: [fixed(-0.75), fixed(0.1)] });
  one.step(200);
  let [x, y] = [0, 0];
  for (let n = 1; n < one.length; n++) {
    [x, y] = [x * x - y * y - 0.75, 2 * x * y + 0.1];
    expect(Math.hypot(one.wide[2 * n] - x, one.wide[2 * n + 1] - y)).toBeLessThan(1e-9 * Math.max(1, Math.hypot(x, y)));
  }
  expect([one.done, one.length]).toEqual([true, 37]);
});

test('period 3 at the nucleus, and Newton finds it from 1e-6 away', () => {
  const one = reference(NUCLEUS);
  expect(period(one, [0, 0])).toBe(3);
  for (const from of [[1e-6, 0], [0, 1e-6], [-7e-7, 7e-7]]) expect(Math.hypot(...nucleus(one, from, 3))).toBeLessThan(1e-14);
});

test('Newton rebases from far: 24 angles at 1e-2 all land on the period 3 nucleus', () => {
  const one = reference(NUCLEUS);
  for (let k = 0; k < 24; k++) {
    const a = (k * Math.PI) / 12;
    expect(Math.hypot(...nucleus(one, [1e-2 * Math.cos(a), 1e-2 * Math.sin(a)], 3))).toBeLessThan(1e-14);
  }
});

test('size is positive and small at the period 3 nucleus and 1 for the main set', () => {
  const [re, im] = size(reference(NUCLEUS), [0, 0], 3);
  expect([re > 0.018 && re < 0.02, Math.abs(im) < 1e-12]).toEqual([true, true]);
  expect(size(reference(0), [0, 0], 1)).toEqual([1, -0]);
});

test('an orbit slice always steps once and goes on only while the frame has time to spare', () => {
  const steps = [];
  const o = { done: false, step: (n) => steps.push(n) };
  slice(o, () => false);
  const left = [true, true, false];
  slice(o, () => left.shift());
  expect(steps).toEqual([32, 32, 32, 32]);
});
