import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { defaults } from '../../lib/app/spec.js';
import { rgb } from '../../lib/color.js';
import { rng } from '../../lib/rand.js';
import { WHEEL } from '../../lib/spin/engine.js';
import { PAD, SOURCE } from '../../lib/spin/scene.js';
import { PAGE, SPEC, make, units } from './scene.js';

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

const open = (value, { still = false, w = 800, h = 600, unit = math } = {}) => {
  const log = [];
  const paces = [];
  const facts = [];
  const view = { rand: rng(1), look: () => ({ paper: PAPER, accent: ACCENT }), still, w, h, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SOURCE), ...defaults(SPEC), seed: 1, level: 2, math: unit, onPace: (p) => paces.push(p), onFacts: (f) => facts.push(f), ...value });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    for (const sheet of sheets) sheet.log.length = 0;
    scene.draw();
    return { main: [...log], sheets: sheets.map((sheet) => [...sheet.log]) };
  };
  return { view, scene, log, paces, facts, at };
};

const draws = (log) => log.filter(([name]) => name === 'drawImage');

const reach = (w, h) => Math.min(w, h) / 2 - PAD * 2;

const turned = (frame) => frame.sheets.flat().find(([name]) => name === 'rotate')?.[1];

test('the turntable alone fills the stage at any aspect with no text on it, and a still turns nothing', () => {
  const still = open({}, { still: true });
  const frame = still.at(0);
  expect(draws(frame.main)).toEqual([['drawImage', 'layer', 400 - reach(800, 600), 300 - reach(800, 600)]]);
  expect(frame.main.some(([name]) => name === 'fillText')).toBe(false);
  expect(turned(frame)).toBe(0);
  expect(draws(open({}, { w: 780, h: 1688 }).at(0).main)).toEqual([['drawImage', 'layer', 390 - reach(780, 1688), 844 - reach(780, 1688)]]);
  expect(still.facts.at(-1)).toMatchObject({ code: '495', name: 'carpet', side: 9, fills: 64, order: 4, peak: 1 });
});

test('the wheel view swaps in the exact wheel on the whole stage, drawn once, the pace quiet', () => {
  const live = open({ rpm: 900 });
  for (let n = 1; n <= 60; n++) live.at((n * 1000) / 60);
  live.scene.set({ view: 'wheel' });
  const frame = live.at(1100);
  expect(draws(frame.main)).toEqual([['drawImage', 'layer', 400 - reach(800, 600), 300 - reach(800, 600), 2 * reach(800, 600), 2 * reach(800, 600)]]);
  expect(turned(frame)).toBe(undefined);
  expect(images.at(-1).width).toBe(WHEEL);
  expect(live.paces.at(-1)).toBe(null);
  expect(live.at(1200).main).toEqual([]);
});

test('a live rpm change keeps the angle the turntable reached and turns on from there', () => {
  const live = open({ rpm: 60 });
  for (let t = 0; t <= 500; t += 100) live.at(t);
  live.scene.set({ rpm: 120 });
  expect(turned(live.at(600)) * (180 / Math.PI)).toBeCloseTo(180 + 72, 6);
});

test('the scene owns the pause: stopped, the platter holds its angle and the pace goes quiet; rpm 0 spins up live', () => {
  const live = open({ rpm: 900 });
  for (let n = 1; n <= 60; n++) live.at((n * 1000) / 60);
  const held = turned(live.at(1000));
  live.scene.toggle();
  expect(live.scene.playing()).toBe(false);
  expect(turned(live.at(1100))).toBe(held);
  expect(live.paces.at(-1)).toBe(null);
  live.scene.toggle();
  expect([live.scene.playing(), turned(live.at(1200)) === held]).toEqual([true, false]);
  const idle = open({ rpm: 0 });
  idle.at(0);
  idle.scene.set({ rpm: 33 });
  idle.at(100);
  expect(turned(idle.at(200))).toBeGreaterThan(0);
});

test('a design change reads the new source and tells its facts, a look change only repaints', () => {
  const calls = [];
  const unit = { ...math, spin: { ...math.spin, profile: (...args) => (calls.push('profile'), math.spin.profile(...args)) } };
  const live = open({ level: 1 }, { unit });
  live.at(0);
  const before = images.length;
  live.scene.set({ ramp: 'fire' });
  live.at(16);
  expect([calls.length, images.length > before]).toEqual([1, true]);
  live.scene.set({ level: 2, code: '1' });
  expect([calls.length, live.facts.at(-1).code, live.facts.at(-1).fills]).toEqual([2, '1', 1]);
});

test('the pace reports the screen rate, the degrees a frame and what the eye sees twice a second', () => {
  const live = open({ rpm: 900 });
  for (let n = 1; n <= 60; n++) live.at((n * 1000) / 60);
  expect(live.paces).toHaveLength(2);
  expect([live.paces[0].hz, live.paces[0].deg, live.paces[0].seen].map((x) => Math.round(x * 1000) / 1000)).toEqual([60, 90, 0]);
  const still = open({}, { still: true });
  still.at(0);
  still.at(0);
  expect(still.paces).toEqual([]);
});

