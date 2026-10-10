import { expect, test } from 'bun:test';
import * as math from '../../../pkgs/mrlyjs/math.js';
import { rng } from '../../lib/scene.js';
import { cap, census, classes, codes, facts, gallery, grow, named, next, resolve, roll, title, total, word } from './engine.js';

math.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/math/mrlyjs_math_bg.wasm', import.meta.url)).arrayBuffer() });

test('the level cap keeps a grown design under the cell budget in each dimension', () => {
  expect([cap(2), cap(3)]).toEqual([5, 3]);
});

test('a space lists its codes up to the limit and its classes where the unit can walk them', () => {
  expect([total(math, 2, 2), total(math, 3, 2), total(math, 2, 3), total(math, 3, 3)]).toEqual([16, 256, 512, 134217728]);
  expect([codes(math, 2, 2).length, codes(math, 2, 3).length, codes(math, 3, 3)]).toEqual([16, 512, null]);
  expect([classes(math, 2, 2).length, classes(math, 3, 2).length, classes(math, 2, 3).length, classes(math, 3, 3)]).toEqual([6, 22, 26, null]);
  expect(classes(math, 2, 2).map((one) => one.code)).toEqual(['0', '1', '3', '6', '7', '15']);
  expect(classes(math, 3, 2)).toBe(classes(math, 3, 2));
});

test('a code resolves to its space: blank or junk rolls from the seed, a number wraps', () => {
  const rand = rng(5);
  const rolled = resolve('', 16, rand);
  expect(rolled >= 1 && rolled < 16 && Number.isInteger(rolled)).toBe(true);
  expect(new Set(Array.from({ length: 64 }, (_, seed) => resolve('', 16, rng(seed)))).has(0)).toBe(false);
  expect(resolve('abc', 16, rng(5))).toBe(rolled);
  expect([resolve('7', 16, rand), resolve(' 23 ', 16, rand), resolve('-1', 16, rand), resolve('3.7', 16, rand)]).toEqual([7, 7, 15, 3]);
  expect(Number(roll(math, 3, 3, () => 0.5))).toBe(2 ** 26);
  expect([roll(math, 2, 2, () => 0), roll(math, 2, 2, () => 0.999)]).toEqual(['1', '15']);
});

test('every named design grows back from its derived code in both bases', () => {
  for (const dim of [2, 3]) {
    for (const base of [2, 3]) {
      const list = named(math, dim, base);
      expect(list.length).toBe(dim === 2 ? 10 : 12);
      for (const { code, name } of list) {
        const word = name[0].toUpperCase() + name.slice(1);
        const seed = dim === 2 ? math.two.named(word, base, 1, 0) : math.three.named(word, base, 1);
        const grown = dim === 2 ? math.two.create(code, base, 1, 0, base) : math.three.create(code, base, 1, base);
        expect([dim, base, name, Array.from(grown.types)]).toEqual([dim, base, name, Array.from(seed.types)]);
      }
    }
  }
  expect(word(named(math, 2, 2), 7)).toBe('carpet');
  expect(word(named(math, 3, 2), '23')).toBe('carpet');
  expect(word(named(math, 2, 2), 11)).toBe('');
});

test('the facts of a code name its class, its orbit and its form, and the counts read the grown cell', () => {
  expect(facts(math, 2, 2, '11')).toEqual({ rep: '7', orbit: 4, anf: expect.any(String), degree: 2 });
  expect(facts(math, 2, 3, '495')).toMatchObject({ rep: '255', orbit: 9, anf: '', degree: -1 });
  expect(title(math, 2, 2, '7')).toBe('bang dim 2, code 7');
  expect(census(math, 3, grow(math, 3, 2, '23', 2))).toMatchObject({ fills: 400, voids: 329, surface: 1056 });
  expect(census(math, 2, grow(math, 2, 2, '7', 3))).toMatchObject({ fills: 512, voids: 217, perimeter: 496 });
});

test('the gallery filters to all, classes or named, and stepping wraps around the list', () => {
  expect(gallery(math, 2, 2, 'all').length).toBe(16);
  expect(gallery(math, 2, 2, 'classes')).toEqual(['0', '1', '3', '6', '7', '15']);
  expect(gallery(math, 3, 3, 'named').length).toBe(12);
  expect(gallery(math, 3, 3, 'all')).toBe(null);
  const list = ['0', '1', '3'];
  expect([next(list, '1', 1), next(list, '3', 1), next(list, '0', -1), next(list, '9', 1), next(null, '9', 1)]).toEqual(['3', '0', '3', '0', '9']);
});
