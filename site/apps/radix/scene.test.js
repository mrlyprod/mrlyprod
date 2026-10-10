import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { hit } from '../../lib/keys.js';
import { defaults, describe } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { keymap, live } from '../../lib/scene.jsx';
import { FADE, GAP, HOLD, PAD, STEP, lattice, lay, study } from './engine.js';
import { PAGE, SPEC, make } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });
num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args.map((arg) => (arg && typeof arg === 'object' ? `page${arg.id}` : arg))]),
      set: (_, key, v) => {
        log.push([key, v]);
        return true;
      },
    },
  );

const pages = [];

globalThis.OffscreenCanvas = class {
  constructor(w, h) {
    this.width = w;
    this.height = h;
    this.id = pages.length;
    this.log = [];
    pages.push(this);
  }
  getContext() {
    return pen(this.log);
  }
};

const open = (value, still = false, w = 800, h = 600) => {
  const log = [];
  const told = [];
  const levels = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 2, t: 0 };
  view.wake = () => {
    view.still = false;
    told.push('wake');
  };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 1, code: '', shape: 'twindragon', level: 6, math, num, onLevel: (one) => levels.push(one), ...value });
  const quiet = log.length;
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, told, levels, log, quiet, at };
};

const laid = (page) => page.log.slice(page.log.findLastIndex(([name]) => name === 'clearRect')).filter(([name]) => name === 'rect' || name === 'moveTo').length;

const shown = (log) => log.filter(([name]) => name === 'drawImage').map(([, page]) => Number(page.slice(4)));

const alphas = (log) => log.filter(([name]) => name === 'globalAlpha').map(([, a]) => a);

test('make draws nothing, the first draw lays level one on a layer in the accent and fades it in', () => {
  pages.length = 0;
  const live = open({});
  expect(live.quiet).toBe(0);
  const log = live.at(FADE / 2);
  expect(pages).toHaveLength(1);
  expect(laid(pages[0])).toBe(2);
  expect(pages[0].log).toContainEqual(['fillStyle', '#008cff']);
  expect(shown(log)).toEqual([0]);
  expect(alphas(log)).toEqual([0.5, 1]);
});

test('a level fades over the one before it on two layers, then the earlier layer is let go', () => {
  pages.length = 0;
  const live = open({});
  live.at(STEP);
  const mid = live.at(STEP + FADE / 2);
  expect(pages).toHaveLength(2);
  expect(shown(mid).sort()).toEqual([0, 1]);
  expect(alphas(mid)).toEqual([0.75, 0.5, 1]);
  const done = live.at(STEP + FADE);
  expect(shown(done)).toHaveLength(1);
  expect(alphas(done)).toEqual([1, 1]);
  live.at(2 * STEP + FADE / 2);
  expect(pages).toHaveLength(2);
  expect([laid(pages[0]), laid(pages[1])].sort((a, b) => a - b)).toEqual([4, 8]);
});

test('a picture at a t is the same reached in one jump or by steps', () => {
  pages.length = 0;
  const stepped = open({});
  for (let t = 0; t < 3 * STEP + FADE / 3; t += 100) stepped.at(t);
  const last = [...stepped.at(3 * STEP + FADE / 3)];
  const layers = pages.map(laid).sort((a, b) => a - b);
  pages.length = 0;
  const jumped = open({});
  const once = [...jumped.at(3 * STEP + FADE / 3)];
  expect(alphas(once)).toEqual(alphas(last));
  expect(pages.map(laid).sort((a, b) => a - b)).toEqual(layers);
  expect(layers).toEqual([8, 16]);
});

test('a draw at the same t paints nothing more, a resize or a theme change repaints', () => {
  const live = open({});
  const t = 2 * STEP + FADE;
  live.at(t);
  expect(live.at(t)).toEqual([]);
  live.scene.size();
  expect(shown(live.at(t))).toHaveLength(1);
  live.scene.theme();
  expect(shown(live.at(t))).toHaveLength(1);
  expect(live.at(t)).toEqual([]);
});

