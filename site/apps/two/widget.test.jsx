import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { Frame } from '../../lib/frame.jsx';
import { Pick } from '../designs/pick.jsx';
import { Widget } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { seed: 1, base: 3, code: '495', number: 3, level: 3, x: 5, y: 5, crop: 'ball', radius: 16, touch: 0, invert: 0, tint: '' };

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Two"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
});

test('the picker pins the plane through dims and keeps the base and the code', () => {
  const pinned = renderToStaticMarkup(<Pick value={{ dim: 2, base: 3, code: '495' }} onChange={() => {}} math={math} dims={[2]} />);
  expect(pinned).not.toContain('aria-label="Dim"');
  expect(pinned).toContain('aria-label="Base"');
  expect(pinned).toContain('value="495"');
  expect(renderToStaticMarkup(<Pick value={{ dim: 2, base: 3, code: '495' }} onChange={() => {}} math={math} />)).toContain('aria-label="Dim"');
});
