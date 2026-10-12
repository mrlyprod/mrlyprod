import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { asked, defaults, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { RDEN } from './engine.js';
import { PAGE, SPEC, make, name, plan, stage } from './scene.js';

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

const ACCENT = '#008cff';

const start = (over) => ({ ...defaults(SPEC), seed: 1, base: 2, code: '7', ...over });

const open = (value, w = 1600, h = 1000) => {
  const log = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: ACCENT }), still: false, w, h, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...start(value), math });
  const quiet = log.length;
  scene.draw();
  return { scene, log, made: log.slice(), quiet };
};

const said = (log) => log.filter(([name]) => name === 'fillText').map(([, text]) => text);

test('the scene is still: make draws nothing, and the first draw lays the crop alone on the stage', () => {
  const live = open({});
  expect([live.quiet, live.scene.still]).toEqual([0, true]);
  expect(live.made.slice(1, 3)).toEqual([['fillStyle', '#000000'], ['fillRect', 0, 0, 1600, 1000]]);
  expect(live.made[4]).toEqual(['fillRect', 350, 50, 900, 900]);
  expect(said(live.made)).toEqual([]);
  live.log.length = 0;
  live.scene.draw();
  expect(live.log).toEqual([]);
});

test('a radius set keeps the design and the circle count and only recounts the crop, and the count waits for its toggle', () => {
  const stages = {};
  expect(plan(math, start({}), rng(1), stages).table).toBe(null);
  const first = plan(math, start({ circle: 1 }), rng(1), stages);
  const moved = plan(math, start({ circle: 1, radius: 30 }), rng(1), stages);
  expect(moved.types).toBe(first.types);
  expect(moved.table).toBe(first.table);
  expect(moved.tally).not.toBe(first.tally);
  const r = plan(math, start({ circle: 1, radius: 30, r: 9 }), rng(1), stages);
  expect(r.tally).toBe(moved.tally);
  expect(r.table).toBe(first.table);
  expect(r.facts().circle).toMatchObject({ side: 729, r: 9, seen: r.table.seen[9] });
});

test('the exact edge clips the touching crop to the shape, and the anti crop to its outside', async () => {
  const clips = async (value) => {
    await open(value).scene.shot(512);
    return pages.at(-1).log.filter(([name]) => name === 'clip').map((row) => row.slice(1));
  };
  expect(await clips({})).toEqual([]);
  expect(await clips({ edge: 1 })).toEqual([[]]);
  expect(await clips({ edge: 1, mode: 'anti' })).toEqual([['evenodd']]);
  expect(open({ edge: 1, policy: 'inside' }).scene.facts().note).toBe('exact edge');
});

test('a drag anywhere on the stage sets the radius to the distance from the centre of the picture', () => {
  const live = open({});
  const box = stage(1600, 1000);
  const cx = box.x + box.w / 2;
  const cy = box.y + box.h / 2;
  expect(live.scene.drag(cx, cy)).toEqual({ radius: 0 });
  expect(live.scene.drag(cx + box.w / 4, cy)).toEqual({ radius: RDEN / 4 });
  expect(live.scene.drag(0, 0)).toEqual({ radius: 108 });
});

test('the picture is centred in nine tenths of the short side of any stage, a phone included', () => {
  const fits = (w, h) => {
    const box = stage(w, h);
    return [box.w === box.h, box.w === 0.9 * Math.min(w, h), box.x * 2 + box.w === w, box.y * 2 + box.h === h];
  };
  for (const [w, h] of [[1600, 1000], [780, 1690], [1000, 1000]]) expect(fits(w, h)).toEqual([true, true, true, true]);
});

test('the level, the count and r are kept as asked in the url and read at the cap, so a smaller number brings them back', () => {
  const raw = start({ number: 7, level: 5, count: 9, r: 700 });
  expect(asked(SPEC, raw, { math })).toMatchObject({ level: 5, count: 9, r: 700 });
  expect(tidy(SPEC, raw, { math })).toMatchObject({ level: 2, count: 3, r: 342 });
  const read = plan(math, raw, rng(1)).value;
  expect([read.level, read.count, read.r]).toEqual([2, 3, 342]);
  const back = plan(math, { ...raw, number: 3 }, rng(1)).value;
  expect([back.level, back.count, back.r]).toEqual([5, 6, 700]);
});

test('the svg holds a rect for each run of the kept cells, clipped by the true outline under the exact edge', () => {
  const plain = open({}).scene.svg();
  const kept = plan(math, start({}), rng(1)).kept.grid.data.reduce((sum, on) => sum + (on ? 1 : 0), 0);
  expect(plain.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="270" height="270"')).toBe(true);
  expect(plain).not.toContain('clipPath');
  expect([...plain.matchAll(/width="(\d+)" height="10"/g)].reduce((sum, [, w]) => sum + Number(w) / 10, 0)).toBe(kept);
  const edge = open({ edge: 1 }).scene.svg();
  expect(edge).toContain('<clipPath id="crop"><path d="M0 135a135 135');
  expect(open({ edge: 1, mode: 'anti' }).scene.svg()).toContain('<path d="M0 0H270V270H0ZM0 135a');
});

test('a shot draws the crop alone, square at the size, over the paper', async () => {
  const blob = await open({}).scene.shot(1024);
  const page = pages.at(-1);
  expect([page.width, page.height, blob.type]).toEqual([1024, 1024, 'image/png']);
  expect(page.log.slice(0, 2)).toEqual([['fillStyle', '#000000'], ['fillRect', 0, 0, 1024, 1024]]);
  expect(page.log.some(([name]) => name === 'fillText')).toBe(false);
});

test('an export is named by the design, the shape and the radius it shows', () => {
  expect([name(start({})), name(start({ shape: 'box', radius: 30, mode: 'anti' }))]).toEqual(['crop-7-ball-60', 'crop-7-box-30-anti']);
});

test('the primary flips the mode and names the one it goes to', () => {
  const rows = (value) => PAGE.keys.filter((row) => row.button && row.when(value)).map((row) => row.label);
  expect(rows({ mode: 'crop' })).toEqual(['Anti']);
  expect(rows({ mode: 'anti' })).toEqual(['Crop']);
});
