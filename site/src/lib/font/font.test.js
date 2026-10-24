import { expect, test } from "bun:test";
import * as crate from "@mrlyprod/mrly/font";
import FONT from "./font.gen.json" with { type: "json" };
import { animate, fold, letters, merge } from "./font.js";

const WORDMARK = "MRLYPROD";

test("the wordmark writes itself in stroke order, one cell a frame", () => {
  const write = animate(WORDMARK, 1);
  expect([write.rows, write.cols, write.fps]).toEqual([7, 49, 25]);
  expect(write.frames[0]).toEqual([]);
  expect(write.frames.length).toBe(104);
  for (let i = 1; i < write.frames.length; i++) expect(write.frames[i].length).toBe(write.frames[i - 1].length + 1);
  expect(write.frames[1]).toEqual([5 * 49 + 1]);
  const grid = letters(WORDMARK).grid;
  const lit = grid.flat().filter(Boolean).length;
  expect(write.frames[103].length).toBe(lit);
});

test("the eight letters fold into an X in twenty-two frames", () => {
  const folded = merge(WORDMARK, 1);
  expect(folded.length).toBe(22);
  expect(folded[0]).toEqual(animate(WORDMARK, 1).frames[103]);
  const x = [];
  for (const [r, row] of FONT.X.rows.entries())
    for (const [c, ch] of [...row].entries()) if (ch === "1") x.push((1 + r) * 49 + 22 + c);
  expect(folded[21]).toEqual(x);
});

test("a word folds from its own letters and lands on the X at its centre", () => {
  const { rows, cols, grid } = letters("wiki");
  const folded = fold("wiki");
  expect([folded.rows, folded.cols]).toEqual([rows, cols]);
  expect(folded.frames[0]).toEqual(grid.flat().flatMap((on, i) => (on ? [i] : [])));
  const x = [];
  const top = (rows - 5) >> 1;
  const left = (cols - 5) >> 1;
  for (const [r, row] of FONT.X.rows.entries())
    for (const [c, ch] of [...row].entries()) if (ch === "1") x.push((top + r) * cols + left + c);
  expect(folded.frames.at(-1)).toEqual(x);
});

test("any string writes itself and a lone glyph has nothing to merge", () => {
  for (const text of ["a", "hi", "mrly.net", "(1)"]) {
    const write = animate(text, 2);
    const { rows, cols, grid } = letters(text);
    expect([write.rows, write.cols]).toEqual([rows + 4, cols + 4]);
    expect(write.frames.length).toBe(grid.flat().filter(Boolean).length + 1);
  }
  expect(merge("A", 1).length).toBe(1);
});

test("the kit matches the crate frame for frame", () => {
  for (const text of ["MRLYPROD", "SIERPINSKI", "mrly.net", "(1)", "Hi 42", "A"]) {
    expect(animate(text, 1)).toEqual(JSON.parse(JSON.stringify(crate.animate(text, 1))));
  }
});

test("font.gen.json is the crate: every glyph, its rows and its path", () => {
  const all = crate.all();
  expect(Object.keys(FONT).sort()).toEqual(all.map((one) => one.char).sort());
  for (const one of all)
    expect([one.char, FONT[one.char]]).toEqual([one.char, { rows: one.rows, path: crate.path(one.char) }]);
});
