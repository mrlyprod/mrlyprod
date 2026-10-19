import { expect, test } from 'bun:test';
import { placed, slot } from './drag.js';

const box = (left, top, width = 10, height = 10) => ({ left, top, width, height, right: left + width, bottom: top + height });

test('a draft moves its id to the index among the others, null takes it out, and no draft keeps the list', () => {
  const list = ['a', 'b', 'c'];
  expect(placed(list, null)).toBe(list);
  expect(placed(list, { id: 'a', index: 1 })).toEqual(['b', 'a', 'c']);
  expect(placed(list, { id: 'c', index: 0 })).toEqual(['c', 'a', 'b']);
  expect(placed(list, { id: 'b', index: null })).toEqual(['a', 'c']);
  expect(placed(list, { id: 'z', index: 9 })).toEqual(['a', 'b', 'c', 'z']);
});

test('the slot is the first item the pointer reads before: rows by reading order, a row by x, a column by y', () => {
  const grid = [box(0, 0), box(20, 0), box(0, 20), box(20, 20)];
  expect(slot(grid, 2, 5)).toBe(0);
  expect(slot(grid, 8, 5)).toBe(1);
  expect(slot(grid, 40, 5)).toBe(2);
  expect(slot(grid, 0, 15)).toBe(2);
  expect(slot(grid, 26, 25)).toBe(4);
  expect(slot([], 0, 0)).toBe(0);
  const row = [box(0, 10), box(20, 10)];
  expect(slot(row, 12, 0, 'x')).toBe(1);
  const column = [box(0, 0, 100), box(0, 20, 100)];
  expect(slot(column, 90, 12, 'y')).toBe(1);
});
