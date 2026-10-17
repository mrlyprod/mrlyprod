import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import * as num from '@mrlyprod/mrly/num';
import { hit } from '../../lib/app/keys.jsx';
import { defaults, describe, tidy } from '../../lib/app/spec.js';
import { keymap, live } from '../../lib/app/page.jsx';
import { GAP, PAD, lattice, lay, preset, study } from './engine.js';
import { PAGE, SPEC, file, make } from './scene.js';

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

const base = (over) => ({ ...defaults(SPEC), seed: 1, ...over });

const named = (name, over) => base({ ...preset(num, math, name), ...over });

const open = (value = base(), still = false, w = 800, h = 600) => {
  pages.length = 0;
  const log = [];
  const told = [];
  const view = { look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 2, t: 0 };
  const scene = make({ id: 'screen', getContext: () => pen(log) }, view, { ...value, math, num, onFacts: (one) => told.push(one) });
  const quiet = log.length;
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return [...log];
  };
  const set = (patch) => scene.set({ ...value, ...patch });
  return { view, scene, told, log, quiet, at, set };
};

const laid = (page) => page.log.slice(page.log.findLastIndex(([name]) => name === 'clearRect'));

const cells = (page) => laid(page).filter(([name]) => name === 'rect' || name === 'moveTo').length;

const shown = (log) => log.filter(([name]) => name === 'drawImage').map(([, page]) => page);

const alphas = (log) => log.filter(([name]) => name === 'globalAlpha').map(([, a]) => a);

test('make draws nothing and tells the facts of its level once', () => {
  const live = open();
  expect(live.quiet).toBe(0);
  expect(live.told).toHaveLength(1);
  expect(live.told[0]).toMatchObject({ level: 5, fill: 1024, distinct: 1024, cap: 8 });
});

test('the first draw lays the level on a layer in the accent and shows it whole', () => {
  const live = open(base({ line: 0, level: 2 }));
  const log = live.at(0);
  expect(pages).toHaveLength(1);
  expect(cells(pages[0])).toBe(16);
  expect(pages[0].log).toContainEqual(['fillStyle', '#008cff']);
  expect(shown(log)).toEqual(['page0']);
});

test('a line traces every word in order with one stroke, fitted to the words and not their cells', () => {
  const live = open();
  live.at(0);
  const marks = laid(pages[0]);
  expect([marks.filter(([name]) => name === 'moveTo').length, marks.filter(([name]) => name === 'lineTo').length, marks.filter(([name]) => name === 'stroke').length]).toEqual([1, 1023, 1]);
  expect(marks).toContainEqual(['strokeStyle', '#008cff']);
  const xs = marks.filter(([name]) => name === 'moveTo' || name === 'lineTo').map(([, x]) => x);
  expect(Math.min(...xs)).toBeCloseTo(2 * PAD);
  expect(Math.max(...xs)).toBeGreaterThan(800 - 3 * PAD);
  expect(Math.max(...xs)).toBeLessThanOrEqual(800 - 2 * PAD);
});

test('a draw with nothing new paints nothing more, a resize or a theme change repaints', () => {
  const live = open();
  live.at(100);
  expect(live.at(100)).toEqual([]);
  live.scene.size();
  expect(shown(live.at(100))).toHaveLength(1);
  live.scene.theme();
  expect(shown(live.at(100))).toHaveLength(1);
  expect(live.at(100)).toEqual([]);
});

test('every level of every preset fits inside the padding, on a wide stage and a tall one', () => {
  for (const [name, w, h] of [['gasket', 1184, 756], ['gasket', 390, 700], ['twindragon', 390, 700], ['koch', 700, 400], ['carpet', 800, 600], ['flowsnake', 390, 700]]) {
    const live = open(named(name, { line: 0, level: 1 }), false, w, h);
    const top = live.told[0].cap;
    for (let level = 1; level <= top; level++) {
      live.set({ ...preset(num, math, name), line: 0, level });
      const page = pages[Number(shown(live.at(level * 1000)).at(-1).slice(4))];
      expect(cells(page)).toBe(live.told.at(-1).fill);
      const marks = laid(page).filter(([key]) => key === 'moveTo' || key === 'lineTo' || key === 'rect');
      const xs = marks.flatMap(([key, x, , width]) => (key === 'rect' ? [x, x + width] : [x]));
      const ys = marks.flatMap(([key, , y, , height]) => (key === 'rect' ? [y, y + height] : [y]));
      expect(Math.min(...xs)).toBeGreaterThanOrEqual(2 * PAD - 1);
      expect(Math.max(...xs)).toBeLessThanOrEqual(w - 2 * PAD + 1);
      expect(Math.min(...ys)).toBeGreaterThanOrEqual(2 * PAD - 1);
      expect(Math.max(...ys)).toBeLessThanOrEqual(h - 2 * PAD + 1);
    }
  }
});

