import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { defaults } from '../../lib/knobs.js';
import { Frame } from '../../lib/frame.jsx';
import { SPEC } from './scene.js';
import { Widget, rows, typed } from './widget.jsx';

const value = { ...defaults(SPEC), seed: 1 };

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Tourbillon"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
});

test('the readings name the angle on screen, its quarter turn and the stack, and flag weights the blend drops', () => {
  const f = { angle: 22.5, playing: false, size: 512, layers: 28, scales: [1, 3], angles: [22.5, 45], mean: 0.5, rms: 0.1, faded: 0.53, centre: 0.5, peaks: [[0.5, 0.25, 0.75]], period: 4, classes: 4, pairs: 10, unused: true };
  const said = Object.fromEntries(rows(f));
  expect(said).toMatchObject({ angle: '22.50 deg', 'quarter turn': '4 increments', classes: 4, 'pairs sharing': 10, angles: '22.5 45.0', peaks: '0.5000 0.2500 0.7500', weights: 'unused by this blend', pixels: '512 by 512' });
  expect(Object.fromEntries(rows({ ...f, period: null, unused: false }))).toMatchObject({ 'quarter turn': 'never closes' });
  expect('weights' in Object.fromEntries(rows({ ...f, unused: false }))).toBe(false);
  expect([rows({ error: 'no' }), rows(null)]).toEqual([[['error', 'no']], []]);
});

test('a typed angle lands inside a full turn, junk and outside angles are dropped', () => {
  expect([typed('22.5'), typed(' 90 '), typed(''), typed('abc'), typed('361'), typed('-1')]).toEqual([22.5, 90, null, null, null, null]);
});
