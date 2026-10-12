import { expect, test } from 'bun:test';
import { art, SHAPES, tiles } from './engine.js';

const inside = ([x, y], pts) => {
  let hit = false;
  for (let i = 0, j = pts.length - 1; i < pts.length; j = i++) {
    const [xi, yi] = pts[i];
    const [xj, yj] = pts[j];
    if (yi > y !== yj > y && x < ((xj - xi) * (y - yi)) / (yj - yi) + xi) hit = !hit;
  }
  return hit;
};

test('every tiling covers the screen once, with no gap and no overlap', () => {
  const covers = {};
  for (const shape of SHAPES) {
    const { cells } = tiles(shape, 301, 203, 37);
    const counts = new Set();
    for (let x = 0.37; x < 301; x += 3.71) for (let y = 0.53; y < 203; y += 3.13) counts.add(cells.filter((cell) => inside([x, y], cell.pts)).length);
    covers[shape] = [...counts];
  }
  expect(covers).toEqual(Object.fromEntries(SHAPES.map((shape) => [shape, [1]])));
});

test('a seeded mosaic never repaints in the palette it had', () => {
  const runs = [1, 2, 3, 4, 5].map((seed) => Array.from({ length: 24 }, (_, k) => art(seed, k, '').hues));
  expect(runs.every((run) => run.every((hues, k) => k === 0 || hues !== run[k - 1]))).toBe(true);
});
