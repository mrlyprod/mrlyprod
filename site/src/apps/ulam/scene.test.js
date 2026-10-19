import { expect, test } from 'bun:test';
import * as num from '@mrlyprod/mrly/num';
import { palette as hues } from '../../lib/theme/palette.gen.js';
import { defaults, describe } from '../../lib/app/spec.js';
import { rng } from '../../lib/rand.js';
import { live } from '../../lib/app/page.jsx';
import { EDGE, fitted, landed, span } from './engine.js';
import { PAGE, SPEC, make, named } from './scene.js';

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

const open = (over = {}, still = false, w = 800, h = 600) => {
  const log = [];
  const told = [];
  const value = { ...defaults(SPEC), seed: 1, ...over };
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#ffd100' }), still, w, h, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...value, num, onFacts: (facts) => told.push(facts) });
  const quiet = log.length;
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  const set = (patch) => {
    Object.assign(value, patch);
    log.length = 0;
    scene.set({ ...value });
    scene.draw();
    return log;
  };
  return { view, scene, told, log, quiet, at, set };
};

const laid = () => {
  const log = sheets.at(-1).log;
  return log.slice(log.findLastIndex(([name]) => name === 'clearRect') + 1);
};

const rects = (log) => log.filter(([name]) => name === 'rect');

const cells = () => rects(laid()).length;

const painted = (log, color) => {
  const from = log.findIndex(([name, v]) => name === 'fillStyle' && v === color);
  if (from < 0) return 0;
  const to = log.findIndex(([name], i) => i > from && name === 'fillStyle');
  return rects(log.slice(from, to < 0 ? undefined : to)).length;
};

test('make draws nothing, the first draw lays the one at the centre, the ground winds with the numbers, and a picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({ side: 101 });
  expect(stepped.quiet).toBe(0);
  stepped.at(0);
  expect(cells()).toBe(1);
  for (let t = 0; t <= 3000; t += 100) stepped.at(t);
  const total = cells();
  const stepLog = [...stepped.log];
  const jumped = open({ side: 101 });
  const jumpLog = [...jumped.at(3000)];
  expect(cells()).toBe(total);
  expect(jumpLog).toEqual(stepLog);
  expect(total).toBe(landed(num, 'Square', 50, 3000 / span(50)));
  expect(total).toBeLessThan(101 * 101);
  expect(stepLog.find(([name]) => name === 'drawImage')).toEqual(['drawImage', 'layer', 0, 0]);
});

test('under reduced motion the first draw lays the whole sheet, a cell for every number when faint and the marks alone when not, and a second draw is a no-op', () => {
  const faint = open({ side: 101, a: 0 }, true);
  faint.at(0);
  expect(cells()).toBe(101 * 101);
  expect(faint.at(0)).toEqual([]);
  const bare = open({ side: 101, a: 0, faint: 0 }, true);
  bare.at(0);
  expect(cells()).toBe(num.prime.prime_count(101 * 101));
});

test('the sheet is centred with the margin for the pills on the tighter axis, and the ground is its cells and nothing more', () => {
  const live = open({ side: 101, a: 0 }, true);
  live.at(0);
  const boxes = rects(laid());
  const left = Math.min(...boxes.map(([, x]) => x));
  const right = Math.max(...boxes.map(([, x, , w]) => x + w));
  const top = Math.min(...boxes.map(([, , y]) => y));
  const bottom = Math.max(...boxes.map(([, , y, , h]) => y + h));
  expect(boxes).toHaveLength(101 * 101);
  expect([Math.abs(left + right - 800) <= 1, Math.abs(top + bottom - 600) <= 1]).toEqual([true, true]);
  expect(Math.abs(bottom - top - (600 - 4 * EDGE)) <= 1).toBe(true);
  expect(right - left).toBe(bottom - top);
});

test('the sheet winds once: past the span it stays whole and draws nothing new', () => {
  const live = open({ side: 101, a: 0 });
  const length = span(50);
  live.at(length);
  const full = cells();
  const made = sheets.length;
  expect(full).toBe(101 * 101);
  expect([live.at(length * 3), live.at(length * 50)]).toEqual([[], []]);
  expect([cells(), sheets.length]).toEqual([full, made]);
});

test('the line lands over the layer, orange on a prime and blue on a composite, only through the numbers wound so far', () => {
  const whole = open({ side: 101 }, true);
  const log = whole.at(0);
  const line = num.spiral.diagonal('Square', 101, 4, -2, 41);
  const after = log.slice(log.findIndex(([name]) => name === 'drawImage'));
  expect([painted(after, hues.orange), painted(after, hues.blue)]).toEqual([line.hits, line.values.length - line.hits]);
  const early = open({ side: 101 }).at(0);
  expect(painted(early, hues.orange) + painted(early, hues.blue)).toBe(0);
});

test('a change of c keeps the layer, lands the whole new line at once mid-wind, and tells the host', () => {
  const live = open({ side: 101 });
  live.at(100);
  const made = sheets.length;
  const log = live.set({ c: 43 });
  const line = num.spiral.diagonal('Square', 101, 4, -2, 43);
  expect(sheets.length).toBe(made);
  expect([painted(log, hues.orange), painted(log, hues.blue)]).toEqual([line.hits, line.values.length - line.hits]);
  expect(log.filter(([name]) => name === 'drawImage')).toEqual([['drawImage', 'layer', 0, 0]]);
  expect(live.told.at(-1)).toMatchObject({ c: 43, line: { hits: line.hits } });
  expect(live.scene.facts).toBe(live.told.at(-1));
});

