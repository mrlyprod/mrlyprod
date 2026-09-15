import { expect, test } from 'bun:test';
import { paints, rng } from './frame.js';
import { fell } from './matrix.js';
import { bounce } from './sleep.js';
import { wayfind } from './fractal.js';
import { SAVERS } from './index.js';
import { DESIGNS, kron, seed, tile } from './tiles.js';
import { light } from '../palette.js';

const take = (rand, n) => Array.from({ length: n }, () => rand());

test('a seeded xorshift repeats and an unseeded one does not', () => {
  expect(take(rng(7), 5)).toEqual(take(rng(7), 5));
  expect(take(rng(7), 5)).not.toEqual(take(rng(8), 5));
  expect(take(rng(7), 64).every((n) => n >= 0 && n < 1)).toBe(true);
});

test('the wayfinder keeps the highest escape count that still escapes', () => {
  const feed = [0.95, 0.5, 0.8, 0.5, 0.2, 0.5];
  let i = 0;
  const rand = () => feed[i++ % feed.length];
  const caps = new Set();
  const escape = (x, y, max) => (caps.add(max), x > 0.9 ? max : Math.floor(x * 100));
  const spot = wayfind(escape, { xMin: 0, xMax: 1, yMin: 0, yMax: 1 }, rand);
  expect([...caps]).toEqual([150]);
  expect(spot.x).toBeCloseTo(0.8, 10);
  expect(spot.y).toBeCloseTo(0.5, 10);
});

test('a matrix column resets past the floor on a rare roll', () => {
  expect(fell(101, 100, 0.98)).toBe(true);
  expect(fell(101, 100, 0.97)).toBe(false);
  expect(fell(99, 100, 0.99)).toBe(false);
});

test('the sleeper clamps and flips at both walls', () => {
  expect(bounce(-2, -4, 100)).toEqual({ p: 0, v: 4, hit: true });
  expect(bounce(104, 4, 100)).toEqual({ p: 100, v: -4, hit: true });
  expect(bounce(50, 4, 100)).toEqual({ p: 50, v: 4, hit: false });
});

test('the chrome whitelist is the saver list', async () => {
  const src = await Bun.file(new URL('../chrome.js', import.meta.url)).text();
  const listed = /const SCREENS = \[([^\]]*)\]/.exec(src)?.[1] ?? '';
  expect(listed.split(',').map((one) => one.trim().slice(1, -1))).toEqual(SAVERS);
});

const lit = (t) => t.cells.reduce((a, b) => a + b, 0);

test('every design fills its known cells at n=3 and squares them one level up', () => {
  const counts = { carpet: 8, net: 5, htree: 6, vtree: 6, void: 5 };
  for (const design of DESIGNS) {
    const one = tile(design, 3, 1);
    const two = tile(design, 3, 2);
    expect([design, one.size, lit(one)]).toEqual([design, 3, counts[design]]);
    expect([design, two.size, lit(two)]).toEqual([design, 9, counts[design] ** 2]);
  }
  expect([...tile('carpet', 3, 1).cells]).toEqual([1, 1, 1, 1, 0, 1, 1, 1, 1]);
  expect(kron(seed('void', 5), 2).size).toBe(25);
});

test('a saver reads the tint from --accent and falls back to the palette', () => {
  const canvas = {};
  globalThis.getComputedStyle = () => ({ getPropertyValue: (name) => (name === '--accent' ? ' #00ff00 ' : '') });
  expect(paints(canvas).accent).toBe('#00ff00');
  delete globalThis.getComputedStyle;
  expect(paints(canvas).accent).toBe(light.accent);
});
