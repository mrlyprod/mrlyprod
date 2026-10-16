import { expect, test } from 'bun:test';
import * as font from '@mrlyprod/mrly/font';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { SAMPLES, gallery, slug } from './engine.js';
import { PAGE, SPEC, layout, make, named, place, tile } from './scene.js';

const KINDS = gallery(font);

const PENS = KINDS.flatMap((kind) => kind.glyphs);

const FLAGGED = PENS.filter((one) => one.flag).length;

const open = (value, { still = false, w = 800, h = 600, dpr = 2 } = {}) => {
  const log = [];
  const plans = [];
  const ctx = new Proxy({}, { get: (_, key) => (...args) => log.push([key, ...args]), set: () => true });
  const view = { rand: rng(1), look: () => ({ paper: '#ffffff', accent: '#008cff' }), still, w, h, dpr, t: 0 };
  let now = { ...defaults(SPEC), seed: 1, ...value };
  const scene = make({ getContext: () => ctx }, view, { ...now, font, kinds: KINDS, onPlan: (one) => plans.push(one) });
  const rects = () => log.filter(([key]) => key === 'rect');
  const draw = (t) => {
    log.length = 0;
    view.t = t;
    scene.draw();
  };
  const set = (patch) => {
    now = { ...now, ...patch };
    log.length = 0;
    scene.set(now);
    scene.draw();
  };
  return { log, view, scene, rects, draw, set, plans };
};

const FRAME = 1000 / font.FPS();

test('the scene opens on the wall of pens, every pen writing from empty at once, one cell a frame', () => {
  const live = open({});
  live.draw(0);
  expect(live.rects()).toHaveLength(0);
  live.draw(FRAME);
  expect(live.rects()).toHaveLength(PENS.filter((one) => one.frames.length > 1).length);
  live.draw(FRAME * 1.5);
  expect(live.log).toEqual([]);
});

test('flagged pens stand on a plate while flags are on', () => {
  const live = open({});
  live.draw(0);
  expect(live.log.filter(([key]) => key === 'roundRect')).toHaveLength(FLAGGED);
  live.set({ flags: 0 });
  expect(live.log.filter(([key]) => key === 'roundRect')).toHaveLength(0);
});

test('the text view paints the frame of t and repaints only when the frame changes', () => {
  const live = open({ view: 'text', text: 'hi', loop: 0 });
  live.draw(0);
  expect(live.rects()).toHaveLength(0);
  expect(live.log.map(([key]) => key)).toContain('clearRect');
  live.draw(FRAME / 2);
  expect(live.log).toEqual([]);
  live.draw(FRAME);
  expect(live.rects()).toHaveLength(1);
  live.draw(100000);
  expect(live.rects()).toHaveLength(live.scene.plan().frames.at(-1).length);
  live.draw(100100);
  expect(live.log).toEqual([]);
});

test('under reduced motion the still is every pen whole, or the finished text, the same at any t', () => {
  const wall = open({}, { still: true });
  wall.draw(0);
  expect(wall.rects()).toHaveLength(PENS.reduce((n, one) => n + one.frames.at(-1).length, 0));
  const text = open({ view: 'text', text: 'hi' }, { still: true });
  text.draw(0);
  const full = text.rects();
  expect(full).toHaveLength(text.scene.plan().frames[text.scene.plan().still].length);
  text.draw(5000);
  expect(text.log).toEqual([]);
  text.scene.theme();
  expect(text.rects()).toEqual(full);
});

test('cell 0 fits the text to the stage in whole pixels, cell n gives n px times the dpr, centred, and a cell too big for the stage fits instead', () => {
  const view = { w: 800, h: 600, dpr: 2 };
  expect(layout(view, { rows: 7, cols: 11 }, 0)).toMatchObject({ cell: 66, w: 726, h: 462, x: 37, y: 69 });
  expect(layout(view, { rows: 7, cols: 11 }, 10)).toEqual({ x: 290, y: 230, w: 220, h: 140, cell: 20 });
  expect(layout(view, { rows: 7, cols: 481 }, 40)).toEqual(layout(view, { rows: 7, cols: 481 }, 0));
  expect(layout(view, { rows: 0, cols: 0 }, 0).cell).toBeGreaterThan(0);
});

test('the wall takes the columns that give the biggest whole-pixel cell at any aspect, centred, its last row centred too', () => {
  const phone = tile({ w: 780, h: 1400 }, 108);
  expect(phone.down).toBeGreaterThan(phone.across);
  const desk = tile({ w: 3200, h: 1800 }, 108);
  expect(desk.across).toBeGreaterThan(desk.down);
  expect(Number.isInteger(desk.cell)).toBe(true);
  expect([desk.x, desk.y]).toEqual([Math.floor((3200 - desk.w) / 2), Math.floor((1800 - desk.h) / 2)]);
  const box = { across: 4, down: 2, n: 6 };
  expect([0, 3, 4, 5].map((i) => place(box, i))).toEqual([[0, 0], [27, 0], [9, 9], [18, 9]]);
});

test('the handle gives the text as JSON at the played rate and the shown frame as an SVG of rects, named for the text', () => {
  const live = open({ view: 'text', text: 'hi', speed: 2, cell: 10 });
  live.draw(FRAME);
  const json = live.scene.json();
  expect(json).toMatchObject({ text: 'hi', pad: 1, hold: 25, loop: 1, rows: 7, cols: 11, fps: 50 });
  expect(json.frames).toBe(live.scene.plan().frames);
  const still = live.scene.svg();
  const lit = live.scene.plan().frames[2];
  expect(lit).toHaveLength(2);
  expect(still.match(/<rect /g)).toHaveLength(2);
  expect(still).toContain('width="110" height="70"');
  expect(still).toContain('fill="#008cff"');
  expect(live.scene.name()).toBe('font-hi');
});

