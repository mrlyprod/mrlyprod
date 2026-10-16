import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '@mrlyprod/mrly/math';
import { Frame } from '../../lib/frame.jsx';
import { Widget, again, fate, rows } from './widget.jsx';

const value = { seed: 1, a: '127', b: '239', base: 3, number: 3, level: 4, walkers: 300, finish: 20, speed: 720, heat: 0, tint: '' };

test('the widget mounts its stage, a picture of the boards, once the unit is in hand', () => {
  const page = (unit) => renderToStaticMarkup(<Frame label="Race"><Widget value={value} onChange={() => {}} unit={unit} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(math)).toContain('<canvas role="img"');
});

test('Race again walks the next seed, wrapping at the top', () => {
  expect([again(1), again(4294967295), again('7')]).toEqual([2, 0, 8]);
});

test('the fate says who leads, who won at which step and how a race was judged', () => {
  const sides = [{ code: '127', name: '' }, { code: '495', name: 'carpet' }];
  const at = (phase, reach, over = null) => fate({ phase, reach, over, sides });
  expect([at('ready', [0, 0]), at('running', [3, 2]), at('held', [2, 3]), at('running', [1, 1])]).toEqual(['at the start', 'code 127 leads', 'carpet leads, paused', 'level']);
  expect([at('done', [20, 9], { tick: 812, winner: 0, how: 'goal' }), at('done', [20, 20], { tick: 812, winner: -1, how: 'goal' }), at('done', [5, 6], { tick: 1000, winner: 1, how: 'stall' }), at('done', [5, 5], { tick: 60000, winner: -1, how: 'limit' })]).toEqual(['code 127 wins at step 812', 'dead heat at step 812', 'stalled at step 1000, carpet ahead', 'time up at step 60000, level']);
});

test('the status reads the fate, the step, both distances toward the goal and the facts of the pair, one value where both sides agree', () => {
  const side = { code: '127', name: '', fills: 7, of: 9, lit: 2401, cells: 2401, dimension: 1.7712 };
  const stat = { phase: 'running', tick: 812, reach: [12.34, 9.8], over: null, goal: 20, side: 81, walkers: 300, sides: [side, { ...side, code: '239', cells: 2400 }] };
  expect(rows(stat)).toEqual([['race', 'code 127 leads'], ['step', '812'], ['code 127', '12.3 of 20 cells'], ['code 239', '9.8 of 20 cells'], ['mass', '7 of 9'], ['dimension', '1.771'], ['room', '2401 of 2401 · 2400 of 2401'], ['board', '81 by 81'], ['walkers', '300']]);
  expect(rows({ ...stat, sides: [side, side] }).slice(2, 4).map(([name]) => name)).toEqual(['code 127 A', 'code 127 B']);
  expect(rows(null)).toEqual([]);
});
