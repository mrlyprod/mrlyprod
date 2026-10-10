import { expect, test } from 'bun:test';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults, describe, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { BANDS, BEAT, FLASH, GHOST, HOLD, KEEP, PIP, fold, study } from './engine.js';
import { SPEC, make } from './scene.js';

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
  const passes = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 2, t: 0 };
  view.wake = () => {
    view.still = false;
    told.push('wake');
  };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 1, from: 0.5, num, onPass: (count, t) => passes.push([count, t]), ...value });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, told, passes, log, at };
};

const PLAN = study(num, 0.5, 60);

const whole = PLAN.n - 1;

const laid = () => sheets.at(-1).log.filter(([name]) => name === 'lineTo' || name === 'arc').length;

const rings = (log, x, y) => log.filter(([name, cx, cy]) => name === 'arc' && cx === x && cy === y);

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

test('under reduced motion the first draw lays the whole walk, tells every zero once, and draws no flash', () => {
  const still = open({}, true);
  const log = still.at(0);
  expect(laid()).toBe(whole);
  expect(still.passes).toEqual([[13, PLAN.zeros[12]]]);
  expect(rings(log, 400 - 0, 0).length).toBe(0);
  still.at(0);
  expect(laid()).toBe(whole);
  expect(still.passes).toHaveLength(1);
});

test('a pass through the origin is told with its count and its t, and flashes a ring at the origin that fades within the flash', () => {
  const live = open({ speed: 2 });
  const first = PLAN.zeros[0];
  const walked = (t) => live.at((t - 0.5) * 500);
  const before = walked(first - 0.1);
  expect(live.passes).toEqual([[0, null]]);
  const origin = before.find(([name]) => name === 'arc').slice(1, 3);
  expect(rings(before, ...origin)).toHaveLength(1);
  const just = walked(first + 0.1);
  expect(live.passes).toEqual([[0, null], [1, first]]);
  expect(rings(just, ...origin)).toHaveLength(2);
  const later = live.at((first + 0.1 - 0.5) * 500 + FLASH + 1);
  expect(rings(later, ...origin)).toHaveLength(1);
  expect(live.passes).toHaveLength(2);
});

test('past the span and the hold the walk starts over on a fresh layer and the count resets', () => {
  const live = open({ speed: 2 });
  live.at(30000);
  expect(laid()).toBe(whole);
  expect(live.passes.at(-1)[0]).toBe(13);
  live.at(30000 + HOLD / 2);
  expect(laid()).toBe(whole);
  expect(live.at(30000 + HOLD / 2)).toContainEqual(['globalAlpha', 1]);
  live.at(30000 + HOLD + 50);
  expect(laid()).toBeLessThan(whole / 10);
  expect(live.passes.at(-1)[0]).toBe(0);
});

test('the dots look lays beads on the layer and on the trail, the line look strokes, and both strokes the trail with beads over it', () => {
  const dots = open({ look: 'dots' }, true);
  const trail = dots.at(0);
  expect(sheets.at(-1).log.some(([name]) => name === 'arc')).toBe(true);
  expect(sheets.at(-1).log.some(([name]) => name === 'lineTo')).toBe(false);
  expect(trail.filter(([name]) => name === 'lineTo')).toHaveLength(2);
  expect(trail.filter(([name]) => name === 'fill').length).toBeGreaterThan(2);
  const line = open({ look: 'line' }, true);
  const stroked = line.at(0);
  expect(sheets.at(-1).log.some(([name]) => name === 'lineTo')).toBe(true);
  expect(stroked.filter(([name]) => name === 'lineTo').length).toBeGreaterThan(2);
  expect(stroked.filter(([name]) => name === 'fill')).toHaveLength(1);
  const both = open({ look: 'both' }, true);
  const mixed = both.at(0);
  expect(sheets.at(-1).log.some(([name]) => name === 'lineTo')).toBe(true);
  expect(mixed.some(([name]) => name === 'lineTo')).toBe(true);
  expect(mixed.filter(([name]) => name === 'fill').length).toBeGreaterThan(2);
});

