import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults, optionsOf, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { total } from '../designs/engine.js';
import { FORMULA, KINDS, PAD, cap, coded, fits, flat, lay, pictureOf, picture, place, print, roll, sideOf, spread, study, tower } from './engine.js';
import { SPEC, tidied } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });
num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const units = { num, math };

const value = (over) => tidied({ ...defaults(SPEC), seed: 1, base: 2, code: '9', ...over });

const plan = (over) => study(units, value(over));

const bits = (grid) => Array.from(grid).join('');

const INKS = { plus: 'P', minus: 'M', unlit: 'U', diff: 'D', one: 'O', two: 'T' };

test("the lift kinds and their formulas are the unit's, in its order", () => {
  expect(KINDS).toEqual(num.morse.LIFTS());
  for (const kind of KINDS) expect([kind, FORMULA[kind]]).toEqual([kind, num.morse.Lift.formula(kind)]);
});

test('the level cap keeps the side of every tile side at or under the budget', () => {
  expect([cap(2), cap(3), cap(5)]).toEqual([9, 5, 3]);
});

test('each view keeps its own level: the lifts to side 512, the rounds to ten, the sign power by the tile side, the filter a level short', () => {
  const most = (over) => tidy(SPEC, { ...defaults(SPEC), level: 99, round: 99, power: 99, depth: 99, ...over });
  expect([most({}).level, most({}).round, most({}).power, most({ number: 3 }).power, most({}).depth, most({ number: 5 }).depth]).toEqual([9, 10, 9, 5, 8, 2]);
  expect([defaults(SPEC).round, defaults(SPEC).level, defaults(SPEC).power, defaults(SPEC).depth]).toEqual([6, 6, 5, 3]);
});

test('the three views hold four pictures, the design view reading a power or a filter', () => {
  expect(['word', 'plane', 'design'].map((view) => pictureOf(value({ view })))).toEqual(['word', 'plane', 'power']);
  expect(pictureOf(value({ view: 'design', read: 'filter' }))).toBe('filter');
  expect(['word', 'plane', 'power', 'filter'].map(place)).toEqual([{ view: 'word' }, { view: 'plane' }, { view: 'design', read: 'power' }, { view: 'design', read: 'filter' }]);
});

test('a tile that depends on one axis alone is flat, since its sign power is stripes', () => {
  expect([flat([1, 1, 0, 0], 2), flat([1, 0, 1, 0], 2), flat([0, 0, 0, 0], 2), flat([1, 1, 1, 1], 2)]).toEqual([true, true, true, true]);
  expect([flat([1, 0, 0, 1], 2), flat([1, 0, 0, 0], 2), flat([1, 1, 0, 1, 0, 1, 0, 0, 1], 3)]).toEqual([false, false, false]);
});

test('a rolled code is never empty, full or flat, and is the same for one seed', () => {
  const rolled = Array.from({ length: 40 }, (_, i) => roll(math, 2, 2, rng(i)));
  expect(new Set(rolled)).toEqual(new Set(['1', '2', '4', '6', '7', '8', '9', '11', '13', '14']));
  expect(roll(math, 3, 3, rng(5))).toBe(roll(math, 3, 3, rng(5)));
});

test('a typed code wraps by the code count, and a blank or junk code reads as 9, never a roll', () => {
  expect([coded(math, { code: '21', base: 2 }), coded(math, { code: ' 6 ', base: 2 }), coded(math, { code: '', base: 2 }), coded(math, { code: 'void', base: 3 })]).toEqual(['5', '6', '9', '9']);
});

test('the lifts open on parity, one grid a lift: parity and and fold as void and carpet read plus-minus, xor is parity again, sum fails', () => {
  const lifts = KINDS.map((lift) => plan({ lift, level: 3 }));
  expect(plan({}).facts.kind).toBe('Parity');
  expect(lifts.map((one) => [one.paint, one.side, one.types.length])).toEqual(KINDS.map(() => ['sign', 8, 64]));
  expect(bits(lifts[0].types.slice(0, 8))).toBe('01101001');
  expect(lifts.map(({ facts }) => [facts.kind, facts.formula, facts.folds, facts.name, facts.twin, facts.tile])).toEqual([['Parity', FORMULA.Parity, true, 'void', '', '+-/-+'], ['And', FORMULA.And, true, 'carpet', '', '++/+-'], ['Xor', FORMULA.Xor, true, 'void', 'Parity', '+-/-+'], ['Sum', FORMULA.Sum, false, '', '', '+-/--']]);
  expect(lifts[3].facts).toMatchObject({ faults: 12, first: [1, 3] });
});

