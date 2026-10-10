import { expect, test } from 'bun:test';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults, describe } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { DUST, HOLD, STRIDE, count, layout, study } from './engine.js';
import { FACE, SPEC, make } from './scene.js';

num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args.map((arg) => (arg && typeof arg === 'object' ? 'layer' : arg))]),
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
  const opts = { ...defaults(SPEC), seed: 1, root: 'strip', num, ...value };
  const view = { rand: rng(opts.seed), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 2, t: 0 };
  view.wake = () => {
    view.still = false;
    told.push('wake');
  };
  const scene = make({ getContext: () => pen(log) }, view, opts);
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, told, log, at };
};

const PLAN = study(num, 'strip', 2048);
const SPAN = PLAN.span;
const PERIODS = 3;
const BOX = layout(PLAN.frame, true, 800, 600, 48, 0);
const spot = (x, y) => [(BOX.ox + x * BOX.k) / 800, (BOX.oy - y * BOX.k) / 600];

const arcs = (log) => log.filter(([name]) => name === 'arc').length;

const laid = () => arcs(sheets.at(-1).log);

const bars = () => sheets.at(-1).log.filter(([name]) => name === 'lineTo').length;

const visible = (plan, k) => {
  let n = 0;
  for (let i = 0; i < plan.total; i++) if (plan.data[i * STRIDE + 2] * k >= DUST) n++;
  return n;
};

test('make draws nothing and the first frame lays the root circles with the first octave, each once a period', () => {
  const live = open({});
  expect(live.log).toEqual([]);
  const log = live.at(0);
  expect(laid()).toBe(count(PLAN.data, PLAN.roots, PLAN.total, PLAN.k0) * PERIODS);
  expect(laid()).toBe(4 * PERIODS);
  expect(log.findIndex(([name]) => name === 'fillRect')).toBeLessThan(log.findIndex(([name]) => name === 'drawImage'));
  expect(log.find(([name]) => name === 'fillStyle')).toEqual(['fillStyle', 'rgb(0, 20, 36)']);
});

test('a picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({});
  for (let t = 0; t <= 5000; t += 250) stepped.at(t);
  const stepLines = laid();
  const stepLog = [...stepped.log];
  const jumped = open({});
  const jumpLog = [...jumped.at(5000)];
  expect(laid()).toBe(stepLines);
  expect(jumpLog).toEqual(stepLog);
  expect(stepLines).toBeGreaterThan(4 * PERIODS);
});

test('by the end of the span every circle a pixel can show is laid, and the hold keeps the picture', () => {
  const live = open({});
  live.at(SPAN);
  const whole = visible(PLAN, BOX.k) * PERIODS;
  expect(laid()).toBe(whole);
  expect(whole).toBeLessThan(PLAN.total * PERIODS);
  live.at(SPAN + HOLD / 2);
  expect(laid()).toBe(whole);
  live.at(SPAN + HOLD + 100);
  expect(laid()).toBe(4 * PERIODS);
});

test('under reduced motion one draw lays the whole packing and a second draw adds nothing', () => {
  const still = open({}, true);
  still.at(0);
  const whole = visible(PLAN, BOX.k) * PERIODS;
  expect(laid()).toBe(whole);
  still.at(0);
  expect(laid()).toBe(whole);
});

test('a bounded root draws its hull on the stage under the layer and no band', () => {
  const live = open({ root: '-1,2,2,3', cap: 32, look: 'rings' });
  const log = live.at(0);
  const plan = study(num, '-1,2,2,3', 32);
  expect(laid()).toBe(count(plan.data, plan.roots, plan.total, plan.k0));
  const hull = log.find(([name]) => name === 'arc');
  expect(hull.slice(1, 4)).toEqual([400, 300, 252]);
  expect(log.findIndex(([name]) => name === 'arc')).toBeLessThan(log.findIndex(([name]) => name === 'drawImage'));
  expect(log.some(([name]) => name === 'fillRect')).toBe(false);
});

