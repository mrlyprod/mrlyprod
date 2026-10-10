import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { Frame } from '../../lib/frame.jsx';
import { Widget } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });
num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { seed: 1, base: 2, code: '7', number: 3, top: 300, growth: 'Every', cell: 0, path: 1, tint: '' };

test('the widget mounts its stage once both units are in hand', () => {
  const page = (units) => renderToStaticMarkup(<Frame label="Snail"><Widget value={value} onChange={() => {}} units={units} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page({ math, num })).toContain('<canvas');
});
