import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults, describe } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { EDGE, FADE, FAINT, FILL, HOLD, PAD, SOFT, schedule } from './engine.js';
import { SPEC, make, mip } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });
num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (...args) => log.push([key, ...args.map((arg) => (arg && typeof arg === 'object' ? `page${arg.width}` : arg))]),
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
};

const open = (value, still = false, w = 800, h = 600) => {
  const log = [];
  const told = [];
  const tiles = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 2, t: 0 };
  view.wake = () => {
    view.still = false;
    told.push('wake');
  };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 1, base: 2, code: '7', number: 2, top: 20, math, num, onTile: (tile) => tiles.push(tile.n), ...value });
  const quiet = log.length;
  const at = (t) => {
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  return { view, scene, told, tiles, log, quiet, at };
};

const TIMES = schedule(2, 20);

const laid = (log) => log.filter(([name]) => name === 'fillRect').length;

const strokes = (log) => log.filter(([name]) => name === 'strokeRect').length;

const art = (log) => log.filter(([name]) => name === 'drawImage').map(([, page]) => page);

const bursts = (log) => {
  const runs = [];
  let run = 0;
  for (const [name] of log) {
    if (name === 'drawImage') run++;
    else if (run) {
      runs.push(run);
      run = 0;
    }
  }
  return runs;
};

const alphas = (log) => log.filter(([name]) => name === 'globalAlpha').map(([, a]) => a);

test('make draws nothing and presses one stack of mips per level in the accent', () => {
  pages.length = 0;
  const live = open({});
  expect(live.quiet).toBe(0);
  const sides = pages.map((page) => page.width);
  expect(sides).toEqual([2, 1, 4, 2, 1, 8, 4, 2, 1, 16, 8, 4, 2, 1]);
  expect(pages[9].log).toContainEqual(['fillStyle', '#008cff']);
  expect(pages[9].log.filter(([name]) => name === 'rect').reduce((sum, [, , , len]) => sum + len, 0)).toBe(3 ** 4);
  expect(pages[10].log).toContainEqual(['drawImage', 'page16', 0, 0, 8, 8]);
});

test('the first draw lays the first tile alone, then the tiles follow the clock', () => {
  const live = open({});
  const first = live.at(0);
  expect(laid(first)).toBe(1);
  expect(first.filter(([name]) => name === 'arc')).toHaveLength(1);
  expect(laid(live.at(TIMES.length / 2))).toBeGreaterThan(1);
  expect(laid(live.at(TIMES.length))).toBe(20);
});

test('a draw at the same t paints nothing more, a resize or a theme change repaints', () => {
  const live = open({});
  live.at(1000);
  expect(live.at(1000)).toEqual([]);
  live.scene.size();
  expect(laid(live.at(1000))).toBeGreaterThan(0);
  live.scene.theme();
  expect(laid(live.at(1000))).toBeGreaterThan(0);
  expect(live.at(1000)).toEqual([]);
});

test('a picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({});
  let last;
  for (let t = 0; t <= 1500; t += 100) last = [...stepped.at(t)];
  const jumped = open({});
  expect([...jumped.at(1500)]).toEqual(last);
  expect(laid(last)).toBeGreaterThan(2);
});

test('under reduced motion the one draw lays every tile with the whole box fitted inside the padding, a pinned cell too until Again wakes it', () => {
  for (const cell of [0, 2]) {
    const still = open({ cell }, true);
    const log = still.at(0);
    expect(laid(log)).toBe(20);
    const boxes = log.filter(([name]) => name === 'fillRect');
    const left = Math.min(...boxes.map(([, x]) => x));
    const right = Math.max(...boxes.map(([, x, , w]) => x + w));
    const top = Math.min(...boxes.map(([, , y]) => y));
    const bottom = Math.max(...boxes.map(([, , y, , h]) => y + h));
    expect(bottom - top).toBeCloseTo(600 - 4 * PAD);
    expect(right - left).toBeLessThanOrEqual(800 - 4 * PAD + 1e-6);
    expect((top + bottom) / 2).toBeCloseTo(300);
    expect((left + right) / 2).toBeCloseTo(400);
    expect(still.at(0)).toEqual([]);
  }
  const pinned = open({ cell: 2 }, true);
  pinned.at(0);
  pinned.scene.again();
  expect(pinned.at(200).filter(([name, , , w]) => name === 'fillRect' && w === 4)).toHaveLength(1);
});

test('the shell holds whole and still after the drawing, fades out over a second, and the next drawing fades in from nothing', () => {
  const live = open({});
  expect(laid(live.at(TIMES.length))).toBe(20);
  expect(live.at(TIMES.length + HOLD / 2)).toEqual([]);
  const half = live.at(TIMES.length + HOLD + FADE / 2);
  expect(laid(half)).toBe(20);
  expect(Math.max(...alphas(half))).toBeCloseTo(0.5, 9);
  expect(Math.max(...alphas(live.at(TIMES.length + HOLD + FADE - 10)))).toBeLessThan(0.001);
  const next = live.at(TIMES.length + HOLD + FADE + 1);
  expect(laid(next)).toBe(1);
  expect(Math.max(...alphas(next))).toBeLessThan(0.01);
});

