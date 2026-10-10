import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { named } from '../designs/engine.js';
import { BUDGET, DIM, NUMBERS, SHAPES, cap, cutting, design, grow, rows, staged, study, tag } from './engine.js';
import { SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = (over) => ({ ...defaults(SPEC), seed: 1, base: 2, code: '23', ...over });

test('the level cap keeps the grown cube under the budget at every number', () => {
  expect(NUMBERS.map(cap)).toEqual([5, 3, 2, 2]);
  for (const n of NUMBERS) expect([n, n ** (DIM * cap(n)) <= BUDGET, n ** (DIM * (cap(n) + 1)) > BUDGET]).toEqual([n, true, true]);
});

test('grow caps the level at the number, cutting keeps only the keys the view uses, and staged hands the scene nothing else', () => {
  expect([grow({ number: 3, level: 5 }), grow({ number: '2', level: '5' }), grow({ number: 9, level: 0 })]).toEqual([{ number: 3, level: 3 }, { number: 2, level: 5 }, { number: 3, level: 1 }]);
  expect(rows('solid').map((row) => row.key)).toEqual(['view']);
  expect([rows('slice'), rows('diagonal'), rows('hex')].map((list) => list.map((row) => row.key))).toEqual([['view', 'axis', 'at'], ['view', 'at'], ['view']]);
  expect(cutting({ view: 'slice', axis: '1', at: 0.25 })).toEqual({ view: 'slice', axis: 1, at: 0.25 });
  expect(cutting({ view: 'bogus', axis: 1, at: 0.25 })).toEqual({ view: 'solid' });
  const value = { seed: 1, base: 2, code: '23', number: 4, level: 5, view: 'hex', axis: 1, at: 0.3, crop: '', radius: 16, spin: 1, tint: '' };
  expect(staged(value)).toEqual({ seed: 1, base: 2, code: '23', crop: '', radius: 16, spin: 1, tint: '', number: 4, level: 2, view: 'hex' });
  expect(staged({ ...value, at: 0.9 })).toEqual(staged(value));
});

test("the shapes of the spec are the crate's named 3D shapes", () => {
  expect(SHAPES).toEqual(math.shape.shapes(DIM));
});

test('a blank code rolls one from the seed, never the empty design, and a given code is kept', () => {
  const rolled = (seed) => design(math, value({ code: '' }), rng(seed)).code;
  expect(rolled(9)).toBe(rolled(9));
  expect(new Set(Array.from({ length: 12 }, (_, i) => rolled(i + 1))).size).toBeGreaterThan(1);
  expect(Array.from({ length: 40 }, (_, i) => rolled(i)).includes('0')).toBe(false);
  expect(design(math, value({ code: ' 23 ' }), rng(1))).toEqual({ base: 2, code: '23' });
  expect(design(math, value({ base: 3, code: '100' }), rng(1))).toEqual({ base: 3, code: '100' });
});

test('the carpet of 3 at level 3 reads its name, counts and dimension, and a level past the cap steps down', () => {
  const plan = study(math, value({}), rng(1));
  expect(plan.facts).toMatchObject({ code: '23', base: 2, number: 3, level: 3, side: 27, name: 'carpet', title: 'bang dim 3, code 23', fills: 8000, voids: 11683, surface: 18048, faces: 33024, euler: -1408, crop: null });
  expect(plan.facts.dimension).toBeCloseTo(Math.log(20) / Math.log(3), 6);
  expect(plan.cut).toBeNull();
  expect(study(math, value({ number: 5, level: 5 }), rng(1)).facts).toMatchObject({ level: 2, side: 25 });
  expect(study(math, value({ number: 2, level: 5 }), rng(1)).facts).toMatchObject({ level: 5, side: 32 });
  expect(study(math, value({ base: 3, code: '100', level: 2 }), rng(1)).facts).toMatchObject({ base: 3, side: 9, fills: 9, title: 'bang dim 3, base 3, code 100' });
});

test('the status tag is short: a name or the word code, then the code, led by b3 on base 3', () => {
  const read = (over) => tag(study(math, value(over), rng(1)).facts);
  expect([read({}), read({ code: '70' }), read({ base: 3, code: '100', level: 2 })]).toEqual(['carpet 23', 'code 70', 'b3 code 100']);
});

test('a ball crop keeps the touching cells and tallies the filled cells in, cut and out', () => {
  const plan = study(math, value({ crop: 'ball', radius: 16 }), rng(1));
  expect(plan.facts.crop).toEqual({ in: 2320, cut: 1224, out: 4456 });
  expect([plan.facts.fills, plan.facts.surface, plan.cell.shape]).toEqual([2320 + 1224, 8400, [27, 27, 27]]);
  expect(study(math, value({ crop: 'ball', radius: 32 }), rng(1)).facts.fills).toBe(8000);
  expect(study(math, value({ crop: 'box', radius: 8 }), rng(1)).facts.fills).toBeLessThan(8000);
});

test('a slice fixes one axis at the plane and reads the flat census', () => {
  const plan = study(math, value({ view: 'slice', axis: 2, at: 0.5 }), rng(1));
  expect([plan.cut.kind, plan.cut.side, plan.cut.flat.shape]).toEqual(['slice', 27, [27, 27]]);
  expect(plan.cut.facts).toEqual({ reading: 'z 14 of 27', fills: 64, voids: 665, euler: 64 });
  expect(study(math, value({ view: 'slice', axis: 0, at: 0 }), rng(1)).cut.facts.reading).toBe('x 1 of 27');
  expect(study(math, value({ view: 'slice', axis: 1, at: 1 }), rng(1)).cut.facts.reading).toBe('y 27 of 27');
});

test('a diagonal cut lists the filled cells on x + y + z = height, projected, and only those the crop keeps', () => {
  const plan = study(math, value({ view: 'diagonal', at: 0.5 }), rng(1));
  expect([plan.cut.kind, plan.cut.height, plan.cut.points.length]).toEqual(['diagonal', 39, 306]);
  expect(plan.cut.facts).toEqual({ reading: 'height 39 of 78', cells: 306, support: '0 to 78', least: 1, most: 306, constant: 'no' });
  const xtree = named(math, DIM, 2).find((one) => one.name === 'xtree').code;
  const lean = study(math, value({ view: 'diagonal', code: xtree, level: 2, at: 0.3 }), rng(1));
  expect(lean.cut.facts.cells).toBe(math.three.diagonal_slice(xtree, 3, 2, 2, lean.cut.height).length);
  const cropped = study(math, value({ view: 'diagonal', at: 0.5, crop: 'ball', radius: 16 }), rng(1));
  expect([cropped.cut.points.length > 0, cropped.cut.points.length < 306]).toEqual([true, true]);
  const gasket = study(math, value({ view: 'diagonal', code: '126', number: 2, level: 4, at: 0.4 }), rng(1));
  expect(gasket.cut.facts).toMatchObject({ least: 81, most: 81, constant: 'yes' });
  const empty = study(math, value({ view: 'diagonal', code: '0' }), rng(1));
  expect([empty.cut.points, empty.cut.facts.support, empty.cut.facts.cells]).toEqual([[], 'none', 0]);
});

test('a hexagon cut reads its triangles, pieces and holes, on an even side too', () => {
  const plan = study(math, value({ view: 'hex' }), rng(1));
  expect([plan.cut.kind, plan.cut.hex.cell.shape, plan.cut.codes]).toEqual(['hex', [54, 107], { fill: 1, void: 0 }]);
  expect(plan.cut.facts).toEqual({ reading: 'hexagon of 27', triangles: 4374, fills: 2250, voids: 2124, pieces: 1, holes: 49, euler: 1 });
  const even = study(math, value({ view: 'hex', code: '126', number: 2, level: 5 }), rng(1)).cut;
  expect([even.hex.cell.shape, even.facts]).toEqual([[64, 127], { reading: 'hexagon of 32', triangles: 6144, fills: 1944, voids: 4200, pieces: 691, holes: 960, euler: 1 }]);
});
