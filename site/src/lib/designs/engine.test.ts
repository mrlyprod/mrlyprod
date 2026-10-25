import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { rng } from "../rand.ts";
import { census, classes, codes, drawn, facts, gallery, grow, next, roll, settle, title, total } from "./engine.ts";

test("a space lists its codes up to the limit and its classes where the unit can walk them", () => {
  expect([total(math, 2, 2), total(math, 3, 2), total(math, 2, 3), total(math, 3, 3)]).toEqual([
    16, 256, 512, 134217728,
  ]);
  expect([codes(math, 2, 2)!.length, codes(math, 2, 3)!.length, codes(math, 3, 3)]).toEqual([16, 512, null]);
  expect([
    classes(math, 2, 2)!.length,
    classes(math, 3, 2)!.length,
    classes(math, 2, 3)!.length,
    classes(math, 3, 3),
  ]).toEqual([6, 22, 26, null]);
  expect(classes(math, 2, 2)!.map((one) => one.code)).toEqual(["0", "1", "3", "6", "7", "15"]);
  expect(classes(math, 3, 2)).toBe(classes(math, 3, 2));
});

test("a code settles in its space: blank or junk rolls from the seed, a number wraps", () => {
  const rolls = Array.from({ length: 64 }, (_, seed) => Number(roll(math, 2, 2, rng(seed))));
  expect(rolls.every((one) => Number.isInteger(one) && one >= 1 && one < 16)).toBe(true);
  expect(roll(math, 3, 3, rng(5))).toBe(roll(math, 3, 3, rng(5)));
  expect([
    settle(math, 2, 2, "", rng(5)),
    settle(math, 2, 2, "abc", rng(5)),
    settle(math, 2, 2, " 23 ", rng(5)),
    settle(math, 2, 2, "-1", rng(5)),
    settle(math, 2, 2, "3.7", rng(5)),
  ]).toEqual([roll(math, 2, 2, rng(5)), roll(math, 2, 2, rng(5)), "7", "15", "3"]);
});

test("the facts of a code name its class, its orbit and its form, and the counts read the grown cell", () => {
  expect(facts(math, 2, 2, "11")).toEqual({ rep: "7", orbit: 4, anf: expect.any(String), degree: 2 });
  expect(facts(math, 2, 3, "495")).toMatchObject({ rep: "255", orbit: 9, anf: "", degree: -1 });
  expect(title(math, 2, 2, "7")).toBe("bang dim 2, code 7");
  expect(census(math, 3, grow(math, 3, 2, "23", 2))).toMatchObject({ fills: 400, voids: 329, surface: 1056 });
  expect(census(math, 2, grow(math, 2, 2, "7", 3))).toMatchObject({ fills: 512, voids: 217, perimeter: 496 });
});

test("the gallery filters to all, classes or named, and stepping wraps around the list", () => {
  expect(gallery(math, 2, 2, "all")!.length).toBe(16);
  expect(gallery(math, 2, 2, "classes")).toEqual(["0", "1", "3", "6", "7", "15"]);
  expect(gallery(math, 3, 3, "named")!.length).toBe(12);
  expect(gallery(math, 3, 3, "all")).toBe(null);
  const list = ["0", "1", "3"];
  expect([next(list, "1", 1), next(list, "3", 1), next(list, "0", -1), next(list, "9", 1), next(null, "9", 1)]).toEqual(
    ["3", "0", "3", "0", "9"],
  );
});

test("a reroll draws another class from the seed, the same one for the same seed, and rolls a code where no list is walked", () => {
  const list = gallery(math, 3, 2, "classes");
  const draws = Array.from({ length: 32 }, (_, seed) => drawn(math, 3, 2, list, "23", seed));
  expect([
    draws.every((one) => list!.includes(one) && one !== "23"),
    new Set(draws).size > 4,
    drawn(math, 3, 2, list, "23", 9),
  ]).toEqual([true, true, draws[9]]);
  expect(drawn(math, 3, 3, null, "1", 5)).toBe(roll(math, 3, 3, rng(5)));
});

test("a line design grows along one axis as a one-row strip, and its classes and facts read in its own space", () => {
  const strip = grow(math, 1, 2, "1", 2);
  expect([strip.shape, Array.from(strip.types), grow(math, 1, 3, "5", 1).shape]).toEqual([
    [1, 9],
    [1, 0, 1, 0, 0, 0, 1, 0, 1],
    [1, 3],
  ]);
  expect([classes(math, 1, 2)!.map((one) => one.code), facts(math, 1, 2, "2"), facts(math, 1, 3, "4")]).toEqual([
    ["0", "1", "3"],
    { rep: "1", orbit: 2, anf: "x", degree: 1 },
    { rep: "1", orbit: 3, anf: "", degree: -1 },
  ]);
});