test('a stage thinner than its padding still draws every arc at a positive radius', () => {
  const live = open({ root: '-1,2,2,3', cap: 32 }, false, 1, 1);
  const radii = [...live.at(0), ...sheets.at(-1).log].filter(([name]) => name === 'arc').map(([, , , r]) => r);
  expect(radii.length).toBeGreaterThan(0);
  expect(radii.every((r) => r > 0)).toBe(true);
});

test('the Ford knob ticks the line at each Ford circle as it lands and draws the rest faint, on the strip alone', () => {
  const live = open({ ford: 1 });
  live.at(0);
  expect(bars()).toBe(3 * PERIODS);
  expect(sheets.at(-1).log).toContainEqual(['fillStyle', '#008cff']);
  expect(sheets.at(-1).log).toContainEqual(['fillStyle', 'rgb(0, 42, 77)']);
  live.at(SPAN);
  expect(bars()).toBe(PLAN.fords.reduce((n, v) => n + v, 0) * PERIODS);
  const off = open({ root: '-1,2,2,3', ford: 1, cap: 32 });
  off.at(SPAN);
  expect(bars()).toBe(0);
  const plain = open({});
  plain.at(SPAN);
  expect(bars()).toBe(0);
});

test('the stack lights a bar per node under the line as the circle resting there lands', () => {
  const live = open({ order: 8 });
  live.at(0);
  expect(bars()).toBe(3 * PERIODS);
  live.at(SPAN);
  expect(bars()).toBe(23 * PERIODS);
  expect(sheets.at(-1).log).toContainEqual(['strokeStyle', 'rgba(0, 140, 255, 1)']);
  const off = open({ root: '-1,2,2,3', order: 8 });
  off.at(SPAN);
  expect(bars()).toBe(0);
});

test('labels write the curvature on the circles big enough, in the site face', () => {
  const live = open({ look: 'labels' });
  live.at(0);
  const texts = sheets.at(-1).log.filter(([name]) => name === 'fillText');
  expect(texts).toHaveLength(4 * PERIODS);
  expect(new Set(texts.map(([, text]) => text))).toEqual(new Set(['2', '8']));
  expect(sheets.at(-1).log.some(([name, v]) => name === 'font' && String(v).includes(FACE))).toBe(true);
  expect(live.scene.font).toBe(FACE);
  live.at(SPAN);
  expect(sheets.at(-1).log.filter(([name]) => name === 'fillText').length).toBeLessThan(visible(PLAN, BOX.k) * PERIODS);
});

test('a tap reads the circle under it through the crate, a second tap lets go, and a gap reads nothing', () => {
  const live = open({});
  live.at(0);
  expect(live.scene.tap(...spot(0.5, 0.125))).toEqual({ k: 8, x: 4, y: 1, line: true, ford: true, num: 1, den: 2 });
  expect(live.log.filter(([name, v]) => name === 'lineWidth' && (v === 8 || v === 4))).toHaveLength(2);
  expect(live.scene.tap(...spot(0.5, 0.125))).toBeNull();
  expect(live.scene.tap(0.5, 0.01)).toBeNull();
  expect(live.scene.tap(...spot(1.5, 0.125))).toEqual({ k: 8, x: 12, y: 1, line: true, ford: true, num: 3, den: 2 });
  expect(live.scene.tap(...spot(0.05, 0.5))).toEqual({ k: 2, x: 0, y: 1, line: true, ford: true, num: 0, den: 1 });
  const round = open({ root: '-1,2,2,3', cap: 32 });
  round.at(0);
  expect(round.scene.tap(0.5, (300 - 2 / 3 * 276) / 600)).toEqual({ k: 3, x: 0, y: 2, line: false, ford: false, num: undefined, den: undefined });
});

test('with once the growth ends in a still picture that never regrows, and without it the loop starts over', () => {
  const page = open({ once: true });
  page.at(SPAN);
  const whole = laid();
  page.at(SPAN + HOLD + 100);
  expect(laid()).toBe(whole);
  page.at(10 * (SPAN + HOLD));
  expect(laid()).toBe(whole);
  const lock = open({});
  lock.at(SPAN + HOLD + 100);
  expect(laid()).toBe(4 * PERIODS);
});