test('on the wall the JSON holds every pen with its counts and the SVG every lit cell', () => {
  const live = open({}, { still: true });
  live.draw(0);
  const json = live.scene.json();
  expect(json.pens).toHaveLength(PENS.length);
  expect(json.pens.find((one) => one.char === 'A')).toMatchObject({ strokes: 2, floor: 2, flag: '' });
  const cells = PENS.reduce((n, one) => n + one.frames.at(-1).length, 0);
  expect(live.scene.svg().match(/<rect /g).length).toBeLessThanOrEqual(cells);
  expect(live.scene.name()).toBe('font-pens');
});

test('an export is named for the text, the sample the seed picks when it is blank, or the wall', () => {
  expect(PAGE.name).toBe(named);
  const blank = open({ view: 'text', text: '' });
  expect([named({ view: 'text', text: 'Hi there', seed: 1 }), blank.scene.name(), named({ view: 'pens', text: 'hi', seed: 1 })]).toEqual(['font-hi-there', `font-${slug(blank.scene.plan().text)}`, 'font-pens']);
});

test('a new seed on the wall rewrites the pens in a new order, not all at once', () => {
  const end = FRAME * 10000;
  const live = open({ loop: 0 });
  live.draw(end);
  const full = live.rects().length;
  live.scene.write();
  live.draw(end + FRAME * 3);
  const once = live.rects().length;
  live.draw(end * 2);
  live.set({ seed: 2 });
  live.draw(end * 2 + FRAME * 3);
  expect(live.rects().length).toBeLessThan(once);
  live.draw(end * 3);
  expect(live.rects()).toHaveLength(full);
});

test('a blank text takes the sample the seed picks, and a new seed picks again', () => {
  const live = open({ view: 'text', text: '  ' });
  expect(SAMPLES).toContain(live.scene.plan().text);
  expect(live.scene.plan().text).toBe(open({ view: 'text', text: '' }).scene.plan().text);
  expect(defaults(SPEC).text).toBe('');
  const seen = new Set([live.scene.plan().text]);
  for (let seed = 2; seed < 12; seed++) {
    live.set({ seed });
    seen.add(live.scene.plan().text);
  }
  expect(seen.size).toBeGreaterThan(1);
});

test('a speed change keeps the frame on the stage and plays on at the new rate', () => {
  const live = open({ view: 'text', text: 'hello', loop: 0 });
  live.draw(FRAME * 10.5);
  expect(live.rects()).toHaveLength(10);
  live.set({ speed: 4 });
  expect(live.rects()).toHaveLength(10);
  live.draw(FRAME * 10.5 + FRAME / 4);
  expect(live.rects()).toHaveLength(11);
});

test('a glyph added to the text goes on from the written text and writes only itself', () => {
  const live = open({ view: 'text', text: 'hi', loop: 0 });
  live.draw(100000);
  const was = live.scene.plan().still;
  live.set({ text: 'hi!' });
  expect(live.rects()).toHaveLength(was);
  live.draw(100000 + FRAME);
  expect(live.rects()).toHaveLength(was + 1);
  live.set({ text: 'ho' });
  expect(live.rects().length).toBeLessThan(was + 1);
});

test('a hold change on the text keeps the picture', () => {
  const live = open({ view: 'text', text: 'hi', hold: 10 });
  live.draw(FRAME * 30);
  const before = live.rects();
  live.set({ hold: 40 });
  expect(live.rects()).toEqual(before);
});

test('a hold or loop change on the wall keeps every pen where it was', () => {
  const live = open({ hold: 5 });
  live.draw(FRAME * 9);
  const before = live.rects();
  live.set({ hold: 50 });
  expect(live.rects()).toEqual(before);
  live.set({ loop: 0 });
  expect(live.rects()).toEqual(before);
});

test('a tap on a pen rewrites that pen alone; Write rewrites them all; a tap on the text rewrites it', () => {
  const live = open({}, { w: 780, h: 1400 });
  live.draw(100000);
  const whole = live.rects().length;
  const box = live.scene.box();
  const [cx, cy] = place(box, 0);
  live.log.length = 0;
  expect(live.scene.tap(box.x + (cx + 3) * box.cell, box.y + (cy + 3) * box.cell)).toBe(0);
  expect(live.rects()).toHaveLength(whole - PENS[0].frames.at(-1).length);
  expect(live.scene.tap(0, 0)).toBe(-1);
  live.log.length = 0;
  live.scene.write();
  expect(live.rects()).toHaveLength(0);
  const text = open({ view: 'text', text: 'hi', loop: 0 });
  text.draw(100000);
  text.log.length = 0;
  text.scene.tap(10, 10);
  expect(text.rects()).toHaveLength(0);
});

test('a view change keeps the canvas and starts the new view from empty', () => {
  const live = open({});
  live.draw(100000);
  live.set({ view: 'text', text: 'hi' });
  expect(live.rects()).toHaveLength(0);
  expect(live.plans.at(-1)).toMatchObject({ text: 'hi', rows: 7, cols: 11 });
  live.draw(100000 + FRAME);
  expect(live.rects()).toHaveLength(1);
});
