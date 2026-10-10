import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { defaults } from '../../lib/knobs.js';
import { rng } from '../../lib/scene.js';
import { COPIES, RATIO, RINGS, bounds, cap, copies, fit, mesh, parity, study, turned, wrap } from './engine.js';
import { SPEC } from './scene.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

const CARPET = { ...defaults(SPEC), base: 2, code: '23' };

const read = (value, seed = 1) => study(math, { ...CARPET, ...value }, rng(seed));

test('the level cap keeps six side squared times the copies under the budget', () => {
  expect([cap(3, 1), cap(5, 1), cap(7, 1), cap(9, 1)]).toEqual([3, 2, 2, 1]);
  expect([cap(3, 7), cap(5, 7), cap(7, 7), cap(3, 19), cap(3, 61)]).toEqual([3, 2, 1, 2, 2]);
});

test('the rings count the hexagons of the disc', () => {
  expect([0, 1, 2, 3, 4].map((rings) => copies(math, rings))).toEqual([1, 7, 19, 37, 61]);
  expect(Array.from({ length: RINGS + 1 }, (_, rings) => copies(math, rings))).toEqual(COPIES);
});

test('the disc keeps the parity of the uncropped sheet it sits in', () => {
  for (const projection of ['cut', 'iso', 'pro']) {
    for (const [number, level] of [[3, 1], [3, 2], [5, 1]]) {
      const one = math.six[`${projection}_design`]('23', number, level, 2);
      for (let rings = 1; rings <= 4; rings++) {
        const disc = math.six.tessellate(one, math.six.radial_mask(rings + 1, one.orientation));
        const tile = math.six.tile_cell(one, 2 * rings + 1, 2 * rings + 1, false);
        const [th, tw] = disc.shape;
        const [, W] = tile.cell.shape;
        const seat = () => {
          for (let dy = 0; dy + th <= tile.cell.shape[0]; dy++) {
            for (let dx = 0; dx + tw <= W; dx++) {
              let same = true;
              for (let i = 0; same && i < disc.types.length; i++) {
                if (disc.types[i] === 2) continue;
                same = disc.types[i] === tile.cell.types[(Math.floor(i / tw) + dy) * W + (i % tw) + dx];
              }
              if (same) return [dx, dy];
            }
          }
          return null;
        };
        const [dx, dy] = seat();
        expect([projection, number, level, rings, parity(one, rings, number ** level)]).toEqual([projection, number, level, rings, (tile.start + dx + dy) % 2]);
      }
    }
  }
});

test('the study reads the carpet in each projection and counts the skin of the iso', () => {
  const iso = read({ projection: 'iso' }).facts;
  expect(iso).toMatchObject({ code: '23', name: 'carpet', title: 'bang dim 3, code 23', level: 2, side: 9, copies: 1, triangles: 486, fills: 486, voids: 0, pieces: 1, holes: 0, euler: 1 });
  const cut = read({ projection: 'cut' }).facts;
  expect(cut).toMatchObject({ triangles: 486, fills: 306, voids: 180, pieces: 1, holes: 7 });
  expect(read({ projection: 'pro' }).facts).toMatchObject({ fills: 384, voids: 102 });
});

test('invert draws the anti design, so the fills and the voids swap', () => {
  const { facts, one } = read({ projection: 'cut', invert: 1 });
  expect([facts.fills, facts.voids]).toEqual([180, 306]);
  expect(Array.from(one.cell.types)).toEqual(Array.from(math.six.anti(math.six.cut_design('23', 3, 2, 2)).cell.types));
});

test('a level over the cap is lowered and the rings grow the sheet alone', () => {
  const deep = read({ number: 9, level: 3 });
  expect([deep.facts.level, deep.facts.side]).toEqual([1, 9]);
  const ringed = read({ projection: 'cut', radius: 1 });
  expect([ringed.facts.copies, ringed.facts.fills, ringed.sheet.cell.shape, ringed.one.cell.shape]).toEqual([7, 306, [54, 89], [18, 35]]);
  expect(math.six.census(ringed.sheet, false).fills).toBe(7 * 306);
});

test('a blank code rolls one from the seed, the same for the same seed', () => {
  const rolled = (seed) => read({ code: '' }, seed).facts.code;
  expect(rolled(5)).toBe(rolled(5));
  expect(new Set([1, 2, 3, 4, 5, 6].map(rolled)).size).toBeGreaterThan(1);
  expect(rolled(5)).not.toBe('0');
});

test('the mesh groups the triangles by type in paint order at the true aspect, turned on request', () => {
  const one = math.six.cut_design('23', 3, 1, 2);
  const flat = mesh(math, one, false);
  expect(flat.groups.map((group) => [group.name, group.points.length / 6])).toEqual([['VOID', 12], ['FILL', 42]]);
  expect(flat.box.map((v) => Math.round(v * 1000) / 1000)).toEqual([0, 0, 12, Math.round(12 * RATIO * 1000) / 1000]);
  expect(flat.across).toBe(true);
  const turned90 = mesh(math, one, true);
  expect(turned90.box.map((v) => Math.round(v * 1000) / 1000)).toEqual([-Math.round(12 * RATIO * 1000) / 1000, 0, 0, 12]);
  const iso = mesh(math, math.six.iso_design('23', 3, 1, 2), false);
  expect(iso.groups.map((group) => group.name)).toEqual(['UP', 'LEFT', 'RIGHT']);
  expect(iso.groups.reduce((sum, group) => sum + group.points.length / 6, 0)).toBe(54);
});

test('turned says when a hexagon must spin a quarter to point the way asked', () => {
  expect([turned('Horizontal', 'flat'), turned('Horizontal', 'pointy'), turned('Vertical', 'flat'), turned('Vertical', 'pointy')]).toEqual([false, true, true, false]);
});

test('fit centres a box inside the canvas with its padding', () => {
  expect(fit([0, 0, 12, 6], 200, 100, 10)).toEqual({ k: 80 / 6, ox: 100 - 6 * (80 / 6), oy: 50 - 3 * (80 / 6) });
  expect(bounds([])).toEqual([0, 0, 1, 1]);
});

test('wrap gives the rect tile its true aspect and turns it a quarter when asked', () => {
  const text = '<svg width="36" height="24" viewBox="0 0 36 24" xmlns="http://www.w3.org/2000/svg">\n<polygon points="0,0 2,4 4,0" fill="#008cff" stroke="none"/>\n</svg>';
  const flat = wrap(text, true, false);
  expect(flat).toBe(`<svg xmlns="http://www.w3.org/2000/svg" width="36" height="20.78" viewBox="0 0 36 20.78"><g transform="scale(1 0.866)"><polygon points="0,0 2,4 4,0" fill="#008cff" stroke="none"/></g></svg>`);
  const spun = wrap(text, true, true);
  expect(spun.startsWith('<svg xmlns="http://www.w3.org/2000/svg" width="20.78" height="36" viewBox="0 0 20.78 36"><g transform="translate(20.785 0) rotate(90) scale(1 0.866)">')).toBe(true);
  expect(wrap('<p>no</p>', true, false)).toBe('<p>no</p>');
});
