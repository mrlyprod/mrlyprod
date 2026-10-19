import { expect, test } from 'bun:test';
import { palette } from '../../lib/theme/palette.gen.js';
import * as num from '@mrlyprod/mrly/num';
import { hit } from '../../lib/app/keys.jsx';
import { defaults } from '../../lib/app/spec.js';
import { rng } from '../../lib/rand.js';
import { keymap, live } from '../../lib/app/page.jsx';
import { GROW, PAD, lay, study } from './engine.js';
import { PAGE, SPEC, make, named, sketch } from './scene.js';
import { THUMB } from './engine.js';

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

const BASE = { ...defaults(SPEC), seed: 1, limit: 12 };

const open = (value, still = false, w = 800, h = 600) => {
  const log = [];
  const told = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#ff00ff' }), still, w, h, dpr: 2, t: 0 };
  view.wake = () => {
    view.still = false;
    told.push('wake');
  };
  const scene = make({ getContext: () => pen(log) }, view, { ...BASE, num, ...value });
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, told, log, at };
};

const PLAN = study(num, 'gaussian', 12);

const whole = PLAN.count - 1;

const laid = () => sheets.at(-1).log.filter(([name]) => name === 'rect' || name === 'arc').length;

const arcs = (log) => log.filter(([name]) => name === 'arc');

test('the window is still and opens whole: one draw lays every point, a draw after is a no-op, and a resize or a theme lays it again', () => {
  const live = open({});
  expect([live.scene.still, live.scene.every]).toEqual([true, undefined]);
  expect(live.at(0).length).toBeGreaterThan(0);
  expect(laid()).toBe(whole);
  expect(live.at(1000)).toEqual([]);
  const was = sheets.length;
  live.scene.size();
  live.at(2000);
  expect([sheets.length, laid()]).toEqual([was + 1, whole]);
  live.scene.theme();
  live.at(3000);
  expect([sheets.length, laid()]).toEqual([was + 2, whole]);
  expect(live.at(4000)).toEqual([]);
});

test('Grow paints the window again norm by norm from now, once, and wakes a still scene', () => {
  const live = open({});
  live.at(5000);
  live.scene.grow();
  live.at(5000 + GROW / 4);
  const early = laid();
  expect(early).toBeGreaterThan(0);
  expect(early).toBeLessThan(whole / 8);
  live.at(5000 + GROW);
  expect(laid()).toBe(whole);
  expect(live.at(5000 + 3 * GROW)).toEqual([]);
  const still = open({}, true);
  still.at(0);
  still.scene.grow();
  expect(still.told).toEqual(['wake']);
});

test('a growing picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({});
  stepped.scene.grow();
  for (let t = 0; t <= GROW / 2; t += 250) stepped.at(t);
  const count = laid();
  const log = [...stepped.log];
  const jumped = open({});
  jumped.scene.grow();
  const once = [...jumped.at(GROW / 2)];
  expect([laid(), once]).toEqual([count, log]);
  expect(count).toBeLessThan(whole);
});

test('a tap finds the nearest point of the window and nothing off it', () => {
  const live = open({});
  live.at(0);
  const box = lay(PLAN.frame, 800, 600, PAD * 2);
  expect(live.scene.at((box.ox + 2 * box.k) / 800, (box.oy - 3 * box.k) / 600)).toMatchObject({ a: 2, b: 3, norm: 13, fate: 1 });
  expect(live.scene.at(0.5, 0.5)).toMatchObject({ a: 0, b: 0, fate: 5 });
  expect(live.scene.at(0, 0)).toBe(null);
});

test('a clicked point is ringed with its associates and its dashed conjugate, alone without its units or at norm 1', () => {
  const box = lay(PLAN.frame, 800, 600, PAD * 2);
  const rings = (value) => arcs(open(value).at(0).filter((row, i, all) => all.findIndex(([name]) => name === 'drawImage') < i));
  const split = rings({ click: '2,3' });
  expect(split).toHaveLength(1 + 3 + 1);
  expect(split.at(-1).slice(1, 3)).toEqual([box.ox + 2 * box.k, box.oy - 3 * box.k]);
  expect(open({ click: '2,3' }).at(0)).toContainEqual(['setLineDash', [8, 6]]);
  expect([rings({ click: '2,3', units: 0 }).length, rings({ click: '0,1' }).length, rings({ click: '99,0' }).length, rings({}).length]).toEqual([1, 1, 0, 0]);
});

