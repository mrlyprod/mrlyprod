import { expect, test } from "bun:test";
import { rows } from "./widget.tsx";

test("the readouts show skill in tenths, asked, taught, invented and open, the day and the world in words", () => {
  const told = {
    skill: 87,
    asked: 579,
    taught: 19,
    invented: 44,
    open: 0,
    told: 7,
    understood: 46,
    puzzles: 54,
    deepest: 6,
    reach: 7,
    solved: 45,
    day: 90,
    length: 120,
    pupils: 16,
    answers: "oracle" as const,
    over: null,
  };
  expect(rows(told)).toEqual([
    ["skill", "8.7"],
    ["asked", "579"],
    ["taught", "19"],
    ["invented", "44"],
    ["open", "0"],
    ["day", "90 of 120"],
    [
      "world",
      "16 pupils, 54 puzzles: 46 understood, 7 only told; first worked out 45; deepest understood 6, deepest known 7",
    ],
  ]);
  expect([rows({ ...told, over: "done" })[5][0], rows(null)]).toEqual(["ran", []]);
});
