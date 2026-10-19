import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import * as num from '@mrlyprod/mrly/num';
import { palette } from '../../lib/theme/palette.gen.js';
import { defaults } from '../../lib/app/spec.js';
import { keymap, labels, live } from '../../lib/app/page.jsx';
import { rgb } from '../../lib/color.js';
import { rng } from '../../lib/rand.js';
import { PAD, lay, survey } from './engine.js';
import { PAGE, SPEC, file, make, sketch, stamp, tidied } from './scene.js';

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => {
        if (key === 'createImageData') return (w, h) => ({ width: w, height: h, data: new Uint8ClampedArray(w * h * 4) });
        return (...args) => log.push([key, ...args.map((arg) => (arg && typeof arg === 'object' && !Array.isArray(arg) ? 'layer' : arg))]);
      },
      set: (_, key, v) => {
        log.push([key, v]);
        return true;
      },
    },
  );

const sheets = [];

globalThis.OffscreenCanvas = class {
  constructor(w, h) {
    this.width = w;
    this.height = h;
    sheets.push(this);
  }
  getContext() {
    return { createImageData: (w, h) => ({ width: w, height: h, data: new Uint8ClampedArray(w * h * 4) }), putImageData: (image) => (this.image = image) };
  }
};

const BASE = { ...defaults(SPEC), seed: 1, base: 2, code: '9', level: 3 };

const open = (value, w = 800, h = 600) => {
  const log = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#6768fa' }), still: true, w, h, dpr: 1, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...BASE, num, math, ...value });
  const draw = () => {
    log.length = 0;
    scene.draw();
    return [...log];
  };
  return { view, scene, log, draw };
};

const drawn = (log) => log.filter(([name]) => name === 'drawImage').map((call) => call.slice(2));

const rects = (log) => log.filter(([name]) => name === 'fillRect');

const pixels = (sheet) => Array.from(sheet.image.data.slice(0, 8));

test('a lift opens as one still picture over the whole stage, sampled nearest, nothing written, and a draw after is a no-op', () => {
  const one = open({});
  const log = one.draw();
  const box = lay(800, 600, 8, 0, PAD);
  expect(drawn(log)).toEqual([[0, 0, 8, 8, box.x, box.y, box.size, box.size]]);
  expect(log).toContainEqual(['imageSmoothingEnabled', false]);
  expect(log.some(([name]) => name === 'fillText')).toBe(false);
  expect(one.draw()).toEqual([]);
  one.scene.size();
  expect(drawn(one.draw())).toHaveLength(1);
});

test('a stamp paints every cell opaque in the hue of its value', () => {
  const sheet = stamp(Uint8Array.from([0, 1, 1, 0]), 2, 2, [[1, 2, 3], [4, 5, 6]]);
  expect(Array.from(sheet.image.data)).toEqual([1, 2, 3, 255, 4, 5, 6, 255, 4, 5, 6, 255, 1, 2, 3, 255]);
});

test('a grid is stamped plus orange and minus blue, and the design filter its difference pink', () => {
  const was = sheets.length;
  open({ view: 'design', power: 1 });
  open({ view: 'design', read: 'filter', fold: 'design', depth: 1 });
  const [power, filter] = sheets.slice(was).map(pixels);
  expect(power).toEqual([...rgb(palette.orange), 255, ...rgb(palette.blue), 255]);
  expect(filter.slice(4)).toEqual([...rgb(palette.pink), 255]);
});

test('a set keeps the scene: a new level stamps afresh and draws, a cell alone lays the same stamp again', () => {
  const one = open({});
  one.draw();
  const was = sheets.length;
  one.scene.set({ ...BASE, level: 4 });
  expect(sheets.length).toBe(was + 1);
  expect(drawn(one.draw()).map((call) => call[2])).toEqual([16]);
  one.scene.set({ ...BASE, level: 4, cell: 2 });
  expect(sheets.length).toBe(was + 1);
  expect(drawn(one.draw()).map((call) => call[6])).toEqual([32]);
  expect(one.scene.facts().side).toBe(16);
});

