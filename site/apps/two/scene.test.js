import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { lay } from './engine.js';
import { SPEC, make } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args]),
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
    pages.push(this);
  }
  getContext() {
    return pen(this.log);
  }
  convertToBlob(options) {
    return Promise.resolve({ type: options.type, page: this });
  }
};

const open = (value, w = 800, h = 600) => {
  const log = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w, h, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 1, base: 3, code: '495', level: 2, math, ...value });
  const quiet = log.length;
  scene.draw();
  const made = log.slice();
  const at = () => {
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, log, made, quiet, at };
};

const rects = (log) => log.filter(([name]) => name === 'rect');

const cells = (log) => rects(log).reduce((sum, [, , , w]) => sum + Math.round(w / 12), 0);

test('make draws nothing, the first draw lays the sheet, one rect per run in the accent, and a draw repaints only after a resize or a theme change', () => {
  const live = open({});
  expect(live.quiet).toBe(0);
  expect(rects(live.made).length).toBeGreaterThan(0);
  expect(cells(live.made)).toBe(live.scene.facts.fills);
  expect(live.made).toContainEqual(['fillStyle', '#008cff']);
  expect(live.at()).toEqual([]);
  live.scene.size();
  expect(rects(live.log).length).toBeGreaterThan(0);
  live.scene.theme();
  expect(rects(live.log).length).toBeGreaterThan(0);
  expect(live.at()).toEqual([]);
});

test('the sheet sits centred inside nine tenths of the short side on whole pixels', () => {
  const live = open({ crop: '' });
  const box = lay(45, 45, 800, 600, 0.9);
  const drawn = rects(live.made);
  expect(drawn.every((row) => row.slice(1).every(Number.isInteger))).toBe(true);
  expect(Math.min(...drawn.map(([, x]) => x))).toBe(box.x);
  expect(Math.min(...drawn.map(([, , y]) => y))).toBe(box.y);
  expect(Math.max(...drawn.map(([, x, , w]) => x + w))).toBe(box.x + 45 * box.px);
  expect(Math.max(...drawn.map(([, , y, , h]) => y + h))).toBe(box.y + 45 * box.px);
});

test('the same seed gives the same picture, in one jump or by steps, and the inverse draws the voids', () => {
  expect(open({ code: '' }).made).toEqual(open({ code: '' }).made);
  expect(cells(open({ crop: '' }).made) + cells(open({ crop: '', invert: 1 }).made)).toBe(45 * 45);
});

test('the svg is one rect per run in the accent with no ground', () => {
  const live = open({});
  const text = live.scene.svg();
  expect(text.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="450" height="450" viewBox="0 0 450 450"')).toBe(true);
  expect(text.match(/<rect /g)).toHaveLength(rects(live.made).length);
  expect(text).toContain('fill="#008cff"');
  expect(text).not.toContain('fill="#000000"');
});

test('a shot draws the sheet edge to edge at the size over the paper and hands back a blob of the kind', async () => {
  const live = open({ x: 6, y: 3, crop: '' });
  const blob = await live.scene.shot(1024);
  const page = pages.at(-1);
  expect([page.width, page.height, blob.type]).toEqual([1024, 512, 'image/png']);
  expect(page.log.slice(0, 2)).toEqual([['fillStyle', '#000000'], ['fillRect', 0, 0, 1024, 512]]);
  const drawn = rects(page.log);
  expect(drawn.length).toBe(rects(live.made).length);
  expect(Math.max(...drawn.map(([, x, , w]) => x + w))).toBe(1024);
  expect(Math.max(...drawn.map(([, , y, , h]) => y + h))).toBe(512);
  expect((await live.scene.shot(512, 'webp')).type).toBe('image/webp');
});

test('the level runs to the cap of the side and the sheet, and the radius to the reach of the shape', () => {
  const level = (value) => tidy(SPEC, { ...defaults(SPEC), level: 5, ...value }).level;
  expect([level({ number: 3, x: 1, y: 1 }), level({ number: 3 }), level({ number: 5 }), level({ number: 7, x: 12, y: 12 })]).toEqual([5, 4, 2, 1]);
  const radius = (value) => tidy(SPEC, { ...defaults(SPEC), radius: 32, ...value }).radius;
  expect([radius({ crop: 'ball' }), radius({ crop: 'box' }), radius({ crop: 'diamond' }), radius({ crop: 'octagon' }), radius({ crop: 'triangle' }), radius({ crop: '' })]).toEqual([23, 16, 32, 22, 32, 32]);
});
