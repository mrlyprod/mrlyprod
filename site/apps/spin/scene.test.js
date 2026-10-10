import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { rgb, rng } from '../../lib/scene.js';
import { OUT, SAMPLES, STEPS, WHEEL } from './engine.js';
import { DESIGN, PAGE, SPEC, make } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const brief = (arg) => (arg && typeof arg === 'object' ? 'layer' : arg);

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args.map(brief)]),
      set: (_, key, v) => {
        log.push([key, v]);
        return true;
      },
    },
  );

const sheets = [];
const images = [];

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

globalThis.ImageData = class {
  constructor(data, w, h) {
    this.data = data;
    this.width = w;
    this.height = h;
    images.push(this);
  }
};

const ACCENT = '#008cff';
const PAPER = '#000000';

const open = (value, { still = false, w = 800, h = 600, unit = math, paces = [] } = {}) => {
  const log = [];
  const from = sheets.length;
  const view = { rand: rng(1), look: () => ({ paper: PAPER, accent: ACCENT }), still, w, h, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(DESIGN), ...defaults(SPEC), seed: 1, code: '495', level: 2, math: unit, onPace: (p) => paces.push(p), ...value });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    for (const sheet of sheets) sheet.log.length = 0;
    scene.draw();
    return { main: [...log], sheets: sheets.slice(from).map((sheet) => [...sheet.log]) };
  };
  return { view, scene, log, paces, at };
};

const draws = (log) => log.filter(([name]) => name === 'drawImage');

test('the stage lays the turntable and the wheel side by side, the wheel a raster of its disc, and a still turns nothing', () => {
  const still = open({}, { still: true });
  const { main, sheets: layers } = still.at(0);
  expect(draws(main)).toEqual([['drawImage', 'layer', 208 - 176, 300 - 176], ['drawImage', 'layer', 592 - 176, 300 - 176, 352, 352]]);
  expect(main.filter(([name]) => name === 'arc')).toHaveLength(2);
  const table = layers.find((log) => log.some(([name]) => name === 'rotate'));
  expect(table).toContainEqual(['rotate', 0]);
  expect(table).toContainEqual(['clearRect', 0, 0, 352, 352]);
  const wheel = images.at(-1);
  expect([wheel.width, wheel.height, wheel.data.length]).toEqual([WHEEL, WHEEL, WHEEL * WHEEL * 4]);
  expect(still.scene.every).toBe(undefined);
  expect(still.scene.facts).toMatchObject({ code: '495', side: 9, fills: 64, order: 4, petals: 12, inner: 4.5, peak: 1 });
  expect(still.scene.facts.profile).toHaveLength(STEPS);
});

test('the wheel is painted to its peak, so a lone cell lights its ring in full while the facts keep the exact mean', () => {
  const dot = open({ code: '1', level: 2 }, { still: true });
  dot.at(0);
  const wheel = images.at(-1);
  let top = 0;
  for (let i = 3; i < wheel.data.length; i += 4) top = Math.max(top, wheel.data[i]);
  expect(top).toBe(255);
  expect(dot.scene.facts.peak).toBeLessThan(0.05);
  expect(dot.scene.facts.disc).toBeGreaterThan(dot.scene.facts.inner);
});

test('at rpm 0 the turntable rests between ticks and reports no pace', () => {
  const live = open({ rpm: 0 });
  expect(live.scene.every).toBe(1000);
  live.at(0);
  live.at(1000);
  expect(live.paces).toEqual([]);
  expect(live.at(2000).sheets.flat()).toContainEqual(['rotate', 0]);
});

test('the needle keys set the demo rpm presets through the scene', () => {
  const got = [];
  PAGE.actions.needle({ needle: (rpm) => got.push(rpm) }, { key: '5' });
  PAGE.actions.needle({ needle: (rpm) => got.push(rpm) }, { key: '1' });
  expect(got).toEqual([900, 33]);
  expect(PAGE.keys.find((row) => row.act === 'needle').key).toEqual(['1', '2', '3', '4', '5']);
});

test('a picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({});
  let last;
  for (let t = 0; t <= 7000; t += 100) last = stepped.at(t);
  const jumped = open({});
  jumped.at(0);
  expect(jumped.at(7000)).toEqual(last);
  const turned = last.sheets.flat().find(([name]) => name === 'rotate');
  expect(turned[1]).toBeCloseTo(((33 * 7) / 60 - 3) * Math.PI * 2, 9);
});