test('the word draws a cell a letter a round, in two hues, and nothing else', () => {
  const log = open({ view: 'word', round: 3 }).draw();
  expect(rects(log)).toHaveLength(2 + 4 + 8);
  expect(log.filter(([name]) => name === 'fillStyle').slice(0, 2)).toEqual([['fillStyle', palette.orange], ['fillStyle', palette.blue]]);
  expect(log.some(([name]) => name === 'fillText' || name === 'drawImage')).toBe(false);
});

test('coloured by runs, the word draws its letters yellow and pink', () => {
  const log = open({ view: 'word', round: 2, colour: 'runs' }).draw();
  expect(rects(log)).toHaveLength(2 + 4);
  expect(log.filter(([name]) => name === 'fillStyle').map(([, v]) => v).slice(1, 3)).toEqual([palette.yellow, palette.pink]);
});

test('a pinned cell sets the pixels a cell and keeps the grid centred', () => {
  const [, , , , x, , size] = drawn(open({ cell: 4 }).draw())[0];
  expect([x, size]).toEqual([384, 32]);
});

test('the scene wants both units', () => {
  expect(() => make({ getContext: () => pen([]) }, { rand: rng(1), look: () => ({}), w: 1, h: 1, dpr: 1, t: 0 }, { ...defaults(SPEC), num })).toThrow('morse');
});

test('the svg is written in the hues on screen, and offered up to a budget of cells', () => {
  expect(open({ level: 1 }).scene.svg()).toContain(`fill="${palette.orange}"`);
  expect(open({ level: 9 }).scene.svg()).toBeNull();
});

test('Enter plays and stops the rounds on the word view alone, its label following the play', () => {
  const enter = (view, on) => labels(keymap(live(PAGE.keys, { view })), { view }, { playing: () => on }).find((row) => row.key === 'Enter');
  expect([enter('word', false), enter('word', true)].map((row) => [row.act, row.label, row.button])).toEqual([['play', 'Play', true], ['play', 'Stop', true]]);
  expect([enter('plane'), enter('design')]).toEqual([undefined, undefined]);
});

test('the scene is still off the word view, and a set across the word asks for a remount', () => {
  expect([open({}).scene.still, open({ view: 'design' }).scene.still, open({ view: 'word' }).scene.still]).toEqual([true, true, false]);
  expect([open({}).scene.set({ ...BASE, view: 'word' }), open({ view: 'word' }).scene.set({ ...BASE, view: 'plane' }), open({}).scene.set({ ...BASE, view: 'design' })]).toEqual([false, false, undefined]);
});

test('the design chip shows on the design view alone', () => {
  expect(['design', 'plane', 'word'].map((view) => PAGE.design.when({ view }))).toEqual([true, false, false]);
});

test('an export is named by the picture on the stage', () => {
  const name = (over) => file({ ...BASE, ...over }, { math });
  expect([name({ view: 'word', round: 6 }), name({ view: 'word', colour: 'runs' }), name({ lift: 'Sum' }), name({ view: 'design', power: 4 }), name({ view: 'design', read: 'filter', code: 'x' })]).toEqual(['morse-word-6', 'morse-word-6-runs', 'morse-sum-3', 'morse-9-power-4', 'morse-7-filter-3']);
});

test('a thumb draws a grid as one image sampled nearest and the word as its bars, both inside a small edge', () => {
  const ink = { plus: '#ff0000', minus: '#0000ff', unlit: '#000000', diff: '#00ff00', one: '#ffff00', two: '#ff00ff' };
  const thumb = (over) => {
    const log = [];
    sketch(pen(log), 112, 112, 2, survey({ num, math }, tidied({ ...BASE, ...over })), ink);
    return log;
  };
  const box = lay(112, 112, 16, 0, 7);
  expect(drawn(thumb({ level: 4 }))).toEqual([[0, 0, 16, 16, box.x, box.y, box.size, box.size]]);
  const bars = rects(thumb({ view: 'word', round: 3 }));
  expect([bars.length, Math.min(...bars.map(([, x]) => x)), Math.min(...bars.map(([, , y]) => y))]).toEqual([2 + 4 + 8, 7, 7]);
});
