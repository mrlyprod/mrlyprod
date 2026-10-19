import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { Rows, Source, gist, read } from './source.jsx';

const html = (value) => renderToStaticMarkup(<Source value={value} onChange={() => {}} math={math} />);

test('the source fold names the source and its design in its summary', () => {
  expect(gist({}, math)).toBe('Flat · carpet 495 · 3 · 4');
  expect(gist({ source: 'moire' }, math)).toBe('Moire · weave · 9');
  expect(gist({ dim: 3, base: 2 }, math)).toBe('Slice · carpet 23 · 3 · 2');
  expect(gist({ code: '1.5' }, math)).toBe('Flat · 1 · 3 · 4');
});

test('the rows hold the source and its own knobs, never a design picker: the side and level of a plane, the tile of a cube, the scales of a moire', () => {
  const flat = html({ number: 9 });
  expect(flat).toContain('>Side<');
  expect(flat).toContain('max="2"');
  expect(flat).not.toContain('>Code<');
  expect(flat).not.toContain('aria-label="Dim"');
  expect(html({ dim: 3 })).toContain('>Tile<');
  expect(html({ source: 'moire' })).toContain('>Scales up to<');
  expect(html({ source: 'moire' })).not.toContain('>Side<');
});

test('a write tidies the whole source, so a side change caps the level', () => {
  const wrote = [];
  const [, flat] = Rows({ value: { level: 5 }, onChange: (patch) => wrote.push(patch) }).props.children;
  flat.props.children[0].props.onChange('9');
  expect([wrote[0].number, wrote[0].level]).toEqual([9, 2]);
  expect(read({ level: '9', number: '5' }).level).toBe(3);
});
