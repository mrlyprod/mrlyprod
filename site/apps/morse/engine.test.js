import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { defaults, optionsOf, tidy } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { total } from '../designs/engine.js';
import { BEAT, DISTINCT, END, FORMULA, IN, KINDS, OUT, SLIDE, block, camera, cap, choose, ease, flat, lay, panels, phase, picture, print, roll, sideOf, span, study, tower } from './engine.js';
import { SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });
num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const units = { num, math };

const value = (over) => ({ ...defaults(SPEC), seed: 1, base: 2, code: '', level: 3, ...over });

const bits = (grid) => Array.from(grid).join('');

test('the lift kinds and their formulas are the unit\'s, in its order', () => {
  expect(KINDS).toEqual(num.morse.LIFTS());
  for (const kind of KINDS) expect([kind, FORMULA[kind]]).toEqual([kind, num.morse.Lift.formula(kind)]);
});

test('the lifts the seed rolls among are the ones with a grid of their own', () => {
  const grid = (kind) => bits(num.morse.lift(kind, 8));
  expect(new Set(KINDS.map(grid)).size).toBe(DISTINCT.length);
  expect(new Set(DISTINCT.map(grid)).size).toBe(DISTINCT.length);
});

test('the level cap keeps the side of every tile side at or under the budget', () => {
  expect([cap(2), cap(3), cap(5)]).toEqual([9, 5, 3]);
});

test('tidy caps the level by view and tile side: the word at a thousand letters, the filter a level short', () => {
  const level = (over) => tidy(SPEC, { ...defaults(SPEC), level: 99, ...over }).level;
  expect([level({ view: 'plane', number: 5 }), level({ view: 'word', number: 5 }), level({ view: 'lift', number: 3 }), level({ view: 'lift', number: 5 }), level({ view: 'filter', number: 2 }), level({ view: 'filter', number: 5 })]).toEqual([9, 10, 5, 3, 8, 2]);
});

test('a tile that depends on one axis alone is flat, since its sign power is stripes', () => {
  expect([flat([1, 1, 0, 0], 2), flat([1, 0, 1, 0], 2), flat([0, 0, 0, 0], 2), flat([1, 1, 1, 1], 2)]).toEqual([true, true, true, true]);
  expect([flat([1, 0, 0, 1], 2), flat([1, 0, 0, 0], 2), flat([1, 1, 0, 1, 0, 1, 0, 0, 1], 3)]).toEqual([false, false, false]);
});

test('a rolled code is never empty, full or flat, and is the same for one seed', () => {
  const rolled = Array.from({ length: 40 }, (_, i) => roll(math, 2, 2, rng(i)));
  expect(new Set(rolled)).toEqual(new Set(['1', '2', '4', '6', '7', '8', '9', '11', '13', '14']));
  expect(roll(math, 3, 3, rng(5))).toBe(roll(math, 3, 3, rng(5)));
  for (let i = 0; i < 20; i++) {
    const code = roll(math, 3, 3, rng(i));
    expect([code, flat(math.two.create(code, 3, 1, 0, 3).types, 3)]).toEqual([code, false]);
  }
});

test('the seed picks the lift kind when none is named and the code when none is typed, the same code either way', () => {
  const lift = (over) => value({ view: 'lift', ...over });
  const kinds = new Set(Array.from({ length: 60 }, (_, i) => choose(math, lift(), rng(i)).kind));
  expect(kinds).toEqual(new Set(DISTINCT));
  expect(choose(math, lift(), rng(1))).toEqual(choose(math, lift(), rng(1)));
  expect(choose(math, lift({ lift: 'Sum' }), rng(1))).toEqual({ kind: 'Sum', code: choose(math, lift(), rng(1)).code });
  expect(choose(math, lift({ code: '21' }), rng(1)).code).toBe('5');
  expect(choose(math, lift({ code: '9' }), rng(1)).code).toBe('9');
  expect(KINDS.includes(choose(math, lift({ lift: 'junk' }), rng(1)).kind)).toBe(true);
});

test('a code is rolled only for the views that draw a design', () => {
  const code = (view) => choose(math, value({ view }), rng(1)).code;
  expect([code('plane'), code('word')]).toEqual(['', '']);
  expect(code('lift')).not.toBe('');
  expect(code('filter')).toBe(code('lift'));
});

