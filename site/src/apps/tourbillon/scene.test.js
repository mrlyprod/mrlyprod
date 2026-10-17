import { afterAll, expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { defaults, describe, tidy } from '../../lib/app/spec.js';
import { rng } from '../../lib/rand.js';
import { NUDGE, STAMP, SWEEP } from './engine.js';
import { PAGE, SPEC, make } from './scene.js';

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

const start = { ...defaults(SPEC), limit: 21, size: 256, seed: 1 };

const counted = () => {
  const calls = { field: [], render: 0 };
  const tourbillon = { ...math.tourbillon, field: (...args) => (calls.field.push([args[1], args[3]]), math.tourbillon.field(...args)) };
  const moire = { ...math.moire, render: (...args) => (calls.render++, math.moire.render(...args)) };
  return { calls, math: { ...math, tourbillon, moire } };
};

const open = (over = {}, lock = false) => {
  const log = [];
  const { calls, math: door } = counted();
  const view = { rand: rng(1), look: () => ({ paper: '#ffffff', accent: '#008cff' }), still: false, fixed: false, lock, w: 800, h: 600, dpr: 1, t: 0, wake: () => {} };
  const told = { facts: [], turns: [] };
  const scene = make({ getContext: () => pen(log) }, view, { ...start, ...over, math: door, onFacts: (f) => told.facts.push(f), onTurn: (patch) => told.turns.push(patch) });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  const set = (patch) => scene.set({ ...start, ...over, ...patch });
  return { view, scene, at, calls, told, set, last: () => told.facts.at(-1) };
};

const blits = (log) => log.filter(([name]) => name === 'drawImage');

test('the page opens still on the increment, drawn once at the fine size, smoothed, with readings at that angle', () => {
  const page = open();
  const first = page.at(0);
  expect(blits(first)).toEqual([['drawImage', 256, 118, 18, 564, 564]]);
  expect(first).toContainEqual(['imageSmoothingEnabled', true]);
  expect(blits(page.at(5000))).toEqual([]);
  expect(page.calls.field).toEqual([[256, 1]]);
  expect(page.last()).toMatchObject({ angle: 1, playing: false, size: 256, layers: 11, period: 90, classes: 11, pairs: 0, unused: false });
});

test('in the lock the spin plays from t with no hand, on the degrees schedule', () => {
  const lock = open({ sched: 'golden' }, true);
  lock.at(0);
  lock.at(1000);
  expect(lock.scene.playing()).toBe(true);
  expect(lock.calls.field.at(-1)[1]).toBeCloseTo(1 + SWEEP);
});

test('a take started while stopped plays', () => {
  const page = open();
  page.view.fixed = true;
  page.at(0);
  page.at(1000);
  expect([page.scene.playing(), page.calls.field.at(-1)[1]]).toEqual([true, 1 + SWEEP]);
});

test('play sweeps at six degrees a second, stamps the angle each second, and stop lands it at the fine size', () => {
  const page = open({ size: 1024, sched: 'golden' });
  page.at(0);
  expect(page.scene.toggle()).toBe(true);
  expect(page.told.turns).toEqual([{ sched: 'degrees' }]);
  page.at(500);
  expect(page.calls.field.at(-1)).toEqual([page.calls.field.at(-1)[0], 1 + SWEEP / 2]);
  expect(page.calls.field.at(-1)[0]).toBeLessThan(1024);
  page.at(STAMP);
  expect(page.told.turns.at(-1)).toEqual({ increment: 1 + SWEEP });
  page.at(1250);
  expect(page.scene.toggle()).toBe(false);
  expect(page.told.turns.at(-1)).toEqual({ increment: 8.5 });
  page.at(1300);
  expect(page.calls.field.at(-1)).toEqual([1024, 8.5]);
  expect([page.scene.playing(), page.last().angle, page.last().playing]).toEqual([false, 8.5, false]);
});

test('the echo of a stamp keeps the spin playing, a hand on the increment stops it there', () => {
  const page = open();
  page.at(0);
  page.scene.toggle();
  page.at(STAMP);
  page.set({ increment: 1 + SWEEP });
  page.at(STAMP + 100);
  expect(page.scene.playing()).toBe(true);
  page.set({ increment: 45 });
  page.at(STAMP + 200);
  expect([page.scene.playing(), page.calls.field.at(-1)[1]]).toEqual([false, 45]);
});

test('a nudge lands the angle on screen 0.05 degrees over or under', () => {
  const page = open({ increment: 10 });
  page.at(0);
  expect(page.scene.nudge(1)).toBeCloseTo(10 + NUDGE, 9);
  page.scene.toggle();
  page.at(1000);
  expect(page.scene.nudge(-1)).toBe(16);
  expect([page.scene.playing(), page.told.turns.at(-1)]).toEqual([false, { increment: 16 }]);
});

test('a drag across the picture turns the increment by hand, a tap plays and stops', () => {
  const page = open({ increment: 10 });
  page.at(0);
  expect(page.scene.touch(0.5, 0.5, true)).toBe(null);
  expect(page.scene.touch(0.6, 0.5, false)).toEqual({ increment: 19 });
  page.scene.lift();
  expect(page.scene.playing()).toBe(false);
  page.scene.touch(0.5, 0.5, true);
  page.scene.lift();
  expect(page.scene.playing()).toBe(true);
});

test('a paint key repaints the held field, a plan key restacks it, and a drag of values goes live then fine', () => {
  const page = open();
  page.at(0);
  page.set({ ramp: 'tint', levels: 4 });
  page.at(16);
  expect([page.calls.field.length, page.calls.render]).toEqual([1, 2]);
  page.set({ set: 'primes' });
  page.at(32);
  page.at(40);
  expect([page.calls.field.length, page.last().layers]).toEqual([2, 7]);
  page.set({ set: 'primes', size: 1024, increment: 2 });
  page.at(48);
  page.set({ set: 'primes', size: 1024, increment: 3 });
  page.at(64);
  page.at(80);
  expect(page.calls.field.slice(2)).toEqual([[1024, 2], [page.calls.field[3][0], 3], [1024, 3]]);
  expect(page.calls.field[3][0]).toBeLessThan(1024);
});

test('the page keeps its defaults', () => {
  expect(tidy(SPEC, defaults(SPEC))).toEqual(defaults(SPEC));
});

test('the primary is the Enter row, and its label follows the scene playing', () => {
  const [row] = PAGE.keys;
  expect([row.button, row.label(start, { playing: () => false }), row.label(start, { playing: () => true })]).toEqual([true, 'Play', 'Stop']);
});

test('an export is named by the increment on screen', () => {
  expect([PAGE.name({ ...start, increment: 22.5 }), PAGE.name({ ...start, increment: 8.181818 })]).toEqual(['tourbillon-22.5', 'tourbillon-8.181818']);
});

test('the stack, draw and look rows show in the pane, the spin rows are the widget fold', () => {
  expect(describe(SPEC, start).map((group) => group.name)).toEqual(['Stack', 'Draw', 'Look']);
});

test('without the unit the scene is blank', () => {
  const log = [];
  const scene = make({ getContext: () => pen(log) }, { rand: rng(1), look: () => ({}), still: false, w: 10, h: 10, dpr: 1, t: 0 }, start);
  scene.draw();
  expect(log).toEqual([]);
});
