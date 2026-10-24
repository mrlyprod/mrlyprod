import { expect, test } from "bun:test";
import { turn } from "./turn.js";

test("a turn of half a circle takes blue to orange and twice is the colour again, grey staying grey", () => {
  expect(turn("#008cff", 180)).toBe("#ff7300");
  expect(turn(turn("#008cff", 180), 180)).toBe("#008cff");
  expect(turn("rgb(0, 140, 255)", 180)).toBe("#ff7300");
  expect(turn("#808080", 90)).toBe("#808080");
});
