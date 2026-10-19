import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { defaults, describe, tidy } from '../../lib/app/spec.js';
import { rng } from '../../lib/rand.js';
import { HOLD, ceiling, fills, pair, plan, roots, series } from './engine.js';
import { LABELS, MATCHUPS, PAGE, SPEC, make } from './scene.js';

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

const VALUE = { ...defaults(SPEC), seed: 1 };

const open = (value, { still = false, lock = false, w = 1600, h = 1000 } = {}) => {
  const log = [];
  const tells = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still, lock, w, h, dpr: 2, t: 0, woke: 0 };
  view.wake = () => {
    view.woke++;
    view.still = false;
  };
  const opts = { ...VALUE, math, ...value, onTell: (stat) => tells.push(stat) };
  const scene = make({ getContext: () => pen(log) }, view, opts);
  const sheets = pages.slice(-2);
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  const set = (patch) => {
    log.length = 0;
    scene.set({ ...VALUE, ...value, ...patch });
  };
  return { view, scene, log, tells, sheets, at, set };
};

const ROOTS = roots(rng(1));
const PLAN = plan(math, VALUE, ROOTS.code);
const PLAN_1000 = plan(math, { ...VALUE, walkers: 1000 }, ROOTS.code);

const end = (laid) => {
  const run = series(laid, ROOTS.walk);
  run.finish();
  return run.state.over.tick;
};

const END = end(PLAN);

const ms = (tick, speed = 720) => Math.ceil((tick * 1000) / speed);

const arcs = (log) => log.filter(([name]) => name === 'arc');

const blits = (log) => log.filter(([name]) => name === 'drawImage');

const widths = (log) => log.filter(([name]) => name === 'lineWidth').map(([, w]) => w);

const pixels = (sheet) => sheet.image.data;

const rgba = (sheet, i) => [...pixels(sheet).slice(i * 4, i * 4 + 4)];

const running = () => {
  const live = open({});
  live.at(0);
  live.scene.go();
  return live;
};

test('the primary is the Enter row, its label the phase of the race', () => {
  const [row] = PAGE.keys;
  expect([row.button, ...['ready', 'running', 'held', 'done'].map((phase) => row.label(VALUE, { phase: () => phase })), row.label(VALUE, null)]).toEqual([true, ...Object.values(LABELS), 'Start']);
});

test('the scene owns Space: its toggle is the go of Enter, and it is playing only while the race runs', () => {
  const live = open({});
  live.at(0);
  expect([live.scene.toggle === live.scene.go, live.scene.playing()]).toEqual([true, false]);
  live.scene.toggle();
  expect(live.scene.playing()).toBe(true);
  live.scene.toggle();
  expect(live.scene.playing()).toBe(false);
});

test('an export is named by the pair and the seed', () => {
  expect(PAGE.name(VALUE)).toBe('race-127-239-1');
});

test('the page opens on 127 against 239 at step 0 and waits for Start', () => {
  const live = open({});
  expect(live.log).toEqual([]);
  expect(blits(live.at(0))).toHaveLength(2);
  expect(live.tells.at(-1)).toMatchObject({ phase: 'ready', tick: 0, goal: 20, side: 81, walkers: 300 });
  expect(live.tells.at(-1).sides.map((side) => [side.code, side.fills, side.lit])).toEqual([['127', 7, 2401], ['239', 7, 2401]]);
  expect(live.at(60000)).toEqual([]);
  expect(live.scene.stat().tick).toBe(0);
});

test('Start runs the race from the moment it is pressed at the speed in steps a second, Pause holds it and Resume runs on from that step', () => {
  const live = open({});
  live.at(5000);
  expect(live.scene.go()).toBe('running');
  live.at(5000 + ms(100));
  expect(live.scene.stat().tick).toBe(100);
  expect(live.scene.go()).toBe('held');
  live.at(20000);
  expect(live.scene.stat().tick).toBe(100);
  live.scene.go();
  live.at(20000 + ms(50));
  expect(live.scene.stat().tick).toBe(150);
  expect(live.tells.map((stat) => stat.phase).filter((phase, i, all) => phase !== all[i - 1])).toEqual(['ready', 'running', 'held', 'running']);
});

