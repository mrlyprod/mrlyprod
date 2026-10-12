import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { CUT, IN, OUT, REFINE, SHAPES, census, circle, counted, grid, keep, layers, outline, room, shape } from './engine.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const carpet = grid(math, '7', 3, 3, 2);

const fills = math.two.fills({ shape: carpet.shape, types: carpet.data });

const sum = (list) => list.reduce((all, [, , len]) => all + len, 0);

const count = (data) => data.reduce((all, on) => all + (on ? 1 : 0), 0);

const half = (anti = false) => shape(math, 'ball', 60, anti);

test('the shapes are the named shapes of the plane', () => {
  expect(SHAPES.map(([name]) => name)).toEqual(math.shape.shapes(2));
});

test('the census splits the filled cells into in, cut and out, and anti swaps in for out', () => {
  const plain = census(math, carpet, half());
  const anti = census(math, carpet, half(true));
  expect(plain.filled[IN] + plain.filled[CUT] + plain.filled[OUT]).toBe(fills);
  expect([anti.filled[IN], anti.filled[CUT], anti.filled[OUT]]).toEqual([plain.filled[OUT], plain.filled[CUT], plain.filled[IN]]);
  expect(plain.after).toBe(Number(math.two.perimeter({ shape: carpet.shape, types: math.shape.crop(carpet, half(), true).data })));
});

test('inside keeps the in cells, touching adds the cut ones, and a refined policy rebuilds the rim on a finer lattice', () => {
  const { filled } = census(math, carpet, half());
  expect(count(keep(math, carpet, half(), 'inside', 3).grid.data)).toBe(filled[IN]);
  expect(count(keep(math, carpet, half(), 'touching', 3).grid.data)).toBe(filled[IN] + filled[CUT]);
  const one = keep(math, carpet, half(), 'refined1', 3);
  expect([one.grid.shape, one.extra, one.over]).toEqual([[81, 81], 1, false]);
  expect(count(one.grid.data)).toBeGreaterThan(9 * filled[IN]);
  expect(count(one.grid.data)).toBeLessThan(9 * (filled[IN] + filled[CUT]));
  const big = keep(math, grid(math, '7', 3, 5, 2), half(), 'refined2', 3);
  expect(2187 ** 2).toBeGreaterThan(REFINE);
  expect([big.extra, big.over]).toEqual([0, true]);
});

test('the layers light the kept in cells, ink the kept cut ones and fade the filled cells the crop removed', () => {
  const made = half();
  const regions = math.shape.regions(made, carpet.shape).data;
  const { filled } = census(math, carpet, made);
  const look = layers(carpet, regions, keep(math, carpet, made, 'touching', 3), false);
  expect([sum(look.lit), sum(look.cut), sum(look.faint)]).toEqual([filled[IN], filled[CUT], filled[OUT]]);
  const fine = layers(carpet, regions, keep(math, carpet, made, 'refined1', 3), false);
  expect([fine.coarse, fine.side, sum(fine.faint), sum(fine.cut)]).toEqual([27, 81, fills, 0]);
});

test('the outline of a polygon is its walls met in turn, and of the ball its centre and radius', () => {
  const sorted = (points) => points.map((p) => p.map((v) => Math.round(v * 1e9) / 1e9 + 0)).sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  expect(outline(shape(math, 'ball', 30, false))).toEqual({ circle: [0.5, 0.5, 0.25] });
  expect(sorted(outline(shape(math, 'box', 60, false)).points)).toEqual([[0, 0], [0, 1], [1, 0], [1, 1]]);
  expect(sorted(outline(shape(math, 'triangle', 60, false)).points)).toEqual([[0, 1], [0.5, 0], [1, 1]]);
  expect(outline(shape(math, 'octagon', 60, false)).points).toHaveLength(8);
});

test('the circle count runs to the side less one from the corner and to half of that from the centre', () => {
  const corner = circle(math, '7', 3, 4, 2, 'corner');
  const centre = circle(math, '7', 3, 4, 2, 'centre');
  expect([corner.side, corner.seen.length, centre.seen.length]).toEqual([81, 81, 41]);
  expect([corner.mass, corner.d]).toEqual([8, math.counts.dimension('7', 3, 2, 2)]);
  expect(corner.seen[80]).toBeGreaterThanOrEqual(corner.inside[80]);
});

test('the count stays in its budget', () => {
  expect([counted(math, 2), counted(math, 3)]).toEqual([9, 6]);
  expect(room(math, 3, 9, 'corner')).toEqual({ deep: 6, side: 729, most: 728 });
  expect(room(math, 3, 2, 'centre')).toEqual({ deep: 2, side: 9, most: 4 });
});
