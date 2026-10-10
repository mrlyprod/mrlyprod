import { expect, test } from 'bun:test';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { hit } from '../../lib/keys.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { keymap, live } from '../../lib/scene.jsx';
import { HOLD, PAD, lay, study } from './engine.js';
import { PAGE, SPEC, make } from './scene.js';

num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args.map((arg) => (arg && typeof arg === 'object' && !Array.isArray(arg) ? 'layer' : arg))]),
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
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 1, roll: 0, ring: 'square', limit: 12, num, ...value });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, told, log, at };
};

const PLAN = study(num, 'square', 12);

const marks = (sheet) => sheet.log.filter(([name]) => name === 'rect' || name === 'arc').length;

const laid = () => marks(sheets.at(-1));

const whole = PLAN.count - 1;

test('a picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({});
  for (let t = 0; t <= 7000; t += 250) stepped.at(t);
  const count = laid();
  const log = [...stepped.log];
  const jumped = open({});
  const once = [...jumped.at(7000)];
  expect(laid()).toBe(count);
  expect(once).toEqual(log);
  expect(count).toBeGreaterThan(0);
  expect(count).toBeLessThan(whole);
});

test('under reduced motion the first draw lays the whole window and a second draw adds nothing', () => {
  const still = open({}, true);
  still.at(0);
  expect(laid()).toBe(whole);
  still.at(0);
  expect(laid()).toBe(whole);
  expect(sheets.length).toBeGreaterThan(0);
});

test('with no grow the scene is a still: one draw lays it, a draw after is a no-op, and a resize or a theme lays it again', () => {
  const live = open({ grow: 0 });
  expect(live.scene.every).toBe(1000);
  expect(live.at(0).length).toBeGreaterThan(0);
  expect(laid()).toBe(whole);
  expect(live.at(1000)).toEqual([]);
  const was = sheets.length;
  live.scene.size();
  expect(sheets.length).toBe(was + 1);
  expect(laid()).toBe(whole);
  live.scene.theme();
  expect(sheets.length).toBe(was + 2);
  expect(live.at(2000)).toEqual([]);
});

test('past the span and the hold the window starts over on a fresh layer', () => {
  const live = open({ grow: 4 });
  live.at(4000);
  expect(laid()).toBe(whole);
  live.at(4000 + HOLD / 2);
  expect(laid()).toBe(whole);
  live.at(4000 + HOLD + 50);
  expect(laid()).toBeLessThan(whole / 10);
});

test('a tap picks the nearest point, draws its rings and the picture stays pure, and a tap off the window clears it', () => {
  const live = open({});
  live.at(3000);
  const box = lay(PLAN.frame, 800, 600, PAD * 2);
  const u = (box.ox + 2 * box.k) / 800;
  const v = (box.oy - 3 * box.k) / 600;
  const picked = live.scene.at(u, v);
  expect(picked).toMatchObject({ a: 2, b: 3, norm: 13, fate: 1 });
  const rings = live.log.filter(([name]) => name === 'arc');
  expect(rings).toHaveLength(1 + 3 + 1);
  expect(rings.at(-1).slice(1, 3)).toEqual([box.ox + 2 * box.k, box.oy - 3 * box.k]);
  expect(live.log).toContainEqual(['setLineDash', [8, 6]]);
  expect(live.scene.at(0.5, 0.5)).toMatchObject({ a: 0, b: 0, fate: 5 });
  expect(live.scene.at(0, 0)).toBe(null);
  expect(live.at(3000).filter(([name]) => name === 'arc')).toEqual([]);
});

test('again restarts the growth from now and wakes a still scene', () => {
  const live = open({});
  live.at(6000);
  const before = laid();
  live.scene.again();
  live.at(6100);
  expect(laid()).toBeLessThan(before / 4);
  const still = open({}, true);
  still.at(0);
  still.scene.again();
  expect(still.told).toEqual(['wake']);
  still.at(10);
  expect(laid()).toBeLessThan(10);
  const fixed = open({ grow: 0 }, true);
  fixed.at(0);
  fixed.scene.again();
  expect(fixed.told).toEqual([]);
});

test('the hex ring lays circles and the square ring rects, each in the shades of the look', () => {
  const hex = open({ ring: 'hex' }, true);
  hex.at(0);
  const round = sheets.at(-1).log;
  expect(round.some(([name]) => name === 'arc')).toBe(true);
  expect(round.some(([name]) => name === 'rect')).toBe(false);
  expect(hex.scene.facts.word).toBe('hex');
  const square = open({ look: 'fate' }, true);
  square.at(0);
  const flat = sheets.at(-1).log;
  expect(flat.some(([name]) => name === 'rect')).toBe(true);
  expect(flat).toContainEqual(['fillStyle', '#008cff']);
  expect(flat).toContainEqual(['fillStyle', 'rgb(0, 70, 128)']);
  expect(flat).toContainEqual(['fillStyle', 'rgba(0, 140, 255, 0.12)']);
  expect(flat.some(([name]) => name === 'stroke')).toBe(true);
});

test('a pinned cell size sets the lattice spacing in device pixels', () => {
  const live = open({ cell: 4 }, true);
  live.at(0);
  const rects = sheets.at(-1).log.filter(([name]) => name === 'rect');
  const xs = rects.map(([, x]) => x);
  expect(Math.max(...xs) - Math.min(...xs)).toBe(24 * 8);
});

test('a rolled window follows the seed and its look, and a named one is exactly the value', () => {
  const rolled = (seed) => open({ roll: 1, seed, ring: 'hex', limit: 3, look: 'plain' }).scene.facts;
  const a = rolled(1);
  expect(rolled(1)).toEqual(a);
  const seen = new Set(Array.from({ length: 30 }, (_, seed) => JSON.stringify([rolled(seed).word, rolled(seed).reach, rolled(seed).look])));
  expect(seen.size).toBeGreaterThan(10);
  const named = open({ ring: 'hex', limit: 5, look: 'norm' }).scene.facts;
  expect([named.word, named.reach, named.look]).toEqual(['hex', 5, 'norm']);
});

test('the svg of the scene is written in the shades on screen', () => {
  const live = open({}, true);
  live.at(0);
  const text = live.scene.svg();
  expect(text).toContain('fill="#008cff"');
  expect(text).toContain('fill="rgba(0, 140, 255, 0.12)"');
  expect(text.match(/<rect /g).length).toBeGreaterThanOrEqual(PLAN.facts.primes);
});

test('at grow 0 the Again row leaves the keys map, so neither Enter nor the primary button is left dead', () => {
  const map = (grow) => keymap(live(PAGE.keys, tidy(SPEC, { grow })));
  expect([map(0).find((row) => row.button), hit(map(0), { key: 'Enter' })]).toEqual([undefined, null]);
  expect([map(12).find((row) => row.button)?.act, hit(map(12), { key: 'Enter' })?.act]).toEqual(['again', 'again']);
});
