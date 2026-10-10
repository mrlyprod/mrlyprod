import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { Frame } from '../../lib/frame.jsx';
import { Knobs } from '../../lib/knobs.jsx';
import { SPEC } from './scene.js';
import { Widget } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });
num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { seed: 1, code: '', shape: '', level: 20, grow: 1, cell: 0, tint: '' };

test('the widget mounts its stage once both units are in hand', () => {
  const page = (units) => renderToStaticMarkup(<Frame label="Radix"><Widget value={value} onChange={() => {}} units={units} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page({ math, num })).toContain('<canvas');
});

test('the Cell knob reads Fit at 0 and its pixels above it', () => {
  const cell = (n) => renderToStaticMarkup(<Knobs spec={SPEC} value={{ ...value, cell: n }} onChange={() => {}} />).replaceAll('<!-- -->', '').match(/<span>Cell<\/span><span class="num">([^<]+)<\/span>/)[1];
  expect([cell(0), cell(8)]).toEqual(['Fit', '8 px']);
});
