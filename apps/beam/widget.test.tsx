import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts count the stars lit, the taps and the par, and say lit at the end", () => {
  const told = { taps: 2, par: 4, lit: 3, of: 5, over: null };
  expect(rows(told)).toEqual([
    ["stars", "3 of 5"],
    ["taps", 2],
    ["par", 4],
  ]);
  expect([rows({ ...told, over: "done" })[0][0], rows(null)]).toEqual(["lit", []]);
});
