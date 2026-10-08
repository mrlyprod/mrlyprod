import { expect, test } from 'bun:test';
import { wayfind } from './fractal.js';

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
