import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { Frame } from '../../lib/frame.jsx';
import { defaults } from '../../lib/knobs.js';
import { SPEC } from './scene.js';
import { Widget } from './widget.jsx';

test('the stage mounts once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Slices"><Widget value={{ ...defaults(SPEC), seed: 1, base: 2, code: '23' }} onChange={() => {}} unit={unit} /></Frame>);
  expect([page(undefined).includes('<canvas'), page(math).includes('<canvas')]).toEqual([false, true]);
});
