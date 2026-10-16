import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { defaults } from '../../lib/knobs.js';
import { Frame } from '../../lib/frame.jsx';
import { SPEC } from './scene.js';
import { FOLDS, Widget, called } from './widget.jsx';

test('every spec row has one home in the pane: a fold, or the design picker for base and code', () => {
  const homes = [...FOLDS.flatMap(([, keys]) => keys), 'base', 'code'];
  expect([...homes].sort()).toEqual(SPEC.map((row) => row.key).sort());
});

test('the widget mounts its stage on the units the page loaded', () => {
  const page = (units) => renderToStaticMarkup(<Frame label="Spirograph"><Widget value={{ ...defaults(SPEC), seed: 1 }} onChange={() => {}} units={units} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page({ math })).toContain('<canvas');
});

test('the wheel fold names the design it draws, and only that one', () => {
  expect([called({ code: '495' }, { code: '495', name: 'carpet' }), called({ code: 'abc' }, { code: '495', name: 'carpet' }), called({ code: '13' }, { code: '13', name: null })]).toEqual(['carpet 495', 'code abc', 'code 13']);
});
