import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { PRESETS } from './engine.js';
import { Moires, toMoire } from './shelves.jsx';

const look = { ramp: 'heat', levels: 64, invert: 0, tint: '' };

test('the moire shelf lists the four presets, the one in hand lit, and a pick turns the source to it', () => {
  const html = renderToStaticMarkup(<Moires value={{ ...look, source: 'moire', preset: 'hive' }} onChange={() => {}} math={math} />);
  expect([...html.matchAll(/class="label">([^<]*)</g)].map(([, name]) => name)).toEqual(PRESETS);
  expect(html.match(/class="on"/g)).toHaveLength(1);
  expect(html).toContain('aria-current="true" class="on" role="listitem"><canvas');
  expect([toMoire({ source: 'design' }, 'hive'), toMoire({}, 'nope')].map(({ source, preset }) => [source, preset])).toEqual([['moire', 'hive'], ['moire', 'weave']]);
  expect(renderToStaticMarkup(<Moires value={look} onChange={() => {}} math={math} />)).not.toContain('class="on"');
});
