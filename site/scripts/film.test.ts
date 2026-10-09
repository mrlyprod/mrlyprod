import { expect, test } from "bun:test";
import { roll, sheet, tile } from "./film/page.js";

test("the harness plays each cue at its own t before the frame that passes it, then draws at from + i x step", async () => {
  const seen: [string, number][] = [];
  const make = (canvas: unknown, view: { t: number }) => ({ draw: () => seen.push(["draw", view.t]), trigger: () => seen.push(["trigger", view.t]), exit: () => seen.push(["exit", view.t]) });
  await roll(make, {}, { seed: 7, cues: [[250, "exit"], [0, "trigger"]], from: 100, step: 100, frames: 3, look: {} }, (i: number) => seen.push(["tile", i]));
  expect(seen).toEqual([["trigger", 0], ["draw", 100], ["tile", 0], ["draw", 200], ["tile", 1], ["exit", 250], ["draw", 300], ["tile", 2]]);
});

test("a sheet is 6 columns of 320 by 180, left to right", () => {
  expect([sheet(60), tile(0), tile(5), tile(6), tile(59)]).toEqual([[1920, 1800], [0, 0], [1600, 0], [0, 180], [1600, 1620]]);
});
