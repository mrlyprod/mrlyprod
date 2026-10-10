import { expect, test } from 'bun:test';
import * as font from '../../../pkgs/mrlyjs/font.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { SAMPLES } from './engine.js';
import { SPEC, layout, make } from './scene.js';

font.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/font/mrlyjs_font_bg.wasm', import.meta.url)).arrayBuffer() });

const open = (value, { still = false, w = 800, h = 600, dpr = 2 } = {}) => {
  const log = [];
  const ctx = new Proxy({}, { get: (_, key) => (...args) => log.push([key, ...args]), set: () => true });
  const view = { rand: rng(1), look: () => ({ paper: '#ffffff', accent: '#008cff' }), still, w, h, dpr, t: 0 };
  const scene = make({ getContext: () => ctx }, view, { ...defaults(SPEC), ...value, font });
  const rects = () => log.filter(([key]) => key === 'rect');
  const draw = (t) => {
    log.length = 0;
    view.t = t;
    scene.draw();
  };
  return { log, view, scene, rects, draw };
};

test('the scene paints the frame of t and repaints only when the frame changes', () => {
  const live = open({ text: 'hi', loop: 0 });
  live.draw(0);
  expect(live.rects()).toHaveLength(0);
  expect(live.log.map(([key]) => key)).toContain('clearRect');
  live.draw(20);
  expect(live.log).toEqual([]);
  live.draw(40);
  expect(live.rects()).toHaveLength(1);
  live.draw(100000);
  expect(live.rects()).toHaveLength(live.scene.plan.frames.at(-1).length);
  live.draw(100100);
  expect(live.log).toEqual([]);
});

test('under reduced motion the still is the finished text, the same at any t', () => {
  const live = open({ text: 'hi' }, { still: true });
  live.draw(0);
  const full = live.rects();
  expect(full).toHaveLength(live.scene.plan.frames[live.scene.plan.still].length);
  live.draw(5000);
  expect(live.log).toEqual([]);
  live.scene.theme();
  expect(live.rects()).toEqual(full);
});

test('cell 0 fits the board to the stage in whole pixels, cell n gives n px times the dpr, centred, and a cell too big for the stage fits instead', () => {
  const view = { w: 800, h: 600, dpr: 2 };
  expect(layout(view, { rows: 7, cols: 11 }, 0)).toMatchObject({ cell: 66, w: 726, h: 462, x: 37, y: 69 });
  expect(layout(view, { rows: 7, cols: 11 }, 10)).toEqual({ x: 290, y: 230, w: 220, h: 140, cell: 20 });
  expect(layout(view, { rows: 7, cols: 481 }, 40)).toEqual(layout(view, { rows: 7, cols: 481 }, 0));
  expect(layout(view, { rows: 0, cols: 0 }, 0).cell).toBeGreaterThan(0);
});

test('the handle gives the animation as JSON at the played rate and the shown frame as an SVG of rects', () => {
  const live = open({ text: 'hi', speed: 2, cell: 10 });
  live.draw(40);
  const json = live.scene.json();
  expect(json).toMatchObject({ text: 'hi', pad: 1, hold: 25, loop: 1, rows: 7, cols: 11, fps: 50 });
  expect(json.frames).toBe(live.scene.plan.frames);
  const still = live.scene.svg();
  const lit = live.scene.plan.frames[2];
  expect(lit).toHaveLength(2);
  expect(still.match(/<rect /g)).toHaveLength(2);
  expect(still).toContain('width="110" height="70"');
  expect(still).toContain(`<rect x="${(lit[0] % 11) * 10}" y="${Math.floor(lit[0] / 11) * 10}" width="10" height="10"/>`);
  expect(still).toContain('fill="#008cff"');
});

test('a blank text takes the sample the seed picks', () => {
  const live = open({ text: '  ' });
  expect(SAMPLES).toContain(live.scene.plan.text);
  expect(live.scene.plan.text).toBe(open({ text: '' }).scene.plan.text);
  expect(defaults(SPEC).text).toBe('');
});
