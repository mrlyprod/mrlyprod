import { afterAll, expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { asked, defaults, describe, places, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { BEAT, TOP, colorizer, field, odd, paint, study } from './engine.js';
import { DESIGN, PAGE, SPEC, cellAt, cells, make } from './scene.js';

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (key === 'measureText' ? () => ({ width: 10 }) : (...args) => log.push([key, ...args.map((arg) => (arg && typeof arg === 'object' ? (arg.width ?? 'thing') : arg))])),
      set: (_, key, v) => {
        log.push([key, v]);
        return true;
      },
    },
  );

globalThis.OffscreenCanvas = class {
  constructor(w, h) {
    this.width = w;
    this.height = h;
  }
  getContext() {
    return pen([]);
  }
};

globalThis.ImageData = class {
  constructor(data, width, height) {
    this.data = data;
    this.width = width;
    this.height = height;
  }
};

afterAll(() => {
  delete globalThis.OffscreenCanvas;
  delete globalThis.ImageData;
});

const counted = () => {
  const calls = { stack: 0, render: 0 };
  const moire = { ...math.moire, stack: (...args) => (calls.stack++, math.moire.stack(...args)), render: (...args) => (calls.render++, math.moire.render(...args)) };
  return { calls, math: { ...math, moire } };
};

const start = { ...defaults(SPEC), ...DESIGN, seed: 1 };

const open = (over = {}, hooks = {}) => {
  const log = [];
  const { calls, math: door } = counted();
  const view = { rand: rng(1), look: () => ({ paper: '#ffffff', accent: '#008cff' }), still: false, w: 800, h: 600, dpr: 1, t: 0, wake: () => {} };
  const told = { facts: [], limits: [] };
  const scene = make({ getContext: () => pen(log) }, view, { ...start, ...over, math: door, onFacts: (f) => told.facts.push(f), onLimit: (k) => told.limits.push(k), ...hooks });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  const set = (patch) => scene.set({ ...start, ...over, ...patch });
  return { view, scene, log, at, calls, told, set };
};

const blits = (log) => log.filter(([name]) => name === 'drawImage');

test('the page opens still on the void stacked to the top scale and draws it once, crisp, with its facts', () => {
  const page = open();
  const first = page.at(0);
  expect(blits(first)).toEqual([['drawImage', 512, 118, 18, 564, 564]]);
  expect(first).toContainEqual(['imageSmoothingEnabled', false]);
  expect(blits(page.at(5000))).toEqual([]);
  expect(page.calls.stack).toBe(1);
  expect(page.told.facts.at(-1)).toEqual({ view: 'field', name: 'void', code: '9', base: 2, level: 1, layers: 21, scales: odd(TOP), size: 512 });
});

test('the default runs from the ground to the tint, so a dark page gets a dark field and a light page a paper one', () => {
  const made = field(math, study(math, start, rng(1)), 64);
  const green = (paper) => {
    const tones = paint(math, made, colorizer(start.ramp, start.levels, { paper, accent: '#008cff' }), start.levels, start.invert).colors.filter((_, i) => i % 4 === 1);
    return [Math.min(...tones), Math.max(...tones)];
  };
  expect([green('#000000'), green('#ffffff')]).toEqual([[0, 140], [140, 255]]);
  made.field.free();
});

test('the default lights every layer on both diagonals, so the X is the top of the field', () => {
  const made = field(math, study(math, start, rng(1)), 64);
  const cross = Array.from({ length: 64 }, (_, k) => [made.data[k * 65], made.data[k * 63 + 63]]).flat();
  expect([made.field.max(), Math.min(...cross)]).toEqual([21, 21]);
  made.field.free();
});

test('play climbs two scales a beat from the limit on screen, tells the page each step, and stops where it stands', () => {
  const page = open({ limit: 9 });
  page.at(0);
  expect(page.scene.toggle()).toBe(true);
  page.at(0);
  page.at(BEAT - 1);
  page.at(BEAT);
  expect(page.told.limits).toEqual([11, 13]);
  expect([page.told.facts.at(-1).layers, page.scene.playing()]).toEqual([7, true]);
  page.set({ limit: 13 });
  page.at(BEAT * 2);
  expect(page.told.limits).toEqual([11, 13, 15]);
  expect(page.scene.toggle()).toBe(false);
  page.at(BEAT * 9);
  expect([page.told.limits.length, page.scene.playing()]).toEqual([3, false]);
});

test('a take started while stopped climbs', () => {
  const page = open({ limit: 9 });
  page.view.fixed = true;
  page.at(0);
  page.at(BEAT);
  expect([page.scene.playing(), page.told.limits]).toEqual([true, [11, 13]]);
});

test('a hand on the limit while playing climbs on from there, and the climb wraps at the top', () => {
  const page = open({ limit: 9 });
  page.at(0);
  page.scene.toggle();
  page.at(100);
  page.set({ limit: TOP });
  page.at(200);
  expect(page.told.limits).toEqual([11, 1]);
});

