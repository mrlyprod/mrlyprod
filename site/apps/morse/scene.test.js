import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults, describe } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { BEAT, END, PAD, SLIDE, VIEWS, lay, panels, span, tower } from './engine.js';
import { SPEC, make } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });
num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

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
    this.log = [];
    sheets.push(this);
  }
  getContext() {
    return pen(this.log);
  }
};

const open = (value, still = false, w = 800, h = 600) => {
  const log = [];
  const told = [];
  const view = { rand: rng(value.seed ?? 1), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 2, t: 0 };
  view.wake = () => {
    view.still = false;
    told.push('wake');
  };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 1, base: 2, code: '', lift: 'Parity', level: 3, num, math, ...value });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, told, log, at };
};

const drawn = (log) => log.filter(([name]) => name === 'drawImage');

const alpha = (log) => log.filter(([name]) => name === 'globalAlpha').map(([, v]) => v);

const BOX = lay({ x: 0, y: 0, w: 800, h: 600 }, 8, 0, PAD * 2);

test('a picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({});
  for (let t = 0; t <= 5000; t += 250) stepped.at(t);
  const log = [...stepped.log];
  const jumped = open({});
  expect([...jumped.at(5000)]).toEqual(log);
  expect(drawn(log)).toHaveLength(1);
});

test('the still is the whole grid at the last level, one draw of the source onto its box, sampled nearest', () => {
  const still = open({}, true);
  const log = still.at(0);
  expect(drawn(log)).toEqual([['drawImage', 'layer', 0, 0, 8, 8, BOX.x, BOX.y, BOX.size, BOX.size]]);
  expect(log).toContainEqual(['imageSmoothingEnabled', false]);
  expect(alpha(log)).toEqual([1, 1]);
  const source = sheets.at(-1);
  expect([source.width, source.height]).toEqual([8, 8]);
  expect(source.log.map(([name]) => name)).toEqual(['putImageData']);
});

test('the growth shows the corner window a level at a time and slides between two levels', () => {
  const live = open({});
  expect(drawn(live.at(BEAT / 2))[0].slice(2, 6)).toEqual([0, 0, 2, 2]);
  expect(drawn(live.at(BEAT + SLIDE + BEAT / 2))[0].slice(2, 6)).toEqual([0, 0, 4, 4]);
  const [, , x, y, s] = drawn(live.at(BEAT + SLIDE / 2))[0];
  expect([x, y]).toEqual([0, 0]);
  expect(s).toBeGreaterThan(2);
  expect(s).toBeLessThan(4);
  expect(drawn(live.at(span(3) - END / 2))[0].slice(2, 6)).toEqual([0, 0, 8, 8]);
  expect(drawn(live.at(span(3) + BEAT / 2))[0].slice(2, 6)).toEqual([0, 0, 2, 2]);
  expect(alpha(live.at(0))[0]).toBe(0);
});

test('with no grow the scene is a still: one draw lays it, a draw after is a no-op, and a resize or a theme lays it again', () => {
  const fixed = open({ grow: 0 });
  expect(fixed.scene.every).toBe(1000);
  const first = drawn(fixed.at(0));
  expect(first).toHaveLength(1);
  expect(first[0].slice(2, 6)).toEqual([0, 0, 8, 8]);
  expect(fixed.at(1000)).toEqual([]);
  fixed.scene.size();
  expect(drawn(fixed.log)).toHaveLength(1);
  const was = sheets.length;
  fixed.log.length = 0;
  fixed.scene.theme();
  expect(sheets.length).toBe(was + 1);
  expect(drawn(fixed.log)).toHaveLength(1);
  expect(open({}).scene.every).toBeUndefined();
});

test('the word view lays a row a stage, the rows arriving one by one and the next fading in', () => {
  const rows = tower(800, 600, 3, PAD * 2);
  const still = open({ view: 'word' }, true);
  const whole = drawn(still.at(0));
  expect(whole.map((row) => row.slice(2))).toEqual(rows.rows.map((row) => [rows.x, row.y, rows.w, row.h]));
  expect(sheets.slice(-3).map((sheet) => [sheet.width, sheet.height])).toEqual([[2, 1], [4, 1], [8, 1]]);
  const live = open({ view: 'word' });
  expect(drawn(live.at(BEAT / 2))).toHaveLength(1);
  const sliding = live.at(BEAT + SLIDE / 2);
  expect(drawn(sliding)).toHaveLength(2);
  expect(alpha(sliding)).toEqual([1, 0.5, 1]);
  expect(drawn(live.at(2 * (BEAT + SLIDE)))).toHaveLength(3);
});