test('under reduced motion the one draw lays the top level whole, fitted inside the padding', () => {
  pages.length = 0;
  const still = open({}, true);
  const log = still.at(0);
  expect(pages).toHaveLength(1);
  expect(laid(pages[0])).toBe(64);
  expect(alphas(log)).toEqual([1, 1]);
  const marks = pages[0].log.filter(([name]) => name === 'moveTo' || name === 'lineTo');
  const xs = marks.map(([, x]) => x);
  const ys = marks.map(([, , y]) => y);
  expect(Math.min(...xs)).toBeGreaterThanOrEqual(2 * PAD - 1);
  expect(Math.max(...xs)).toBeLessThanOrEqual(800 - 2 * PAD + 1);
  expect(Math.min(...ys)).toBeGreaterThanOrEqual(2 * PAD - 1);
  expect(Math.max(...ys)).toBeLessThanOrEqual(600 - 2 * PAD + 1);
  expect(still.at(0)).toEqual([]);
});

test('every level of every shape fits inside the padding, on a wide stage and a tall one, so no early cell is cut', () => {
  for (const [shape, code, w, h] of [['gasket', '', 1184, 756], ['gasket', '', 390, 700], ['twindragon', '', 390, 700], ['koch', '', 700, 400], ['tile', '495', 800, 600]]) {
    pages.length = 0;
    const live = open({ shape, code, level: 6 }, false, w, h);
    for (let level = 1; level <= live.scene.facts.level; level++) {
      const page = pages[shown(live.at((level - 1) * STEP + FADE)).at(-1)];
      const marks = page.log.slice(page.log.findLastIndex(([name]) => name === 'clearRect')).filter(([name]) => name === 'moveTo' || name === 'lineTo' || name === 'rect');
      const xs = marks.flatMap(([name, x, , width]) => (name === 'rect' ? [x, x + width] : [x]));
      const ys = marks.flatMap(([name, , y, , height]) => (name === 'rect' ? [y, y + height] : [y]));
      expect(marks.length).toBeGreaterThan(0);
      expect(Math.min(...xs)).toBeGreaterThanOrEqual(2 * PAD - 1);
      expect(Math.max(...xs)).toBeLessThanOrEqual(w - 2 * PAD + 1);
      expect(Math.min(...ys)).toBeGreaterThanOrEqual(2 * PAD - 1);
      expect(Math.max(...ys)).toBeLessThanOrEqual(h - 2 * PAD + 1);
    }
  }
});

test('past the hold the growth starts over, fading level one in over the top level', () => {
  const live = open({});
  const loop = 6 * STEP + HOLD;
  live.at(6 * STEP + FADE);
  expect(live.at(6 * STEP + HOLD / 2)).toEqual([]);
  const again = live.at(loop + FADE / 2);
  expect(shown(again)).toHaveLength(2);
  expect(alphas(again)).toEqual([0.75, 0.5, 1]);
  expect(live.levels.at(-1).level).toBe(1);
});

test('big cells are polygons with a gap and small ones are squares at least a device pixel wide', () => {
  pages.length = 0;
  const live = open({ level: 17 });
  live.at(FADE);
  expect(pages[0].log.filter(([name]) => name === 'moveTo')).toHaveLength(2);
  expect(pages[0].log.filter(([name]) => name === 'lineTo')).toHaveLength(6);
  live.at(16 * STEP + FADE);
  const tiny = pages.find((page) => page.log.some(([name]) => name === 'rect'));
  const rects = tiny.log.filter(([name]) => name === 'rect');
  expect(rects).toHaveLength(131072);
  expect(rects.every(([, , , w, h]) => w >= 2 && w === h)).toBe(true);
});