test('the plane study reads a lift as a sign grid lit on plus, folded against its corner tile', () => {
  const parity = study(units, value({ lift: 'Parity' }), rng(1));
  expect(parity.facts).toEqual({ view: 'plane', level: 3, kind: 'Parity', formula: 't(i) xor t(j)', side: 8, plus: 32, minus: 32, folds: true, faults: 0, first: null, tile: '+-/-+', twin: '' });
  expect([parity.number, parity.side, parity.anchor, parity.panels.length, parity.rows]).toEqual([2, 8, [0, 0], 1, []]);
  expect(bits(parity.panels[0].types.slice(0, 8))).toBe('10010110');
  const sum = study(units, value({ lift: 'Sum' }), rng(1));
  expect(sum.facts).toMatchObject({ kind: 'Sum', folds: false, faults: 12, first: [1, 3], tile: '+-/--', twin: '' });
  expect(study(units, value({ lift: 'And', level: 2 }), rng(1)).facts).toMatchObject({ side: 4, plus: 10, minus: 6, folds: true, tile: '++/+-' });
  expect(study(units, value({ lift: 'Xor' }), rng(1)).facts.twin).toBe('Parity');
});

test('the word study lays the substitution stages as rows and counts the runs of the last', () => {
  const plan = study(units, value({ view: 'word' }), rng(1));
  expect(plan.facts).toEqual({ view: 'word', level: 3, letters: 8, plus: 4, minus: 4, runs: 6, longest: 2, singles: 4, doubles: 2, doubling: true, rule: true, cube: true });
  expect(plan.rows.map(bits)).toEqual(['10', '1001', '10010110']);
  expect([plan.panels, plan.side, plan.number]).toEqual([[], 8, 2]);
});

test('the lift study sets a design beside its plus-minus power, anchored on the tile\'s first filled cell', () => {
  const nine = study(units, value({ view: 'lift', code: '9', level: 2 }), rng(1));
  expect(nine.facts).toEqual({ view: 'lift', level: 2, code: '9', name: 'void', title: 'bang dim 2, code 9', number: 2, base: 2, side: 4, cells: 16, filled: 4, plus: 8, agree: 12, differ: 4, tile: '+-/-+', exact: true });
  expect(nine.panels.map((panel) => bits(panel.types))).toEqual(['1000010000100001', '1001011001101001']);
  expect(nine.anchor).toEqual([0, 0]);
  const six = study(units, value({ view: 'lift', code: '6', level: 2 }), rng(1));
  expect([six.anchor, six.facts.tile, six.facts.exact]).toEqual([[0, 1], '-+/+-', true]);
  const carpet = study(units, value({ view: 'lift', code: '495', number: 3, base: 3, level: 2 }), rng(1));
  expect(carpet.facts).toMatchObject({ name: 'carpet', side: 9, filled: 64, plus: 65, agree: 80, differ: 1, tile: '+++/+-+/+++', exact: false });
  expect(print([0, 1, 1, 1], 2)).toBe('+-/--');
});

test('the plus-minus filter blows a level up and exclusive-ors it with the next, which leaves the tile repeated and half the sites off Thue-Morse', () => {
  const nine = study(units, value({ view: 'filter', code: '9', level: 2 }), rng(1));
  expect(nine.facts).toEqual({ view: 'filter', level: 2, fold: 'sign', code: '9', name: 'void', title: 'bang dim 2, code 9', number: 2, base: 2, side: 8, cells: 64, differ: 32, closed: true, morse: 32, half: true, tile: '+-/-+' });
  expect([nine.level, nine.side, nine.anchor, nine.rows]).toEqual([3, 8, [0, 0], []]);
  expect(nine.panels.map((panel) => bits(panel.types.slice(0, 8)))).toEqual(['11000011', '10010110', '10101010']);
  expect(study(units, value({ view: 'filter', code: '7', level: 2 }), rng(1)).facts).toMatchObject({ differ: 16, closed: true, morse: 32, half: true, tile: '++/+-' });
});

