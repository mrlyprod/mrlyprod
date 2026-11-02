import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts count the rings home, the turns and the par, and say whole at the end", () => {
  const told = { taps: 2, par: 7, home: 3, of: 5, over: null };
  expect(rows(told)).toEqual([
    ["home", "3 of 5"],
    ["turns", 2],
    ["par", 7],
  ]);
  expect([rows({ ...told, over: "done" })[0][0], rows(null)]).toEqual(["whole", []]);
});
