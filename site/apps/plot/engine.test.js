import { expect, test } from 'bun:test';
import * as num from '../../../pkgs/mrlyjs/num.js';
import { held, mix, polynomial, recurrence, room, rule } from './engine.js';

num.initSync({ module: await Bun.file(new URL('../../../pkgs/mrlyjs/pkg/num/mrlyjs_num_bg.wasm', import.meta.url)).arrayBuffer() });

const PERIMETER = ['16', '80', '496', '3536', '26992', '212048', '1684720', '13442768'];

test('a rule and its polynomial spell themselves the way the page prints them', () => {
  expect(recurrence([['11', '1'], ['-24', '1']])).toBe('a(n) = 11 a(n-1) - 24 a(n-2)');
  expect(recurrence([['1', '1'], ['1', '1']])).toBe('a(n) = a(n-1) + a(n-2)');
  expect(recurrence([['0', '1'], ['-1', '2']])).toBe('a(n) = - 1/2 a(n-2)');
  expect(recurrence([])).toBe('a(n) = 0');
  expect(polynomial([['1', '1'], ['-11', '1'], ['24', '1']])).toBe('x^2 - 11x + 24');
  expect(polynomial([['1', '1'], ['-20', '1']])).toBe('x - 20');
});

test('the carpet perimeter reads the order-two rule with roots 8 and 3, and the growth is its largest root', () => {
  const read = rule(num, PERIMETER);
  expect([read.order, read.recurrence, read.polynomial, read.growth, read.read]).toEqual([2, 'a(n) = 11 a(n-1) - 24 a(n-2)', 'x^2 - 11x + 24', 8, 8]);
  expect(read.exponent).toBeCloseTo(Math.log10(8), 12);
  const none = rule(num, ['1', '2', '4', '7', '12', '19', '30']);
  expect([none.order, Number.isNaN(none.growth), none.from]).toEqual([null, true, 'no rule fits, so no growth is read']);
});

test('the rule reads only the terms that fit 128 bits, and a mix stops where 128 bits do', () => {
  const wide = Array.from({ length: 32 }, (_, i) => String(20n ** BigInt(i + 1)));
  expect(held(wide)).toHaveLength(28);
  expect(rule(num, wide)).toMatchObject({ order: 1, growth: 20, read: 28 });
  expect(room('hadamard', wide, wide, 0)).toBe(13);
  const made = mix(num, wide, wide, 'hadamard', 0);
  expect([made.terms.length, made.cut, made.terms[12]]).toEqual([13, true, String((20n ** 13n) ** 2n)]);
});

test('the mix runs every operation on the pair or on the first alone', () => {
  const a = ['1', '2', '3', '4', '5'];
  const b = ['1', '1', '1', '1', '1'];
  const run = (op, arg = 2) => mix(num, a, b, op, arg).terms;
  expect(run('add')).toEqual(['2', '3', '4', '5', '6']);
  expect(run('sub')).toEqual(['0', '1', '2', '3', '4']);
  expect(run('hadamard')).toEqual(a);
  expect(run('cauchy')).toEqual(['1', '3', '6', '10', '15']);
  expect(run('shift')).toEqual(['3', '4', '5']);
  expect(run('decimate')).toEqual(['1', '3', '5']);
  expect(run('delta')).toEqual(['1', '1', '1', '1']);
  expect(run('sigma')).toEqual(['1', '3', '6', '10', '15']);
  expect(run('scale', -3)).toEqual(['-3', '-6', '-9', '-12', '-15']);
  expect(mix(num, a, b, 'nope', 0).error).toBe('no operation nope');
});
