import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { Frame } from '../../lib/frame.jsx';
import { Widget, said } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = { seed: 1, a: '127', b: '239', base: 3, number: 3, level: 4, walkers: 300, finish: 50, speed: 60, heat: 20, tint: '' };

test('the widget mounts its stage once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Race"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas');
});

test('the status says who leads, who won, how a race was judged and the tally of the series', () => {
  expect(said({ over: null, reach: [3, 2], tally: [0, 0, 0] })).toBe('A leads');
  expect(said({ over: null, reach: [2, 3], tally: [2, 1, 0] })).toBe('B leads 2-1');
  expect(said({ over: null, reach: [0, 0], tally: [0, 0, 0] })).toBe('level');
  expect(said({ over: { tick: 412, winner: 0, how: 'goal' }, reach: [20, 12], tally: [1, 0, 0] })).toBe('A wins 1-0');
  expect(said({ over: { tick: 412, winner: -1, how: 'goal' }, reach: [20, 20], tally: [0, 0, 1] })).toBe('draw 0-0');
  expect(said({ over: { tick: 4738, winner: 1, how: 'stall' }, reach: [15, 16], tally: [0, 0, 0] })).toBe('stalled, B ahead');
  expect(said({ over: { tick: 60000, winner: -1, how: 'limit' }, reach: [5, 5], tally: [0, 0, 1] })).toBe('time up, level 0-0');
});
