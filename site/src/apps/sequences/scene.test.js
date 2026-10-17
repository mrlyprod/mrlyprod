import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import { defaults, describe, tidy } from '../../lib/app/spec.js';
import { rng } from '../../lib/rand.js';
import { pen, said } from '../../../test/pen.js';
import { PAGE, SPEC, make, named } from './scene.js';

const ROWS = [
  { key: '2-2-7-fills-level', dim: 2, base: 2, code: '7', label: 'carpet 7', measure: 'fills', axis: 'level', cells: [1, 1, 1, 1, 0, 1, 1, 1, 1], terms: ['8', '64', '512', '4096', '32768'] },
  { key: '2-2-7-euler-level', dim: 2, base: 2, code: '7', label: 'carpet 7', measure: 'euler', axis: 'level', cells: [1, 1, 1, 1, 0, 1, 1, 1, 1], terms: ['0', '-8', '-72', '-680', '-4680'] },
];

const open = (rows, still = false, value = {}) => {
  const log = [];
  const live = { current: { rows, version: 1 } };
  const view = { rand: rng(1), look: () => ({ paper: '#ffffff', accent: '#008cff' }), still, w: 800, h: 500, dpr: 1, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { ...defaults(SPEC), ...value, live, math });
  const frame = (t) => {
    log.length = 0;
    view.t = t;
    scene.draw();
    return [...log];
  };
  return { scene, view, live, frame, log };
};

test('no rows draw a hint, and a picked row draws its design beside the chart with its side and level', () => {
  expect(said(open([]).frame(0))).toContain('pick a row');
  const one = open(ROWS.slice(0, 1), true).frame(0);
  expect(said(one)).toContain('side 3, level 5');
  expect(one.filter(([key]) => key === 'rect').length).toBe(32768);
});

test('a live feed repaints only on a new version or while new terms grow in, and a still repaints on every draw', () => {
  const live = open(ROWS.slice(0, 1));
  expect(live.frame(0).length).toBeGreaterThan(0);
  expect(live.frame(1000).length).toBeGreaterThan(0);
  expect(live.frame(1100)).toEqual([]);
  live.live.current = { rows: ROWS, version: 2 };
  expect(live.frame(1200).length).toBeGreaterThan(0);
  const still = open(ROWS, true);
  expect(still.frame(0)).toEqual(still.frame(0));
  expect(still.frame(0).length).toBeGreaterThan(0);
});

test('set takes a new chart without replaying the reveal: the next frame paints the new kind at once', () => {
  const live = open(ROWS.slice(0, 1));
  live.frame(0);
  live.frame(1000);
  expect(live.frame(1100)).toEqual([]);
  live.scene.set({ chart: 'diffs', digits: 2 });
  const next = live.frame(1110);
  expect(said(next)).toContain('each row the differences of the row above');
  expect(said(next)).toContain('32768');
});

test('a tap on a pin draws the design at that term, a tap on a legend row names the row, and the arrows step the term', () => {
  const live = open(ROWS.slice(0, 1), true);
  live.frame(0);
  const slot = live.log.length;
  expect(slot).toBeGreaterThan(0);
  const got = live.scene.tap(560, 300);
  expect(got?.term).toBeGreaterThanOrEqual(0);
  expect(said(live.frame(0))).toContain(`side 3, level ${got.term + 1}`);
  live.scene.step(-1);
  expect(said(live.frame(0))).toContain(`side 3, level ${Math.max(1, got.term)}`);
  const two = open(ROWS, true);
  two.frame(0);
  expect(two.scene.tap(300, 22)).toEqual({ row: 1 });
});

test('the chart knobs come first, the digit base shows only on the digit chart, and the cube hides its base', () => {
  expect(describe(SPEC, defaults(SPEC))[0].name).toBe('Chart');
  const keys = (value) => describe(SPEC, { ...defaults(SPEC), ...value }).flatMap(({ rows }) => rows.map((row) => row.key));
  expect([keys({ chart: 'pins' }).includes('digits'), keys({ chart: 'digits' }).includes('digits')]).toEqual([false, true]);
  expect([keys({ dim: 2 }).includes('base'), keys({ dim: 3 }).includes('base')]).toEqual([true, false]);
  expect([tidy(SPEC, { dim: 3, base: 3 }).base, tidy(SPEC, { dim: 2, base: 3 }).base, tidy(SPEC, { dim: '', base: '' }).dim]).toEqual(['', 3, '']);
  expect(PAGE.about.length).toBeGreaterThanOrEqual(2);
});

test('an export is named by the lead row picked, or the seed when none is, and the chart', () => {
  expect(PAGE.name).toBe(named);
  expect([named({ seed: 9, pick: '2-2-7-fills-side.2-3-495-fills-level', chart: 'pins' }), named({ seed: 9, pick: '', chart: 'log' })]).toEqual(['sequences-2-2-7-fills-side-pins', 'sequences-9-log']);
});
