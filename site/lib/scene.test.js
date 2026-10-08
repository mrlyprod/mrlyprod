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
