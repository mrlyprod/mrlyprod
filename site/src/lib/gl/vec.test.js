import { expect, test } from 'bun:test';
import { basis, clamp01, cross, dot, len, rot, smooth } from './vec.js';

test('basis is a right-handed orthonormal frame and a quarter turn about forward takes right to up', () => {
  const [r, u, f] = basis([0.3, -0.2, 0.9], [0, 1, 0]);
  expect([len(r), len(u), len(f)].map((v) => v.toFixed(9))).toEqual(['1.000000000', '1.000000000', '1.000000000']);
  expect([dot(r, u), dot(u, f), dot(f, r)].map((v) => Math.abs(v) < 1e-12)).toEqual([true, true, true]);
  cross(r, u).forEach((v, i) => expect(v).toBeCloseTo(f[i], 12));
  rot(r, f, Math.PI / 2).forEach((v, i) => expect(v).toBeCloseTo(u[i], 12));
});

test('clamp01 holds a value in [0, 1] and smooth is the smoothstep of the clamped value', () => {
  expect([clamp01(-1), clamp01(0.25), clamp01(3)]).toEqual([0, 0.25, 1]);
  expect([smooth(-1), smooth(0.5), smooth(2), smooth(0.25)]).toEqual([0, 0.5, 1, 0.15625]);
});