test('a tall stage stacks the discs', () => {
  const { main } = open({}, { w: 600, h: 800 }).at(0);
  expect(draws(main)).toEqual([['drawImage', 'layer', 300 - 176, 208 - 176], ['drawImage', 'layer', 300 - 176, 592 - 176, 352, 352]]);
});

test('the wheel alone rests between ticks and draws one disc with no turn', () => {
  const live = open({ show: 'wheel' });
  const { main, sheets: layers } = live.at(0);
  expect(live.scene.every).toBe(1000);
  expect(draws(main)).toEqual([['drawImage', 'layer', 400 - 268, 300 - 268, 536, 536]]);
  expect(layers.flat().some(([name]) => name === 'rotate')).toBe(false);
});

test('the rosette stacks the copies by the blend over the corner circle and paints them scaled to the top', () => {
  const calls = [];
  const unit = { ...math, spin: { ...math.spin, radial: (...args) => (calls.push(args), math.spin.radial(...args)) } };
  const live = open({ show: 'radial', copies: 4, blend: 'sum' }, { unit });
  const { main } = live.at(0);
  expect(calls).toHaveLength(1);
  expect(calls[0].slice(1)).toEqual([9, OUT, 4, 0.25, 'Sum', SAMPLES]);
  const rose = images.at(-1);
  expect([rose.width, rose.height]).toEqual([OUT, OUT]);
  let top = 0;
  for (let i = 3; i < rose.data.length; i += 4) top = Math.max(top, rose.data[i]);
  expect(top).toBe(255);
  expect(draws(main)).toEqual([['drawImage', 'layer', 400 - 268, 300 - 268, 536, 536]]);
});

test('afterglow veils the platter with the paper at one over the glow and keeps the trail, a step back clears it', () => {
  const live = open({ glow: 4 });
  live.at(0);
  const { sheets: layers } = live.at(100);
  const table = layers.find((log) => log.some(([name]) => name === 'rotate'));
  expect(table).toContainEqual(['globalAlpha', 0.25]);
  expect(table).toContainEqual(['fillStyle', PAPER]);
  expect(table.some(([name]) => name === 'clearRect')).toBe(false);
  const back = live.at(50).sheets.find((log) => log.some(([name]) => name === 'rotate'));
  expect(back.some(([name]) => name === 'clearRect')).toBe(true);
  const still = open({ glow: 4 }, { still: true }).at(0).sheets.find((log) => log.some(([name]) => name === 'rotate'));
  expect(still.some(([name]) => name === 'clearRect')).toBe(true);
});

test('the pace reports the screen rate, the degrees a frame and what the eye sees twice a second', () => {
  const live = open({ rpm: 900 });
  for (let n = 1; n <= 60; n++) live.at((n * 1000) / 60);
  expect(live.paces).toHaveLength(2);
  expect(live.paces[0].hz).toBeCloseTo(60, 6);
  expect(live.paces[0].deg).toBeCloseTo(90, 6);
  expect(live.paces[0].seen).toBeCloseTo(0, 6);
  const slow = open({ rpm: 33 });
  for (let n = 1; n <= 30; n++) slow.at((n * 1000) / 60);
  expect(slow.paces).toHaveLength(1);
  expect(slow.paces[0].seen).toBeCloseTo(3.3, 6);
  const still = open({}, { still: true });
  still.at(0);
  still.at(0);
  expect(still.paces).toEqual([]);
});

test('a theme change recolours the layers and a size lays them again without a new raster', () => {
  const live = open({});
  live.at(0);
  const before = images.length;
  live.view.look = () => ({ paper: PAPER, accent: '#ff0000' });
  live.scene.theme();
  expect(images.length).toBe(before + 1);
  const wheel = images.at(-1);
  const lit = wheel.data.findIndex((_, i) => i % 4 === 3 && wheel.data[i] > 0);
  expect(Array.from(wheel.data.slice(lit - 3, lit))).toEqual(rgb('#ff0000'));
  live.view.w = 600;
  live.view.h = 800;
  live.scene.size();
  expect(images.length).toBe(before + 1);
  expect(draws(live.at(0).main)[0]).toEqual(['drawImage', 'layer', 300 - 176, 208 - 176]);
});

test('without the unit the scene is blank and throws nothing', () => {
  const live = open({}, { unit: null });
  expect(live.at(0).main).toEqual([['setTransform', 1, 0, 0, 1, 0, 0], ['clearRect', 0, 0, 800, 600]]);
  expect(live.scene.facts).toBe(undefined);
});
