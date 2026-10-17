import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { values } from '../../lib/app/page.jsx';
import { CANON } from '../../lib/sequences/engine.js';
import { SPEC } from './scene.js';
import { Widget } from './widget.jsx';

test('before the unit loads the widget renders no stage and claims no bar on the server', () => {
  const value = values({ seed: 0, q: '', pick: CANON }, SPEC);
  expect(Object.keys(value)).toEqual(['seed', 'q', 'pick', 'chart', 'terms', 'digits', 'dim', 'base', 'measure', 'axis']);
  expect(renderToStaticMarkup(<Widget value={value} onChange={() => {}} onReady={() => {}} />)).toBe('');
});
