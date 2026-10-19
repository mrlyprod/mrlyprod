import { expect, test } from 'bun:test';
import * as math from '@mrlyprod/mrly/math';
import * as num from '@mrlyprod/mrly/num';
import { describe, labelOf, maxOf, minOf, tidy } from '../../lib/app/spec.js';
import { shown } from '../../lib/app/knobs.jsx';
import { rng } from '../../lib/rand.js';
import { ledger } from '../../lib/sequences/engine.js';
import { pen, said } from '../../../test/pen.js';
import { CARDS, build } from './cards.js';
import { PAGE, SPEC, forms, make, named } from './scene.js';

const units = { math, num, book: ledger(math) };

const open = (card, step, w = 1200, h = 700) => {
  const log = [];
  const live = { current: { made: build(units, card, step), card, shown: `level ${step}`, version: 1 } };
  const view = { rand: rng(1), look: () => ({ paper: '#ffffff', accent: '#008cff' }), still: true, w, h, dpr: 1, t: 0 };
  const scene = make({ getContext: () => pen(log) }, view, { live });
  scene.draw();
  return { scene, log };
};

test('a card draws its numbered title, its story, its picture and its lines with their records', () => {
  const { log } = open(1, 3);
  const text = said(log);
  expect(text).toContain('2 · The carpet');
  expect(text.some((one) => one.startsWith('Eight of nine cells'))).toBe(true);
  expect(text).toContain('filled cells');
  expect(text).toContain('512');
  expect(text.some((one) => one.includes('A001018'))).toBe(true);
  expect(log.filter(([key]) => key === 'rect')).toHaveLength(512);
});

test('every card paints on a desk and on a phone without a throw', () => {
  CARDS.forEach((one, i) => {
    for (const [w, h] of [[1200, 700], [390, 700]]) expect([one.key, w, said(open(i, one.first, w, h).log).length > 3]).toEqual([one.key, w, true]);
  });
});

test('a card is a still that draws when asked, and a tap on the picture asks for the next step and a tap elsewhere does nothing', () => {
  const { scene } = open(1, 3);
  expect(scene.still).toBe(true);
  expect(scene.tap(300, 400)).toEqual({ picture: true });
  expect(scene.tap(1150, 20)).toBe(null);
});

test('a line spells its forms and its records, and asks for three terms before it names one', () => {
  expect(forms({ closed: 'k(3k - 2)', records: { formula: 'k(3k - 2)', found: [{ record: { id: 'A000567', name: 'Octagonal numbers: n*(3*n-2)' }, at: 2 }], short: false } })).toBe('k(3k - 2) · A000567 Octagonal numbers: n*(3*n-2), from index 2');
  expect(forms({ closed: '', records: { formula: '', found: [], short: true } })).toBe('three terms name the record');
  expect(PAGE.keys.find((row) => row.button).act).toBe('next');
});

test('the library picks the card, the Card group only slides its step, labelled, ranged and shown as the card says, the step held inside the card', () => {
  expect(describe(SPEC, { card: 0, step: 5 }).flatMap(({ rows }) => rows.map((row) => row.key))).toEqual(['step']);
  const step = SPEC.find((row) => row.key === 'step');
  const value = { card: 0, step: 5 };
  expect([labelOf(step, value), minOf(step, value), maxOf(step, value), shown(step, 5, value)]).toEqual(['Side up to', 4, 8, 'side 9']);
  expect([tidy(SPEC, { card: '1', step: 9 }), tidy(SPEC, {})]).toEqual([{ card: 1, step: 5 }, { card: 0, step: CARDS[0].first }]);
});

test('an export is named by the card and its step', () => {
  expect(PAGE.name).toBe(named);
  expect(named({ card: 1, step: 3 })).toBe('tour-2-carpet-3');
});
