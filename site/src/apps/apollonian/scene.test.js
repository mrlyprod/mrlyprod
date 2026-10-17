import { expect, test } from 'bun:test';
import * as num from '@mrlyprod/mrly/num';
import { defaults } from '../../lib/app/spec.js';
import { rng } from '../../lib/rand.js';
import { HOLD, count, hit, layout, roll, study } from './engine.js';
import { FACE, SPEC, fg, file, make } from './scene.js';

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

const PAPER = '#000000';
const ACCENT = '#008cff';

const open = (value, { still = false, lock = false, w = 800, h = 600 } = {}) => {
  const log = [];
  const told = [];
  const opts = { ...defaults(SPEC), seed: 1, num, onTell: (facts) => told.push(facts), ...value };
  const view = { rand: rng(opts.seed), look: () => ({ paper: PAPER, accent: ACCENT }), still, lock, w, h, dpr: 2, t: 0 };
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
const BOX = layout(PLAN.frame, 800, 600, 48);
const GAP = (() => {
  for (let y = 0.05; y < 1; y += 0.01) for (let x = 0.05; x < 1; x += 0.01) if (hit(PLAN.data, PLAN.total, x, y) < 0) return [x, y];
})();
const spot = (x, y) => [(BOX.ox + x * BOX.k) / 800, (BOX.oy - y * BOX.k) / 600];

const named = (log, name) => log.filter(([one]) => one === name);

const arcs = (log) => named(log, 'arc').length;

const laid = () => arcs(sheets.at(-1).log);

test('make draws nothing, and the page opens on the strip to 2048, the whole packing laid at once in one period square at the centre of the stage, the two lines alone beside it', () => {
  const live = open({});
  expect(live.log).toEqual([]);
  const log = live.at(0);
  expect(laid()).toBe(PLAN.total);
  expect(named(sheets.at(-1).log, 'clip')).toHaveLength(1);
  expect(named(sheets.at(-1).log, 'rect')[0]).toEqual(['rect', 148, 48, 504, 504]);
  expect(named(sheets.at(-1).log, 'lineTo')).toHaveLength(0);
  expect([named(log, 'lineTo').length, named(log, 'fillText').length]).toEqual([2, 0]);
});

test('the page is a still scene and the lock is not', () => {
  expect([open({}).scene.still, open({}, { lock: true }).scene.still]).toEqual([true, false]);
});

test('an export is named by the root and the cap', () => {
  expect([file({ root: 'strip', cap: 2048 }), file({ root: '-1,2,2,3', cap: 32 })]).toEqual(['apollonian-strip-2048', 'apollonian-bounded-1-2-2-3-32']);
});

test('a draw with nothing new paints nothing, so a still page costs no frame', () => {
  const live = open({});
  live.at(0);
  expect(live.at(16)).toEqual([]);
  expect(live.at(SPAN + HOLD + 100)).toEqual([]);
});

test('in the lock the packing grows an octave a second from the first grown circles, holds, then starts over', () => {
  const lock = open({}, { lock: true });
  lock.at(0);
  expect(laid()).toBe(count(PLAN.data, PLAN.roots, PLAN.total, PLAN.k0));
  lock.at(SPAN / 2);
  expect(laid()).toBe(count(PLAN.data, PLAN.roots, PLAN.total, 128));
  lock.at(SPAN);
  expect(laid()).toBe(PLAN.total);
  expect(lock.at(SPAN + HOLD / 2)).toEqual([]);
  lock.at(SPAN + HOLD + 100);
  expect(laid()).toBeLessThan(PLAN.total / 100);
});

test('in the lock a picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({}, { lock: true });
  for (let t = 0; t <= 5000; t += 250) stepped.at(t);
  const lines = laid();
  const jumped = open({}, { lock: true });
  const log = [...jumped.at(5000)];
  expect(laid()).toBe(lines);
  expect(log.filter(([name]) => name !== 'clearRect')).toEqual(open({}, { lock: true }).at(5000).filter(([name]) => name !== 'clearRect'));
});

test('under reduced motion the lock lays the whole packing in one draw', () => {
  open({}, { lock: true, still: true }).at(0);
  expect(laid()).toBe(PLAN.total);
});

