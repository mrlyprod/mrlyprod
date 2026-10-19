import { expect, test } from 'bun:test';
import * as font from '@mrlyprod/mrly/font';
import { pen } from '../../../test/pen.js';
import { SLOT, gallery, study } from './engine.js';
import { ink, shelved } from './gallery.jsx';

const KINDS = gallery(font);

const PENS = KINDS.flatMap((kind) => kind.glyphs);

test('a pen is inked as its last frame, one square a cell, centred on one square slot', () => {
  const glyph = study(font, font.glyph('i'));
  const log = [];
  ink(pen(log), glyph, 72, 72, '#ff8800');
  const unit = Math.floor(72 / (SLOT + 2));
  const [c] = glyph.frames.at(-1);
  const squares = log.filter(([key]) => key === 'fillRect');
  expect([squares.length, log.some(([key, v]) => key === 'fillStyle' && v === '#ff8800')]).toEqual([glyph.frames.at(-1).length, true]);
  expect(squares.some(([, x, y, w, h]) => x === Math.floor((72 - SLOT * unit) / 2) + (Math.floor((SLOT - glyph.cols) / 2) + (c % glyph.cols)) * unit && y > 0 && w === unit && h === unit)).toBe(true);
});

test('the shelves follow the filter and the search: a kind, the flagged pens, a letter or a word of its name', () => {
  const count = (kind, query = '') => shelved(KINDS, kind, query).flatMap((one) => one.glyphs).length;
  expect([count('all'), count('digit'), count('flag'), count('all', 'a'), count('all', '7')]).toEqual([font.supported().length, KINDS.find((one) => one.id === 'digit').glyphs.length, PENS.filter((one) => one.flag).length, 2, 1]);
  expect(shelved(KINDS, 'all', 'capital letter a').flatMap((one) => one.glyphs).map((one) => one.char)).toContain('A');
  expect(shelved(KINDS, 'upper', 'zzzz')).toEqual([]);
});
