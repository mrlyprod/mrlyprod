import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { HOLD, STILL, plan, roots, series } from './engine.js';
import { PAGE, SPEC, make, units } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args.map((arg) => (arg && typeof arg === 'object' ? 'page' : arg))]),
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
    this.log = [];
    this.image = null;
    pages.push(this);
  }
  getContext() {
    const log = pen(this.log);
    return new Proxy(log, { get: (target, key) => (key === 'putImageData' ? (image, ...rest) => (this.image = image) && target.putImageData(image, ...rest) : target[key]) });
  }
};

globalThis.ImageData = class {
  constructor(w, h) {
    this.width = w;
    this.height = h;
    this.data = new Uint8ClampedArray(w * h * 4);
  }
};

const CODES = { a: '127', b: '239' };

const open = (value, still = false, w = 800, h = 400) => {
  const log = [];
  const stats = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 1, ...CODES, math, onStat: (stat) => stats.push(stat), ...value });
  const sheets = pages.slice(-2);
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, log, stats, sheets, at };
};

const ROOTS = roots(rng(1));
const PLAN = plan(math, { ...defaults(SPEC), seed: 1, ...CODES }, ROOTS.code);
const PLAN_1000 = plan(math, { ...defaults(SPEC), seed: 1, ...CODES, walkers: 1000 }, ROOTS.code);

const END = (() => {
  const run = series(PLAN, ROOTS.walk);
  run.finish();
  return run.state.over.tick;
})();

const arcs = (log) => log.filter(([name]) => name === 'arc');

const blits = (log) => log.filter(([name]) => name === 'drawImage');

const pixels = (sheet) => sheet.image.data;

test('the page binds Enter to Random, records, and a saver thunk names the unit', () => {
  expect(PAGE.keys.map((row) => [row.key, row.act, row.button])).toEqual([['Enter', 'random', true]]);
  expect(PAGE.record).toBe(true);
  expect(typeof units).toBe('function');
});

test('make draws nothing, the first draw lays both boards with their goal rings and tells the status', () => {
  const live = open({});
  expect(live.log).toEqual([]);
  const log = live.at(0);
  expect(blits(log).map((row) => row.slice(2))).toEqual([[60, 38, 324, 324], [416, 38, 324, 324]]);
  expect(arcs(log)).toHaveLength(2);
  expect(arcs(log).map(([, , , r]) => r)).toEqual([80, 80]);
  expect(log).toContainEqual(['strokeStyle', 'rgba(0, 140, 255, 0.45)']);
  expect(log).toContainEqual(['strokeStyle', 'rgba(255, 115, 0, 0.45)']);
  expect(live.stats.map((stat) => stat.tick)).toEqual([0]);
  expect(live.stats[0]).toMatchObject({ race: 0, goal: 20, side: 81, reach: [0, 0], over: null, tally: [0, 0, 0] });
});

test('the boards paint each filled cell in its shade, holes clear, walkers full, the trail an alpha of the heat', () => {
  const live = open({});
  live.at(0);
  const [a, b] = live.sheets;
  const { filled, home } = PLAN.sides[0];
  const data = pixels(a);
  for (let i = 0; i < 100; i++) expect(data[i * 4 + 3]).toBe(filled[i] ? 26 : 0);
  expect([data[home * 4], data[home * 4 + 1], data[home * 4 + 2], data[home * 4 + 3]]).toEqual([0, 140, 255, 255]);
  const other = PLAN.sides[1].home;
  expect([pixels(b)[other * 4], pixels(b)[other * 4 + 1], pixels(b)[other * 4 + 2]]).toEqual([255, 115, 0]);
  live.at(1000);
  const warm = pixels(a).filter((_, k) => k % 4 === 3 && _ > 26 && _ < 255).length;
  expect(warm).toBeGreaterThan(10);
  expect(a.log.filter(([name]) => name === 'putImageData')).toHaveLength(2);
});

test('a picture at a t is the same reached in one jump or by steps, and a draw with no new tick paints nothing', () => {
  const stepped = open({});
  for (let t = 0; t <= 3000; t += 50) stepped.at(t);
  const stepLog = [...stepped.log];
  const stepPixels = Uint8ClampedArray.from(pixels(stepped.sheets[0]));
  const jumped = open({});
  const jumpLog = [...jumped.at(3000)];
  expect(jumpLog).toEqual(stepLog);
  expect(Uint8ClampedArray.from(pixels(jumped.sheets[0]))).toEqual(stepPixels);
  expect(jumped.at(3000)).toEqual([]);
  expect(jumped.at(3005)).toEqual([]);
});

test('the reach ring grows with the swarm and the winner is drawn thick at the finish', () => {
  const live = open({});
  const early = arcs(live.at(1000));
  expect(early).toHaveLength(4);
  const [, , , goalA, , reachA] = [...early[0], ...early[1]];
  expect(reachA).toBeGreaterThan(0);
  expect(reachA).toBeLessThan(goalA);
  const log = live.at((END * 1000) / 60 + 1);
  const widths = log.filter(([name]) => name === 'lineWidth').map(([, w]) => w);
  expect(widths).toContain(6);
  expect(live.stats.at(-1).over).toMatchObject({ winner: 0, how: 'goal' });
});

test('the reach ring is thin when the race is judged without a side reaching the goal', () => {
  const live = open({ a: '75', b: '39' });
  const log = live.at((3800 * 1000) / 60);
  expect(live.stats.at(-1).over).toMatchObject({ how: 'stall' });
  const widths = log.filter(([name]) => name === 'lineWidth').map(([, w]) => w);
  expect(widths).not.toContain(6);
  expect(widths).toContain(3);
});