test('a set on the same scene re-cuts the window for a new reach, restyles for a new colour, and keeps a point the window still holds', () => {
  const live = open({ click: '2,3' });
  live.at(0);
  live.scene.set({ ...BASE, limit: 3, click: '2,3' });
  live.at(100);
  expect(laid()).toBe(study(num, 'gaussian', 3).count - 1);
  expect(arcs(live.log)).toHaveLength(5);
  live.scene.set({ ...BASE, limit: 1, click: '2,3' });
  expect(arcs(live.at(200))).toEqual([]);
  live.scene.set({ ...BASE, look: 'plain', click: '2,3' });
  live.at(300);
  expect(sheets.at(-1).log).toContainEqual(['fillStyle', '#ff00ff']);
  expect(sheets.at(-1).log).not.toContainEqual(['fillStyle', palette.blue]);
});

test('a set mid-growth to a new reach shows the whole new window at once', () => {
  const live = open({});
  live.scene.grow();
  live.at(GROW / 4);
  live.scene.set({ ...BASE, limit: 20 });
  live.at(GROW / 4 + 16);
  expect(laid()).toBe(study(num, 'gaussian', 20).count - 1);
});

test('the eisenstein ring lays circles and the gaussian ring rects, in the hue of each fate', () => {
  const hex = open({ ring: 'eisenstein' }, true);
  hex.at(0);
  const round = sheets.at(-1).log;
  expect([round.some(([name]) => name === 'arc'), round.some(([name]) => name === 'rect')]).toEqual([true, false]);
  const square = open({}, true);
  square.at(0);
  const flat = sheets.at(-1).log;
  for (const hue of [palette.blue, palette.orange, palette.pink, palette.green, 'rgba(255, 255, 255, 0.16)']) expect(flat).toContainEqual(['fillStyle', hue]);
  expect(flat.some(([name]) => name === 'rect')).toBe(true);
});

test('a pinned cell size sets the lattice spacing in device pixels', () => {
  const live = open({ cell: 4 }, true);
  live.at(0);
  const xs = sheets.at(-1).log.filter(([name]) => name === 'rect').map(([, x]) => x);
  expect(Math.max(...xs) - Math.min(...xs)).toBe(24 * 8);
});

test('a walk names the next prime from the clicked point and a turn its next associate', () => {
  expect([open({}).scene.walk(1), open({ click: '1,1' }).scene.walk(1), open({}).scene.walk(-1)]).toEqual(['1,1', '1,2', '12,7']);
  expect([open({ click: '2,3' }).scene.turn(1), open({ click: '2,3' }).scene.turn(-1), open({}).scene.turn(1)]).toEqual(['-3,2', '3,-2', null]);
});

test('the svg of the scene is written in the hues on screen', () => {
  const live = open({}, true);
  live.at(0);
  const text = live.scene.svg();
  expect(text).toContain(`fill="${palette.blue}"`);
  expect(text).toContain('fill="rgba(255, 255, 255, 0.16)"');
  expect(text.match(/<rect /g).length).toBeGreaterThanOrEqual(PLAN.facts.primes);
});

test('an export is named by the ring off the gaussian, the reach and a colour off fate', () => {
  expect(PAGE.name).toBe(named);
  expect([named(BASE), named({ ...BASE, ring: 'eisenstein', limit: 40, look: 'norm' })]).toEqual(['gaussian-12', 'gaussian-eisenstein-40-norm']);
});

test('Enter grows, the arrows walk, and the turn keys join the map once a point is clicked', () => {
  const map = (value) => keymap(live(PAGE.keys, value));
  expect([map({}).find((row) => row.button)?.act, hit(map({}), { key: 'Enter' })?.act, hit(map({}), { key: 'ArrowRight' })?.act]).toEqual(['grow', 'grow', 'walk']);
  expect([hit(map({ click: '' }), { key: 'ArrowUp' }), hit(map({ click: '2,3' }), { key: 'ArrowUp' })?.act]).toEqual([null, 'turn']);
});

test('Grow and the walk are also in the Edit menu', () => {
  expect(PAGE.keys.filter((row) => row.menu).map((row) => [row.act, row.menu])).toEqual([['grow', 'Edit'], ['walk', 'Edit']]);
});

test('a library thumb fills the window cut at the thumb reach, rects for the gaussian ring and circles for the eisenstein', () => {
  const thumb = (set) => {
    const log = [];
    sketch(num, set, pen(log), 112, 112, 1, { paper: '#000000', accent: '#ff00ff' });
    return log;
  };
  const square = thumb({ ring: 'gaussian', limit: 40, look: 'fate' });
  const round = thumb({ ring: 'eisenstein', limit: 12, look: 'norm' });
  const count = (log, name) => log.filter(([key]) => key === name).length;
  expect([count(square, 'rect'), count(square, 'arc')]).toEqual([study(num, 'gaussian', THUMB).count - 1, 0]);
  expect([count(round, 'rect'), count(round, 'arc') > 0]).toEqual([0, true]);
});
