import { expect, test } from 'bun:test';
import { palette } from '../kit/theme/palette.js';
import { light } from '../kit/theme/theme.js';
import { board, paints, rng, run } from './scene.js';

const take = (rand, n) => Array.from({ length: n }, () => rand());

test('a seeded xorshift repeats, differs by seed, stays in [0, 1) and spreads small seeds', () => {
  expect(take(rng(7), 5)).toEqual(take(rng(7), 5));
  expect(take(rng(7), 5)).not.toEqual(take(rng(8), 5));
  expect(take(rng(7), 64).every((n) => n >= 0 && n < 1)).toBe(true);
  expect(new Set(Array.from({ length: 100 }, (_, i) => Math.floor(rng(i + 1)() * 4))).size).toBe(4);
});

test('a scene reads the tint from --accent, or --<hue> when its tint names one of the settings hues, and falls back to the palette', () => {
  const canvas = {};
  const css = { '--accent': ' #00ff00 ', '--red': '#ff0000' };
  globalThis.getComputedStyle = () => ({ getPropertyValue: (name) => css[name] ?? '' });
  expect([paints(canvas).accent, paints(canvas, '').accent, paints(canvas, 'red').accent, paints(canvas, 'gray').accent]).toEqual(['#00ff00', '#00ff00', '#ff0000', '#00ff00']);
  delete globalThis.getComputedStyle;
  expect([paints(canvas).accent, paints(canvas, 'mint').accent]).toEqual([light.accent, palette.mint]);
});

test('a board is whole device pixels a cell, centred in the view', () => {
  expect(board({ w: 1001, h: 600 }, 16, 16, 0.92)).toEqual({ x: 228, y: 28, w: 544, h: 544, cell: 34 });
  expect(board({ w: 1000, h: 300 }, 40, 10)).toEqual({ x: 0, y: 25, w: 1000, h: 250, cell: 25 });
  expect(board({ w: 10, h: 10 }, 64, 64).cell).toBe(1);
});

const stage = (fonts) => {
  const frames = [];
  Object.assign(globalThis, { window: new EventTarget(), document: Object.assign(new EventTarget(), { hidden: false, fonts }), requestAnimationFrame: (fn) => frames.push(fn), cancelAnimationFrame: () => {} });
  return frames;
};

const unstage = () => {
  for (const name of ['window', 'document', 'requestAnimationFrame', 'cancelAnimationFrame']) delete globalThis[name];
};

test('a scene takes its time from the frame loop, capped per frame', () => {
  const frames = stage();
  const seen = [];
  const stop = run({ clientWidth: 4, clientHeight: 4 }, (canvas, view) => ({ draw: () => seen.push([view.t, view.rand()]) }), { seed: 7 });
  for (const now of [1000, 1016, 1500]) frames.shift()(now);
  stop();
  unstage();
  expect(seen).toEqual([0, 16, 116].map((t, n) => [t, take(rng(7), 3)[n]]));
});

test('pause freezes play time and draws nothing; play resumes from the frozen time', () => {
  stage();
  const queue = new Map();
  let id = 0;
  Object.assign(globalThis, { requestAnimationFrame: (fn) => (queue.set(++id, fn), id), cancelAnimationFrame: (n) => queue.delete(n) });
  const next = (now) => {
    const [[n, fn]] = queue;
    queue.delete(n);
    fn(now);
  };
  const seen = [];
  const stop = run({ clientWidth: 4, clientHeight: 4 }, (canvas, view) => ({ draw: () => seen.push(view.t) }));
  next(1000);
  next(1016);
  stop.pause();
  const waiting = queue.size;
  stop.play();
  next(9000);
  next(9032);
  stop();
  unstage();
  expect([seen, waiting]).toEqual([[0, 16, 16, 48], 0]);
});

test('the stop from run carries what make returned', () => {
  stage();
  const stop = run({ clientWidth: 4, clientHeight: 4 }, () => ({ draw: () => {}, trigger: () => 'fired' }));
  stop();
  unstage();
  expect(stop.scene.trigger()).toBe('fired');
});

test('a scene that names a font draws nothing until the font loads', async () => {
  let loaded;
  const asked = [];
  const frames = stage({ load: (font) => (asked.push(font), new Promise((r) => (loaded = r))) });
  const stop = run({ clientWidth: 4, clientHeight: 4 }, () => ({ font: 'MrlyFont', draw: () => {} }));
  const early = frames.length;
  loaded();
  await Promise.resolve();
  const late = frames.length;
  stop();
  unstage();
  expect([asked, early, late]).toEqual([['1em "MrlyFont"'], 0, 1]);
});

