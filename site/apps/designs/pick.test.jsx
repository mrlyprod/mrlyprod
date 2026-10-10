import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { Pick as Select, Text } from '../../lib/knobs.jsx';
import { Pick, Picker, Thumbs } from './pick.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const options = (html) => [...html.matchAll(/<option value="([^"]*)"[^>]*>([^<]*)<\/option>/g)].map((found) => [found[1], found[2]]);

test('Pick renders the named options of 2D base 2 and writes a chosen code', () => {
  const html = renderToStaticMarkup(<Pick value={{ dim: 2, base: 2, code: '7' }} onChange={() => {}} math={math} />);
  const named = options(html).slice(1);
  expect(named.length).toBe(10);
  expect(named).toContainEqual(['7', 'carpet']);
  expect(named).toContainEqual(['8', 'point']);
  expect(html).toContain('<option value="7" selected="">carpet</option>');
  const wrote = [];
  const tree = Picker({ value: { dim: 2, base: 2, code: '7' }, onChange: (patch) => wrote.push(patch), math }).props.children;
  tree.find((node) => node?.type === Select).props.onChange('14');
  tree.find((node) => node?.type === Text).props.onChange(' 3 ');
  expect(wrote).toEqual([{ dim: 2, base: 2, code: '14' }, { dim: 2, base: 2, code: '3' }]);
});

test('Pick shows a strip of the classes for a small space and none for the cube at base 3', () => {
  const strip = (value) => (renderToStaticMarkup(<Picker value={value} onChange={() => {}} math={math} />).match(/<canvas /g) ?? []).length;
  expect([strip({ dim: 2, base: 2, code: '7' }), strip({ dim: 3, base: 2, code: '23' }), strip({ dim: 3, base: 3, code: '1' })]).toEqual([6, 22, 0]);
});

test('Thumbs mark the picked code and name the named ones', () => {
  const html = renderToStaticMarkup(<Thumbs math={math} dim={2} base={2} codes={['5', '7']} code="7" names={[{ code: '7', name: 'carpet' }]} onPick={() => {}} />);
  expect(html).toContain('aria-pressed="true" aria-label="carpet 7"');
  expect(html).toContain('aria-pressed="false" aria-label="bang 5"');
  expect([...html.matchAll(/<small>([^<]*)<\/small>/g)].map((found) => found[1])).toEqual(['5', 'carpet']);
});

test('Thumbs take the full width of their row, so a wrapping flex column cannot size the grid as one column', () => {
  const html = renderToStaticMarkup(<Thumbs math={math} dim={2} base={2} codes={['5']} code="5" names={[]} onPick={() => {}} />);
  expect(html).toContain('<div class="designs" style="width:100%">');
});
