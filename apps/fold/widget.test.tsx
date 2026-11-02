import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts count moves against par, say done or stuck at the end, and count the holes", () => {
  const told = { moves: 3, par: 7, over: null, holes: 4, of: 12 };
  expect(rows(told)).toEqual([
    ["moves", 3],
    ["par", 7],
    ["holes", "4 of 12"],
  ]);
  expect([rows({ ...told, over: "done" })[0][0], rows({ ...told, over: "stuck" })[0][0], rows(null)]).toEqual([
    "done in",
    "stuck after",
    [],
  ]);
});
