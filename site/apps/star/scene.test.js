import { afterAll, expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { MOST } from './engine.js';
import { PAGE, SPEC, make } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args.map((arg) => (arg && typeof arg === 'object' ? (arg.width ?? 'thing') : arg))]),
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

const start = { ...defaults(SPEC), layers: 6, size: 128, seed: 1 };

const counted = () => {
  const calls = { volume: 0, render: 0 };
  const moire = { ...math.moire, volume: (...args) => (calls.volume++, math.moire.volume(...args)), render: (...args) => (calls.render++, math.moire.render(...args)) };
  return { calls, math: { ...math, moire } };
};

const open = (over = {}) => {
  const log = [];
  const { calls, math: door } = counted();
  const view = { rand: rng(1), look: () => ({ paper: '#ffffff', accent: '#008cff' }), still: false, w: 800, h: 600, dpr: 1, t: 0 };
  const told = [];
  const made = make({ getContext: () => pen(log) }, view, { ...start, ...over, math: door, onFacts: (f) => told.push(f) });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    made.draw();
    return log;
  };
  const set = (patch) => made.set({ ...start, ...over, ...patch });
  return { scene: made, at, calls, told, set, last: () => told.at(-1) };
};

const blits = (log) => log.filter(([name]) => name === 'drawImage');

test('the star is a still: drawn once, smoothed, then only on a change, with its readings', () => {
  const page = open();
  const first = page.at(0);
  expect(blits(first)).toEqual([['drawImage', 128, 118, 18, 564, 564]]);
  expect(first).toContainEqual(['imageSmoothingEnabled', true]);
  expect([blits(page.at(5000)), page.calls.volume, page.scene.every, page.scene.still]).toEqual([[], 6, undefined, true]);
  expect(page.last()).toMatchObject({ layers: 6, half: 0, deepest: 11, exact: 6, size: 128, branch: '2 mod 4', slope: null });
});

test('moving L adds or drops cuts without recutting, W reads again without restacking, a paint key repaints', () => {
  const page = open();
  page.at(0);
  page.set({ layers: 8 });
  page.at(16);
  page.set({ layers: 4 });
  page.at(32);
  expect([page.calls.volume, page.last().layers]).toEqual([8, 4]);
  page.set({ layers: 4, half: 2 });
  page.at(48);
  expect([page.calls.volume, page.calls.render, page.last().target]).toEqual([8, 3, -1 / 6]);
  page.set({ layers: 4, half: 2, ramp: 'heat' });
  expect(blits(page.at(64))).toHaveLength(1);
  expect(page.calls.render).toBe(4);
});

test('a tap adds a cut, and past the top it starts over', () => {
  const page = open();
  page.at(0);
  expect([page.scene.touch(0.5, 0.5, true), page.scene.touch(0.6, 0.5, false)]).toEqual([{ layers: 8 }, null]);
  page.set({ layers: MOST });
  expect(page.scene.touch(0.5, 0.5, true)).toEqual({ layers: 4 });
});

test('the page keeps its defaults', () => {
  expect(tidy(SPEC, defaults(SPEC))).toEqual(defaults(SPEC));
});

test('an export is named by its layer count', () => {
  expect(PAGE.name({ ...start, layers: 28 })).toBe('star-28');
});

test('without the unit the scene is blank', () => {
  const log = [];
  const made = make({ getContext: () => pen(log) }, { rand: rng(1), look: () => ({}), still: false, w: 10, h: 10, dpr: 1, t: 0 }, start);
  made.draw();
  expect(log).toEqual([]);
});