test('the design filter keeps what the next level empties, the lit sites times the tile\'s empty cells', () => {
  const nine = study(units, value({ view: 'filter', code: '9', level: 2, fold: 'design' }), rng(1));
  expect(nine.facts).toMatchObject({ fold: 'design', side: 8, differ: 8, closed: true, morse: 24, half: false });
  expect(nine.panels.map((panel) => bits(panel.types.slice(0, 8)))).toEqual(['11000000', '10000000', '01000000']);
});

test('the Thue-Morse grid lives at tile side two, so a wider tile has nothing to score against', () => {
  const carpet = study(units, value({ view: 'filter', code: '495', number: 3, base: 3, level: 1 }), rng(1));
  expect(carpet.facts).toMatchObject({ side: 9, differ: 9, closed: true, morse: null, half: false });
});

test('the tile side starts at the base, so base 3 never offers side 2 and no two codes share a tile', () => {
  const row = SPEC.find((one) => one.key === 'number');
  const offered = (base) => optionsOf(row, { base }).map(([, label]) => label);
  expect([offered(2), offered(3), offered('3'), offered(7)]).toEqual([['2', '3', '5'], ['3', '5'], ['3', '5'], ['2', '3', '5']]);
  const side = (over) => {
    const raw = { ...defaults(SPEC), view: 'lift', ...over };
    return sideOf({ ...raw, ...tidy(SPEC, raw) });
  };
  expect([side({ base: 3, number: 2 }), side({ base: 3, number: 3 }), side({ base: 3, number: 5 }), side({ base: 2, number: 3 }), side({ base: 2, number: 2 })]).toEqual([3, 3, 5, 3, 2]);
  expect(study(units, value({ view: 'lift', code: '7', base: 3, level: 1 }), rng(1)).facts.number).toBe(3);
  for (const base of [2, 3]) {
    for (const [number] of optionsOf(row, { base })) {
      const width = number || base;
      const tiles = new Set(Array.from({ length: total(math, 2, base) }, (_, code) => bits(math.two.create(String(code), width, 1, 0, base).types)));
      expect([base, width, tiles.size]).toEqual([base, width, total(math, 2, base)]);
    }
  }
});

test('the same study is kept across a seed, a tint or a cell change, and made afresh for another level', () => {
  const plan = study(units, value({ lift: 'Parity' }), rng(1));
  expect(study(units, value({ lift: 'Parity', seed: 7, tint: 'red', cell: 4 }), rng(7))).toBe(plan);
  expect(study(units, value({ lift: 'Parity', level: 4 }), rng(1))).not.toBe(plan);
  expect(study(units, value({ lift: 'Parity' }), rng(1))).not.toBe(plan);
});

test('the clock holds each level a beat, slides to the next, holds the last, fades and loops', () => {
  expect(span(3)).toBe(3 * BEAT + 2 * SLIDE + END);
  expect(phase(0, 3, false)).toEqual({ level: 1, p: 0, alpha: 0 });
  expect(phase(IN / 2, 3, false)).toEqual({ level: 1, p: 0, alpha: 0.5 });
  expect(phase(BEAT - 1, 3, false)).toEqual({ level: 1, p: 0, alpha: 1 });
  expect(phase(BEAT + SLIDE / 2, 3, false)).toEqual({ level: 1, p: ease(0.5), alpha: 1 });
  expect(phase(BEAT + SLIDE, 3, false)).toEqual({ level: 2, p: 0, alpha: 1 });
  expect(phase(2 * (BEAT + SLIDE), 3, false)).toEqual({ level: 3, p: 0, alpha: 1 });
  expect(phase(span(3) - END, 3, false)).toEqual({ level: 3, p: 0, alpha: 1 });
  expect(phase(span(3) - OUT / 2, 3, false)).toEqual({ level: 3, p: 0, alpha: 0.5 });
  expect(phase(span(3), 3, false)).toEqual({ level: 1, p: 0, alpha: 0 });
  expect(phase(-OUT / 3, 3, false).level).toBe(3);
  expect(phase(BEAT + SLIDE / 2, 3, true)).toEqual({ level: 3, p: 0, alpha: 1 });
  expect([ease(0), ease(0.5), ease(1)]).toEqual([0, 0.5, 1]);
});

