import { expect, test } from 'bun:test';
import { defaults, describe, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { KINDS } from './chart.js';
import { SPEC, glyph, grow, make, shade } from './scene.js';

const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) => (key === 'measureText' ? (text) => ({ width: String(text).length * 7 }) : (...args) => log.push([key, ...args])),
      set: (_, key, value) => {
        log.push([key, value]);
        return true;
      },
    },
  );

const ROWS = [
  { key: '2-2-7-fills-level', label: 'carpet 7', measure: 'fills', axis: 'level', cells: [1, 1, 1, 1, 0, 1, 1, 1, 1], terms: ['8', '64', '512', '4096', '32768'] },
  { key: '2-2-7-euler-level', label: 'carpet 7', measure: 'euler', axis: 'level', terms: ['0', '-8', '-72', '-680', '-4680'] },
];

const open = (value, still = false) => {
  const log = [];
  const view = { rand: rng(1), look: () => ({ paper: '#ffffff', accent: '#008cff' }), still, w: 800, h: 500, dpr: 2, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), ...value });
  const frame = (t) => {
    log.length = 0;
    view.t = t;
    scene.draw();
    return [...log];
  };
  return { scene, view, frame };
};

test('the reveal grows pin by pin and lands on the full chart, which a still draws at once', () => {
  expect([grow(0, 0, 5), grow(1, 4, 5), grow(0.4, 4, 5), grow(0.4, 0, 5) > 0.9]).toEqual([0, 1, 0, true]);
  const live = open({ rows: ROWS });
  const early = live.frame(100);
  const done = live.frame(900);
  expect(early).not.toEqual(done);
  expect(live.frame(2000)).toEqual([]);
  const still = open({ rows: ROWS }, true);
  expect(still.frame(0)).toEqual(done);
});

test('every chart kind draws the rows without a throw, and no rows draw a hint', () => {
  for (const [chart] of KINDS) {
    const live = open({ rows: ROWS, chart, digits: 3, terms: 5 });
    expect([chart, live.frame(900).length > 20]).toEqual([chart, true]);
  }
  const empty = open({ rows: [] });
  expect(empty.frame(900)).toContainEqual(['fillText', 'pick a row', 400, 250]);
});

test('an all-zero chart draws its one rule, a chart with a top draws two', () => {
  const zeros = [{ key: '2-2-0-fills-level', label: 'empty 0', measure: 'fills', axis: 'level', terms: ['0', '0', '0', '0', '0'] }];
  const rules = (rows) => open({ rows, chart: 'steps' }, true).frame(0).filter(([key, text, x]) => key === 'fillText' && x < 110 && /^\d/.test(text));
  expect(rules(zeros).map(([, text]) => text)).toEqual(['0']);
  expect(rules([{ ...ROWS[0], terms: ['8', '21', '40', '65'] }]).map(([, text]) => text)).toEqual(['0', '65']);
});

test('rows wear their own shades of the accent, a live feed repaints only on a new version, and a still repaints on every draw', () => {
  expect(shade('#008cff', '#ffffff', 0, 3)).toBe('rgb(0, 140, 255)');
  expect(shade('#008cff', '#ffffff', 2, 3)).toBe('rgb(158, 211, 255)');
  expect(shade('#008cff', '#ffffff', 0, 1)).toBe('rgb(0, 140, 255)');
  const live = { current: { rows: ROWS.slice(0, 1), version: 1 } };
  const log = [];
  const view = { rand: rng(1), look: () => ({ paper: '#000000', accent: '#008cff' }), still: false, w: 400, h: 300, dpr: 1, t: 2000 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), live });
  scene.draw();
  const first = log.length;
  scene.draw();
  expect(log.length).toBe(first);
  live.current = { rows: ROWS, version: 2 };
  scene.draw();
  expect(log.length).toBeGreaterThan(first);
  expect(log.filter(([key, v]) => key === 'fillStyle' && v === 'rgb(0, 140, 255)').length).toBeGreaterThan(0);
  view.still = true;
  const before = log.length;
  scene.draw();
  expect(log.length).toBeGreaterThan(before);
});

test('the legend draws a row with cells as its side-3 design, a slice per layer, and a bare row as a swatch', () => {
  expect(glyph(ROWS[0].cells)).toEqual({ slices: 1, width: 3 });
  expect(glyph(Array(27).fill(1))).toEqual({ slices: 3, width: 11 });
  expect([glyph(undefined), glyph([1, 0])]).toEqual([null, null]);
  const boxes = open({ rows: ROWS }).frame(900).filter(([key]) => key === 'fillRect');
  expect(boxes).toHaveLength(10);
  expect(boxes.slice(0, 9).map(([, x]) => x)).toEqual([28, 35, 42, 28, 35, 42, 28, 35, 42]);
});

test('the cube offers base 2 alone: base 3 tidies to 2 and the Base row leaves the knobs', () => {
  expect([tidy(SPEC, { dim: 3, base: 3 }).base, tidy(SPEC, { dim: 2, base: 3 }).base]).toEqual([2, 3]);
  const keys = (value) => describe(SPEC, value).flatMap(({ rows }) => rows.map((row) => row.key));
  expect([keys({ dim: 2 }).includes('base'), keys({ dim: 3 }).includes('base')]).toEqual([true, false]);
});
