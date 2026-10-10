import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { Frame } from '../../lib/frame.jsx';
import { Widget } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { seed: 1, dim: 2, base: 2, code: '', level: 3 };

test('the widget mounts its stage once the unit is in hand, and a cube only once three is too', () => {
  const page = (unit, three, patch = {}) => renderToStaticMarkup(<Frame label="Designs"><Widget value={{ ...value, ...patch }} onChange={() => {}} unit={unit} three={three} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
  expect(page(math, undefined, { dim: 3 })).not.toContain('<canvas');
  expect(page(math, {}, { dim: 3 })).toContain('<canvas');
});