test('the camera window at a level is the block chain of the anchor, and a slide zooms between two levels', () => {
  const plain = { number: 2, level: 3, anchor: [0, 0] };
  expect([block(plain, 1), block(plain, 2), block(plain, 3)]).toEqual([[0, 0, 2], [0, 0, 4], [0, 0, 8]]);
  expect(camera(plain, 1, 0.5)).toEqual([0, 0, 2 * Math.SQRT2]);
  expect(camera(plain, 3, 0.5)).toEqual([0, 0, 8]);
  const corner = { number: 2, level: 3, anchor: [1, 1] };
  expect([block(corner, 1), block(corner, 2), block(corner, 3)]).toEqual([[6, 6, 2], [4, 4, 4], [0, 0, 8]]);
  expect(camera(corner, 1, 0.5)).toEqual([5, 5, 2 * Math.SQRT2]);
  expect(block({ number: 3, level: 2, anchor: [0, 2] }, 1)).toEqual([6, 0, 3]);
});

test('one grid takes the stage, several split it across in landscape and down in portrait', () => {
  expect(panels(800, 600, 1)).toEqual([{ x: 0, y: 0, w: 800, h: 600 }]);
  expect(panels(800, 600, 2)).toEqual([{ x: 0, y: 0, w: 400, h: 600 }, { x: 400, y: 0, w: 400, h: 600 }]);
  expect(panels(600, 800, 2)).toEqual([{ x: 0, y: 0, w: 600, h: 400 }, { x: 0, y: 400, w: 600, h: 400 }]);
  expect(panels(900, 600, 3).map((panel) => panel.x)).toEqual([0, 300, 600]);
  expect(panels(600, 900, 3).map((panel) => panel.y)).toEqual([0, 300, 600]);
});

test('a grid is laid centred on whole pixels a cell, fitted to the panel or pinned to a cell size', () => {
  expect(lay({ x: 0, y: 0, w: 800, h: 600 }, 8, 0, 32)).toEqual({ x: 156, y: 56, size: 488, px: 61 });
  expect(lay({ x: 400, y: 0, w: 400, h: 600 }, 8, 4, 32)).toEqual({ x: 584, y: 284, size: 32, px: 4 });
  expect(lay({ x: 0, y: 0, w: 100, h: 100 }, 512, 0, 10).px).toBe(1);
});

test('the tower lays one row a stage, each letter whole pixels wide, the rows spaced down the stage', () => {
  const rows = tower(800, 600, 3, 32);
  expect([rows.x, rows.w, rows.unit]).toEqual([32, 736, 92]);
  expect(rows.rows).toEqual([{ y: 33, h: 172 }, { y: 214, h: 172 }, { y: 395, h: 172 }]);
});

test('a stage narrower than the word fits it, a pixel a letter at most', () => {
  const narrow = tower(300, 600, 10, 16);
  expect([narrow.x, narrow.w, narrow.unit]).toEqual([16, 268, 1]);
});

test('the svg lays the lit cells as rects: one sheet for a plane, two side by side for a lift, three for a filter, a row a stage for the word', () => {
  const plane = picture(study(units, value({ lift: 'Parity', level: 2 }), rng(1)), '#008cff');
  expect(plane.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 40 40"><g fill="#008cff" shape-rendering="crispEdges">')).toBe(true);
  expect(plane.match(/<rect /g)).toHaveLength(6);
  expect(plane).toContain('<rect x="10" y="10" width="20" height="10"/>');
  const lift = picture(study(units, value({ view: 'lift', code: '9', level: 2 }), rng(1)), 'A');
  expect(lift).toContain('width="84" height="40"');
  expect(lift.match(/<rect /g)).toHaveLength(4 + 6);
  expect(lift).toContain('<rect x="44" y="0" width="10" height="10"/>');
  const filter = picture(study(units, value({ view: 'filter', code: '9', level: 1 }), rng(1)), 'A');
  expect(filter).toContain('width="128" height="40"');
  const word = picture(study(units, value({ view: 'word', level: 2 }), rng(1)), 'A');
  expect(word).toContain('width="40" height="90"');
  expect(word.match(/<rect /g)).toHaveLength(3);
  expect(word).toContain('<rect x="0" y="0" width="20" height="40"/>');
});