test('at an odd level the cells of the turned lattice are drawn wider than its pitch, so no holes show between them', () => {
  pages.length = 0;
  const live = open({ level: 17, grow: 0 }, true, 4000, 3000);
  live.at(0);
  const plan = study(num, math, { ...defaults(SPEC), seed: 1, code: '', shape: 'twindragon', level: 17 }, rng(1));
  const pitch = lattice(plan.norm, plan.sides, 17).spacing * lay(plan.stage, 4000, 3000, PAD * 2).k;
  const rects = pages[0].log.filter(([name]) => name === 'rect');
  expect(rects).toHaveLength(131072);
  expect(rects.every(([, , , w, h]) => w === h && Math.abs(w - pitch * Math.SQRT2) < 1e-6)).toBe(true);
  expect(pitch).toBeGreaterThan(2);
});

test('the scene tells each level once with its fill and its distinct count', () => {
  const live = open({ shape: 'terdragon', level: 4 });
  live.at(0);
  live.at(STEP);
  live.at(STEP + 1);
  live.at(3 * STEP);
  expect(live.levels).toEqual([{ level: 1, fill: 3, distinct: 3 }, { level: 2, fill: 9, distinct: 7 }, { level: 4, fill: 81, distinct: 43 }]);
});

test('again restarts the growth from now and wakes a still scene', () => {
  const live = open({});
  live.at(4 * STEP);
  expect(live.levels.at(-1).level).toBe(5);
  live.scene.again();
  expect(live.levels.at(-1).level).toBe(1);
  live.at(4 * STEP + STEP + FADE);
  expect(live.levels.at(-1).level).toBe(2);
  const still = open({}, true);
  still.at(0);
  still.scene.again();
  expect(still.told).toEqual(['wake']);
});

test('with grow off the scene rests at the top level, draws on its interval only when touched, and again does nothing', () => {
  const live = open({ grow: 0 });
  expect(live.scene.every).toBe(1000);
  expect(alphas(live.at(0))).toEqual([1, 1]);
  expect(live.at(5000)).toEqual([]);
  live.scene.again();
  expect(live.at(5001)).toEqual([]);
  live.scene.size();
  expect(live.log.filter(([name]) => name === 'drawImage')).toHaveLength(1);
});

test('with grow off the Again key leaves the map, so it is no primary and Enter does nothing', () => {
  const map = (grow) => keymap(live(PAGE.keys, { ...defaults(SPEC), grow }));
  expect([map(1).some((row) => row.button), map(0).some((row) => row.button)]).toEqual([true, false]);
  expect([hit(map(1), { key: 'Enter' })?.act, hit(map(0), { key: 'Enter' })]).toEqual(['again', null]);
});

test('a cell size pins the scale so a top cell is that many css pixels wide', () => {
  pages.length = 0;
  const live = open({ shape: 'tile', code: '495', level: 2, cell: 8 }, true);
  live.at(0);
  const marks = pages[0].log.filter(([name]) => name === 'moveTo' || name === 'lineTo');
  expect(marks.filter(([name]) => name === 'moveTo')).toHaveLength(64);
  const xs = marks.slice(0, 4).map(([, x]) => x);
  const ys = marks.slice(0, 4).map(([, , y]) => y);
  expect(Math.max(...xs) - Math.min(...xs)).toBeCloseTo(16 * (1 - GAP));
  expect(Math.max(...ys) - Math.min(...ys)).toBeCloseTo(16 * (1 - GAP));
});

test('the level knob hides for a random shape and the facts name the design', () => {
  const shown = (shape) => describe(SPEC, { ...defaults(SPEC), shape }).flatMap(({ rows }) => rows.map((row) => row.key));
  expect(shown('')).toEqual(['shape', 'grow', 'cell', 'tint']);
  expect(shown('koch')).toEqual(['shape', 'level', 'grow', 'cell', 'tint']);
  expect(open({}).scene.facts).toMatchObject({ kind: 'twindragon', name: 'Twindragon', level: 6, fill: 64 });
});
