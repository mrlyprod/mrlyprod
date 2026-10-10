import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { frame } from '../../../pkgs/mrlyjs/view/index.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { fit, paint } from './cuts.js';
import { study } from './engine.js';
import { SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const INK = { fill: [0, 140, 255, 255], faint: [0, 140, 255, 18] };

const pen = (log) => new Proxy({}, { get: (_, key) => (...args) => log.push([key, ...args]) });

const cut = (view, over) => study(math, { ...defaults(SPEC), seed: 1, base: 2, code: '23', view, ...over }, rng(1)).cut;

const inside = (x0, y0, x1, y1, w, h) => x0 >= w.x - 1e-6 && x1 <= w.x + w.w + 1e-6 && y0 >= w.y - 1e-6 && y1 <= w.y + w.h + 1e-6;

test('the slice paints one rect per cell on the centred square, fills in the ink and voids faint', () => {
  const log = [];
  const box = frame(0, 0, 300, 200);
  paint(pen(log), box, cut('slice'), INK);
  const rects = log.filter(([key]) => key === 'rect');
  expect([rects.length, rects.filter((one) => one[5] === INK.fill).length, rects.filter((one) => one[5] === INK.faint).length]).toEqual([729, 64, 665]);
  expect(rects.every(([, x, y, w, h]) => inside(x, y, x + w, y + h, box.square()))).toBe(true);
});

test('the hexagon paints one triangle per fill and void and none for the backdrop', () => {
  const log = [];
  paint(pen(log), frame(0, 0, 300, 200), cut('hex'), INK);
  const tris = log.filter(([key]) => key === 'triangle');
  expect([tris.length, tris.filter((one) => one[4] === INK.fill).length]).toEqual([4374, 2250]);
});

test('the diagonal paints one disc per cell, fitted inside the frame, and nothing for an empty design', () => {
  const log = [];
  const box = frame(10, 20, 300, 200);
  paint(pen(log), box, cut('diagonal'), INK);
  const discs = log.filter(([key]) => key === 'disc');
  expect(discs.length).toBe(306);
  expect(discs.every(([, x, y, r]) => inside(x - r, y - r, x + r, y + r, box))).toBe(true);
  const none = [];
  paint(pen(none), box, cut('diagonal', { code: '0' }), INK);
  paint(pen(none), box, null, INK);
  expect(none).toEqual([]);
});

test('fit centres a point cloud in the frame keeping its proportions', () => {
  const { scale, place } = fit([[0, 0], [4, 2]], frame(0, 0, 100, 100));
  expect(scale).toBe(25);
  expect([place([0, 0]), place([4, 2])]).toEqual([[0, 25], [100, 75]]);
  expect(fit([[1, 1]], frame(0, 0, 10, 10), 1).scale).toBe(5);
});
