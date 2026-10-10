import { afterAll, expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, describe, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { BEAT, FINE, HOLD, LIVE, live } from './engine.js';
import { DESIGN, REST, SPEC, make } from './scene.js';

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
    this.log = [];
  }
  getContext() {
    return pen(this.log);
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

const open = (over, still = false) => {
  const log = [];
  const view = { rand: rng(1), look: () => ({ paper: '#ffffff', accent: '#008cff' }), still, w: 800, h: 600, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), ...DESIGN, seed: 1, math, ...over });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, log, at };
};

const puts = (log) => log.filter(([name]) => name === 'putImageData');

const blits = (log) => log.filter(([name]) => name === 'drawImage');

test('the tourbillon computes a live field each new frame and a fine one on a repaint of the same t', () => {
  const live = open({ kind: 'tourbillon', limit: 21 });
  expect(blits(live.at(0))).toEqual([['drawImage', LIVE, 118, 18, 564, 564]]);
  expect(blits(live.at(16))).toEqual([['drawImage', LIVE, 118, 18, 564, 564]]);
  expect(blits(live.at(16))).toEqual([['drawImage', FINE, 118, 18, 564, 564]]);
  expect(blits(live.at(32))).toEqual([['drawImage', LIVE, 118, 18, 564, 564]]);
  expect(live.scene.every).toBe(undefined);
});

test('a take draws the fine size every frame, and a deep stack draws a smaller live raster', () => {
  const take = open({ kind: 'tourbillon', limit: 21 });
  take.view.fixed = true;
  expect(blits(take.at(0))).toEqual([['drawImage', FINE, 118, 18, 564, 564]]);
  expect(blits(take.at(33))).toEqual([['drawImage', FINE, 118, 18, 564, 564]]);
  const deep = open({ kind: 'tourbillon', limit: 99 });
  expect(blits(deep.at(0))).toEqual([['drawImage', live(50), 118, 18, 564, 564]]);
  expect(live(50)).toBeLessThan(LIVE);
});

test('a still tourbillon draws the increment itself at the fine size, the same picture twice', () => {
  const still = open({ kind: 'tourbillon', limit: 21, increment: 22.5 }, true);
  const first = still.at(0);
  expect(blits(first)).toEqual([['drawImage', FINE, 118, 18, 564, 564]]);
  expect(first).toContainEqual(['imageSmoothingEnabled', true]);
  expect(blits(still.at(0))).toEqual([]);
  expect(still.scene.facts).toMatchObject({ kind: 'tourbillon', period: 4, layers: 11 });
});

test('the moire stacks one more scale a beat and a still stacks them all, crisp', () => {
  const live = open({ kind: 'moire', code: '495', limit: 9 });
  expect(blits(live.at(0))).toHaveLength(1);
  expect(blits(live.at(100))).toHaveLength(0);
  expect(blits(live.at(BEAT + 1))).toHaveLength(1);
  expect(blits(live.at(BEAT * 5 + HOLD / 2))).toHaveLength(1);
  expect(blits(live.at(BEAT * 5 + HOLD - 1))).toHaveLength(0);
  expect(blits(live.at(BEAT * 5 + HOLD + 1))).toHaveLength(1);
  const still = open({ kind: 'moire', code: '495', limit: 9 }, true);
  const log = still.at(0);
  expect(blits(log)).toEqual([['drawImage', FINE, 118, 18, 564, 564]]);
  expect(log).toContainEqual(['imageSmoothingEnabled', false]);
  expect(still.scene.facts).toMatchObject({ kind: 'moire', name: 'carpet', layers: 5 });
});

test('the star is a still that rests between ticks and repaints on a theme or a size', () => {
  const live = open({ kind: 'star', limit: 9 });
  expect(live.scene.every).toBe(REST);
  expect(blits(live.at(0))).toEqual([['drawImage', FINE, 118, 18, 564, 564]]);
  expect(live.at(REST)).toEqual([]);
  live.scene.theme();
  expect(blits(live.at(REST * 2))).toHaveLength(1);
  live.scene.size();
  live.view.w = 400;
  expect(blits(live.at(REST * 3))).toEqual([['drawImage', FINE, 12, 112, 376, 376]]);
});

test('again restarts the sweep from now', () => {
  const live = open({ kind: 'moire', code: '495', limit: 9 });
  live.at(BEAT * 4);
  live.scene.again();
  expect(blits(live.at(BEAT * 4 + 1))).toHaveLength(0);
  expect(blits(live.at(BEAT * 5 + 1))).toHaveLength(1);
});

test('a picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({ kind: 'tourbillon', limit: 21 });
  for (let t = 0; t < 2000; t += 100) stepped.at(t);
  const walked = [...stepped.at(2000)];
  const jumped = open({ kind: 'tourbillon', limit: 21 });
  expect(jumped.at(2000)).toEqual(walked);
  expect(blits(walked)).toHaveLength(1);
});

test('without the unit the scene is blank and reads no facts', () => {
  const log = [];
  const scene = make({ getContext: () => pen(log) }, { rand: rng(1), look: () => ({}), still: false, w: 10, h: 10, dpr: 1, t: 0 }, { ...defaults(SPEC), seed: 1 });
  scene.draw();
  expect([log, scene.facts]).toEqual([[], null]);
});

test('the hive lattice is a combine of the moire face alone', () => {
  const combine = (value) => tidy(SPEC, { ...defaults(SPEC), combine: 'hive', ...value }).combine;
  expect([combine({ kind: 'moire' }), combine({ kind: 'tourbillon' }), combine({ kind: 'star' })]).toEqual(['hive', 'sum', 'sum']);
});

test('the Turn knob and its Spin group show on the tourbillon face alone', () => {
  const groups = (kind) => describe(SPEC, { ...defaults(SPEC), kind }).map(({ name }) => name);
  expect([groups('tourbillon').includes('Spin'), groups('moire').includes('Spin'), groups('star').includes('Spin')]).toEqual([true, false, false]);
});