test('the boards paint sites in the line ink and holes in the deep ink, walkers in full shade and the trail mixing toward it', () => {
  const live = running();
  const [a, b] = live.sheets;
  const { filled, home } = PLAN.sides[0];
  for (let i = 0; i < 200; i++) expect(rgba(a, i)).toEqual(filled[i] ? [221, 221, 223, 255] : [232, 232, 233, 255]);
  expect(rgba(a, home)).toEqual([0, 140, 255, 255]);
  expect(rgba(b, PLAN.sides[1].home)).toEqual([255, 115, 0, 255]);
  live.at(ms(300));
  const warm = [...filled].map((_, i) => i).filter((i) => filled[i] && rgba(a, i)[0] < 221 && rgba(a, i)[0] > 0);
  expect(warm.length).toBeGreaterThan(10);
  expect(a.log.filter(([name]) => name === 'putImageData')).toHaveLength(2);
});

test('a picture at a time is the same reached in one jump or by steps, and a draw with no new step paints nothing', () => {
  const stepped = running();
  for (let t = 0; t <= 1000; t += 20) stepped.at(t);
  const stepLog = [...stepped.log];
  const stepPixels = Uint8ClampedArray.from(pixels(stepped.sheets[0]));
  const jumped = running();
  const jumpLog = [...jumped.at(1000)];
  expect(jumpLog).toEqual(stepLog);
  expect(Uint8ClampedArray.from(pixels(jumped.sheets[0]))).toEqual(stepPixels);
  expect(jumped.at(1000)).toEqual([]);
  expect(jumped.at(1001)).toEqual([]);
});

test('the race stops at the goal with the winner thick, the home squares in the foreground ink, and keeps the result up', () => {
  const live = running();
  const log = live.at(ms(END));
  expect(widths(log)).toEqual(expect.arrayContaining([6, 4]));
  expect(log).toContainEqual(['strokeStyle', '#000000']);
  expect(live.scene.phase()).toBe('done');
  expect(live.tells.at(-1)).toMatchObject({ phase: 'done', tick: END, over: { winner: 0, how: 'goal' } });
  expect(live.at(ms(END) + 60000)).toEqual([]);
  expect(live.scene.stat().tick).toBe(END);
});

test('a race judged without a side at the goal keeps both reach rings thin', () => {
  const live = open({ a: '75', b: '39' });
  live.at(0);
  live.scene.go();
  const log = live.at(ms(end(plan(math, { ...VALUE, a: '75', b: '39' }, roots(rng(1)).code))));
  expect(live.tells.at(-1).over).toMatchObject({ how: 'stall' });
  expect(widths(log)).not.toContain(6);
  expect(widths(log)).toContain(3);
});

test('under reduced motion the page waits at step 0 and Start wakes the loop', () => {
  const live = open({}, { still: true });
  expect(blits(live.at(0))).toHaveLength(2);
  expect(live.scene.stat()).toMatchObject({ phase: 'ready', tick: 0 });
  live.scene.go();
  expect([live.view.woke, live.view.still]).toEqual([1, false]);
});

test('a fixed frame starts a race that waits, so a take is never a still', () => {
  const live = open({});
  live.view.fixed = true;
  live.at(1000);
  live.at(1000 + ms(10));
  expect(live.scene.stat()).toMatchObject({ phase: 'running', tick: 10 });
});

test('the lock runs from t alone, holds the finish, then races again on fresh boards', () => {
  const lock = open({}, { lock: true });
  lock.at(0);
  lock.at(ms(END) + HOLD / 2);
  expect(lock.scene.stat()).toMatchObject({ tick: END, over: { how: 'goal' } });
  lock.at(ms(END + Math.ceil((HOLD * 720) / 1000) + 2));
  expect(lock.scene.stat()).toMatchObject({ tick: 2, over: null });
  expect(lock.tells).toEqual([]);
});

