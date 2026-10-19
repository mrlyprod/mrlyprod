import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { asked, defaults, tidy } from '../../lib/app/spec.js';
import { rng } from '../../lib/rand.js';
import { PAGE, SPEC, make, tones } from './scene.js';

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args]),
      set: (_, key, value) => {
        log.push([key, value]);
        return true;
      },
    },
  );

const VALUE = { ...defaults(SPEC), seed: 1, base: 2, code: '23' };

const open = (value, w = 800, h = 600) => {
  const log = [];
  const told = [];
  const asked = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w, h, dpr: 2, t: 0 };
  const canvas = { getContext: (kind) => (asked.push(kind), pen(log)) };
  const scene = make(canvas, view, { ...VALUE, math, onFacts: (facts) => told.push(facts), ...value });
  const quiet = log.length;
  scene.draw();
  const count = (name) => log.filter(([key]) => key === name).length;
  return { log, view, scene, count, quiet, told, asked };
};

const ONE = { view: 'cut', level: 1, x: 1, y: 1 };

test('the sheet is a still: make paints nothing, a draw paints one filled path per type with no stroke', () => {
  const live = open(ONE);
  expect([live.quiet, live.scene.still, live.scene.every]).toEqual([0, true, undefined]);
  expect([live.count('beginPath'), live.count('fill'), live.count('stroke'), live.count('moveTo')]).toEqual([2, 2, 0, 54]);
  expect(live.log.filter(([key]) => key === 'fillStyle').map(([, v]) => v)).toEqual(['rgba(0, 140, 255, 0.102)', 'rgba(0, 140, 255, 1)']);
});

test('a set keeps the canvas and tells the new facts, and the draw after it paints the new sheet', () => {
  const live = open(ONE);
  live.log.length = 0;
  live.scene.set({ ...VALUE, ...ONE, x: 2 });
  live.scene.draw();
  expect([live.count('moveTo'), live.told.at(-1).copies, live.asked.length]).toEqual([108, 2, 1]);
  live.log.length = 0;
  live.scene.set({ ...VALUE, ...ONE, x: 2, tint: 'red' });
  live.scene.draw();
  expect([live.count('moveTo'), live.told.at(-1)]).toEqual([108, live.told.at(-2)]);
});

test('the hexagon is centred in the canvas and a quarter turn stands it on its side', () => {
  const centre = (log) => {
    const xs = log.filter(([key]) => key === 'moveTo' || key === 'lineTo');
    const x = xs.map(([, v]) => v);
    const y = xs.map(([, , v]) => v);
    return [Math.min(...x) + Math.max(...x), Math.min(...y) + Math.max(...y), Math.max(...x) - Math.min(...x), Math.max(...y) - Math.min(...y)];
  };
  const [cx, cy, w, h] = centre(open(ONE).log);
  expect([cx, cy]).toEqual([800, 600]);
  expect(w).toBeGreaterThan(h);
  const [tx, ty, tw, th] = centre(open({ ...ONE, turn: 1 }).log);
  expect([tx, ty]).toEqual([800, 600]);
  expect(th).toBeGreaterThan(tw);
  expect(Math.round(th)).toBe(Math.round(600 - 2 * 48));
});

test('the iso shades its three faces from the accent toward the paper', () => {
  const live = open({ ...ONE, view: 'iso' });
  expect(live.log.filter(([key]) => key === 'fillStyle').map(([, v]) => v)).toEqual(['rgba(0, 140, 255, 1)', 'rgba(0, 101, 184, 1)', 'rgba(0, 70, 128, 1)']);
  expect(tones({ accent: '#ffffff', paper: '#000000' }).LEFT).toEqual([184, 184, 184, 255]);
});

test('the svg is the sheet as drawn and the tile svg the interlocking rect of one hexagon, both in the accent', () => {
  const live = open({ ...ONE, x: 2 });
  const sheet = live.scene.svg();
  expect(sheet.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="')).toBe(true);
  expect(sheet).toContain('fill="#008cff"');
  expect(sheet).toContain('fill="#008cff1a"');
  expect(sheet).not.toContain('rotate');
  expect(sheet.match(/fill="#008cff/g).length).toBe(108);
  const tile = live.scene.tile();
  expect(tile.match(/<polygon/g).length).toBeGreaterThan(54);
  expect(open({ ...ONE, turn: 1 }).scene.svg()).toContain('rotate(90)');
});

test('the scene tells its facts at make and wants the unit', () => {
  const live = open({ view: 'cut', level: 2, sheet: 'disc', radius: 1 });
  expect(live.told[0]).toMatchObject({ code: '23', name: 'carpet', side: 9, copies: 7, fills: 7 * 306 });
  expect(() => make({ getContext: () => pen([]) }, live.view, VALUE)).toThrow('six: the scene wants @mrlyprod/mrly/math as opts.math');
});

test('the level runs to the cap of the side and the copies, and the url keeps the level asked for', () => {
  const level = (value) => tidy(SPEC, { ...VALUE, level: 3, ...value }, { math }).level;
  const one = { x: 1, y: 1 };
  expect([level({ ...one, number: 3 }), level({ ...one, number: 5 }), level({ ...one, number: 9 }), level({}), level({ sheet: 'disc', radius: 1 }), level({ sheet: 'disc', radius: 4 })]).toEqual([3, 2, 1, 2, 3, 2]);
  expect(asked(SPEC, { ...VALUE, level: 3 }, { math })).toEqual({ level: 3 });
});

test('the view leads the spec as a visible row, so a phone gets the switch', () => {
  expect([SPEC[0].key, SPEC[0].when, SPEC[0].group]).toEqual(['view', undefined, undefined]);
});

test('an export is named by the design shown and its view', () => {
  expect([PAGE.name({ ...VALUE, view: 'iso' }, { math }), PAGE.name({ ...VALUE, code: '', seed: 5 }, { math })]).toEqual(['six-23-iso', 'six-23-cut']);
});

test('Enter crops a rectangle of two or more each way, uncrops a cropped one, and leaves a disc alone', () => {
  const enter = (value) => PAGE.keys.filter((row) => row.key === 'Enter' && row.when({ ...VALUE, ...value })).map((row) => row.label);
  expect([enter({}), enter({ overhang: 0 }), enter({ x: 1 }), enter({ sheet: 'disc' })]).toEqual([['Crop'], ['Uncrop'], [], []]);
});
