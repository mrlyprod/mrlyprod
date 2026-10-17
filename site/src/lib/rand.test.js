import { expect, test } from 'bun:test';
import { rng } from './rand.js';

const take = (rand, n) => Array.from({ length: n }, () => rand());

test('a seeded xorshift repeats, differs by seed, stays in [0, 1) and spreads small seeds', () => {
  expect(take(rng(7), 5)).toEqual(take(rng(7), 5));
  expect(take(rng(7), 5)).not.toEqual(take(rng(8), 5));
  expect(take(rng(7), 64).every((n) => n >= 0 && n < 1)).toBe(true);
  expect(new Set(Array.from({ length: 100 }, (_, i) => Math.floor(rng(i + 1)() * 4))).size).toBe(4);
});
