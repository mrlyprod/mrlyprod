import { expect, test } from 'bun:test';
import { decade, diffs, digits, label, log10, ratios, scale, sums, ticks } from './chart.js';

test('log10 reads a big integer string past the double range and scale goes log over two decades', () => {
  expect(log10('1000')).toBeCloseTo(3, 12);
  expect(log10('1' + '0'.repeat(40))).toBeCloseTo(40, 9);
  expect(log10('0')).toBe(-Infinity);
  expect(log10('-8')).toBeCloseTo(Math.log10(8), 12);
  const wide = scale([{ terms: ['8', '64', '512', '4096'] }]);
  expect([wide.log, wide.floor, wide.top]).toEqual([true, 0, 4]);
  expect(wide.at('4096')).toBeCloseTo(log10('4096') / 4, 12);
  const flat = scale([{ terms: ['8', '21', '40', '65'] }]);
  expect([flat.log, flat.at('65'), flat.at('0')]).toEqual([false, 1, 0]);
});

test('sums, ratios and differences run in exact integers', () => {
  expect(sums(['8', '64', '512'])).toEqual(['8', '72', '584']);
  expect(ratios(['8', '64', '512', '0', '5'])).toEqual([8, 8, 0, null]);
  expect(diffs(['8', '21', '40', '65'], 6)).toEqual([['8', '21', '40', '65'], ['13', '19', '25'], ['6', '6'], ['0']]);
  expect(diffs(['1', '2', '4'], 1)).toEqual([['1', '2', '4'], ['1', '2']]);
});

test('digits spell a term in a base and the labels stay short', () => {
  expect(digits('10', 2)).toEqual([1, 0, 1, 0]);
  expect(digits('26', 3)).toEqual([2, 2, 2]);
  expect(digits('-7', 10)).toEqual([7]);
  expect(digits('x', 10)).toEqual([]);
  expect(label('1234567')).toBe('1234567');
  expect(label('12345678')).toBe('1.23e7');
  expect(label('-4680')).toBe('-4680');
  expect([decade(0), decade(3), decade(6)]).toEqual(['1', '1000', '1e6']);
  expect(ticks(24, 3, 2)).toEqual([[0, '3'], [2, '7'], [4, '11'], [6, '15'], [8, '19'], [10, '23'], [12, '27'], [14, '31'], [16, '35'], [18, '39'], [20, '43'], [22, '47']]);
});