test('the lift view draws the design and its sign power side by side, or stacked when the stage is tall', () => {
  const wide = open({ view: 'lift', code: '9', level: 2 }, true);
  const pair = drawn(wide.at(0));
  const left = lay({ x: 0, y: 0, w: 400, h: 600 }, 4, 0, PAD * 2);
  const right = lay({ x: 400, y: 0, w: 400, h: 600 }, 4, 0, PAD * 2);
  expect(pair).toEqual([['drawImage', 'layer', 0, 0, 4, 4, left.x, left.y, left.size, left.size], ['drawImage', 'layer', 0, 0, 4, 4, right.x, right.y, right.size, right.size]]);
  const tall = open({ view: 'lift', code: '9', level: 2 }, true, 600, 800);
  const stack = drawn(tall.at(0));
  expect(stack[0][7]).toBeLessThan(stack[1][7]);
  expect(stack[0][6]).toBe(stack[1][6]);
  const star = open({ view: 'lift', code: '6', level: 2 });
  expect(drawn(star.at(BEAT / 2))[0].slice(2, 6)).toEqual([2, 0, 2, 2]);
});

test('the filter view draws the blown-up level, the next level and their difference in a row, each whole', () => {
  const still = open({ view: 'filter', code: '9', level: 1 }, true);
  const row = drawn(still.at(0));
  const boxes = panels(800, 600, 3).map((panel) => lay(panel, 4, 0, PAD * 2));
  expect(row).toEqual(boxes.map((box) => ['drawImage', 'layer', 0, 0, 4, 4, box.x, box.y, box.size, box.size]));
});

test('a pinned cell sets the pixels a cell and keeps the grid centred', () => {
  const pinned = open({ cell: 4 }, true);
  const [, , , , , , x, y, size] = drawn(pinned.at(0))[0];
  expect([x, y, size]).toEqual([368, 268, 64]);
});

test('again restarts the growth from now and wakes a still scene, and does nothing with no grow', () => {
  const live = open({});
  live.at(3000);
  live.scene.again();
  expect(drawn(live.at(3000 + BEAT / 2))[0].slice(2, 6)).toEqual([0, 0, 2, 2]);
  const still = open({}, true);
  still.at(0);
  still.scene.again();
  expect(still.told).toEqual(['wake']);
  expect(drawn(still.at(10))[0].slice(2, 6)).toEqual([0, 0, 2, 2]);
  const fixed = open({ grow: 0 }, true);
  fixed.at(0);
  fixed.scene.again();
  expect(fixed.told).toEqual([]);
});

test('again shows the first level whole at once, so a paused page is not left blank', () => {
  const live = open({});
  live.at(3000);
  live.log.length = 0;
  live.scene.again();
  expect(drawn(live.log).map((call) => call.slice(2, 6))).toEqual([[0, 0, 2, 2]]);
  expect(alpha(live.log)).toEqual([1, 1]);
});

test('the scene wants both units', () => {
  expect(() => make({ getContext: () => pen([]) }, { rand: rng(1), look: () => ({}), w: 1, h: 1, dpr: 1, t: 0 }, { ...defaults(SPEC), num })).toThrow('morse');
});

test('the svg is written in the accent on screen', () => {
  const live = open({ level: 2 }, true);
  live.at(0);
  expect(live.scene.svg()).toContain('fill="#008cff"');
});

test('the svg is offered up to a budget of cells, so the plus-minus filter at level 8 offers no file', () => {
  const offered = (over) => typeof open(over, true).scene.svg === 'function';
  expect([offered({ level: 8 }), offered({ level: 9 }), offered({ view: 'filter', code: '9', level: 6 }), offered({ view: 'filter', code: '9', level: 8 }), offered({ view: 'word', level: 10 })]).toEqual([true, false, true, false, true]);
});

test('the scene hands the widget the facts of what it drew', () => {
  expect(open({ level: 2 }, true).scene.facts).toMatchObject({ view: 'plane', kind: 'Parity', side: 4 });
});

test('the lift pick shows on the plane, the fold on the filter, the tile side on the lift and the filter, and the cell everywhere but the word', () => {
  const shown = (view) => describe(SPEC, { ...defaults(SPEC), view }).flatMap(({ rows }) => rows.map((row) => row.key));
  expect(shown('plane')).toEqual(['view', 'lift', 'level', 'cell', 'grow', 'tint']);
  expect(shown('word')).toEqual(['view', 'level', 'grow', 'tint']);
  expect(shown('lift')).toEqual(['view', 'number', 'level', 'cell', 'grow', 'tint']);
  expect(shown('filter')).toEqual(['view', 'fold', 'number', 'level', 'cell', 'grow', 'tint']);
});

test('every segment fits the right bar, so the four views are a pick', () => {
  const room = 198;
  const width = (options) => options.reduce((sum, [, label]) => sum + label.length * 7 + 24, 4 + 2 * (options.length - 1));
  const segments = SPEC.filter((row) => row.kind === 'segment');
  expect(segments.map((row) => row.key)).toEqual(['fold']);
  expect(segments.every((row) => width(row.options) <= room)).toBe(true);
  expect(width(VIEWS)).toBeGreaterThan(room);
});
