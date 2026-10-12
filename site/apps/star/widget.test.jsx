import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { Frame } from '../../lib/frame.jsx';
import { CARPET, LEAST, MOST, law } from './engine.js';
import { SPEC } from './scene.js';
import { Widget, drawn, rows } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { ...defaults(SPEC), seed: 1 };

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Star"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
});

test('the readings lead with the decay, mark a layer off the law, and say when the slope is refused', () => {
  const star = new math.six.star.Star(CARPET);
  const f = { ...law(math, star, 30, 2), size: 384, low: 0.32, high: 1 };
  star.free();
  const said = Object.fromEntries(rows(f));
  expect(rows(f).slice(0, 4).map(([name]) => name)).toEqual(['layers', 'excess x L', 'slope', 'width law']);
  expect(said).toMatchObject({ slope: 'needs L divisible by four', 'width law': '-0.166667', 'L mod 4': '2 mod 4', band: '2 cells about it', 'ink range': '0.3200 to 1.0000', pixels: '384 by 384' });
  expect(said['arm shares'].startsWith('1/1 5/9 (law 1/3)')).toBe(true);
  expect([rows({ error: 'no' }), rows(null)]).toEqual([[['error', 'no']], []]);
});

test('a reroll draws an even L inside the slider', () => {
  expect([drawn(() => 0).layers, drawn(() => 0.9999).layers, drawn(() => 0.5).layers % 2]).toEqual([LEAST, MOST, 0]);
});
