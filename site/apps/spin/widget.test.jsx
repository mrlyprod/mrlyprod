import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { Frame } from '../../lib/frame.jsx';
import { Design, Needles, Profile, Spectrum, Widget, read } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { seed: 1, base: 3, code: '', number: 3, level: 3, rpm: 33, glow: 1, show: 'both', copies: 6, blend: 'mean', tint: '' };

test('the widget mounts its stage only once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Spin"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
});

test('the design group holds the picker pinned to the plane, the number and a level capped by it', () => {
  const html = renderToStaticMarkup(<Design value={{ ...value, code: '495' }} onChange={() => {}} math={math} />);
  expect(html).toContain('<option value="495" selected="">carpet</option>');
  expect(html).toContain('<input type="text" value="495"/>');
  expect(html).toContain('aria-label="Base"');
  expect(html).not.toContain('aria-label="Dim"');
  expect(html).toContain('>Number<');
  expect(html).toContain('max="5"');
  expect(renderToStaticMarkup(<Design value={{ ...value, number: 9, level: 5 }} onChange={() => {}} math={math} />)).toContain('max="2"');
  expect(renderToStaticMarkup(<Design value={value} onChange={() => {}} math={null} />)).toContain('loading');
});

test('a design write drops the dim the picker carries and a number change caps the level with it', () => {
  const wrote = [];
  const tree = Design({ value: { ...value, level: 5 }, onChange: (patch) => wrote.push(patch), math }).props.children;
  tree[0].props.onChange({ dim: 2, base: 2, code: '7' });
  tree[1].props.onChange('9');
  expect(wrote).toEqual([{ base: 2, code: '7' }, { number: 9, level: 2 }]);
  expect(read({ base: '2', code: ' 7 ', number: '9', level: '9' })).toEqual({ base: 2, code: '7', number: 9, level: 2 });
  expect(read({ number: '3', level: '9' }).level).toBe(5);
});

test('the needles are the demo presets as buttons, the one at the rpm lit', () => {
  const html = renderToStaticMarkup(<Needles rpm={900} onChange={() => {}} />);
  expect(html.match(/<button /g)).toHaveLength(5);
  expect(html).toContain('class="on">900<');
  expect(html).not.toContain('class="on">33<');
  const wrote = [];
  Needles({ rpm: 33, onChange: (patch) => wrote.push(patch) }).props.children[4].props.onClick();
  expect(wrote).toEqual([{ rpm: 900 }]);
});

test('the profile chart draws the circle mean over the radius with the dark disc and the inner edge marked', () => {
  const profile = Float32Array.from([0, 0, 0.5, 1, 0.25]);
  const html = renderToStaticMarkup(<Profile profile={profile} peak={1} disc={2} inner={3} reach={4} />);
  expect(html).toContain('<polyline points="0.0,48.0 60.0,48.0 120.0,24.0 180.0,0.0 240.0,36.0"');
  expect(html).toContain('<polygon points="0,48 0.0,48.0');
  expect(html.match(/<line /g)).toHaveLength(2);
  expect(html).toContain('x1="120.0"');
  expect(html).toContain('x1="180.0"');
  expect(renderToStaticMarkup(<Profile profile={profile} peak={1} disc={0} inner={3} reach={4} />).match(/<line /g)).toHaveLength(1);
});

test('the spectrum draws one bar per order with power, the orders the stack keeps in the accent', () => {
  const html = renderToStaticMarkup(<Spectrum power={[10, 0, 0, 0, 4, 0, 1]} copies={4} />);
  expect(html.match(/<rect /g)).toHaveLength(3);
  expect(html).toContain('fill="var(--fg)"');
  expect(html).toContain('fill="var(--accent)"');
  expect(html).toContain('fill="var(--dim)"');
  expect(html).toContain('height="48"');
});
