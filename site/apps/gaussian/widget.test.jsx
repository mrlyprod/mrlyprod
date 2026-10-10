import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { Frame } from '../../lib/frame.jsx';
import { Knobs } from '../../lib/knobs.jsx';
import { SPEC } from './scene.js';
import { Widget } from './widget.jsx';

num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { seed: 1, ring: 'square', limit: 12, look: 'fate', faint: 1, tint: '', cell: 0, grow: 12 };

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Gaussian"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(num)).toContain('<canvas');
});

test('the Cell knob reads Fit at 0, not 0 px, and its pixels once pinned', () => {
  const read = (cell) => renderToStaticMarkup(<Knobs spec={SPEC} value={{ ...value, cell }} onChange={() => {}} />).replaceAll('<!-- -->', '').match(/<span>Cell<\/span><span class="num">([^<]+)<\/span>/)[1];
  expect([read(0), read(8)]).toEqual(['Fit', '8 px']);
});