test('the word stacks the rounds of the substitution, and bit parity agrees letter for letter', () => {
  const word = plan({ view: 'word', round: 3 });
  expect([word.paint, word.types, word.rows.map(bits)]).toEqual(['sign', null, ['01', '0110', '01101001']]);
  expect(word.facts).toMatchObject({ round: 3, letters: 8, plus: 4, minus: 4, agree: true });
});

test('the word read as a schedule of two designs has the exponent (1/2) log2 6, above its prefix rate and the periodic control', () => {
  const f = plan({ view: 'word' }).facts;
  expect([f.zero, f.one, f.prefix]).toEqual(['htree', 'carpet', 64]);
  expect(f.exponent).toBeCloseTo(Math.log2(6) / 2, 14);
  expect([f.rate.toFixed(9), f.control.toFixed(9)]).toEqual(['1.259598912', '1.231436085']);
});

test('coloured by runs, each letter takes the length of its run, no run reaches three, and the boundaries spell the period-doubling word', () => {
  const runs = plan({ view: 'word', round: 6, colour: 'runs' });
  expect([runs.paint, runs.rows.map((row) => row.length)]).toEqual(['runs', [2, 4, 8, 16, 32, 64]]);
  expect(bits(runs.rows.at(-1).slice(0, 8))).toBe('12211222');
  expect(runs.facts).toMatchObject({ letters: 64, longest: 2, ones: 32, cube: true, doubling: true });
  expect(plan({ view: 'word', round: 1, colour: 'runs' }).facts).toMatchObject({ letters: 2, runs: 2, cube: true, doubling: true });
  expect(bits(spread([1, 2, 1], 4))).toBe('1221');
});

test('the sign power draws the power of the plus-minus tile of a design', () => {
  const nine = plan({ view: 'design', power: 2 });
  expect(nine.facts).toEqual({ view: 'power', level: 2, code: '9', name: 'void', title: 'bang dim 2, code 9', number: 2, base: 2, side: 4, cells: 16, filled: 4, plus: 8, agree: 12, differ: 4, tile: '+-/-+', exact: true });
  expect([bits(nine.types), nine.paint]).toEqual(['0110100110010110', 'sign']);
  const carpet = plan({ view: 'design', code: '495', number: 3, base: 3, power: 2 });
  expect(carpet.facts).toMatchObject({ name: 'carpet', side: 9, filled: 64, plus: 65, agree: 80, differ: 1, tile: '+++/+-+/+++', exact: false });
  expect(print([0, 1, 1, 1], 2)).toBe('+-/--');
});

test('the plus-minus filter draws the difference of a level blown up and the next, the tile repeated, off Thue-Morse at half the sites', () => {
  const nine = plan({ view: 'design', read: 'filter', depth: 2 });
  expect(nine.facts).toEqual({ view: 'filter', level: 2, fold: 'sign', code: '9', name: 'void', title: 'bang dim 2, code 9', number: 2, base: 2, side: 8, cells: 64, differ: 32, closed: true, form: 'the tile repeated', morse: 32, exact: false, half: true, share: 0.5, kin: true, tile: '+-/-+' });
  expect([bits(nine.types.slice(0, 8)), nine.paint]).toEqual(['01010101', 'sign']);
  expect(plan({ view: 'design', read: 'filter', code: '7', depth: 2 }).facts).toMatchObject({ differ: 16, closed: true, morse: 32, half: true, kin: false });
  const halves = Array.from({ length: 16 }, (_, code) => [1, 2, 3].map((depth) => plan({ view: 'design', read: 'filter', code: String(code), depth }).facts).every((f) => f.closed && f.half));
  expect(halves.every(Boolean)).toBe(true);
});

