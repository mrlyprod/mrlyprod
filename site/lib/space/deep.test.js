import { expect, test } from 'bun:test';
import { rng } from '../scene.js';
import { deep, fixed, float, nucleus, orbit, period, score, size } from './deep.js';
import { gl } from './fake.js';

const NUCLEUS = -1.7548776662466927;

const VIEW = { rand: rng(7), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 1280, h: 720, dpr: 1, t: 0 };

const reference = (re, im = 0) => {
  const one = orbit({ c: [fixed(re), fixed(im)] });
  one.step();
  return one;
};

const probe = (paint) => {
  const out = new Uint8Array(64 * 64 * 4);
  for (let y = 0; y < 64; y++) {
    for (let x = 0; x < 64; x++) {
      const [inside, count, per] = paint(x, y);
      const q = inside ? 0 : Math.round(count * 8);
      out.set([q >> 8, q & 255, (inside ? 128 : 0) + (per >> 8), per & 255], 4 * (y * 64 + x));
    }
  }
  return out;
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

test('score rejects an all-interior and a flat probe and ranks a boundary tile first', () => {
  expect(score(probe(() => [true, 0, 3]), 64, 64)).toEqual([]);
  expect(score(probe(() => [false, 10, 1]), 64, 64)).toEqual([]);
  const blob = probe((x, y) => {
    const r = Math.hypot(x - 43.5, y - 19.5);
    return r < 2 ? [true, 0, 5] : [false, 10 + 40 / (1 + (r - 2) ** 2), 5];
  });
  const [first] = score(blob, 64, 64);
  expect([first.tx, first.ty, first.kind]).toEqual([5, 2, 'island']);
});

test('the home orbit goes to the GPU whole, as an RG32F texture 4096 wide', () => {
  const fake = gl();
  const sent = [];
  fake.texImage2D = (...args) => sent.push(args.map((one) => (one instanceof Float32Array ? one.slice() : one)));
  deep(fake, VIEW, { kind: 'set', seed: 3 });
  const [, , inner, w, , , format, type, data] = sent.find((one) => one[2] === fake.RG32F);
  expect([inner, w, format, type, data[2], data[3]]).toEqual([fake.RG32F, 4096, fake.RG, fake.FLOAT, -0.75, 0]);
});