test('a change of mark, tint or faint lays the layer again', () => {
  const live = open({ side: 101 }, true);
  live.at(0);
  const made = sheets.length;
  live.set({ mark: 'twin' });
  expect(sheets.length).toBe(made + 1);
  live.set({ tint: 'red' });
  expect(sheets.length).toBe(made + 2);
  live.set({ faint: 0 });
  expect(sheets.length).toBe(made + 3);
});

test('a change of side or lattice reads the sheet again and lets go of the tap, and a change of c keeps it', () => {
  const live = open({ side: 21 }, true);
  live.at(0);
  live.scene.pick(0.5, 0.5);
  live.set({ c: 43 });
  expect(live.scene.facts.pick).toMatchObject({ n: 1 });
  live.set({ side: 31 });
  expect([live.scene.facts.pick, live.scene.facts.top]).toEqual([null, 31 * 31]);
  live.set({ lattice: 'hex' });
  expect(live.scene.facts).toMatchObject({ lattice: 'hex', top: num.spiral.Lattice.count('Hex', 31) });
});

test('a tap names the cell under it with its ring, its place and its factors, rings it, a second tap lets go, and one off the sheet says so', () => {
  const live = open({ side: 21 }, true);
  live.at(0);
  const px = fitted('square', 10, 800, 600, EDGE * 2);
  expect(live.scene.pick(0.5, 0.5)).toMatchObject({ n: 1, x: 0, y: 0, ring: 0, prime: false, factors: [] });
  expect(live.log.filter(([name]) => name === 'arc')).toHaveLength(1);
  expect(live.scene.pick(0.5 + px / 800, 0.5)).toMatchObject({ n: 2, x: 1, y: 0, ring: 1, prime: true });
  expect(live.scene.pick(0.5 + px / 800, 0.5 + px / 600)).toMatchObject({ n: 9, ring: 1, prime: false, factors: [[3, 2]] });
  expect(live.told.at(-1).pick).toMatchObject({ n: 9 });
  expect(live.scene.pick(0.5 + px / 800, 0.5 + px / 600)).toBe(null);
  expect(live.scene.pick(0.999, 0.999)).toEqual({ off: true });
  expect(live.told.at(-1).pick).toEqual({ off: true });
});

test('a tap mid-wind lays the whole sheet at once', () => {
  const live = open({ side: 101, a: 0 });
  live.at(100);
  live.scene.pick(0.5, 0.5);
  expect(cells()).toBe(101 * 101);
});

test('a tap on the hexagon sheet finds the hexagon under it', () => {
  const live = open({ side: 21, lattice: 'hex' }, true);
  live.at(0);
  const px = fitted('hex', 10, 800, 600, EDGE * 2);
  expect(live.scene.pick(0.5, 0.5)).toMatchObject({ n: 1 });
  expect(live.scene.pick(0.5 + px / 800, 0.5)).toMatchObject({ n: 2, ring: 1 });
  const [q, r] = num.spiral.Lattice.xy('Hex', 7);
  expect(live.scene.pick(0.5 + (px * (Number(q) + Number(r) / 2)) / 800, 0.5 + (px * Math.sqrt(3) * 0.5 * Number(r)) / 600)).toMatchObject({ n: 7, prime: true });
});

test('a resize lays the same sheet to the new stage', () => {
  const live = open({ side: 41, faint: 0, a: 0 }, true, 400, 300);
  live.at(0);
  const total = cells();
  live.view.w = 1200;
  live.view.h = 900;
  live.scene.size();
  live.at(0);
  expect(cells()).toBe(total);
  expect(rects(laid()).some(([, x]) => x > 800)).toBe(true);
});

test('the svg and the csv come from the scene, the csv empty without a line', () => {
  const live = open({ side: 21 }, true);
  expect(live.scene.svg()).toContain('<rect ');
  expect(live.scene.csv().split('\n')[0]).toBe('k,n,prime');
  expect(open({ side: 21, a: 0 }, true).scene.csv()).toBe('');
});

test("the knobs open on Euler's line over a whole square sheet of side 201, every knob shown, the sheet and its quadratic first", () => {
  const groups = describe(SPEC, defaults(SPEC));
  expect(groups.map(({ name, rows }) => [name, rows.map((row) => row.key)])).toEqual([['Sheet', ['lattice', 'side', 'a', 'b', 'c']], ['Mark', ['mark', 'faint', 'tint']]]);
  expect(defaults(SPEC)).toEqual({ lattice: 'square', side: 201, a: 4, b: -2, c: 41, mark: 'prime', faint: 1, tint: 'yellow' });
  expect(describe(SPEC, { ...defaults(SPEC), a: 0 }).flatMap(({ rows }) => rows)).toHaveLength(SPEC.length);
});

test('an export is named by the lattice, the side, the line while one is lit, and a mark off the primes', () => {
  expect(PAGE.name).toBe(named);
  expect(named(defaults(SPEC))).toBe('ulam-square-201-4k2-2k+41');
  expect(named({ ...defaults(SPEC), lattice: 'hex', side: 21, a: 3, b: 0, c: -7, mark: 'mobius' })).toBe('ulam-hex-21-3k2-7-mobius');
  expect(named({ ...defaults(SPEC), a: 0 })).toBe('ulam-square-201');
});

test('Next c and the arrows are keys only while a line is lit', () => {
  const acts = (a) => live(PAGE.keys, { ...defaults(SPEC), a }).map((row) => row.act);
  expect([acts(4), acts(0)]).toEqual([['next', 'line'], []]);
});