test('a value with no root, as the lock runs one, rolls its whole picture from the seed', () => {
  const told = [];
  for (const seed of [3, 4, 5]) {
    const view = { rand: rng(seed), look: () => ({ paper: PAPER, accent: ACCENT }), still: true, w: 400, h: 300, dpr: 1, t: 0 };
    make({ getContext: () => pen([]) }, view, { seed, num, onTell: (facts) => told.push(facts) });
    const want = roll(rng(seed));
    expect([told.at(-1).name, told.at(-1).cap]).toEqual([want.root, want.cap]);
  }
});

test('a bounded root draws its hull under the layer, unclipped and with no lines', () => {
  const live = open({ root: '-1,2,2,3', cap: 32, look: 'rings' });
  const log = live.at(0);
  const plan = study(num, '-1,2,2,3', 32);
  expect(laid()).toBe(plan.total);
  expect(named(log, 'arc')[0].slice(1, 4)).toEqual([400, 300, 252]);
  expect(log.findIndex(([name]) => name === 'arc')).toBeLessThan(log.findIndex(([name]) => name === 'drawImage'));
  expect([named(log, 'fill').length, named(sheets.at(-1).log, 'clip').length, named(log, 'lineTo').length]).toEqual([0, 0, 0]);
  expect(live.told.at(-1).shadow).toBeNull();
});

test('a stage thinner than its padding still draws every arc at a positive radius', () => {
  const live = open({ root: '-1,2,2,3', cap: 32 }, { w: 1, h: 1 });
  const radii = [...live.at(0), ...sheets.at(-1).log].filter(([name]) => name === 'arc').map(([, , , r]) => r);
  expect(radii.length).toBeGreaterThan(0);
  expect(radii.every((r) => r > 0)).toBe(true);
});

test('the inks: six hues by octave with the roots in the foreground, octave shades of the accent, the Ford circles apart, or one ink', () => {
  const styles = (ink) => {
    open({ ink, look: 'rings' }).at(0);
    return new Set(named(sheets.at(-1).log, 'strokeStyle').map(([, v]) => v));
  };
  expect(styles('size')).toEqual(new Set(['#ffffff', '#008cff', '#00cad8', '#32cc58', '#ffd100', '#ff8f2c', '#ff325a']));
  expect(styles('shade').size).toBe(PLAN.octaves + 1);
  expect(styles('ford')).toEqual(new Set(['#ffffff', '#008cff', 'rgb(115, 115, 115)']));
  expect(styles('one')).toEqual(new Set(['#ffffff']));
  expect([fg('#000000'), fg('#ffffff'), fg('rgb(20, 20, 20)')]).toEqual(['#ffffff', '#000000', '#ffffff']);
});

test('the discs: a wash fills each under its own stroke, solid fills it with a paper edge, rings only strokes', () => {
  const pens = (look) => {
    open({ look, ink: 'one' }).at(0);
    const log = sheets.at(-1).log;
    return [named(log, 'fill').length > 0, [...new Set(named(log, 'fillStyle').map(([, v]) => v))], [...new Set(named(log, 'strokeStyle').map(([, v]) => v))]];
  };
  expect(pens('wash')).toEqual([true, ['rgba(255, 255, 255, 0.15)'], ['#ffffff']]);
  expect(pens('solid')).toEqual([true, ['rgb(115, 115, 115)', '#ffffff'], [PAPER]]);
  expect(pens('rings')).toEqual([false, [], ['#ffffff']]);
});

test('the curvatures toggle writes each circle\'s k on the ones big enough, in the site face', () => {
  const live = open({ labels: 1 });
  live.at(0);
  const texts = named(sheets.at(-1).log, 'fillText');
  expect(texts.length).toBeGreaterThan(4);
  expect(texts.length).toBeLessThan(PLAN.total);
  expect(texts.slice(0, 3).map(([, text]) => text)).toEqual(['2', '2', '8']);
  expect(sheets.at(-1).log.some(([name, v]) => name === 'font' && String(v).includes(FACE))).toBe(true);
  expect(live.scene.font).toBe(FACE);
  open({}).at(0);
  expect(named(sheets.at(-1).log, 'fillText')).toHaveLength(0);
});

