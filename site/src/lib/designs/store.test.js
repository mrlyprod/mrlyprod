import { expect, test } from 'bun:test';
import { allows, choose, current, fits, keys, place, read, recent, saved, star, starred } from './store.js';

const memory = () => {
  const box = new Map();
  return { getItem: (key) => (box.has(key) ? box.get(key) : null), setItem: (key, value) => box.set(key, String(value)), removeItem: (key) => box.delete(key) };
};

test('a slot names the url keys of a design or fixes them, and reads, fits and places a design through them', () => {
  const arcs = { dim: 2, base: 'base', code: 'code', bases: [2, 3, 4, 5] };
  const designs = { dim: 'dim', base: 'base', code: 'code' };
  expect(keys(arcs)).toEqual(['base', 'code']);
  expect(read(arcs, { base: 3, code: ' 42 ' })).toEqual({ dim: 2, base: 3, code: '42' });
  expect([fits(arcs, { dim: 2, base: 5, code: '7' }), fits(arcs, { dim: 3, base: 2, code: '7' }), fits(designs, { dim: 3, base: 3, code: '' }), fits(designs, null)]).toEqual([true, false, false, false]);
  expect([place(arcs, { dim: 2, base: 3, code: 7 }), place(designs, { dim: 3, base: 2, code: '23' })]).toEqual([{ base: 3, code: '7' }, { dim: 3, base: 2, code: '23' }]);
});

test('a slot with spaces allows only those dim and base pairs', () => {
  const plot = { dim: 'dim', base: 'base', code: 'code', spaces: [[2, 2], [2, 3], [3, 2]] };
  expect([allows(plot, 2, 3), allows(plot, 3, 2), allows(plot, 3, 3), fits(plot, { dim: 3, base: 3, code: '5' })]).toEqual([true, true, false, false]);
});

test('choosing a design makes it current and the head of recent, once; a star saves it and a second star lets it go', () => {
  globalThis.localStorage = memory();
  try {
    choose({ dim: 2, base: 2, code: '7' });
    choose({ dim: 3, base: 2, code: '23' });
    choose({ dim: 2, base: 2, code: '7' });
    choose({ dim: 2, base: 2, code: '' });
    star({ dim: 3, base: 2, code: '23' });
    const kept = [current(), recent().map((one) => one.code), starred({ dim: 3, base: 2, code: '23' })];
    star({ dim: 3, base: 2, code: '23' });
    expect([...kept, saved()]).toEqual([{ dim: 2, base: 2, code: '7' }, ['7', '23'], true, []]);
  } finally {
    delete globalThis.localStorage;
  }
});
