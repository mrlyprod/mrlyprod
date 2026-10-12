import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { DEN, SHAPES, lay, outline, study, tag } from './engine.js';
import { SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const value = (over) => ({ ...defaults(SPEC), base: 3, code: '495', ...over });

const count = (types) => types.reduce((sum, on) => sum + (on ? 1 : 0), 0);

const grid = ({ shape: [rows, cols], types }) => Array.from({ length: rows }, (_, row) => Array.from(types.subarray(row * cols, (row + 1) * cols)));

const flip = (rows) => rows.map((row) => [...row].reverse());

const transpose = (rows) => rows[0].map((_, col) => rows.map((row) => row[col]));

test('the crop options are the named shapes of the plane after none', () => {
  expect(SHAPES.slice(1).map(([name]) => name)).toEqual(math.shape.shapes(2));
});

test('the outline reads the radius out of 32 on the unit box', () => {
  const sixteenth = outline(math, 'ball', 16);
  expect(sixteenth.Ball.radius).toEqual({ num: 1, den: 2 });
  expect(sixteenth.Ball.center).toEqual([{ num: 1, den: 2 }, { num: 1, den: 2 }]);
  expect(DEN).toBe(32);
});

test('a crop on a sheet that is not square keeps the shape centred on the short side and drops the rest', () => {
  const wide = study(math, value({ level: 1, x: 4, y: 2, crop: 'ball', radius: 16 }), rng(1));
  const rows = grid(wide.drawn);
  expect(wide.drawn.shape).toEqual([6, 12]);
  expect(rows.every((row) => row.slice(0, 3).every((on) => !on) && row.slice(9).every((on) => !on))).toBe(true);
  expect(count(wide.drawn.types)).toBe(wide.facts.fills);
  expect(rows).toEqual(flip(rows));
  expect(rows).toEqual([...rows].reverse());
  expect(wide.facts.cells).toBeLessThan(36);
  expect(wide.facts.cells).toBeGreaterThan(0);
  const tall = study(math, value({ level: 1, x: 2, y: 4, crop: 'ball', radius: 16 }), rng(1));
  expect(grid(tall.drawn)).toEqual(transpose(rows));
});

test('the study crops the sheet, counts the kept cells and inverts within the shape', () => {
  const plan = study(math, value({ level: 2 }), rng(1));
  const { facts } = plan;
  const tile = math.two.create('495', 3, 2, 0, 3);
  expect([facts.code, facts.level, facts.capped, facts.side, facts.cols, facts.rows]).toEqual(['495', 2, false, 9, 45, 45]);
  expect(facts.cells).toBeLessThan(45 * 45);
  expect(facts.fills).toBeLessThan(25 * math.two.fills(tile));
  expect(facts.fills + facts.voids).toBe(facts.cells);
  expect(count(plan.drawn.types)).toBe(facts.fills);
  expect(plan.runs.length).toBeGreaterThan(0);
  expect(plan.runs.reduce((sum, [, , len]) => sum + len, 0)).toBe(facts.fills);
  const inverted = study(math, value({ level: 2, invert: 1 }), rng(1));
  expect(count(inverted.drawn.types)).toBe(facts.voids);
  expect(inverted.facts).toEqual(facts);
  const plain = study(math, value({ level: 2, crop: '' }), rng(1));
  expect([plain.facts.cells, plain.facts.fills]).toEqual([45 * 45, 25 * math.two.fills(tile)]);
  expect(count(study(math, value({ level: 2, crop: '', invert: 1 }), rng(1)).drawn.types)).toBe(plain.facts.voids);
  expect(facts.dimension).toBeCloseTo(math.counts.dimension('495', 3, 2, 3));
  expect(typeof facts.perimeter).toBe('number');
});

test('touching keeps the cells the edge cuts as well, in the crop, the counts and the inverse', () => {
  const shape = outline(math, 'ball', 16);
  const tally = math.shape.census(shape, { shape: [45, 45], data: new Uint8Array(45 * 45) }).cells;
  const inside = study(math, value({ level: 2 }), rng(1));
  const touching = study(math, value({ level: 2, touch: 1 }), rng(1));
  expect(inside.facts.cells).toBe(tally[2]);
  expect(touching.facts.cells).toBe(tally[2] + tally[1]);
  expect(touching.facts.fills).toBeGreaterThan(inside.facts.fills);
  expect(touching.drawn.types.every((on, i) => !inside.drawn.types[i] || on)).toBe(true);
  expect(touching.facts.fills + touching.facts.voids).toBe(touching.facts.cells);
  const inverse = study(math, value({ level: 2, touch: 1, invert: 1 }), rng(1));
  expect(count(inverse.drawn.types)).toBe(touching.facts.voids);
  expect(inverse.drawn.types.some((on, i) => on && !inside.drawn.types[i] && !touching.drawn.types[i])).toBe(true);
});

test('a blank or junk code rolls one from the seed, and a level over the cap is lowered and said', () => {
  const rolled = study(math, value({ code: '', level: 1, x: 1, y: 1 }), rng(7)).facts.code;
  expect(rolled).toBe(study(math, value({ code: 'junk', level: 1, x: 1, y: 1 }), rng(7)).facts.code);
  expect(Number(rolled)).toBeGreaterThan(0);
  const big = study(math, value({ level: 5 }), rng(1)).facts;
  expect([big.level, big.capped, big.side]).toEqual([4, true, 81]);
  expect(study(math, value({ code: '7', base: 2, level: 1 }), rng(1)).facts.name).toBe('carpet');
});

test('the status tag is the name or the bare code, short enough to leave the seed room on a phone bar', () => {
  const tags = [study(math, value({ code: '486' }), rng(1)).facts, study(math, value({ code: '495' }), rng(1)).facts].map(tag);
  expect(tags).toEqual(['code 486', 'carpet 495']);
  expect(tags.every((one) => one.length <= 12)).toBe(true);
});

test('the layout fits the sheet in the short side and centres it', () => {
  expect(lay(45, 45, 800, 600, 0.9)).toEqual({ px: 12, x: 130, y: 30 });
  expect(lay(90, 45, 800, 600, 0.9)).toEqual({ px: 8, x: 40, y: 120 });
});
