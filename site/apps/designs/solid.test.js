import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { NY, PHI, PX, PZ, camera, exposed, extent, order, solid, walk } from './solid.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const sponge = (level) => {
  const cell = math.three.create(23, 3, level, 2);
  return { cell, shape: solid(math.three.quads(cell), cell.shape[0]) };
};

test('the solid keeps one mask per cell and as many exposed faces as the census surface', () => {
  const { cell, shape } = sponge(2);
  expect(shape.n).toBe(9);
  expect(shape.cells.length).toBeLessThanOrEqual(math.three.fills(cell));
  expect(exposed(shape)).toBe(Number(math.three.census(cell).surface));
  const one = solid(math.three.quads(math.three.ones(3, 1)), 3);
  expect([one.cells.length, exposed(one)]).toEqual([26, 54]);
});

test('the camera sees the top and the two sides that face it, shaded by their lean', () => {
  const front = camera(0, PHI, 10);
  expect(front.visible).toBe((1 << PZ) | (1 << NY));
  expect(front.at(1, 0, 0).map((v) => +v.toFixed(6))).toEqual([10, 0]);
  expect(front.at(0, 0, 1).map((v) => +v.toFixed(6))).toEqual([0, +(-10 * Math.cos(PHI)).toFixed(6)]);
  const turned = camera(-Math.PI / 4, PHI, 10);
  expect(turned.visible).toBe((1 << PZ) | (1 << PX) | (1 << NY));
  expect(turned.tone[PZ]).toBe(1);
  expect(turned.tone[PX]).toBeGreaterThan(turned.tone[NY]);
  expect(extent(9, PHI)[0]).toBeCloseTo(9 * Math.SQRT2, 6);
});

test('the walk paints far cells before near ones and only the faces the camera sees', () => {
  const { shape } = sponge(2);
  const cam = camera(0.3, PHI, 10);
  const sorted = order(shape, cam);
  const depth = (i) => {
    const key = shape.cells[i];
    const n = shape.n;
    return cam.depth(Math.floor(key / (n * n)) - n / 2, (Math.floor(key / n) % n) - n / 2, (key % n) - n / 2);
  };
  for (let i = 1; i < sorted.length; i++) expect(depth(sorted[i - 1])).toBeGreaterThanOrEqual(depth(sorted[i]));
  let want = 0;
  for (const mask of shape.masks) for (let face = 0; face < 6; face++) if (mask & cam.visible & (1 << face)) want++;
  const seen = [];
  expect(walk(shape, cam, 0, 0, (face, ...pts) => seen.push([face, pts.length]))).toBe(want);
  expect(seen.length).toBe(want);
  expect(seen.every(([face, n]) => (cam.visible & (1 << face)) !== 0 && n === 8)).toBe(true);
});

test('a quarter turn of the sponge shows the same number of faces', () => {
  const { shape } = sponge(1);
  const count = (theta) => walk(shape, camera(theta, PHI, 10), 0, 0, () => {});
  expect(count(0.4)).toBe(count(0.4 + Math.PI / 2));
});
