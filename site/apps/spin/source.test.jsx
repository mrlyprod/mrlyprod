import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { Rows, Source, gist, read } from './source.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const html = (value) => renderToStaticMarkup(<Source value={value} onChange={() => {}} math={math} />);

test('the source fold names the source and its design in its summary', () => {
  expect(gist({}, math)).toBe('Flat · carpet 495 · 3 · 4');
  expect(gist({ source: 'moire' }, math)).toBe('Moire · weave · 9');
  expect(gist({ source: 'slice' }, math)).toBe('Slice · carpet 23 · 3 · 2');
});

test('a flat source shows the plane picker, the side and a level capped by it; a slice the cube picker and its tile', () => {
  const flat = html({ number: 9 });
  expect(flat).toContain('<option value="495" selected="">carpet</option>');
  expect(flat).not.toContain('aria-label="Dim"');
  expect(flat).toContain('max="2"');
  const slice = html({ source: 'slice' });
  expect(slice).toContain('<input type="text" value="23"/>');
  expect(slice).toContain('>Tile<');
  expect(html({ source: 'moire' })).toContain('>Scales up to<');
  expect(renderToStaticMarkup(<Source value={{}} onChange={() => {}} math={null} />)).toContain('loading');
});

test('a write tidies the whole source, so a side change caps the level and a slice pick lands on its own keys', () => {
  const wrote = [];
  const [, flat] = Rows({ value: { level: 5 }, onChange: (patch) => wrote.push(patch), math }).props.children;
  flat.props.children[1].props.onChange('9');
  expect([wrote[0].number, wrote[0].level]).toEqual([9, 2]);
  const slice = Rows({ value: { source: 'slice' }, onChange: (patch) => wrote.push(patch), math }).props.children[3];
  slice.props.children[0].props.onChange({ dim: 3, base: 3, code: '7' });
  expect([wrote[1].sbase, wrote[1].scode, wrote[1].code]).toEqual([3, '7', '495']);
  expect(read({ level: '9', number: '5' }).level).toBe(3);
});

test('a blank or junk code names the code the seed settles on', () => {
  for (const code of ['', 'abc', '-3', '1.5']) expect(html({ code, seed: 4 })).toContain('Flat · ');
  expect(gist({ code: '1.5' }, math)).toBe('Flat · 1 · 3 · 4');
  expect(gist({ source: 'slice', scode: 'x', seed: 4 }, math)).toBe(gist({ source: 'slice', scode: 'y', seed: 4 }, math));
});
