import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { Frame } from '../../lib/frame.jsx';
import { Knobs } from '../../lib/knobs.jsx';
import { Pick } from '../designs/pick.jsx';
import { SPEC } from './scene.js';
import { Widget } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { seed: 1, base: 2, code: '', number: 3, level: 3, view: 'solid', axis: 2, at: 0.5, crop: '', radius: 16, spin: 1, tint: '' };

const page = (unit, three, patch = {}) => renderToStaticMarkup(<Frame label="Three"><Widget value={{ ...value, ...patch }} onChange={() => {}} unit={unit} three={three} /></Frame>);

test('the widget mounts its stage once the unit is in hand, the solid only once three is too', () => {
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).not.toContain('<canvas');
  expect(page(math, {})).toContain('<canvas');
  expect(page(math, undefined, { view: 'slice' })).toContain('<canvas');
  expect(page(math, undefined, { view: 'hex' })).toContain('<canvas');
});

test('the picker pins the cube through dims, so no dim segment shows', () => {
  const html = renderToStaticMarkup(<Pick value={{ dim: 3, base: 2, code: '23' }} onChange={() => {}} math={math} dims={[3]} />);
  expect([html.includes('aria-label="Dim"'), html.includes('aria-label="Base"'), html.includes('value="23"')]).toEqual([false, true, true]);
});

test('the Level knob runs to the cap of the number, and the Cut group shows only the rows the view uses', () => {
  const rows = (patch) => renderToStaticMarkup(<Knobs spec={SPEC} value={{ ...value, ...patch }} onChange={() => {}} />);
  const max = (html) => /type="range" min="1" max="(\d)"/.exec(html)[1];
  expect([max(rows({})), max(rows({ number: 2 })), max(rows({ number: 5 }))]).toEqual(['3', '5', '2']);
  const labels = (html) => [...html.matchAll(/<h3>([^<]*)<\/h3>|<label><span>([^<]*)<\/span>|role="radiogroup" aria-label="([^"]*)"/g)].map((found) => found[1] ?? found[2] ?? found[3]);
  const upto = (html) => labels(html).slice(0, labels(html).indexOf('Crop'));
  expect(upto(rows({}))).toEqual(['Grow', 'Number', 'Level', 'Cut', 'View']);
  expect(upto(rows({ view: 'slice' }))).toEqual(['Grow', 'Number', 'Level', 'Cut', 'View', 'Axis', 'Plane']);
  expect(upto(rows({ view: 'diagonal' }))).toEqual(['Grow', 'Number', 'Level', 'Cut', 'View', 'Plane']);
  expect(upto(rows({ view: 'hex' }))).toEqual(['Grow', 'Number', 'Level', 'Cut', 'View']);
});
