import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import { values } from '../../lib/scene.jsx';
import { SPEC } from './scene.js';
import { Widget } from './widget.jsx';

test('before the unit loads the widget renders no stage and claims no bar on the server', () => {
  const value = values({ seed: 0, q: '', pick: '' }, SPEC);
  expect(Object.keys(value)).toEqual(['seed', 'q', 'pick', 'dim', 'base', 'measure', 'axis', 'chart', 'digits', 'terms']);
  expect(renderToStaticMarkup(<Widget value={value} onChange={() => {}} onReady={() => {}} />)).toBe('');
});
