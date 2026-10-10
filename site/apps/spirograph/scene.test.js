import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, describe } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { HOLD, PAD, RISE, fit, span, study } from './engine.js';
import { SPEC, make } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

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
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 2, t: 0 };
  view.wake = () => {
    view.still = false;
    told.push('wake');
  };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 1, code: '495', math: math, ...value });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, told, log, at };
};

const PLAN = study(math, { ...defaults(SPEC), seed: 1, code: '495' }, rng(1));
const RAIL = PLAN.track.outline.length - 1;
const LENGTH = span(PLAN.beats);

const drawn = () => sheets.at(-1).log.filter(([name]) => name === 'lineTo').length - RAIL;

const fills = (log) => log.filter(([name, , , , w]) => name === 'drawImage' && w !== undefined);

test('a picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({});
  for (let t = 0; t <= 7000; t += 250) stepped.at(t);
  const stepLines = drawn();
  const stepLog = [...stepped.log];
  const jumped = open({});
  const jumpLog = [...jumped.at(7000)];
  expect(drawn()).toBe(stepLines);
  expect(jumpLog).toEqual(stepLog);
  expect(stepLines).toBeGreaterThan(0);
});

test('under reduced motion the first draw lays the whole trace, the shape under it and the wheel at the end', () => {
  const still = open({}, true);
  const log = still.at(0);
  const whole = PLAN.facts.pencils * (PLAN.samples - 1);
  expect(drawn()).toBe(whole);
  const box = fit(PLAN.frame, 800, 600, PAD * 2);
  const [cx, cy] = math.spirograph.pose(PLAN.track, PLAN.track.total);
  const [, x, y, r] = log.find(([name]) => name === 'arc');
  expect([x, y, r]).toEqual([box.ox + cx * box.k, box.oy - cy * box.k, PLAN.track.wheel * box.k]);
  expect(fills(log)).toHaveLength(1);
  still.at(0);
  expect(drawn()).toBe(whole);
});

test('past the span and the hold the trace starts over on a fresh layer', () => {
  const live = open({});
  live.at(LENGTH);
  const full = drawn();
  live.at(LENGTH + HOLD / 2);
  expect(drawn()).toBe(full);
  live.at(LENGTH + HOLD + 100);
  expect(drawn()).toBeLessThan(full / 10);
});

test('once the trace closes the shape between the walls fades in under it over the disc', () => {
  const live = open({});
  expect(fills(live.at(LENGTH / 2))).toEqual([]);
  const log = live.at(LENGTH + RISE / 2);
  const [fill] = fills(log);
  const box = fit(PLAN.frame, 800, 600, PAD * 2);
  const { x, y, radius } = PLAN.shape.disc;
  expect(fill.slice(2)).toEqual([box.ox + (x - radius) * box.k, box.oy - (y + radius) * box.k, 2 * radius * box.k, 2 * radius * box.k]);
  expect(log[log.indexOf(fill) - 1]).toEqual(['globalAlpha', 0.5]);
  expect(log.findIndex(([name]) => name === 'drawImage')).toBe(log.indexOf(fill));
  expect(fills(live.at(LENGTH + HOLD - 1))[0]).toBeDefined();
  expect(fills(open({ kind: 'line' }, true).at(0))).toEqual([]);
});

test('again restarts the drawing from now and wakes a still scene', () => {
  const live = open({});
  live.at(4000);
  const before = drawn();
  live.scene.again();
  live.at(4100);
  expect(drawn()).toBeLessThan(before / 4);
  const still = open({}, true);
  still.at(0);
  still.scene.again();
  expect(still.told).toEqual(['wake']);
  still.at(10);
  expect(drawn()).toBeLessThan(100);
});

test('a resize or a theme change lays the layer again at the same share', () => {
  const live = open({});
  live.at(3000);
  const count = drawn();
  live.scene.size();
  live.at(3000);
  expect(drawn()).toBe(count);
  live.scene.theme();
  live.at(3000);
  expect(drawn()).toBe(count);
});

test('a stage narrower than the padding still lays every sheet at a whole positive size', () => {
  const before = sheets.length;
  const tiny = open({}, false, 30, 20);
  tiny.at(1000);
  tiny.scene.size();
  tiny.at(2000);
  const laid = sheets.slice(before);
  expect(laid.length).toBeGreaterThan(2);
  expect(laid.every(({ width, height }) => Number.isInteger(width) && Number.isInteger(height) && width > 0 && height > 0)).toBe(true);
});

test('the svg is a sheet of css pixels holding the track and one path per pencil in the shade of its band', () => {
  const live = open({});
  live.at(1000);
  const text = live.scene.svg();
  expect(text).toContain('width="400" height="300" viewBox="0 0 400 300"');
  expect(text.match(/<path /g)).toHaveLength(1 + live.scene.facts.pencils);
  expect(text).toContain('stroke-dasharray="4 6"');
  expect(text).toContain('stroke="rgb(0, 140, 255)"');
});

test('sides show on polygon tracks and laps on every track but a circle', () => {
  const shown = (kind) => describe(SPEC, { kind }).flatMap(({ rows }) => rows.map((row) => row.key)).filter((key) => key === 'sides' || key === 'laps');
  expect(['in', 'out', 'line', 'polyin', 'polyout'].map(shown)).toEqual([[], [], ['laps'], ['sides', 'laps'], ['sides', 'laps']]);
});
