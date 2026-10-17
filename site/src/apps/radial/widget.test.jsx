import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { Frame } from '../../lib/app/frame.jsx';
import { rng } from '../../lib/rand.js';
import { Widget, rolled, rows, wrap } from './widget.jsx';

test('the widget mounts its stage only once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Radial"><Widget value={{ seed: 1, copies: 6, full: 1, step: 60 }} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
});

test('a reroll draws the design, the copies and a blend other than meet, on a full turn', () => {
  const all = Array.from({ length: 40 }, (_, seed) => rolled(math, { source: 'flat' }, rng(seed)));
  expect(all.every((one) => one.copies >= 1 && one.copies <= 36 && one.blend !== 'meet' && one.full === 1 && one.code)).toBe(true);
  expect(new Set(all.map((one) => one.blend)).size).toBeGreaterThan(2);
});

test('a tap wraps the copies round from 36 to 1', () => {
  expect([wrap(6, 1), wrap(36, 1), wrap(1, -1)]).toEqual([7, 1, 36]);
});

test('the readouts are the demo stats and the share of order zero, then the leading orders', () => {
  const facts = { kind: 'flat', code: '495', name: 'carpet', side: 81, copies: 6, step: 60, order: 4, petals: 12, share: 0.612, leading: [{ order: 4, share: 0.1 }] };
  expect(rows(facts)).toEqual([['name', 'carpet 495'], ['side', '81'], ['copies', '6'], ['step', '60°'], ['rotation order', '4'], ['petals', '12'], ['order 0', '61.2%'], ['order 4', '10.0%']]);
});
