import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { defaults } from '../../lib/app/spec.js';
import { Frame } from '../../lib/app/frame.jsx';
import { SPEC } from './scene.js';
import { Widget, status } from './widget.jsx';

const stat = { gen: 3, pop: 5, fate: '', name: 'B3/S23 on carpet', counts: [[3], []], label: 'carpet, side 3, level 1', cells: 8, lattice: 'one', error: '' };

test('the readouts go to the status as rows: a dash for an unread fate, and an error row only on an error', () => {
  expect(status(stat)).toEqual([['gen', 3], ['pop', 5], ['fate', '-'], ['rule', 'B3/S23 on carpet'], ['born', '3'], ['stay', 'none'], ['mask', 'carpet, side 3, level 1'], ['cells', 8], ['lattice', 'one']]);
  expect(status({ ...stat, error: 'boom' }).at(-1)).toEqual(['error', 'boom']);
  expect(status(null)).toEqual([]);
});

test('the widget renders before its unit arrives, with no stage yet', () => {
  expect(renderToStaticMarkup(<Frame label="Life"><Widget value={{ ...defaults(SPEC), seed: 1 }} onChange={() => {}} /></Frame>)).not.toContain('<canvas');
});
