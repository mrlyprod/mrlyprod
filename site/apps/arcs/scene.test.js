import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults, describe as groups } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { HOLD, PAD, drift, fit, study } from './engine.js';
import { SPEC, make } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

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

let owner = sheets;

const arcs = (log) => log.filter(([name]) => name === 'arc').length;

globalThis.OffscreenCanvas = class {
  constructor(w, h) {
    this.width = w;
    this.height = h;
    this.log = [];
    sheets.push(this);
    if (owner !== sheets) owner.push(this);
  }
  getContext() {
    return pen(this.log);
  }
};

const open = (value, still = false, w = 800, h = 600) => {
  const log = [];
  const mine = [];
  owner = mine;
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still, w, h, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), seed: 1, base: 3, code: '495', level: 2, math, ...value });
  const at = (t) => {
    owner = mine;
    view.t = t;
    log.length = 0;
    scene.draw();
    return log;
  };
  const laid = () => mine.reduce((sum, sheet) => sum + arcs(sheet.log), 0);
  return { view, scene, log, at, mine, laid };
};

const PLAN = study(math, { ...defaults(SPEC), seed: 1, base: 3, code: '495', level: 2 }, rng(1));
const ARCS = 2 * PLAN.side * PLAN.side;
const LOOPED = Array.from(PLAN.lit).reduce((sum, k) => sum + PLAN.bounds[k + 1] - PLAN.bounds[k], 0);
const SPAN = PLAN.clock.span;

const moves = (log) => log.filter(([name]) => name === 'setTransform').map(([, ...matrix]) => matrix);

test('a picture at a t is the same reached in one jump or by steps', () => {
  const stepped = open({});
  for (let t = 0; t <= 2000; t += 100) stepped.at(t);
  const log = [...stepped.log];
  const jumped = open({});
  const once = [...jumped.at(2000)];
  expect(jumped.laid()).toBe(stepped.laid());
  expect(once).toEqual(log);
  expect(stepped.laid()).toBeGreaterThan(ARCS);
  expect(stepped.laid()).toBeLessThan(ARCS + LOOPED);
  expect(arcs(once)).toBeGreaterThan(0);
});

test('make draws nothing, the first draw lays every arc faint and the loops light in turn until all are lit and held', () => {
  const live = open({});
  expect(live.log).toEqual([]);
  expect(live.scene.every).toBe(undefined);
  const first = live.at(0);
  expect(live.laid()).toBe(ARCS);
  expect(first.filter(([name]) => name === 'drawImage')).toHaveLength(1);
  const [back, layer] = live.mine;
  expect(back.log).toContainEqual(['strokeStyle', 'rgb(0, 63, 115)']);
  expect(back.log).toContainEqual(['fillStyle', 'rgba(0, 140, 255, 0.08)']);
  expect(layer.log).toContainEqual(['drawImage', 'layer', 0, 0]);
  const done = live.at(SPAN);
  expect(live.laid()).toBe(ARCS + LOOPED);
  expect(arcs(done)).toBe(0);
  expect(layer.log).toContainEqual(['strokeStyle', '#008cff']);
  expect(arcs(live.at(SPAN + HOLD / 2))).toBe(0);
  expect(live.laid()).toBe(ARCS + LOOPED);
});

test('past the span and the hold the loops go faint again by copying the faint layer, with no arc stroked and no canvas made', () => {
  const live = open({});
  live.at(SPAN);
  const was = [sheets.length, live.laid()];
  live.at(SPAN + HOLD + 100);
  expect([sheets.length, live.laid()]).toEqual(was);
  const log = live.mine.at(-1).log;
  const cut = log.findLastIndex(([name]) => name === 'clearRect');
  expect(log[cut + 1]).toEqual(['drawImage', 'layer', 0, 0]);
  expect(arcs(log.slice(cut))).toBe(0);
  live.at(SPAN + HOLD + 200 + SPAN);
  expect(live.laid()).toBe(ARCS + 2 * LOOPED);
});

test('under reduced motion the first draw lays every loop lit and a second draw adds nothing', () => {
  const still = open({}, true);
  expect(arcs(still.at(0))).toBe(0);
  expect(still.laid()).toBe(ARCS + LOOPED);
  const was = [sheets.length, still.laid()];
  still.at(0);
  expect([sheets.length, still.laid()]).toEqual(was);
});

test('with wander off the scene is a still of lit loops: one draw lays it, a draw after is a no-op, and a resize or a theme lays it again', () => {
  const fixed = open({ wander: 0 });
  expect(fixed.scene.every).toBe(1000);
  expect(fixed.at(0).length).toBeGreaterThan(0);
  expect(fixed.laid()).toBe(ARCS);
  expect(fixed.mine.at(-1).log).toContainEqual(['strokeStyle', '#008cff']);
  expect(fixed.at(1000)).toEqual([]);
  const was = sheets.length;
  fixed.scene.size();
  expect(sheets.length).toBe(was + 1);
  fixed.scene.theme();
  expect(sheets.length).toBe(was + 2);
  expect(fixed.at(2000)).toEqual([]);
});