test('big cells are polygons with a gap and small ones are squares at least a device pixel wide', () => {
  const big = open(named('twindragon', { level: 1 }));
  big.at(0);
  expect([laid(pages[0]).filter(([name]) => name === 'moveTo').length, laid(pages[0]).filter(([name]) => name === 'lineTo').length]).toEqual([2, 6]);
  const small = open(named('twindragon', { level: 17 }));
  small.at(0);
  const rects = laid(pages[0]).filter(([name]) => name === 'rect');
  expect(rects).toHaveLength(131072);
  expect(rects.every(([, , , w, h]) => w >= 2 && w === h)).toBe(true);
});

test('at an odd level the cells of the turned lattice are drawn wider than its pitch, so no holes show between them', () => {
  const live = open(named('twindragon', { level: 17 }), true, 4000, 3000);
  live.at(0);
  const plan = study(num, math, named('twindragon'));
  const pitch = lattice(plan.norm, plan.sides, 17).spacing * lay(plan.stage, 4000, 3000, PAD * 2).k;
  const rects = laid(pages[0]).filter(([name]) => name === 'rect');
  expect(rects).toHaveLength(131072);
  expect(rects.every(([, , , w, h]) => w === h && Math.abs(w - pitch * Math.SQRT2) < 1e-6)).toBe(true);
  expect(pitch).toBeGreaterThan(2);
});

test('a cell size pins the scale so a cell is that many css pixels wide', () => {
  const live = open(named('carpet', { level: 2, cell: 8 }), true);
  live.at(0);
  const marks = laid(pages[0]).filter(([name]) => name === 'moveTo' || name === 'lineTo');
  expect(marks.filter(([name]) => name === 'moveTo')).toHaveLength(64);
  const xs = marks.slice(0, 4).map(([, x]) => x);
  const ys = marks.slice(0, 4).map(([, , y]) => y);
  expect(Math.max(...xs) - Math.min(...xs)).toBeCloseTo(16 * (1 - GAP));
  expect(Math.max(...ys) - Math.min(...ys)).toBeCloseTo(16 * (1 - GAP));
});

test('the scene is still, so a set lands on the new level at once in one copy, with no fade', () => {
  const live = open();
  live.at(100);
  live.set({ level: 6 });
  const log = live.at(100);
  expect([live.scene.still, shown(log).length, alphas(log)]).toEqual([true, 1, []]);
  expect(pages.some((page) => page.log.some(([name, from]) => name === 'drawImage' && from === 'pagescreen'))).toBe(false);
});

test('a level layer is kept, so stepping back to it is a copy and not a relay', () => {
  const live = open();
  live.at(100);
  live.set({ level: 6 });
  live.at(1000);
  live.set({ level: 5 });
  live.at(2000);
  expect(pages.flatMap((page) => page.log.filter(([name]) => name === 'stroke'))).toHaveLength(2);
});

test('a set of the digits remakes the design and tells its facts, a set of the look relays it, and an unchanged set does nothing', () => {
  const live = open();
  live.at(100);
  live.set({ d: '0:0_1:0_1:1', t: '0_0_0' });
  expect(live.told.at(-1)).toMatchObject({ digits: 3, fill: 243, code: '7', canonical: true });
  live.at(1000);
  const told = live.told.length;
  live.set({ d: '0:0_1:0_1:1', t: '0_0_0', tint: 'red' });
  expect(live.told).toHaveLength(told);
  expect(shown(live.at(2000))).toHaveLength(1);
  expect(live.at(3000)).toEqual([]);
  live.set({ d: '0:0_1:0_1:1', t: '0_0_0', tint: 'red' });
  expect(live.at(4000)).toEqual([]);
});

test('Enter drops the twists only while a digit is turned', () => {
  const map = (value) => keymap(live(PAGE.keys, value));
  expect([map(base()).some((row) => row.button), map(base({ t: '0_0_0_0' })).some((row) => row.button)]).toEqual([true, false]);
  expect([hit(map(base()), { key: 'Enter' })?.act, hit(map(base({ t: '0_0_0_0' })), { key: 'Enter' })]).toEqual(['flat', null]);
});

test('an export is named by the ring, the base, the level and the drawing', () => {
  expect([file(base()), file(base({ ring: 'gaussian', base: '2:1', level: 3, line: 0 }))]).toEqual(['radix-eisenstein-3-5-line', 'radix-gaussian-2+i-3-cells']);
});

test('the design rows live in the widget pane and only the look rows in the knobs', () => {
  expect(describe(SPEC, base()).flatMap(({ rows }) => rows.map((row) => row.key))).toEqual(['cell', 'tint']);
});

test('the level cap follows the digit count and a base its ring lacks falls back to 3', () => {
  expect(tidy(SPEC, base({ d: '0:0_1:0', level: 40 })).level).toBe(17);
  expect(tidy(SPEC, base({ ring: 'gaussian', base: '3:1' })).base).toBe('3:0');
});
