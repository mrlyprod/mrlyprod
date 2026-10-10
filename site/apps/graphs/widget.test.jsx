import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { Frame } from '../../lib/frame.jsx';
import { Knobs } from '../../lib/knobs.jsx';
import { Pick } from '../designs/pick.jsx';
import { SPEC } from './scene.js';
import { Legend, Widget, note } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { seed: 1, dim: 2, base: 3, code: '495', number: 3, level: 2, graph: 'core', layout: 'grid', grow: 8, spin: 1, tint: '' };

const page = (unit, three, patch = {}) => renderToStaticMarkup(<Frame label="Graphs"><Widget value={{ ...value, ...patch }} onChange={() => {}} unit={unit} three={three} /></Frame>);

test('the widget mounts its stage once the unit is in hand, a cube only once three is too, and the picker offers both dims', () => {
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
  expect(page(math, undefined, { dim: 3, base: 2, code: '23' })).not.toContain('<canvas');
  expect(page(math, {}, { dim: 3, base: 2, code: '23' })).toContain('<canvas');
  const picker = renderToStaticMarkup(<Pick value={{ dim: 2, base: 3, code: '495' }} onChange={() => {}} math={math} />);
  expect([picker.includes('aria-label="Dim"'), picker.includes('aria-label="Base"'), picker.includes('value="495"')]).toEqual([true, true, true]);
});

test('the spin knob shows only on a cube and the level knob runs to the cap', () => {
  const rows = (patch) => renderToStaticMarkup(<Knobs spec={SPEC} value={{ ...value, ...patch }} onChange={() => {}} />);
  expect(rows({})).not.toContain('Spin');
  expect(rows({ dim: 3 })).toContain('Spin');
  const max = (html) => /type="range" min="1" max="(\d)"/.exec(html)[1];
  expect([max(rows({})), max(rows({ layout: '' })), max(rows({ dim: 3, layout: '' })), max(rows({ dim: 3 }))]).toEqual(['4', '3', '2', '3']);
});

test('the legend names each shade the stage uses beside its count, from the junction down, and leaves out one that is not there', () => {
  const html = renderToStaticMarkup(<Legend roles={[0, 5, 9, 7]} tint="" />);
  expect([...html.matchAll(/<\/i>(\w+)<\/span><span class="num">(\d+)/g)].map(([, name, n]) => `${name} ${n}`)).toEqual(['Junction 7', 'Through 9', 'Tip 5']);
  expect(html).toContain('color-mix(in srgb, var(--accent) 100%, var(--art))');
  expect(html).toContain('color-mix(in srgb, var(--accent) 52%, var(--art))');
  expect(renderToStaticMarkup(<Legend roles={[3, 0, 0, 0]} tint="red" />)).toContain('var(--red) 28%');
});

test('the status is the design word or bang, its code, then the graph and layout', () => {
  expect(note({ name: '', code: '487', graph: 'core', layout: 'force' })).toBe('bang 487, core graph, force');
  expect(note({ name: 'carpet', code: '495', graph: 'edge', layout: 'grid' })).toBe('carpet 495, edge graph, grid');
});