test('again restarts the walk from now and wakes a still scene', () => {
  const live = open({});
  live.at(20000);
  const before = laid();
  live.scene.again();
  live.at(20100);
  expect(laid()).toBeLessThan(before / 4);
  const still = open({}, true);
  still.at(0);
  still.scene.again();
  expect(still.told).toEqual(['wake']);
  still.at(10);
  expect(laid()).toBeLessThan(10);
});

test('a resize or a theme lays the ghost again in the paint of the moment', () => {
  const live = open({});
  live.at(5000);
  const was = sheets.length;
  live.scene.size();
  live.at(5000);
  expect(sheets.length).toBe(was + 1);
  live.scene.theme();
  live.at(5000);
  expect(sheets.length).toBe(was + 2);
  expect(sheets.at(-1).log).toContainEqual(['strokeStyle', '#008cff']);
});

test('the svg and the csv of the scene are the walk shown, in the accent', () => {
  const live = open({}, true);
  live.at(0);
  const text = live.scene.svg();
  expect(text).toContain('stroke="#008cff"');
  expect(text).toContain('width="400" height="300"');
  expect(live.scene.csv().split('\n')[1]).toBe('1,14.134725');
  expect(live.scene.facts).toMatchObject({ from: 0.5, to: 60.5, zeros: 13, first: 1 });
});

test('the trail is capped at the span and a from past the top is clamped, while the defaults are a fixed point of tidy', () => {
  const value = { ...defaults(SPEC), seed: 1 };
  expect({ ...value, ...tidy(SPEC, value) }).toEqual(value);
  expect(tidy(SPEC, { ...value, span: 20, trail: 50 }).trail).toBe(20);
  expect(tidy(SPEC, { ...value, from: 1e9 }).from).toBe(10000);
  expect(tidy(SPEC, { ...value, from: -5 }).from).toBe(0);
});

test('the ghost is laid in solid ink and blitted at the ghost alpha, so no seam or crossing darkens it', () => {
  const live = open({});
  const log = live.at(5000);
  const ghost = sheets.at(-1).log;
  expect(ghost.filter(([name]) => name === 'strokeStyle').every(([, color]) => color === '#008cff')).toBe(true);
  expect(ghost).not.toContainEqual(['strokeStyle', expect.stringContaining('rgba')]);
  const blit = log.findIndex(([name]) => name === 'drawImage');
  expect(log[blit - 1]).toEqual(['globalAlpha', GHOST]);
  const dots = open({ look: 'dots' });
  dots.at(5000);
  expect(sheets.at(-1).log.filter(([name]) => name === 'fillStyle').every(([, color]) => color === '#008cff')).toBe(true);
});

test('a bead at the joint of two bands is laid once and the trail strokes with flat caps', () => {
  const live = open({ look: 'both' }, true);
  const log = live.at(0);
  const radius = PIP * 2;
  const beads = log.filter(([name, , , r]) => name === 'arc' && r === radius).map(([, x, y]) => `${x},${y}`);
  expect(beads.length).toBeGreaterThan(BANDS);
  expect(new Set(beads).size).toBe(beads.length);
  expect(log.some(([name, cap]) => name === 'lineCap' && cap !== 'butt')).toBe(false);
});

test('a ring from the last zero of the walk rings out through the hold and is gone before the hold ends', () => {
  const length = (60 / 2) * 1000;
  const live = open({ speed: 2 });
  const origin = live.at(0).find(([name]) => name === 'arc').slice(1, 3);
  const rung = (play) => rings(live.at(play), ...origin).length - 1;
  const zero = ((PLAN.zeros.at(-1) - 0.5) / 2) * 1000;
  expect(zero + FLASH).toBeGreaterThan(length);
  expect(rung(length + 1)).toBe(1);
  expect(rung(zero + FLASH + 1)).toBe(0);
  expect(rung(length + HOLD / 2)).toBe(0);
});