test('under a step every drawn frame adds exactly the step, one per step of wall time and never two in one frame', () => {
  const frames = stage();
  const seen = [];
  const stop = run({ clientWidth: 4, clientHeight: 4 }, (canvas, view) => ({ draw: () => seen.push(view.t) }), { step: 40 });
  for (const now of [1000, 1016, 1032, 1048, 1100, 1200, 1201, 1202, 1203]) frames.shift()(now);
  stop();
  unstage();
  expect(seen).toEqual([40, 80, 120, 160, 200, 240]);
});

test('a debt over four steps resets and counts the frames lost as late, which after hears with t after each draw', () => {
  const frames = stage();
  const heard = [];
  const stop = run({ clientWidth: 4, clientHeight: 4 }, () => ({ draw: () => {} }));
  stop.fix({ step: 40, after: ({ t, late }) => heard.push([t, late]) });
  for (const now of [1000, 1500, 1540]) frames.shift()(now);
  const late = stop.late;
  stop();
  unstage();
  expect([heard, late]).toEqual([[[40, 0], [80, 11], [120, 11]], 11]);
});

test('a fixed frame sets the store, a dpr of its short side over 540 and view.fixed, ignores resizes, and fix(null) returns to the box', () => {
  stage();
  let resize;
  globalThis.ResizeObserver = class {
    constructor(fn) {
      resize = fn;
    }
    observe() {}
    disconnect() {}
  };
  const canvas = { clientWidth: 4, clientHeight: 4 };
  const seen = [];
  const look = (view) => seen.push([view.w, view.h, view.dpr, view.fixed, canvas.width]);
  const stop = run(canvas, (c, view) => (look(view), { draw: () => {}, size: () => look(view) }), { frame: [1080, 1920] });
  canvas.clientWidth = 8;
  resize();
  stop.fix(null);
  stop.fix({ frame: [1280, 720] });
  stop();
  delete globalThis.ResizeObserver;
  unstage();
  expect(seen).toEqual([[1080, 1920, 2, true, 1080], [8, 4, 1, false, 8], [1280, 720, 720 / 540, true, 1280]]);
});

test('an every scene ignores the step', () => {
  stage();
  const ticks = [];
  const real = { setInterval: globalThis.setInterval, clearInterval: globalThis.clearInterval };
  Object.assign(globalThis, { setInterval: (fn, ms) => ticks.push([fn, ms]), clearInterval: () => {} });
  const seen = [];
  const stop = run({ clientWidth: 4, clientHeight: 4 }, (canvas, view) => ({ every: 50, draw: () => seen.push(view.t) }), { step: 40 });
  ticks[0][0]();
  ticks[0][0]();
  stop();
  Object.assign(globalThis, real);
  unstage();
  expect([ticks[0][1], seen]).toEqual([50, [50, 100]]);
});

test('a take with a step wakes a reduced-motion scene, which then draws one step a frame', () => {
  const frames = stage();
  globalThis.matchMedia = () => ({ matches: true, addEventListener: () => {}, removeEventListener: () => {} });
  const seen = [];
  const stop = run({ clientWidth: 4, clientHeight: 4 }, (canvas, view) => ({ draw: () => seen.push([view.t, view.still]) }));
  const idle = frames.length;
  stop.fix({ step: 40, after: () => {} });
  for (const now of [1000, 1040]) frames.shift()(now);
  stop();
  delete globalThis.matchMedia;
  unstage();
  expect([idle, seen]).toEqual([0, [[0, true], [40, false], [80, false]]]);
});

test('fix(null) after a take puts a reduced-motion scene back to stillness, drawn once with no frame loop', () => {
  stage();
  const queue = new Map();
  let id = 0;
  Object.assign(globalThis, { requestAnimationFrame: (fn) => (queue.set(++id, fn), id), cancelAnimationFrame: (n) => queue.delete(n) });
  globalThis.matchMedia = () => ({ matches: true, addEventListener: () => {}, removeEventListener: () => {} });
  const seen = [];
  const stop = run({ clientWidth: 4, clientHeight: 4 }, (canvas, view) => ({ draw: () => seen.push([view.t, view.still]) }));
  stop.fix({ step: 40, after: () => {} });
  const [[n, fn]] = queue;
  queue.delete(n);
  fn(1000);
  stop.fix(null);
  const waiting = queue.size;
  stop();
  delete globalThis.matchMedia;
  unstage();
  expect([seen, waiting]).toEqual([[[0, true], [40, false], [40, true]], 0]);
});
