import { expect, test } from 'bun:test';
import * as font from '../../../pkgs/mrlyjs/font.js';
import { rng } from '../../lib/scene.js';
import { KINDS, LIMIT, SAMPLES, at, gallery, plan, sample, slug } from './engine.js';

font.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/font/mrlyjs_font_bg.wasm', import.meta.url)).arrayBuffer() });

const full = (text) => font.raster(text).reduce((n, row) => n + row.reduce((m, bit) => m + bit, 0), 0);

test('a looping plan writes one cell a frame up to the full text, folds it and comes back to nothing', () => {
  const show = plan(font, { text: 'hi', pad: 1, hold: 0, loop: 1 });
  expect([show.rows, show.cols, show.fps]).toEqual([7, 11, font.FPS()]);
  expect(show.frames[0]).toEqual([]);
  expect(show.frames[show.still]).toHaveLength(full('hi'));
  expect(show.frames.slice(0, show.still + 1).map((frame) => frame.length)).toEqual(Array.from({ length: show.still + 1 }, (_, i) => i));
  expect(show.frames.at(-1)).toEqual([]);
  expect(show.frames.length).toBeGreaterThan(2 * show.still);
});

test('a plan with loop off ends on the finished text', () => {
  const show = plan(font, { text: 'hi', loop: 0 });
  expect(show.frames.length).toBe(show.still + 1);
  expect(show.frames.at(-1)).toHaveLength(full('hi'));
});

test('hold rests the loop after each of its four movements', () => {
  const rested = plan(font, { text: 'hi', hold: 7 }).frames.length;
  expect(rested - plan(font, { text: 'hi', hold: 0 }).frames.length).toBe(4 * 7);
});

test('the frame at t wraps when looping, clamps otherwise, and speed scales the rate', () => {
  const show = { frames: Array.from({ length: 10 }, () => []), fps: 25 };
  expect([0, 39, 40, 400, 440].map((t) => at(show, t, 1, 1))).toEqual([0, 0, 1, 0, 1]);
  expect([0, 400, 10000].map((t) => at(show, t, 1, 0))).toEqual([0, 9, 9]);
  expect(at(show, 40, 2, 1)).toBe(2);
  expect(at({ frames: [[]], fps: 25 }, 9999, 1, 1)).toBe(0);
});

test('a blank text takes a sample the seed picks', () => {
  expect(sample(rng(3))).toBe(sample(rng(3)));
  expect(SAMPLES).toContain(sample(rng(3)));
  expect(new Set([1, 2, 3, 4, 5, 6, 7, 8].map((seed) => sample(rng(seed)))).size).toBeGreaterThan(1);
});

test('a plan cuts the text at the limit', () => {
  expect(plan(font, { text: 'a'.repeat(LIMIT + 50) }).text).toHaveLength(LIMIT);
});

test('the gallery lists every glyph of the font by kind with its name, its strokes on or over the floor and one lift between strokes', () => {
  const kinds = gallery(font);
  expect(kinds.map((kind) => kind.id)).toEqual(KINDS.map((kind) => kind.id));
  const glyphs = kinds.flatMap((kind) => kind.glyphs);
  expect(glyphs.map((glyph) => glyph.char)).toEqual(font.supported());
  expect(glyphs.filter((glyph) => glyph.strokes < glyph.floor || glyph.lifts !== Math.max(0, glyph.strokes - 1))).toEqual([]);
  const A = glyphs.find((glyph) => glyph.char === 'A');
  expect(A).toMatchObject({ label: 'A', name: 'latin capital letter a', rows: 5, cols: 5, strokes: 2, floor: 2, lifts: 1 });
  expect(A.cells).toHaveLength(font.path('A').length);
  expect(glyphs.find((glyph) => glyph.char === ' ')).toMatchObject({ label: 'space', cells: [], strokes: 0, floor: 0, lifts: 0 });
});

test("a glyph's pen order visits each of its lit cells once, on the untrimmed card", () => {
  const glyphs = gallery(font).flatMap((kind) => kind.glyphs);
  expect(glyphs.filter((glyph) => glyph.order.length !== glyph.cells.length || [...glyph.order].sort((a, b) => a - b).join() !== glyph.cells.join())).toEqual([]);
  expect(glyphs.find((glyph) => glyph.char === 'i').order[0]).toBe(21);
});

test('a file name is the text as a short slug', () => {
  expect(slug('hello, world!')).toBe('hello-world');
  expect(slug('  !!  ')).toBe('text');
  expect(slug('the quick brown fox jumps over the lazy dog').length).toBeLessThanOrEqual(24);
});
