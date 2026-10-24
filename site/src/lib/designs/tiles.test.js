import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { DESIGNS, kron, LEVELS, NUMBERS, seed, tile } from "./tiles.js";

const lit = (t) => t.cells.reduce((a, b) => a + b, 0);

test("every design fills its known cells at n=3 and squares them one level up", () => {
  const counts = { carpet: 8, net: 5, htree: 6, vtree: 6, void: 5 };
  for (const design of DESIGNS) {
    const one = tile(design, 3, 1);
    const two = tile(design, 3, 2);
    expect([design, one.size, lit(one)]).toEqual([design, 3, counts[design]]);
    expect([design, two.size, lit(two)]).toEqual([design, 9, counts[design] ** 2]);
  }
  expect([...tile("carpet", 3, 1).cells]).toEqual([1, 1, 1, 1, 0, 1, 1, 1, 1]);
  expect(kron(seed("void", 5), 2).size).toBe(25);
});

test("every design is the bang door design of its name, cell for cell, at every number and level", () => {
  const codes = new Map(math.name.words(2, 2).map(([code, name]) => [name, Number(code)]));
  for (const design of DESIGNS)
    for (const n of NUMBERS)
      for (const level of LEVELS) {
        const door = math.bang.factory.create(codes.get(design), n, 2, 2, level);
        expect([design, n, level, [...tile(design, n, level).cells]]).toEqual([design, n, level, [...door.data]]);
      }
});
