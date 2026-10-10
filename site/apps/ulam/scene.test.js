import { expect, test } from 'bun:test';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults, describe } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { live } from '../../lib/scene.jsx';
import { HOLD, fitted, formula, segments, span } from './engine.js';
import { PAGE, SPEC, make, said } from './scene.js';

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
  const view = { rand: rng(value.seed ?? 1), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 2, t: 0 };
  view.wake = () => {
    view.still = false;
    told.push('wake');
  };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 1, roll: 0, num, ...value });
  const quiet = log.length;
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, told, log, quiet, at };
};

const laid = () => {
  const log = sheets.at(-1).log;
  return log.slice(log.findLastIndex(([name]) => name === 'clearRect') + 1);
};

const rects = (log) => log.filter(([name]) => name === 'rect');

const cells = () => rects(laid()).length;

const ground = () => rects(sheets.at(-2).log);

const hexes = () => laid().filter(([name]) => name === 'moveTo').length;

test('make draws nothing, the first draw lays the ground and no mark yet, and a picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({ fit: 1, rings: 50 });
  expect(stepped.quiet).toBe(0);
  stepped.at(0);
  expect([ground().length, cells()]).toEqual([101 * 101, 0]);
  for (let t = 0; t <= 3000; t += 100) stepped.at(t);
  const total = cells();
  const stepLog = [...stepped.log];
  const jumped = open({ fit: 1, rings: 50 });
  const jumpLog = [...jumped.at(3000)];
  expect(cells()).toBe(total);
  expect(jumpLog).toEqual(stepLog);
  expect(total).toBeGreaterThan(1);
  expect(stepLog.find(([name]) => name === 'drawImage')).toEqual(['drawImage', 'layer', 0, 0]);
});

test('under reduced motion the first draw lays the whole sheet, the ground under a mark for each prime when faint and the marks alone when not, and a second draw is a no-op', () => {
  const faint = open({ fit: 1, rings: 50, a: 0 }, true);
  faint.at(0);
  expect([ground().length, cells()]).toEqual([101 * 101, num.prime.prime_count(101 * 101)]);
  expect(laid().filter(([name]) => name === 'drawImage')).toEqual([['drawImage', 'layer', 0, 0]]);
  expect(faint.at(0)).toEqual([]);
  const bare = open({ fit: 1, rings: 50, a: 0, faint: 0 }, true);
  bare.at(0);
  expect(cells()).toBe(num.prime.prime_count(101 * 101));
  expect(laid().some(([name]) => name === 'drawImage')).toBe(false);
});

test('past the span and the hold the sheet winds again on the same layer, and Again rewinds from now', () => {
  const live = open({ fit: 1, rings: 50 });
  const length = span(50);
  live.at(length);
  const full = cells();
  const made = sheets.length;
  live.at(length + HOLD / 2);
  expect(cells()).toBe(full);
  const bed = sheets.at(-2).log.length;
  live.at(length + HOLD + 50);
  expect(cells()).toBeLessThan(full / 10);
  expect(laid()[0]).toEqual(['drawImage', 'layer', 0, 0]);
  expect(sheets.at(-2).log.length).toBe(bed);
  live.at(length + HOLD + length);
  expect(cells()).toBe(full);
  live.scene.again();
  live.at(length + HOLD + length + 10);
  expect(cells()).toBeLessThan(full / 10);
  expect(sheets.length).toBe(made);
  const still = open({ fit: 1, rings: 50 }, true);
  still.at(0);
  still.scene.again();
  expect(still.told).toEqual(['wake']);
});

test('a filled stage is ground from edge to edge before its first mark lands', () => {
  const live = open({ cell: 10, a: 0 }, false, 400, 300);
  live.at(0);
  const plate = ground();
  expect(cells()).toBe(0);
  expect([Math.min(...plate.map(([, x]) => x)) <= 0, Math.min(...plate.map(([, , y]) => y)) <= 0]).toEqual([true, true]);
  expect([Math.max(...plate.map(([, x, , w]) => x + w)) >= 400, Math.max(...plate.map(([, , y, , h]) => y + h)) >= 300]).toEqual([true, true]);
});