test('a canvas smaller than its padding draws every arc with a radius of zero or more', () => {
  for (const [w, h] of [[1, 150], [300, 1], [1, 1]]) {
    for (const still of [false, true]) {
      const live = open({}, still, w, h);
      const log = [...live.at(0), ...live.at(5000)];
      const arcs = log.filter(([name]) => name === 'arc');
      expect(arcs.length).toBeGreaterThan(0);
      expect(arcs.every(([, x, y, r]) => Number.isFinite(x) && Number.isFinite(y) && r >= 0)).toBe(true);
    }
  }
});

test('a trail is capped to the point budget and the walk keeps it inside', () => {
  const value = { ...defaults(SPEC), seed: 1, from: 10000, span: 300 };
  const tight = tidy(SPEC, { ...value, trail: 300 });
  expect(tight.trail).toBeLessThan(300);
  const plan = study(num, 10000, 300);
  expect(Math.round(tight.trail * plan.per)).toBeLessThanOrEqual(KEEP);
});

test('the walk face shows the walk and look knobs and the stairs face shows the zeros and the reach', () => {
  const names = (value) => describe(SPEC, { ...defaults(SPEC), ...value }).map((group) => [group.name, group.rows.map((row) => row.key)]);
  expect(names({})).toEqual([['Face', ['face']], ['Walk', ['from', 'span', 'speed']], ['Look', ['look', 'trail', 'tint']]]);
  expect(names({ face: 'stairs' })).toEqual([['Face', ['face']], ['Stairs', ['zeros', 'x']], ['Look', ['tint']]]);
});

const climb = (value, still = false) => open({ face: 'stairs', ...value }, still);

test('the stairs fold in one zero a beat, tell each with its t, hold at the full formula and start over', () => {
  const live = climb({ zeros: 10, x: 100 });
  const plan = fold(num, 100, 10);
  live.at(0);
  expect(live.passes).toEqual([[0, null]]);
  live.at(BEAT * 3 + 10);
  expect(live.passes.at(-1)).toEqual([3, plan.gammas[2]]);
  live.at(BEAT * 11 + 10);
  expect(live.passes.at(-1)).toEqual([10, plan.gammas[9]]);
  live.at(BEAT * 11 + HOLD / 2);
  expect(live.passes.at(-1)[0]).toBe(10);
  live.at(BEAT * 11 + HOLD + 10);
  expect(live.passes.at(-1)[0]).toBe(0);
  expect(live.scene.facts).toMatchObject({ face: 'stairs', zeros: 10, x: 100 });
});

test('the stairs draw the filled steps, the dashed guess and the formula, and a still shows every zero folded', () => {
  const live = climb({ zeros: 10 }, true);
  const log = live.at(0);
  expect(live.passes).toEqual([[10, fold(num, 100, 10).gammas[9]]]);
  expect(log.filter(([name]) => name === 'stroke')).toHaveLength(4);
  expect(log.filter(([name]) => name === 'fill')).toHaveLength(1);
  expect(log).toContainEqual(['setLineDash', [8, 10]]);
  expect(log.filter(([name]) => name === 'arc')).toHaveLength(0);
  expect(live.scene.svg()).toContain('<clipPath');
  expect(live.scene.csv().split('\n')).toHaveLength(12);
  expect(live.scene.csv().split('\n')[1]).toBe('1,14.134725');
});

test('again restarts the stairs from the smooth guess and wakes a still scene', () => {
  const live = climb({ zeros: 10 });
  live.at(BEAT * 20);
  live.scene.again();
  expect(live.passes.at(-1)[0]).toBe(0);
  const still = climb({}, true);
  still.at(0);
  still.scene.again();
  expect(still.told).toEqual(['wake']);
});
