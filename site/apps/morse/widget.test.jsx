import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { Frame } from '../../lib/frame.jsx';
import { ROWS, Widget, hands, labels, status } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });
num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { seed: 1, base: 2, code: '', view: 'plane', lift: '', fold: 'sign', number: 0, level: 7, cell: 0, grow: 1, tint: '' };

const page = (unit) => renderToStaticMarkup(<Frame label="Morse"><Widget value={value} onChange={() => {}} units={unit} /></Frame>);

test('the widget mounts its stage once the units are in hand', () => {
  expect(page(undefined)).not.toContain('<canvas');
  expect(page({ num, math })).toContain('<canvas');
});

test('the status names the lift, the word or the design', () => {
  expect(status({ view: 'plane', kind: 'Parity', formula: 't(i) xor t(j)' })).toBe('parity, t(i) xor t(j)');
  expect(status({ view: 'word', letters: 8, runs: 6 })).toBe('8 letters, 6 runs');
  expect(status({ view: 'lift', name: 'void', code: '9', exact: true })).toBe('void 9, the Thue-Morse grid');
  expect(status({ view: 'lift', name: '', code: '13', title: 'bang dim 2, code 13', exact: false })).toBe('bang dim 2, code 13');
  expect(status({ view: 'filter', name: 'void', code: '9', fold: 'sign' })).toBe('void 9, plus-minus filter');
  expect(status({ view: 'filter', name: '', code: '13', title: 'bang dim 2, code 13', fold: 'design' })).toBe('bang dim 2, code 13, design filter');
});

test('the filter counts name what the difference is and how far it sits from Thue-Morse', () => {
  const read = (f) => Object.fromEntries(ROWS.filter.map(([name, get]) => [name, get(f)]));
  expect(read({ fold: 'sign', closed: true, morse: 32, cells: 64, half: true })).toMatchObject({ 'Closed form': 'the tile repeated', 'Off Thue-Morse': '32 of 64, a coin' });
  expect(read({ fold: 'design', closed: true, morse: null })).toMatchObject({ 'Closed form': 'the tile complement', 'Off Thue-Morse': '-' });
});

test('the lift and the filter name their panels in the order they lie', () => {
  expect(labels({ view: 'lift' })).toBe('design, then its sign power');
  expect(labels({ view: 'filter', level: 3 })).toBe('level 3 blown up, level 4, their difference');
  expect([labels({ view: 'plane' }), labels({ view: 'word' })]).toEqual(['', '']);
});

test('again starts the growth from the still face and restarts it while it grows', () => {
  const calls = [];
  const handle = { again: () => calls.push('again') };
  const change = (patch) => calls.push(patch);
  hands(handle, { current: { grow: 0 } }, change).again();
  hands(handle, { current: { grow: 1 } }, change).again();
  expect(calls).toEqual([{ grow: 1 }, 'again']);
});

test('turning goes round the views, the filter last', () => {
  const calls = [];
  const turn = (view, by) => hands({}, { current: { view } }, (patch) => calls.push(patch)).turn(by);
  turn('lift', 1);
  turn('filter', 1);
  turn('plane', -1);
  expect(calls).toEqual([{ view: 'filter' }, { view: 'plane' }, { view: 'filter' }]);
});