test('a paint key repaints the held field, a plan key restacks it, both keep the scene, and the r-matrix view does neither', () => {
  const page = open();
  page.at(0);
  page.set({ ramp: 'tint', levels: 4 });
  expect(blits(page.at(16))).toHaveLength(1);
  expect([page.calls.stack, page.calls.render]).toEqual([1, 2]);
  page.set({ limit: 21 });
  page.at(32);
  expect([page.calls.stack, page.told.facts.at(-1).layers]).toEqual([2, 11]);
  page.set({ view: 'matrix', limit: 21, combine: 'xor', levels: 8, scale: 15, mate: 9 });
  page.at(48);
  expect([page.calls.stack, page.calls.render, page.told.facts.at(-1).row, page.told.facts.at(-1).mate, page.told.facts.at(-1).r > 0]).toEqual([2, 3, 15, 9, true]);
});

test('the r-matrix view draws the grid and no text, and a tap picks the pair under it while a tap on the field plays', () => {
  const page = open({ view: 'matrix', limit: 15 });
  const log = page.at(0);
  expect(blits(log)).toEqual([]);
  expect(log.filter(([name]) => name === 'fillRect')).toHaveLength(49);
  expect(log.filter(([name]) => name === 'fillText' || name === 'font')).toEqual([]);
  expect(open({ view: 'matrix', limit: 1 }).at(0).filter(([name]) => ['drawImage', 'fillRect', 'fillText'].includes(name))).toEqual([]);
  const grid = cells(page.view, 7);
  const tap = (i, j) => page.scene.touch((grid.x + (j + 0.5) * grid.cell) / page.view.w, (grid.y + (i + 0.5) * grid.cell) / page.view.h, true);
  expect([tap(5, 1), tap(1, 5), tap(2, 2), page.scene.touch(0, 0, true)]).toEqual([{ scale: 13, mate: 5 }, { scale: 13, mate: 5 }, { scale: 7 }, null]);
  expect(cellAt(grid, grid.x - 1, grid.y)).toBe(null);
  const field = open();
  field.at(0);
  expect([field.scene.touch(0.5, 0.5, true), field.scene.playing()]).toEqual([null, true]);
});

test('the page keeps its defaults as they are', () => {
  expect(tidy(SPEC, start)).toEqual(defaults(SPEC));
});

test('each view shows only the knobs that move its picture', () => {
  const shown = (view) => describe(SPEC, { ...start, view, crop: 'ball' }).flatMap((group) => group.rows.map((row) => row.key));
  expect(shown('field')).toEqual(['view', 'limit', 'combine', 'size', 'ramp', 'levels', 'invert', 'tint', 'crop', 'radius', 'anti']);
  expect(shown('matrix')).toEqual(['view', 'limit', 'scale', 'mate', 'tint']);
});

test('the primary is the Enter row, and its label follows the scene playing', () => {
  const [row] = PAGE.keys;
  expect([row.button, row.label(start, { playing: () => false }), row.label(start, { playing: () => true }), row.label(start, null)]).toEqual([true, 'Play', 'Stop', 'Play']);
});

test('the view is the first row of the stack, the stack leads the pane, and the design fold sits right behind it', () => {
  expect(SPEC[0]).toMatchObject({ key: 'view', group: 'Stack' });
  expect(describe(SPEC, start)[0].rows.map((row) => row.key)).toEqual(['view', 'limit', 'combine', 'size']);
  const pane = places(SPEC);
  expect([pane.lead, pane.of('Stack'), pane.at('Stack'), pane.of('R-matrix')]).toEqual(['Stack', 0, 1, 3]);
});

test('carpet m runs to the top scale of the limit while the link keeps the asked m, and n hides at m = 3', () => {
  const low = { ...start, limit: 5, scale: 9, mate: 7 };
  expect([tidy(SPEC, low).scale, tidy(SPEC, low).mate, asked(SPEC, low).scale, asked(SPEC, low).mate]).toEqual([5, 3, 9, 7]);
  expect(describe(SPEC, tidy(SPEC, { ...start, view: 'matrix', limit: 3 })).find((group) => group.name === 'R-matrix').rows.map((row) => row.key)).toEqual(['scale']);
});

test('an export is named by the code and the limit on the field, by the limit on the r-matrix', () => {
  expect([PAGE.name(start), PAGE.name({ ...start, view: 'matrix', limit: 15 })]).toEqual(['moire-9-41', 'moire-matrix-15']);
});

test('without the unit the scene is blank', () => {
  const log = [];
  const scene = make({ getContext: () => pen(log) }, { rand: rng(1), look: () => ({}), still: false, w: 10, h: 10, dpr: 1, t: 0 }, start);
  scene.draw();
  expect(log).toEqual([]);
});