test('a filled stage draws only the cells on it, in cells of the asked size, and a hexagon sheet draws hexagons', () => {
  const live = open({ cell: 10, a: 0, faint: 0 }, true, 400, 300);
  live.at(0);
  const rects = laid().filter(([name]) => name === 'rect');
  expect(rects.every(([, x, y, w, h]) => x >= -20 && y >= -20 && x + w <= 420 && y + h <= 320)).toBe(true);
  expect(rects.every(([, , , w, h]) => w === 18 && h === 18)).toBe(true);
  expect(rects.length).toBeLessThan(live.scene.facts.primes);
  expect(rects.length).toBeGreaterThan(40);
  const hex = open({ cell: 10, a: 0, faint: 0, lattice: 'hex' }, true, 400, 300);
  hex.at(0);
  expect(cells()).toBe(0);
  expect(hexes()).toBeGreaterThan(40);
});

test('the landings of a line that never joins are still cells, a half shade solid for a prime and hollow for a composite', () => {
  const live = open({ fit: 1, rings: 100, a: 1, b: 1, c: 41 }, true);
  const log = live.at(0);
  expect(log.filter(([name]) => name === 'lineTo')).toEqual([]);
  const line = num.spiral.diagonal('Square', 201, 1, 1, 41);
  expect(line.hits).toBeGreaterThan(0);
  expect(line.hits).toBeLessThan(line.values.length);
  const layer = laid();
  expect([cells(), ground().length]).toEqual([live.scene.facts.primes + line.values.length - line.hits, 201 * 201]);
  expect(layer.filter(([name, v]) => name === 'fillStyle' && v === 'rgb(0, 70, 128)')).toHaveLength(1);
  const hollow = layer.findIndex(([name, v]) => name === 'strokeStyle' && v === 'rgb(0, 70, 128)');
  expect(hollow).toBeGreaterThan(-1);
  expect(layer.slice(hollow).some(([name]) => name === 'stroke')).toBe(true);
});

test('the line is stroked over the layer through the landings laid so far, and not at all without a line', () => {
  const live = open({ fit: 1, rings: 50, c: 41 });
  const early = live.at(100);
  const strokes = (log) => log.filter(([name]) => name === 'lineTo').length;
  expect(strokes(early)).toBe(0);
  const late = live.at(span(50));
  const line = num.spiral.diagonal('Square', 101, 4, -2, 41);
  expect(strokes(late)).toBe(segments(num, 'square', line.cells).reduce((sum, run) => sum + run.length - 1, 0));
  expect(strokes(late)).toBe(30);
  expect(late.findIndex(([name]) => name === 'stroke')).toBeGreaterThan(late.findIndex(([name]) => name === 'drawImage'));
  expect(strokes(open({ fit: 1, rings: 50, a: 0 }).at(span(50)))).toBe(0);
});

test('a tap names the cell under it with its ring and its factors, rings it, and a second tap on it lets go', () => {
  const live = open({ fit: 1, rings: 10 }, true);
  live.at(0);
  const px = fitted('square', 10, 800, 600);
  const one = live.scene.pick(0.5, 0.5);
  expect(one).toMatchObject({ n: 1, x: 0, y: 0, ring: 0, prime: false, factors: [] });
  expect(said(one)).toBe('1, neither prime nor composite, ring 0');
  expect(live.log.filter(([name]) => name === 'arc')).toHaveLength(1);
  const two = live.scene.pick(0.5 + px / 800, 0.5);
  expect(said(two)).toBe('2 prime, ring 1');
  const nine = live.scene.pick(0.5 + px / 800, 0.5 + px / 600);
  expect(said(nine)).toBe('9 = 3^2, ring 1');
  expect(live.scene.pick(0.5 + px / 800, 0.5 + px / 600)).toBe(null);
  expect(live.scene.pick(0.999, 0.999)).toBe(null);
  expect(said(null)).toBe('');
});

test('a tap on the hexagon sheet finds the hexagon under it', () => {
  const live = open({ fit: 1, rings: 10, lattice: 'hex' }, true);
  live.at(0);
  const px = fitted('hex', 10, 800, 600);
  expect(said(live.scene.pick(0.5, 0.5))).toBe('1, neither prime nor composite, ring 0');
  expect(said(live.scene.pick(0.5 + px / 800, 0.5))).toBe('2 prime, ring 1');
  const [q, r] = num.spiral.Lattice.xy('Hex', 7);
  expect(said(live.scene.pick(0.5 + (px * (Number(q) + Number(r) / 2)) / 800, 0.5 + (px * Math.sqrt(3) * 0.5 * Number(r)) / 600))).toBe('7 prime, ring 1');
});

