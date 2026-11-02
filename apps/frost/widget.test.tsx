import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts count the flowers left, the stones and the turn, and say saved at the end", () => {
  const told = { turn: 6, stones: 2, alive: 4, lost: 1, of: 5, over: null };
  expect(rows(told)).toEqual([
    ["flowers", "4 of 5"],
    ["stones", 2],
    ["turn", 6],
  ]);
  expect([rows({ ...told, over: "done" })[0][0], rows(null)]).toEqual(["saved", []]);
});