test('the design filter lights what the next level empties, the blow-up masked by the tile complement', () => {
  const nine = plan({ view: 'design', read: 'filter', depth: 2, fold: 'design' });
  expect(nine.facts).toMatchObject({ fold: 'design', side: 8, differ: 8, closed: true, form: 'the blow-up, masked by the tile complement', morse: 24, half: false });
  expect([bits(nine.types.slice(0, 8)), nine.paint]).toEqual(['01000000', 'diff']);
});

test('the Thue-Morse grid lives at tile side two, so a wider tile has nothing to score against', () => {
  expect(plan({ view: 'design', read: 'filter', code: '495', number: 3, base: 3, depth: 1 }).facts).toMatchObject({ side: 9, differ: 9, closed: true, morse: null, share: null, half: false });
});

test('the tile side starts at the base, so base 3 never offers side 2 and no two codes share a tile', () => {
  const row = SPEC.find((one) => one.key === 'number');
  const offered = (base) => optionsOf(row, { base }).map(([, label]) => label);
  expect([offered(2), offered(3), offered('3'), offered(7)]).toEqual([['2', '3', '5'], ['3', '5'], ['3', '5'], ['2', '3', '5']]);
  const side = (over) => sideOf(value({ view: 'design', ...over }));
  expect([side({ base: 3, number: 2 }), side({ base: 3, number: 3 }), side({ base: 3, number: 5 }), side({ base: 2, number: 3 }), side({ base: 2, number: 2 })]).toEqual([3, 3, 5, 3, 2]);
  for (const base of [2, 3]) {
    for (const [number] of optionsOf(row, { base })) {
      const width = number || base;
      const tiles = new Set(Array.from({ length: total(math, 2, base) }, (_, code) => bits(math.two.create(String(code), width, 1, 0, base).types)));
      expect([base, width, tiles.size]).toEqual([base, width, total(math, 2, base)]);
    }
  }
});

test('the study is kept across a seed or a cell change, and made afresh for another level', () => {
  const was = plan({});
  expect(plan({ seed: 7, cell: 4 })).toBe(was);
  expect(plan({ level: 4 })).not.toBe(was);
});

test('a grid fills the short side of the stage at any aspect, centred, or is pinned to a cell size', () => {
  expect(lay(800, 600, 8, 0, 16)).toEqual({ x: 116, y: 16, size: 568 });
  expect(lay(390, 844, 64, 0, 16)).toEqual({ x: 16, y: 243, size: 358 });
  expect(lay(800, 600, 8, 4, 16)).toEqual({ x: 384, y: 284, size: 32 });
});

test('the rounds share the whole stage, each row across its width, a gap between letters once a letter is three pixels', () => {
  const rows = [new Uint8Array(2), new Uint8Array(4), new Uint8Array(512)];
  const laid = tower(800, 600, rows, 1);
  expect(laid.map(({ x, step, gap }) => [x, step, gap])).toEqual([[PAD, 384, 1], [PAD, 192, 1], [PAD, 768 / 512, 0]]);
  expect([laid[0].y, laid.at(-1).y + laid.at(-1).h]).toEqual([PAD, 600 - PAD]);
  expect(laid[1].y - (laid[0].y + laid[0].h)).toBe(9);
});

test('the svg lays every cell in its hue: plus and minus on a lift, a row a round of the word', () => {
  const lifts = picture(plan({ level: 1 }), INKS);
  expect(lifts).toBe('<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20"><g fill="P" shape-rendering="crispEdges"><rect x="0" y="0" width="10" height="10"/><rect x="10" y="10" width="10" height="10"/></g><g fill="M" shape-rendering="crispEdges"><rect x="10" y="0" width="10" height="10"/><rect x="0" y="10" width="10" height="10"/></g></svg>');
  const word = picture(plan({ view: 'word', round: 2 }), INKS);
  expect(word).toContain('width="40" height="90"');
  expect(word).toContain('<g fill="P" shape-rendering="crispEdges"><rect x="0" y="0" width="20" height="40"/></g>');
  expect(picture(plan({ view: 'design', read: 'filter', fold: 'design', depth: 1 }), INKS)).toContain('fill="D"');
});

test('the svg is offered up to a budget of cells', () => {
  expect([fits(plan({ level: 8 })), fits(plan({ level: 9 })), fits(plan({ view: 'word', round: 10 }))]).toEqual([true, false, true]);
});