test('the lock rolls a pair of one mass from the seed when no codes are given', () => {
  const lock = open({ a: undefined, b: undefined }, { lock: true });
  expect(lock.scene.stat().sides.map((side) => side.code)).toEqual(pair(math, VALUE, roots(rng(1)).code));
});

test('under reduced motion the lock draws the finished race, or the race where the budget of walker steps ends', () => {
  const still = open({}, { still: true, lock: true });
  expect(blits(still.at(0))).toHaveLength(2);
  expect(still.scene.stat()).toMatchObject({ tick: END, over: { how: 'goal' } });
  expect(still.at(0)).toEqual([]);
  const long = open({ walkers: 1000 }, { still: true, lock: true });
  long.at(0);
  expect(long.scene.stat()).toMatchObject({ tick: PLAN_1000.still, over: null });
});

test('a resize lays the boards again and a theme change repaints in the new shades', () => {
  const live = running();
  live.at(500);
  live.view.w = 1000;
  live.view.h = 1600;
  live.scene.size();
  const [one, two] = blits(live.at(500)).map((row) => row.slice(2, 4));
  expect(one[0]).toBe(two[0]);
  expect(two[1]).toBeGreaterThan(one[1]);
  live.view.look = () => ({ paper: '#000000', accent: '#ff0000' });
  live.scene.theme();
  expect(live.at(500)).toContainEqual(['strokeStyle', '#00ffff']);
  expect(PLAN.sides[1].filled.some((_, i) => rgba(live.sheets[1], i).join() === '0,255,255,255')).toBe(true);
});

test('a new tint or fade keeps the canvas and the race where it is', () => {
  const live = running();
  live.at(ms(200));
  const made = pages.length;
  live.view.look = () => ({ paper: '#000000', accent: '#ff0000' });
  live.set({ tint: 'red', heat: 50 });
  expect(live.at(ms(200))).toContainEqual(['strokeStyle', '#00ffff']);
  expect([pages.length, live.scene.phase(), live.scene.stat().tick]).toEqual([made, 'running', 200]);
});

test('a new speed runs on from the same step at the new pace', () => {
  const live = running();
  live.at(ms(300));
  live.set({ speed: 72 });
  live.at(ms(300) + 1000);
  expect(live.scene.stat().tick).toBe(372);
});

test('a new finish moves the goal ring at once, ends a race already past it and lets a finished race run on to a farther goal', () => {
  const live = running();
  live.at(ms(400));
  const far = Math.max(...live.scene.stat().reach);
  live.set({ finish: Math.floor(far) });
  expect([live.scene.phase(), live.tells.at(-1).over.how, live.tells.at(-1).goal]).toEqual(['done', 'goal', Math.floor(far)]);
  live.set({ finish: 22 });
  expect(live.scene.phase()).toBe('running');
  const log = live.at(ms(400));
  expect(arcs(log)[0][3]).toBe(22 * 9);
});

test('new walkers start a fresh race on the same boards that waits at step 0', () => {
  const live = running();
  live.at(ms(200));
  live.set({ walkers: 50 });
  live.at(ms(400));
  expect(live.scene.stat()).toMatchObject({ phase: 'ready', tick: 0, walkers: 50 });
});

test('a new code grows new boards that wait at step 0, and a new seed walks the same pair afresh at once', () => {
  const coded = running();
  coded.at(ms(200));
  coded.set({ a: '495' });
  expect(coded.scene.stat()).toMatchObject({ phase: 'ready', tick: 0 });
  expect(coded.scene.stat().sides.map((side) => side.code)).toEqual(['495', '239']);
  const seeded = open({});
  seeded.at(0);
  seeded.set({ seed: 2 });
  expect(seeded.scene.phase()).toBe('running');
  expect(seeded.scene.stat().sides.map((side) => side.code)).toEqual(['127', '239']);
  seeded.at(ms(100));
  expect(seeded.scene.stat().tick).toBe(100);
});

