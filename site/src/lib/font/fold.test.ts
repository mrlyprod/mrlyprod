import { expect, test } from "bun:test";
import { fold, markup } from "./fold.ts";

test("the wordmark folds pair by pair, outer letters first, and its eight letters stacked are the X", () => {
  const { cols, left, letters, x } = fold("MRLYPROD");
  expect([cols, left, x.length]).toEqual([47, 21, 21]);
  expect(letters.map(({ track }) => track)).toEqual([
    [6, 12, 18, 21],
    [0, 6, 12, 15],
    [0, 0, 6, 9],
    [0, 0, 0, 3],
    [0, 0, 0, -3],
    [0, 0, -6, -9],
    [0, -6, -12, -15],
    [-6, -12, -18, -21],
  ]);
  expect(markup(fold("MRLYPROD"))).toContain('<g class="x"><path d="M21 0h1v1h-1z');
  expect(() => fold("MRLY")).toThrow();
});
