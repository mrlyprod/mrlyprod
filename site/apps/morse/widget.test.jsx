import { expect, test } from 'bun:test';
import { renderToStaticMarkup } from 'react-dom/server';
import * as math from '../../../pkgs/mrlyjs/math.js';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { Frame } from '../../lib/frame.jsx';
import { defaults } from '../../lib/knobs.js';
import { Knobs } from '../../lib/knobs.jsx';
import { study } from './engine.js';
import { SPEC, tidied } from './scene.js';
import { rng } from '../../lib/scene.js';
import { Design, ROWS, Widget, rerolled, shift, tapped, turned } from './widget.jsx';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });
num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const units = { num, math };

const value = (over) => tidied({ ...defaults(SPEC), seed: 1, base: 2, code: '9', ...over });

const facts = (over) => study(units, value(over)).facts;

const pane = (over) => renderToStaticMarkup(<Knobs spec={SPEC} value={value(over)} onChange={() => {}} />);

const design = (over, given = units) => renderToStaticMarkup(<Design value={value(over)} units={given} facts={facts(over)} onChange={() => {}} />);

const heads = (html) => html.match(/<h3>(\w+)<\/h3>/g);

test('the widget mounts its stage once the units are in hand', () => {
  const page = (given) => renderToStaticMarkup(<Frame label="Morse"><Widget value={value({})} onChange={() => {}} units={given} /></Frame>);
  expect(page(undefined)).not.toContain('<canvas');
  expect(page(units)).toContain('<canvas');
});

test('the knobs hold one View group: the view segment first, then the view own rows, a level reading its side', () => {
  const lifts = pane({});
  expect(heads(lifts)).toEqual(['<h3>View</h3>']);
  expect(lifts.indexOf('class="segment"')).toBeLessThan(lifts.indexOf('Level'));
  expect(lifts).toContain('<span class="num">6 · side 64</span>');
  expect(lifts).toContain('Lifts · Parity, t(i) xor t(j) · 6 · side 64 · Fit');
  expect(pane({ view: 'word' })).toContain('<span class="num">6 · 64 letters</span>');
  expect(pane({ view: 'word' })).not.toContain('Cell');
  expect(pane({ view: 'design', read: 'filter' })).toContain('<span class="num">3 · side 16</span>');
});

test('the Design fold shows only where a design is read and the units are in hand, naming the design and its tile side', () => {
  expect([design({}), design({ view: 'word' }), design({ view: 'design' }, null)]).toEqual(['', '', '']);
  const filter = design({ view: 'design', read: 'filter' });
  expect(heads(filter)).toEqual(['<h3>Design</h3>']);
  expect(filter).toContain('void 9 · side 2');
  expect(filter).toContain('Tile side');
});

test('a lift reads out its formula, its counts and whether it folds', () => {
  expect(ROWS.plane(facts({ level: 3 }))).toEqual([['lift', 'Parity, t(i) xor t(j)'], ['side', '8 by 8'], ['plus', 32], ['minus', 32], ['tile', '+-/-+'], ['kronecker power', 'yes, void read plus-minus']]);
  expect(Object.fromEntries(ROWS.plane(facts({ level: 3, lift: 'Xor' })))['kronecker power']).toBe('yes, the parity grid');
  expect(ROWS.plane(facts({ level: 3, lift: 'Sum' })).slice(-2)).toEqual([['kronecker power', 'no'], ['off its fold', '12 of 64 sites, first at row 1, col 3']]);
});

test('the word reads out its letters, the two rules and the schedule exponent, and its runs when coloured by them', () => {
  const read = Object.fromEntries(ROWS.word(facts({ view: 'word' })));
  expect(read).toMatchObject({ letters: 64, 'the two rules': 'agree letter for letter', 'as a schedule': 'htree at 0, carpet at 1', exponent: '1.292481250360578, (1/2) log2 6, proved', 'rate at 64 letters': '1.259598912', 'periodic control': '1.231436085' });
  const runs = Object.fromEntries(ROWS.word(facts({ view: 'word', colour: 'runs' })));
  expect(runs).toMatchObject({ letters: 64, 'longest run': 2, 'cube free': 'yes, no 000 and no 111', boundaries: 'the period-doubling word' });
});

test('the filter names its closed form, the share off Thue-Morse to four places, and why a wider tile has none', () => {
  const read = (over) => Object.fromEntries(ROWS.filter(facts({ view: 'design', read: 'filter', ...over })));
  expect(read({ depth: 2 })).toMatchObject({ tile: '+-/-+, the Thue-Morse tile', difference: 'exactly the tile repeated', 'thue-morse grid': 'no', 'off thue-morse': '32 of 64 sites, a share of 0.5000, a coin' });
  expect(read({ depth: 2, fold: 'design' })['off thue-morse']).toBe('24 of 64 sites, a share of 0.3750');
  expect(read({ code: '495', base: 3, depth: 1 })['off thue-morse']).toBe('nothing to compare: the grid lives at tile side 2, not 3');
});

test('an arrow moves the view own level and stops at its ends; a tap goes one deeper and wraps', () => {
  expect([shift(value({ view: 'word', round: 10 }), 1), shift(value({ round: 3, view: 'word' }), -1), shift(value({ level: 9 }), 1), shift(value({ view: 'design', read: 'filter', depth: 8 }), 1)]).toEqual([{ round: 10 }, { round: 2 }, { level: 9 }, { depth: 8 }]);
  expect([shift(value({ view: 'word', round: 10 }), 1, true), shift(value({ view: 'design', power: 5, number: 3 }), 1, true)]).toEqual([{ round: 1 }, { power: 1 }]);
});

test('a tap on a lift moves to the next lift, and wraps', () => {
  expect([tapped(value({})), tapped(value({ lift: 'Sum' })), tapped(value({ view: 'word', round: 3 }))]).toEqual([{ lift: 'And' }, { lift: 'Parity' }, { round: 4 }]);
});

test('the side arrows step through the four pictures, one at a time', () => {
  expect([turned(value({}), 1), turned(value({ view: 'design' }), 1), turned(value({ view: 'design', read: 'filter' }), 1), turned(value({ view: 'word' }), -1)]).toEqual([{ view: 'design', read: 'power' }, { view: 'design', read: 'filter' }, { view: 'word' }, { view: 'design', read: 'filter' }]);
});

test('a reroll rolls what the view shows: a new lift and a level, a new round and a colour, or a new code', () => {
  const plane = rerolled(math, value({ view: 'plane', lift: 'Parity' }), rng(3));
  const word = rerolled(math, value({ view: 'word', round: 6 }), rng(3));
  const design = rerolled(math, value({ view: 'design', read: 'power' }), rng(3));
  expect([Object.keys(plane), plane.lift === 'Parity', Object.keys(word), word.round === 6, Object.keys(design)]).toEqual([['lift', 'level'], false, ['round', 'colour'], false, ['code']]);
  expect(rerolled(math, value({ view: 'plane' }), rng(9))).toEqual(rerolled(math, value({ view: 'plane' }), rng(9)));
});
