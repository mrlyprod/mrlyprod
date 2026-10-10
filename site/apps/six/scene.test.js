import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { SPEC, make, tones } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

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

const open = (value, w = 800, h = 600) => {
  const log = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w, h, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 1, base: 2, code: '23', math, ...value });
  const quiet = log.length;
  scene.draw();
  const count = (name) => log.filter(([key]) => key === name).length;
  return { log, view, scene, count, quiet };
};

test('make paints nothing, the first draw paints one filled path per type with no stroke, and the scene rests until a change', () => {
  const live = open({ projection: 'cut', level: 1 });
  expect(live.quiet).toBe(0);
  expect([live.count('beginPath'), live.count('fill'), live.count('stroke'), live.count('moveTo'), live.scene.every]).toEqual([2, 2, 0, 54, 1000]);
  expect(live.log.filter(([key]) => key === 'fillStyle').map(([, v]) => v)).toEqual(['rgba(0, 140, 255, 0.102)', 'rgba(0, 140, 255, 1)']);
  live.log.length = 0;
  live.scene.draw();
  live.view.t = 1000;
  live.scene.draw();
  expect(live.log).toEqual([]);
  live.scene.theme();
  expect(live.count('moveTo')).toBe(54);
  live.log.length = 0;
  live.scene.size();
  expect(live.count('moveTo')).toBe(54);
});

test('the hexagon is centred in the canvas at either aspect and spins a quarter for the other point', () => {
  const centre = (log) => {
    const xs = log.filter(([key]) => key === 'moveTo' || key === 'lineTo');
    const x = xs.map(([, v]) => v);
    const y = xs.map(([, , v]) => v);
    return [Math.min(...x) + Math.max(...x), Math.min(...y) + Math.max(...y), Math.max(...x) - Math.min(...x), Math.max(...y) - Math.min(...y)];
  };
  const wide = open({ projection: 'cut', level: 1, orient: 'flat' }, 800, 600);
  const [cx, cy, w, h] = centre(wide.log);
  expect([cx, cy]).toEqual([800, 600]);
  expect(w).toBeGreaterThan(h);
  const tall = open({ projection: 'cut', level: 1, orient: 'pointy' }, 800, 600);
  const [tx, ty, tw, th] = centre(tall.log);
  expect([tx, ty]).toEqual([800, 600]);
  expect(th).toBeGreaterThan(tw);
  expect(Math.round(th)).toBe(Math.round(600 - 2 * 48));
});

test('the iso shades its three faces from the accent toward the paper', () => {
  const live = open({ projection: 'iso', level: 1 });
  expect(live.log.filter(([key]) => key === 'fillStyle').map(([, v]) => v)).toEqual(['rgba(0, 140, 255, 1)', 'rgba(0, 101, 184, 1)', 'rgba(0, 70, 128, 1)']);
  expect(tones({ accent: '#ffffff', paper: '#000000' }).LEFT).toEqual([184, 184, 184, 255]);
});

test('the svg is the interlocking rect tile of one hexagon in the accent at the true aspect', () => {
  const live = open({ projection: 'cut', level: 1 });
  const text = live.scene.svg();
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="')).toBe(true);
  expect(text).toContain('rotate(90)');
  expect(text).toContain('fill="#008cff"');
  expect(text).toContain('fill="#008cff1a"');
  expect(text.match(/<polygon/g).length).toBeGreaterThan(54);
  expect(open({ projection: 'cut', level: 1, orient: 'flat' }).scene.svg()).not.toContain('rotate');
});

test('the scene hands its facts to the widget and wants the unit', () => {
  const live = open({ projection: 'cut', level: 2, radius: 1 });
  expect(live.scene.facts).toMatchObject({ code: '23', name: 'carpet', side: 9, copies: 7, fills: 306, holes: 7 });
  expect(() => make({ getContext: () => pen([]) }, live.view, { ...defaults(SPEC), seed: 1 })).toThrow('six: the scene wants mrlyjs/math as opts.math');
});

test('the level runs to the cap of the number and the rings', () => {
  const level = (value) => tidy(SPEC, { ...defaults(SPEC), level: 3, ...value }).level;
  expect([level({ number: 3 }), level({ number: 5 }), level({ number: 9 }), level({ number: 3, radius: 1 }), level({ number: 3, radius: 4 }), level({ number: 7, radius: 1 })]).toEqual([3, 2, 1, 3, 2, 1]);
});
