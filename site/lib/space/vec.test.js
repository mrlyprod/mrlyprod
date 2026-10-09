import { expect, test } from 'bun:test';
import { basis, cross, dot, len, rot } from './vec.js';

test('basis is a right-handed orthonormal frame and a quarter turn about forward takes right to up', () => {
  const [r, u, f] = basis([0.3, -0.2, 0.9], [0, 1, 0]);
  expect([len(r), len(u), len(f)].map((v) => v.toFixed(9))).toEqual(['1.000000000', '1.000000000', '1.000000000']);
  expect([dot(r, u), dot(u, f), dot(f, r)].map((v) => Math.abs(v) < 1e-12)).toEqual([true, true, true]);
  cross(r, u).forEach((v, i) => expect(v).toBeCloseTo(f[i], 12));
  rot(r, f, Math.PI / 2).forEach((v, i) => expect(v).toBeCloseTo(u[i], 12));
});
