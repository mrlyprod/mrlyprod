import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { Frame } from '../../lib/frame.jsx';
import { DESIGN, SPEC } from './scene.js';
import { Widget, patch, rows } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { ...defaults(SPEC), ...DESIGN, seed: 1 };

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Moire"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
});

test('the picker writes no dim, the field reads name the stack, and the r-matrix reads name the pair on trial', () => {
  expect(patch({ dim: 2, base: 3, code: '495' })).toEqual({ base: 3, code: '495' });
  expect(Object.fromEntries(rows({ view: 'field', name: 'carpet', code: '495', layers: 5, scales: [1, 3, 5, 7, 9], size: 384 }))).toEqual({ design: 'carpet 495', layers: 5, scales: '1 3 5 7 9', pixels: '384 by 384' });
  expect(Object.fromEntries(rows({ view: 'matrix', row: 9, mate: 3, r: 0.2192645048, prime: false, max: 0.2192645048, at: 3, k: 4 }))).toEqual({ 'scale m': 9, 'scale n': 3, r: '0.219264505', 'row of m': 'largest 0.219264505 at scale 3', matrix: '4 by 4' });
  expect(Object.fromEntries(rows({ view: 'matrix', row: 0, mate: 0, r: null, prime: false, max: 0, at: 0, k: 0 }))['row of m']).toBe('none');
  expect(rows({ error: 'no' })).toEqual([['error', 'no']]);
  expect(rows(null)).toEqual([]);
});
