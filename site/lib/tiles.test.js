import { expect, test } from 'bun:test';
import { DESIGNS, kron, seed, tile } from './tiles.js';

const lit = (t) => t.cells.reduce((a, b) => a + b, 0);

test('every design fills its known cells at n=3 and squares them one level up', () => {
  const counts = { carpet: 8, net: 5, htree: 6, vtree: 6, void: 5 };
  for (const design of DESIGNS) {
    const one = tile(design, 3, 1);
    const two = tile(design, 3, 2);
    expect([design, one.size, lit(one)]).toEqual([design, 3, counts[design]]);
    expect([design, two.size, lit(two)]).toEqual([design, 9, counts[design] ** 2]);
  }
  expect([...tile('carpet', 3, 1).cells]).toEqual([1, 1, 1, 1, 0, 1, 1, 1, 1]);
  expect(kron(seed('void', 5), 2).size).toBe(25);
});
