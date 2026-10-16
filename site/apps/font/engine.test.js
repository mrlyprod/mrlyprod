import { expect, test } from 'bun:test';
import * as font from '@mrlyprod/mrly/font';
import { rng } from '../../lib/scene.js';
import { KINDS, LIMIT, SAMPLES, cells, common, facts, flag, gallery, index, pen, plan, sample, shift, slug } from './engine.js';

const full = (text) => font.raster(text).reduce((n, row) => n + row.reduce((m, bit) => m + bit, 0), 0);

const spots = (show, k) => show.frames[k].map((i) => `${Math.floor(i / show.cols)},${i % show.cols}`).sort().join(' ');

test('a looping plan writes one cell a frame up to the full text, folds it and comes back to nothing, in four movements', () => {
  const show = plan(font, { text: 'hi', pad: 1, hold: 0, loop: 1 });
  expect([show.rows, show.cols, show.fps]).toEqual([7, 11, font.FPS()]);
  expect(show.frames[0]).toEqual([]);
  expect(show.frames[show.still]).toHaveLength(full('hi'));
  expect(show.frames.slice(0, show.still + 1).map((frame) => frame.length)).toEqual(Array.from({ length: show.still + 1 }, (_, i) => i));
  expect(show.frames.at(-1)).toEqual([]);
  expect(show.moves.reduce((n, len) => n + len, 0)).toBe(show.frames.length);
});

test('a plan with loop off ends on the finished text', () => {
  const show = plan(font, { text: 'hi', loop: 0 });
  expect(show.frames.length).toBe(show.still + 1);
  expect(show.moves).toEqual([show.still + 1]);
  expect(show.frames.at(-1)).toHaveLength(full('hi'));
});

test('hold rests the loop after each of its four movements', () => {
  const rested = plan(font, { text: 'hi', hold: 7 }).frames.length;
  expect(rested - plan(font, { text: 'hi', hold: 0 }).frames.length).toBe(4 * 7);
});

test('a frame index wraps when looping and clamps otherwise', () => {
  expect([0, 9, 10, 23, -1].map((k) => index(k, 10, 1))).toEqual([0, 9, 0, 3, 9]);
  expect([-3, 0, 9, 50].map((k) => index(k, 10, 0))).toEqual([0, 0, 9, 9]);
  expect(index(99, 1, 1)).toBe(0);
});

test('a pen writes its frames, rests hold frames on the full glyph, then starts again; with loop off it stays written', () => {
  expect(Array.from({ length: 9 }, (_, k) => pen(k, 4, 2))).toEqual([0, 1, 2, 3, 3, 3, 0, 1, 2]);
  expect([0, 3, 40].map((k) => pen(k, 4, 2, 0))).toEqual([0, 3, 3]);
  expect(pen(7, 1, 0)).toBe(0);
});

test('a hold change keeps the picture: every frame lands on the same cells in the plan of the new hold', () => {
  const was = plan(font, { text: 'hi', hold: 3 });
  for (const hold of [0, 1, 7]) {
    const next = plan(font, { text: 'hi', hold });
    expect(was.frames.map((_, k) => spots(was, k))).toEqual(was.frames.map((_, k) => spots(next, shift(k, was.moves, was.hold, next.hold))));
  }
});

test('a longer text writes the shorter one first, frame for frame, so an added glyph only writes itself', () => {
  const short = plan(font, { text: 'hi', loop: 0 });
  const long = plan(font, { text: 'hi!', loop: 0 });
  expect(common('hi', 'hi!')).toBe('hi');
  expect(common('mrly', 'mrlyprod')).toBe('mrly');
  expect(common('abc', 'xbc')).toBe('');
  expect(cells(font, 'hi')).toBe(short.still);
  expect(cells(font, '')).toBe(0);
  for (let k = 0; k <= short.still; k++) expect(spots(long, k)).toBe(spots(short, k));
});

test('a blank text takes a sample the seed picks', () => {
  expect(sample(rng(3))).toBe(sample(rng(3)));
  expect(SAMPLES).toContain(sample(rng(3)));
  expect(new Set([1, 2, 3, 4, 5, 6, 7, 8].map((seed) => sample(rng(seed)))).size).toBeGreaterThan(1);
});

test('a plan cuts the text at the limit', () => {
  expect(plan(font, { text: 'a'.repeat(LIMIT + 50) }).text).toHaveLength(LIMIT);
});

test('the gallery lists every glyph of the font by kind, each written one cell a frame in its pen order to the whole glyph', () => {
  const kinds = gallery(font);
  expect(kinds.map((kind) => kind.id)).toEqual(KINDS.map((kind) => kind.id));
  const glyphs = kinds.flatMap((kind) => kind.glyphs);
  expect(glyphs.map((glyph) => glyph.char)).toEqual(font.supported());
  expect(glyphs.filter((glyph) => glyph.frames.some((frame, k) => frame.length !== k) || glyph.frames.at(-1).length !== font.path(glyph.char).length)).toEqual([]);
  expect(glyphs.filter((glyph) => glyph.strokes < glyph.floor || glyph.lifts !== Math.max(0, glyph.strokes - 1))).toEqual([]);
  expect(glyphs.find((glyph) => glyph.char === 'A')).toMatchObject({ label: 'A', kind: 'upper', name: 'latin capital letter a', rows: 5, cols: 5, strokes: 2, floor: 2, lifts: 1, flag: '' });
  expect(glyphs.find((glyph) => glyph.char === ' ')).toMatchObject({ label: 'space', frames: [[]], strokes: 0, floor: 0, lifts: 0 });
  expect(glyphs.find((glyph) => glyph.char === '@')).toMatchObject({ rows: 7, cols: 7 });
});

test('a pen is flagged over its floor, or for five lifts or more on it', () => {
  expect([flag(4, 1, 3), flag(7, 7, 6), flag(6, 6, 5), flag(5, 5, 4), flag(2, 2, 1), flag(0, 0, 0)]).toEqual(['over floor', '6 lifts', '5 lifts', '', '', '']);
});

test('the facts count the pens, sum their strokes and spell the flagged ones', () => {
  const kinds = gallery(font);
  const glyphs = kinds.flatMap((kind) => kind.glyphs);
  const seen = facts(kinds);
  expect(seen.pens).toBe(font.supported().length);
  expect(seen.strokes).toBe(glyphs.reduce((n, glyph) => n + font.strokes(glyph.char).length, 0));
  expect(seen.flagged).toBe(glyphs.filter((glyph) => glyph.flag).map((glyph) => glyph.char).join(' '));
  expect(seen.flagged.split(' ')).toContain('D');
  expect(facts([{ glyphs: [] }])).toEqual({ pens: 0, strokes: 0, flagged: 'none' });
});

test('a file name is the text as a short slug', () => {
  expect(slug('hello, world!')).toBe('hello-world');
  expect(slug('  !!  ')).toBe('text');
  expect(slug('the quick brown fox jumps over the lazy dog').length).toBeLessThanOrEqual(24);
});
