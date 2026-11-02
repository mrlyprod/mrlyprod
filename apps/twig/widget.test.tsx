import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts count the forks in order, the spins and the par, and say bloomed at the end", () => {
  const told = { taps: 2, par: 7, neat: 9, of: 15, over: null };
  expect(rows(told)).toEqual([
    ["in order", "9 of 15"],
    ["spins", 2],
    ["par", 7],
  ]);
  expect([rows({ ...told, over: "done" })[0][0], rows(null)]).toEqual(["bloomed", []]);
});