test('the svg is a sheet of css pixels with both boards in the line and deep inks, the trail, the walkers and the rings', () => {
  const live = running();
  live.at(ms(300));
  const text = live.scene.svg();
  expect(text).toContain('width="800" height="500" viewBox="0 0 800 500"');
  for (const ink of ['#008cff', '#ff7300', '#dddddf', '#e8e8e9']) expect(text).toContain(`fill="${ink}"`);
  expect(text.match(/<circle /g)).toHaveLength(4);
  expect(text.match(/<clipPath /g)).toHaveLength(2);
  expect((text.match(/<rect /g) ?? []).length).toBeGreaterThan(600);
});

test('a warm cell of the svg mixes over the line ink as the canvas does', () => {
  const live = running();
  live.at(ms(300));
  const text = live.scene.svg();
  const { filled } = PLAN.sides[0];
  const warm = [...filled].map((_, i) => i).filter((i) => filled[i] && rgba(live.sheets[0], i)[0] < 221 && rgba(live.sheets[0], i)[0] > 0);
  expect(warm.length).toBeGreaterThan(10);
  for (const i of warm.slice(0, 20)) {
    const found = new RegExp(`<rect x="${i % 81}" y="${Math.floor(i / 81)}" width="1" height="1" fill-opacity="([0-9.]+)"/>`).exec(text);
    expect(Math.abs(221 * (1 - Number(found[1])) - rgba(live.sheets[0], i)[0])).toBeLessThan(1);
  }
});

test('the csv is one row of both reaches per step', () => {
  const live = running();
  live.at(ms(100));
  const rows = live.scene.csv().split('\n');
  expect(rows[0]).toBe('step,a,b');
  expect(rows).toHaveLength(102);
  expect(rows[1]).toBe('0,0,0');
  expect(rows[2].split(',').length).toBe(3);
});

test('the status is told at most ten times a second while the race runs', () => {
  const live = running();
  const before = live.tells.length;
  for (let t = 16; t <= 1000; t += 16) live.at(t);
  expect(live.tells.length - before).toBeLessThanOrEqual(11);
  expect(live.tells.length - before).toBeGreaterThanOrEqual(8);
});

test('the scene wants the unit and the level runs to the cap of the number', () => {
  expect(() => make({ getContext: () => pen([]) }, { rand: rng(1), look: () => ({}), w: 1, h: 1, dpr: 1, t: 0 }, {})).toThrow();
  const level = (number) => tidy(SPEC, { ...defaults(SPEC), number, level: 9 }).level;
  expect([level(3), level(5), level(7)]).toEqual([5, 3, 3]);
});

test('the finish is in cells and runs to the most the board offers, read from the number and the level tidy kept', () => {
  const finish = (over) => tidy(SPEC, { ...defaults(SPEC), finish: 60, ...over }).finish;
  expect([defaults(SPEC).finish, finish({}), finish({ level: 5 }), finish({ level: 2 }), finish({ number: 7, level: 'x' })]).toEqual([20, 22, 28, 2, 28]);
});

test('speed opens at 720 a second and reaches 3000 on every board', () => {
  expect(defaults(SPEC).speed).toBe(720);
  for (const [number, level] of [[3, 4], [3, 5], [5, 3], [7, 3]]) expect(tidy(SPEC, { ...defaults(SPEC), number, level, speed: 3000 }).speed).toBe(3000);
});

test('the design chip owns the left code and the base, so the pane hides them and the value keeps them', () => {
  const shown = describe(SPEC, defaults(SPEC)).flatMap((group) => group.rows.map((row) => row.key));
  expect([shown.includes('a'), shown.includes('base'), shown.includes('b'), PAGE.design]).toEqual([false, false, true, { dim: 2, base: 'base', code: 'a' }]);
  expect(tidy(SPEC, { a: '300', base: 2 })).toMatchObject({ a: '300', base: 2 });
});

test('every matchup is a fair pair: one mass, and both rooms reach past the opening finish', () => {
  const value = { base: 3, number: 3, level: 4, finish: 20 };
  for (const [a, b] of MATCHUPS) expect([a, fills(math, value, a) === fills(math, value, b), ceiling(math, value, a) >= value.finish, ceiling(math, value, b) >= value.finish]).toEqual([a, true, true, true]);
});