test('afterglow veils the platter with the paper at one over the glow and keeps the trail, a step back clears it', () => {
  const live = open({ glow: 4 });
  live.at(0);
  const table = (frame) => frame.sheets.find((log) => log.some(([name]) => name === 'rotate'));
  const on = table(live.at(100));
  expect(on).toContainEqual(['globalAlpha', 0.25]);
  expect(on).toContainEqual(['fillStyle', PAPER]);
  expect(on.some(([name]) => name === 'clearRect')).toBe(false);
  expect(table(live.at(50)).some(([name]) => name === 'clearRect')).toBe(true);
});

test('a theme change recolours the wheel and a size lays the stage again without a new wheel', () => {
  const live = open({ ramp: 'tint', levels: 2, view: 'wheel' });
  live.at(0);
  live.view.look = () => ({ paper: PAPER, accent: '#ff0000' });
  live.scene.theme();
  live.at(16);
  const wheel = images.filter((one) => one.width === WHEEL).at(-1);
  const lit = wheel.data.findIndex((v, i) => i % 4 === 0 && v > 0);
  expect(Array.from(wheel.data.slice(lit, lit + 3))).toEqual(rgb('#ff0000'));
  const before = images.filter((one) => one.width === WHEEL).length;
  live.view.w = 600;
  live.view.h = 800;
  live.scene.size();
  expect(draws(live.at(32).main)).toEqual([['drawImage', 'layer', 300 - reach(600, 800), 400 - reach(600, 800), 2 * reach(600, 800), 2 * reach(600, 800)]]);
  expect(images.filter((one) => one.width === WHEEL).length).toBe(before);
});

test('Enter is the primary, Stop or Spin as the scene plays, and 1 to 5 set the needles, on the turntable only', () => {
  const got = [];
  const hands = { needle: (rpm) => got.push(rpm), toggle: () => got.push('toggle') };
  PAGE.actions.needle(hands, { key: '5' });
  PAGE.actions.needle(hands, { key: '1' });
  PAGE.actions.spin(hands);
  expect(got).toEqual([900, 33, 'toggle']);
  const enter = PAGE.keys.find((row) => row.key === 'Enter');
  expect([enter.button, enter.menu, enter.label({}, { playing: () => true }), enter.label({}, { playing: () => false })]).toEqual([true, 'Edit', 'Stop', 'Spin']);
  expect(PAGE.keys.map((row) => [row.when({ view: 'table' }), row.when({ view: 'wheel' })])).toEqual([[true, false], [true, false], [true, false]]);
});

test('the view leads the spec as a visible row, so a phone gets the switch, and the speed rows show on the turntable only', () => {
  expect([SPEC[0].key, SPEC[0].when, SPEC[0].group]).toEqual(['view', undefined, undefined]);
  expect(SPEC.filter((row) => row.when && !row.when({ view: 'wheel' })).map((row) => row.key)).toEqual(['rpm', 'glow']);
});

test('the ramp rows show only where the ramp paints: the wheel, or a moire on the turntable', () => {
  const shown = (value) => SPEC.filter((row) => ['ramp', 'levels', 'invert', 'tint'].includes(row.key) && (!row.when || row.when(value))).map((row) => row.key);
  expect([shown({ view: 'table', source: 'flat' }), shown({ view: 'table', source: 'slice' })]).toEqual([['tint'], ['tint']]);
  expect([shown({ view: 'table', source: 'moire' }), shown({ view: 'wheel', source: 'flat' })]).toEqual([['ramp', 'levels', 'invert', 'tint'], ['ramp', 'levels', 'invert', 'tint']]);
});

test('an export is named by the source, its design and the view, the home design settled by the unit', () => {
  const base = defaults(SOURCE);
  const named = (patch, units) => PAGE.name({ ...base, view: 'table', ...patch }, units);
  expect([named({}, { math }), named({}), named({ source: 'moire', view: 'wheel' }), named({ dim: 3, code: '7' })]).toEqual(['spin-flat-495-table', 'spin-flat-carpet-table', 'spin-moire-weave-wheel', 'spin-slice-7-table']);
});

test('the page takes a design of either dimension and the empty code opens the carpet, never a roll', () => {
  const { when, ...slot } = PAGE.design;
  expect(slot).toEqual({ dim: 'dim', base: 'base', code: 'code' });
  expect([when({ source: 'flat' }), when({ source: 'moire' })]).toEqual([true, false]);
  expect(PAGE.units).toBe(units);
  const home = open({ seed: 1 });
  const other = open({ seed: 9 });
  expect([home.facts.at(-1).code, other.facts.at(-1).code, open({ dim: 3, base: 2 }).facts.at(-1).code]).toEqual(['495', '495', '23']);
});

test('without the unit the scene is blank and throws nothing', () => {
  expect(open({}, { unit: null }).at(0).main).toEqual([['setTransform', 1, 0, 0, 1, 0, 0], ['clearRect', 0, 0, 800, 600]]);
});
