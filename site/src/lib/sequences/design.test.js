import { expect, test } from "bun:test";
import * as math from "@mrlyprod/mrly/math";
import { pen } from "../../../test/pen.js";
import { draw, inks, picture, said, view } from "./design.js";
import { tones } from "./paint.js";

const row = (dim, code, measure, axis, base = 2) => ({
  key: `${dim}-${base}-${code}-${measure}-${axis}`,
  dim,
  base,
  code,
  measure,
  axis,
});

const TONES = tones({ look: () => ({ paper: "#ffffff", accent: "#008cff" }), dpr: 1 });

test("the picture draws the design at the term it is asked for: a plane grid at its side, a cube iso, cut or face on", () => {
  const carpet = picture(math, row(2, "7", "fills", "side"), 11);
  expect([carpet.kind, carpet.number, carpet.level, carpet.shape]).toEqual(["grid", 25, 1, [25, 25]]);
  expect(said(carpet)).toBe("side 25, level 1");
  expect([
    view(row(3, "23", "fills", "level")),
    view(row(3, "23", "triangles", "level")),
    view(row(3, "23", "profills", "level")),
  ]).toEqual(["iso", "cut", "pro"]);
  const sponge = picture(math, row(3, "23", "surface", "level"), 1);
  expect([sponge.kind, sponge.level, sponge.cell.shape.length]).toEqual(["iso", 2, 2]);
  expect(said(picture(math, row(3, "23", "triangles", "level"), 0))).toBe("side 3, level 1, the middle cut");
});

test("a term past the picture budget draws the largest level that fits and says which", () => {
  const deep = picture(math, row(3, "23", "fills", "level"), 9);
  expect([deep.at, deep.level]).toEqual([3, 4]);
  const cut = picture(math, row(3, "23", "triangles", "level"), 9);
  expect(cut.level).toBe(3);
  expect(picture(math, row(2, "7", "fills", "level"), 20).level).toBe(5);
});

test("the grid paints its filled cells as one path over the void, the hexes one path a tone, and voids swap the emphasis", () => {
  const grid = [];
  draw(
    pen(grid),
    { x: 0, y: 0, w: 300, h: 300 },
    picture(math, row(2, "7", "fills", "level"), 1),
    inks(row(2, "7", "fills", "level"), TONES),
  );
  expect(grid.filter(([key]) => key === "rect")).toHaveLength(64);
  expect(grid.filter(([key]) => key === "fill")).toHaveLength(1);
  const iso = [];
  draw(
    pen(iso),
    { x: 0, y: 0, w: 300, h: 300 },
    picture(math, row(3, "23", "fills", "level"), 0),
    inks(row(3, "23", "fills", "level"), TONES),
  );
  expect(iso.filter(([key]) => key === "fill")).toHaveLength(3);
  const plain = inks(row(2, "7", "fills", "level"), TONES);
  const voids = inks(row(2, "7", "voids", "level"), TONES);
  expect(plain.fill).toBe("#008cff");
  expect(voids.fill).not.toBe("#008cff");
});