test('the look keeps the loops or the strands alone, and strands alone never animate', () => {
  const loops = open({ look: 'loops' }, true);
  loops.at(0);
  expect(loops.laid()).toBe(2 * LOOPED);
  const strands = open({ look: 'strands' });
  expect(strands.scene.every).toBe(1000);
  strands.at(0);
  expect(strands.laid()).toBe(ARCS - LOOPED);
  expect(strands.mine.at(-1).log).not.toContainEqual(['strokeStyle', '#008cff']);
  const bare = open({ cells: 0 }, true);
  bare.at(0);
  expect(bare.mine.some(({ log }) => log.some(([name]) => name === 'rect'))).toBe(false);
});

test('wander is a knob only where loops can light, so Show strands hides it', () => {
  const shown = (look) => groups(SPEC, { look }).flatMap((group) => group.rows.map((row) => row.key));
  expect([shown('all').includes('wander'), shown('loops').includes('wander'), shown('strands').includes('wander')]).toEqual([true, true, false]);
});

test('with no design key the scene draws base 3 at level 3 as the page does, a grid that loops', () => {
  const bare = open({ base: undefined, code: '', level: undefined });
  expect(bare.scene.facts).toMatchObject({ base: 3, level: 3, side: 27 });
  expect(bare.scene.facts.loops).toBeGreaterThan(0);
});

test('every loop in progress is traced although a packed schedule starts the loops out of order', () => {
  const packed = study(math, { ...defaults(SPEC), seed: 1, base: 3, code: '495', level: 4 }, rng(1));
  const { start, end, dur, span } = packed.clock;
  expect(Array.from(start).some((s, i) => i && s < start[i - 1])).toBe(true);
  const owed = (t) =>
    Array.from(packed.lit.keys()).reduce((sum, k) => {
      if (start[k] > t || end[k] <= t) return sum;
      const n = packed.bounds[packed.lit[k] + 1] - packed.bounds[packed.lit[k]];
      const f = (t - start[k]) / dur[k];
      const whole = Math.floor(f * n);
      return sum + whole + (f * n - whole > 0 ? 1 : 0);
    }, 0);
  const live = open({ level: 4 });
  for (const t of [span / 4, span / 2, (3 * span) / 4]) {
    expect(owed(t)).toBeGreaterThan(0);
    expect(arcs(live.at(t))).toBe(owed(t));
  }
});

test('a wandering picture sways by the drift of its t, and a still, a fixed or a resting one sits centred', () => {
  const box = fit(9, 800, 600, PAD * 2);
  const centred = [1, 0, 0, 1, 0, 0];
  const live = open({});
  expect(moves(live.at(0)).at(-1)).toEqual(centred);
  const [dx, dy] = drift(3000, box);
  expect([dx !== 0, dy !== 0]).toEqual([true, true]);
  expect(moves(live.at(3000)).at(-1)).toEqual([1, 0, 0, 1, dx, dy]);
  expect(moves(open({}, true).at(0)).at(-1)).toEqual(centred);
  expect(moves(open({ wander: 0 }).at(3000)).at(-1)).toEqual(centred);
});

test('the arcs sit on the square fitted to the stage with the stroke a share of a cell, never under a device pixel', () => {
  const live = open({ width: 0.2 }, true);
  live.at(0);
  const box = fit(9, 800, 600, PAD * 2);
  const log = live.mine.flatMap((sheet) => sheet.log);
  expect(log).toContainEqual(['lineWidth', 0.2 * box.px]);
  const centres = log.filter(([name]) => name === 'arc').map(([, x, y, r]) => [x, y, r]);
  expect(centres.every(([x, y, r]) => r === box.px / 2 && x >= box.x && x <= box.x + 9 * box.px && y >= box.y && y <= box.y + 9 * box.px)).toBe(true);
  const thin = open({ width: 0.04, level: 5 }, true);
  thin.at(0);
  expect(thin.mine.flatMap((sheet) => sheet.log)).toContainEqual(['lineWidth', 2]);
});

test('a design with no loops is a still and the svg is written in the shades on screen', () => {
  const none = open({ base: 2, code: '5', level: 3 });
  expect(none.scene.every).toBe(1000);
  expect(none.scene.facts).toMatchObject({ loops: 0, strands: 16 });
  const live = open({});
  live.at(100);
  const text = live.scene.svg();
  expect(text).toContain('stroke="rgb(0, 63, 115)"');
  expect(text).toContain('stroke="#008cff"');
  expect(text).toContain('fill="rgba(0, 140, 255, 0.08)"');
  expect(text.match(/A5 5 0 0 /g)).toHaveLength(ARCS);
});
