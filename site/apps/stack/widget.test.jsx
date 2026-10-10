import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { Frame } from '../../lib/frame.jsx';
import { DESIGN, SPEC } from './scene.js';
import { Widget, patch, rows, said } from './widget.jsx';
import { defaults } from '../../lib/knobs.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { ...defaults(SPEC), ...DESIGN, seed: 1 };

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Stack"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
});

test('the picker writes no dim, and the status and the readings name each stack', () => {
  expect(patch({ dim: 2, base: 3, code: '495' })).toEqual({ base: 3, code: '495' });
  expect(said({ kind: 'moire', name: 'carpet', code: '495', base: 3, layers: 21 })).toBe('carpet 495');
  expect(said({ kind: 'star', layers: 5 })).toBe('5 cuts');
  expect(said({ kind: 'tourbillon', layers: 12, set: 'primes' })).toBe('12 layers');
  expect(rows({ kind: 'moire', layers: 21, scales: [1, 3, 5, 7, 9, 11, 13], limit: 13, prime: false, max: 0.5, at: 3 })).toEqual([['Layers', 21], ['Scales', '1 3 5 7 ... 13'], ['Largest r', '0.5000 at 3'], ['Scale 13', 'shares a factor']]);
  expect(rows({ kind: 'tourbillon', layers: 11, set: 'odd', weights: 'plain', period: null, classes: 11, pairs: 0, mean: 0.25, rms: 0.1, centre: 0.5 }).map(([name]) => name)).toEqual(['Layers', 'Set', 'Weights', 'Period', 'Classes', 'Pairs', 'Mean', 'Contrast', 'Centre']);
  expect(rows({ kind: 'star', layers: 21, limit: 41, scaled: -0.9, logged: -0.15, slope: null, branch: 'odd' })[4]).toEqual(['Slope', 'needs L mod 4 = 0']);
});