test('a grown tile is a faint fill, its art and a hairline, a composite at less alpha, a unit a bare rect', () => {
  const live = open({ growth: 'Prime' });
  const log = live.at(TIMES.length);
  expect(art(log)).toHaveLength(8);
  expect(strokes(log)).toBe(8);
  expect(laid(log) - art(log).length).toBe(12);
  const seen = alphas(log);
  expect(seen).toContain(1);
  expect(seen).toContain(FAINT);
  expect(seen).toContain(FILL);
  expect(seen).toContain(EDGE);
  expect(seen.every((a) => a > 0 && a <= 1)).toBe(true);
});

test('a tile drawn smaller than its art lays the art again to keep it legible, a crisp one once', () => {
  const big = schedule(3, 300).length;
  const crisp = open({ growth: 'Prime', number: 3, top: 300 }, false, 40000, 30000).at(big);
  const soft = open({ growth: 'Prime', number: 3, top: 300 }, false, 400, 300).at(big);
  expect(new Set(bursts(crisp))).toEqual(new Set([1]));
  expect(new Set(bursts(soft))).toEqual(new Set([SOFT]));
});

test('the mip picked is the smallest at or above the size drawn', () => {
  const stack = [{ width: 16 }, { width: 8 }, { width: 4 }, { width: 2 }, { width: 1 }];
  expect([mip(stack, 0.3).width, mip(stack, 1).width, mip(stack, 1.5).width, mip(stack, 8).width, mip(stack, 9).width, mip(stack, 99).width]).toEqual([1, 1, 2, 8, 16, 16]);
});

test('the path is one stroke through the centres laid so far to the head, and a path of 0 leaves only the head', () => {
  const live = open({});
  const log = live.at(TIMES.length);
  const moves = log.filter(([name]) => name === 'moveTo' || name === 'lineTo');
  expect(moves[0][0]).toBe('moveTo');
  expect(moves).toHaveLength(20);
  expect(log.filter(([name]) => name === 'stroke')).toHaveLength(1);
  const bare = open({ path: 0 }).at(TIMES.length);
  expect(bare.filter(([name]) => name === 'lineTo')).toHaveLength(0);
  expect(bare.filter(([name]) => name === 'arc')).toHaveLength(1);
});

test('the scene tells the number being laid once per tile', () => {
  const live = open({});
  live.at(0);
  live.at(200);
  live.at(TIMES.length / 2);
  live.at(TIMES.length);
  expect(live.tiles[0]).toBe(1);
  expect(live.tiles.at(-1)).toBe(20);
  expect(new Set(live.tiles).size).toBe(live.tiles.length);
});

test('again restarts the drawing from now and wakes a still scene', () => {
  const live = open({});
  live.at(TIMES.length);
  live.scene.again();
  expect(laid(live.at(TIMES.length + 10))).toBe(1);
  const still = open({}, true);
  still.at(0);
  still.scene.again();
  expect(still.told).toEqual(['wake']);
  expect(laid(still.at(1))).toBe(1);
});

test('the svg is the whole shell in the accent, its path following the knob', () => {
  const live = open({});
  const text = live.scene.svg();
  expect(text).toContain('fill="#008cff"');
  expect(text.match(/<use /g)).toHaveLength(19);
  expect(text).toContain('<path d="M');
  expect(open({ path: 0 }).scene.svg()).not.toContain('<path ');
});

test('the facts name the design and the knobs hide nothing', () => {
  const live = open({});
  expect(live.scene.facts).toMatchObject({ code: '7', name: 'carpet', tiles: 20, grown: 19, side: 16 });
  expect(describe(SPEC, defaults(SPEC)).map(({ name, rows }) => [name, rows.length])).toEqual([['Winding', 3], ['Look', 3]]);
});

test('the head dot is on the stage at every frame of the drawing, fitted or pinned, on a tall stage', () => {
  const length = schedule(3, 300).length;
  for (const cell of [0, 2]) {
    const live = open({ cell, number: 3, top: 300 }, false, 780, 1600);
    for (let t = 0; t <= length; t += 40) {
      const [, x, y] = live.at(t).find(([name]) => name === 'arc');
      expect(x >= 0 && x <= 780 && y >= 0 && y <= 1600).toBe(true);
    }
  }
});

test('a cell size centres the stage on the tile being laid at that scale', () => {
  const live = open({ cell: 2 });
  const log = live.at(200);
  const unit = log.find(([name, , , w]) => name === 'fillRect' && w === 4);
  expect(unit).toEqual(['fillRect', 398, 298, 4, 4]);
});