test('a resize on a filled stage reads the rings again and tells the host, and on a whole sheet keeps them', () => {
  const told = [];
  const live = open({ cell: 10, onFacts: (facts) => told.push(facts) }, true, 400, 300);
  live.at(0);
  const before = live.scene.facts.rings;
  live.view.w = 1200;
  live.view.h = 900;
  live.scene.size();
  live.at(0);
  const rects = laid().filter(([name]) => name === 'rect');
  expect(rects.some(([, x]) => x > 500)).toBe(true);
  expect(ground().some(([, x]) => x > 500)).toBe(true);
  expect(before).toBeLessThan(60);
  expect(told).toHaveLength(1);
  expect(told[0].rings).toBeGreaterThan(before);
  expect(live.scene.facts).toBe(told[0]);
  const whole = open({ fit: 1, rings: 20, onFacts: (facts) => told.push(facts) }, true, 400, 300);
  whole.at(0);
  const total = cells();
  whole.view.w = 1200;
  whole.scene.size();
  whole.at(0);
  expect(cells()).toBe(total);
  expect(told).toHaveLength(1);
});

test('the svg and the csv come from the scene, the csv empty without a line', () => {
  const live = open({ fit: 1, rings: 10 }, true);
  live.at(0);
  expect(live.scene.svg()).toContain('<rect ');
  expect(live.scene.csv().split('\n')[0]).toBe('k,n,prime');
  expect(open({ fit: 1, rings: 10, a: 0 }, true).scene.csv()).toBe('');
});

const groups = (value) => describe(SPEC, { ...defaults(SPEC), ...value });

const shown = (value) => groups(value).flatMap(({ rows }) => rows.map((row) => row.key));

test('rings show on a whole sheet and cell on a filled stage', () => {
  expect(shown({ fit: 1 })).toContain('rings');
  expect(shown({ fit: 1 })).not.toContain('cell');
  expect(shown({ fit: 0 })).toContain('cell');
  expect(shown({ fit: 0 })).not.toContain('rings');
});

test('Random hides the lattice and the whole Line group, and a named sheet shows the lattice, a, and b and c only with a line', () => {
  const rolled = (value) => shown(value).filter((key) => ['roll', 'lattice', 'a', 'b', 'c'].includes(key));
  expect(groups({ roll: 1 }).map(({ name }) => name)).toEqual(['Sheet', 'Mark', 'Look']);
  expect(rolled({ roll: 1 })).toEqual(['roll']);
  expect(rolled({ roll: 0, a: 0 })).toEqual(['roll', 'lattice', 'a']);
  expect(rolled({ roll: 0, a: 4 })).toEqual(['roll', 'lattice', 'a', 'b', 'c']);
});

test('the Line arrows are a key on a random sheet and on a named one with a line, and not on a named one with a at zero', () => {
  const arrows = (over) => live(PAGE.keys, { ...defaults(SPEC), ...over }).some((row) => row.act === 'line');
  expect([arrows({ roll: 1 }), arrows({ roll: 0, a: 4 }), arrows({ roll: 0, a: 0 })]).toEqual([true, true, false]);
});

test('Random rolls the lattice and the line from the seed, the same for one seed, and a named sheet draws its own', () => {
  const rolled = (seed) => open({ roll: 1, seed, lattice: 'square', a: 0, b: 0, c: 1 }, true).scene.facts;
  expect(rolled(1)).toEqual(rolled(1));
  const seen = Array.from({ length: 24 }, (_, seed) => rolled(seed));
  expect(seen.every((f) => f.rolled && f.line)).toBe(true);
  expect(new Set(seen.map((f) => f.lattice))).toEqual(new Set(['square', 'hex']));
  expect(new Set(seen.map((f) => formula(f.a, f.b, f.c))).size).toBeGreaterThan(20);
  const named = open({ roll: 0, seed: 7, lattice: 'hex', a: 3, b: 3, c: 1 }, true).scene.facts;
  expect([named.rolled, named.lattice, named.a, named.b, named.c]).toEqual([false, 'hex', 3, 3, 1]);
});