test('a tap reads the circle under it through the crate, a second tap lets go, a gap or the outside of the period reads nothing', () => {
  const live = open({});
  live.at(0);
  expect(live.scene.tap(...spot(0.5, 0.125))).toEqual({ k: 8, x: 4, y: 1, line: true, ford: true, num: 1, den: 2 });
  expect(live.told.at(-1).circle).toMatchObject({ k: 8, num: 1, den: 2 });
  expect(live.log.filter(([name, v]) => name === 'lineWidth' && (v === 8 || v === 4))).toHaveLength(2);
  expect(live.scene.tap(...spot(0.5, 0.125))).toBeNull();
  expect(live.told.at(-1).circle).toBeNull();
  expect(live.scene.tap(...spot(...GAP))).toBeNull();
  expect(live.scene.tap(...spot(0.5, 1.2))).toBeNull();
  expect(live.scene.tap(...spot(-0.2, 0.5))).toBeNull();
  expect(live.scene.tap(...spot(0.05, 0.5))).toEqual({ k: 2, x: 0, y: 1, line: true, ford: true, num: 0, den: 1 });
  const round = open({ root: '-1,2,2,3', cap: 32 });
  round.at(0);
  const box = layout([-1, -1, 1, 1], 800, 600, 48);
  expect(round.scene.tap(0.5, (box.oy - (2 / 3) * box.k) / 600)).toEqual({ k: 3, x: 0, y: 2, line: false, ford: false, num: undefined, den: undefined });
});

test('a set keeps the canvas and the grown plan: a look lays the layer again and keeps the tapped circle, a new root drops it', () => {
  const live = open({});
  live.at(0);
  const first = sheets.length;
  live.scene.tap(...spot(0.5, 0.125));
  live.scene.set({ ...defaults(SPEC), seed: 2 });
  live.at(16);
  expect(sheets.length).toBe(first);
  expect(live.told.at(-1)).toMatchObject({ name: 'strip', cap: 2048, shadow: { order: 32 }, circle: { k: 8 } });
  live.scene.set({ ...defaults(SPEC), seed: 2, look: 'rings' });
  live.at(32);
  expect(sheets.length).toBe(first + 1);
  expect(live.told.at(-1).circle).toMatchObject({ k: 8 });
  live.scene.set({ ...defaults(SPEC), seed: 2, root: '-2,3,6,7', cap: 64 });
  live.at(48);
  expect(laid()).toBe(study(num, '-2,3,6,7', 64).total);
  expect(live.told.at(-1)).toMatchObject({ name: '-2,3,6,7', cap: 64, line: null, shadow: null, circle: null });
});

test('a resize or a theme change lays the layer again', () => {
  const live = open({});
  live.at(0);
  const n = sheets.length;
  live.scene.size();
  live.at(16);
  live.scene.theme();
  live.at(32);
  expect([sheets.length, laid()]).toEqual([n + 2, PLAN.total]);
});

test('the svg is a sheet of css pixels: the period clipped, a circle per drawn disc and the two lines of the strip', () => {
  const live = open({ look: 'rings', ink: 'one' });
  live.at(0);
  const text = live.scene.svg();
  expect(text).toContain('width="400" height="300" viewBox="0 0 400 300"');
  expect(text).toContain('<clipPath id="period">');
  expect(text.match(/<circle /g)).toHaveLength(PLAN.total);
  expect(text.match(/<line /g)).toHaveLength(2);
  expect(text).not.toContain('<text');
  expect(text).toContain('stroke="#ffffff"');
  const round = open({ root: '-1,2,2,3', cap: 32, labels: 1 });
  round.at(0);
  const sheet = round.scene.svg();
  expect(sheet).not.toContain('clipPath');
  expect(sheet.match(/<text /g).length).toBeGreaterThan(3);
});

test('the json carries the root quadruple and every grown circle as an integer triple', () => {
  const live = open({ cap: 64 });
  live.at(0);
  const rows = JSON.parse(live.scene.json());
  expect([rows.root, rows.cap, rows.quadruple, rows.circles.length]).toEqual(['strip', 64, [[0, 0, -1], [0, 0, 1], [2, 0, 1], [2, 2, 1]], 26]);
  expect(rows.circles.slice(0, 2)).toEqual([[8, 4, 1], [8, 4, 7]]);
});