test('under reduced motion the first draw is the finished race and a second draw paints nothing new', () => {
  const still = open({}, true);
  const log = still.at(0);
  expect(blits(log)).toHaveLength(2);
  expect(still.stats.at(-1)).toMatchObject({ tick: END, tally: [1, 0, 0] });
  expect(still.at(0)).toEqual([]);
});

test('under reduced motion a race longer than the budget of walker steps is drawn where the budget ends, not judged', () => {
  const still = open({ walkers: 1000 }, true);
  still.at(0);
  expect(still.stats.at(-1)).toMatchObject({ tick: PLAN_1000.still, over: null, tally: [0, 0, 0] });
  expect(PLAN_1000.still * 2 * 1000).toBeLessThanOrEqual(STILL);
});

test('past the finish and the hold the next race starts on fresh boards', () => {
  const live = open({});
  const ms = (tick) => (tick * 1000) / 60;
  live.at(ms(END) + HOLD / 2);
  expect(live.stats.at(-1)).toMatchObject({ race: 0, tick: END });
  live.at(ms(END) + HOLD + ms(2));
  const now = live.stats.at(-1);
  expect([now.race, now.tick, now.over, now.tally]).toEqual([1, 2, null, [1, 0, 0]]);
});

test('a resize lays the boards again and a theme change repaints in the new shades', () => {
  const live = open({});
  live.at(500);
  live.view.w = 400;
  live.view.h = 800;
  live.scene.size();
  expect(blits(live.at(500)).map((row) => row.slice(2, 4))).toEqual([[38, 60], [38, 416]]);
  live.view.look = () => ({ paper: '#000000', accent: '#ff0000' });
  live.scene.theme();
  const log = live.at(500);
  expect(log).toContainEqual(['strokeStyle', '#00ffff']);
  expect(pixels(live.sheets[1]).slice(PLAN.sides[1].home * 4, PLAN.sides[1].home * 4 + 3)).toEqual(Uint8ClampedArray.from([0, 255, 255]));
});

test('the svg is a sheet of css pixels with both boards as runs, the warm cells, the walkers and the rings', () => {
  const live = open({});
  live.at(1000);
  const text = live.scene.svg();
  expect(text).toContain('width="400" height="200" viewBox="0 0 400 200"');
  expect(text).toContain('fill="#008cff"');
  expect(text).toContain('fill="#ff7300"');
  expect(text.match(/<circle /g)).toHaveLength(4);
  expect(text.match(/<clipPath /g)).toHaveLength(2);
  expect(text.match(/fill-opacity="0.1"/g)).toHaveLength(2);
  expect((text.match(/<rect /g) ?? []).length).toBeGreaterThan(600);
});

test('a warm cell of the svg lies over the faint base so that the two meet the alpha of the canvas', () => {
  const live = open({});
  live.at(1000);
  const text = live.scene.svg();
  const data = pixels(live.sheets[0]);
  const { filled } = PLAN.sides[0];
  const warm = [...filled].map((_, i) => i).filter((i) => data[i * 4 + 3] > 26 && data[i * 4 + 3] < 255);
  expect(warm.length).toBeGreaterThan(10);
  for (const i of warm.slice(0, 20)) {
    const found = new RegExp(`<rect x="${i % 81}" y="${Math.floor(i / 81)}" width="1" height="1" fill-opacity="([0-9.]+)"/>`).exec(text);
    expect(Math.abs(0.1 + 0.9 * Number(found[1]) - data[i * 4 + 3] / 255)).toBeLessThan(0.01);
  }
});

test('the csv is one row of both reaches per step', () => {
  const live = open({});
  live.at(1000);
  const rows = live.scene.csv().split('\n');
  expect(rows[0]).toBe('step,a,b');
  expect(rows).toHaveLength(live.stats.at(-1).tick + 2);
  expect(rows[1]).toBe('0,0,0');
  expect(rows[2].split(',').length).toBe(3);
});

test('the facts name both sides, the board and the goal, and the status is told at most ten times a second', () => {
  const live = open({});
  expect(live.scene.facts).toMatchObject({ goal: 20, side: 81, walkers: 300 });
  expect(live.scene.facts.sides.map((side) => [side.code, side.fills, side.of, side.cells])).toEqual([['127', 7, 9, 2401], ['239', 7, 9, 2401]]);
  for (let t = 0; t <= 1000; t += 16) live.at(t);
  expect(live.stats.length).toBeLessThanOrEqual(12);
  expect(live.stats.length).toBeGreaterThanOrEqual(8);
});

test('the scene wants the unit and the level runs to the cap of the number', () => {
  expect(() => make({ getContext: () => pen([]) }, { rand: rng(1), look: () => ({}), w: 1, h: 1, dpr: 1, t: 0 }, {})).toThrow();
  const level = (number) => tidy(SPEC, { ...defaults(SPEC), number, level: 9 }).level;
  expect([level(3), level(5), level(7)]).toEqual([5, 3, 3]);
});

test('the finish runs to the most the board offers, read from the number and the level tidy kept', () => {
  const finish = (over) => tidy(SPEC, { ...defaults(SPEC), finish: 55, ...over }).finish;
  expect([finish({}), finish({ level: 5 }), finish({ number: 5, level: 3 }), finish({ number: 7, level: 3 }), finish({ number: 7, level: 9 }), finish({ number: 7, level: 'x' })]).toEqual([55, 20, 45, 15, 15, 15]);
  expect(finish({ level: 5, finish: 15 })).toBe(15);
});

test('speed reaches 600 on every board', () => {
  for (const [number, level] of [[3, 4], [3, 5], [5, 3], [7, 3]]) expect(tidy(SPEC, { ...defaults(SPEC), number, level, speed: 600 }).speed).toBe(600);
});