test('again restarts the growth from now and wakes a still scene', () => {
  const live = open({});
  live.at(4000);
  const before = laid();
  live.scene.again();
  live.at(4100);
  expect(laid()).toBeLessThan(before / 4);
  const still = open({}, true);
  still.at(0);
  still.scene.again();
  expect(still.told).toEqual(['wake']);
  still.at(10);
  expect(laid()).toBe(4 * PERIODS);
});

test('the depth and ford knobs show exactly when the seed picks the strip', () => {
  const keys = (value) => describe(SPEC, value).flatMap(({ rows }) => rows.map((row) => row.key));
  const picks = new Set();
  for (let seed = 0; seed < 24; seed++) {
    const value = { ...defaults(SPEC), seed, root: '', cap: 8 };
    const name = open(value).scene.facts.name;
    picks.add(name === 'strip');
    expect(keys(value).filter((key) => key === 'order' || key === 'ford')).toEqual(name === 'strip' ? ['ford', 'order'] : []);
  }
  expect(picks.size).toBe(2);
  expect(keys({ ...defaults(SPEC), seed: 1, root: 'strip' })).toContain('order');
  expect(keys({ ...defaults(SPEC), seed: 1, root: '-1,2,2,3' })).not.toContain('order');
});

test('a resize or a theme change lays the layer again at the same share', () => {
  const live = open({});
  live.at(3000);
  const n = laid();
  live.scene.size();
  live.at(3000);
  expect(laid()).toBe(n);
  live.scene.theme();
  live.at(3000);
  expect(laid()).toBe(n);
});

test('a change of look, tint, ford or depth keeps the grown packing, a depth is read once, and a new root or cap grows again', () => {
  const calls = [];
  const spy = (key) => (...args) => {
    calls.push(key);
    return num.apollonian[key](...args);
  };
  const counted = { ...num, apollonian: { ...num.apollonian, grow: spy('grow'), frame: spy('frame'), touches: spy('touches'), shadow: spy('shadow') } };
  study(counted, '-1,2,2,3', 8);
  const plan = study(counted, 'strip', 512);
  calls.length = 0;
  for (const value of [{ look: 'rings' }, { tint: 'red' }, { ford: 1 }, { order: 4 }, { order: 4, look: 'labels' }]) open({ num: counted, cap: 512, ...value });
  expect(study(counted, 'strip', 512)).toBe(plan);
  expect(calls).toEqual(['shadow']);
  expect(study(counted, 'strip', 256)).not.toBe(plan);
  expect(calls).toEqual(['shadow', 'grow', 'touches', 'frame']);
});

test('the svg is a sheet of css pixels holding the band, one circle per drawn disc and one bar per lit node', () => {
  const live = open({ order: 8 });
  live.at(0);
  const text = live.scene.svg();
  expect(text).toContain('width="400" height="300" viewBox="0 0 400 300"');
  expect(text.match(/<circle /g)).toHaveLength(4 * PERIODS);
  expect(text.match(/<rect /g)).toHaveLength(1);
  expect(text.match(/<line /g)).toHaveLength(3 * PERIODS);
  expect(text).toContain('fill="rgb(0, 77, 140)"');
  const ringed = open({ look: 'labels' });
  ringed.at(0);
  const sheet = ringed.scene.svg();
  expect(sheet).toContain('stroke="#008cff"');
  expect(sheet.match(/<text /g)).toHaveLength(4 * PERIODS);
  expect(sheet).not.toContain('<rect ');
});

test('the json carries the root quadruple and every grown circle as an integer triple', () => {
  const live = open({ cap: 64 });
  live.at(0);
  const rows = JSON.parse(live.scene.json());
  expect([rows.root, rows.cap, rows.quadruple, rows.circles.length]).toEqual(['strip', 64, [[0, 0, -1], [0, 0, 1], [2, 0, 1], [2, 2, 1]], 26]);
  expect(rows.circles.slice(0, 2)).toEqual([[8, 4, 1], [8, 4, 7]]);
});
