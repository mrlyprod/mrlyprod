import type { Pen } from "@mrlyprod/draw";
import { bits, rgb } from "@mrlyprod/ui/kit/glyph.ts";

export const BOX = 9;

const PAINT = {
  k: rgb("#000000"),
  w: rgb("#ffffff"),
  r: rgb("#ff3d40"),
  o: rgb("#ff8f2c"),
  y: rgb("#ffd100"),
  g: rgb("#32cc58"),
  c: rgb("#1ec9f3"),
  b: rgb("#008cff"),
  i: rgb("#6768fa"),
  u: rgb("#d332e9"),
  p: rgb("#ff325a"),
  n: rgb("#b18462"),
};

export function paint(pen: Pen, ...layers: (readonly string[])[]) {
  for (const rows of layers) bits(pen, rows, PAINT);
}

export const FACE = [
  ".yyyyyyy.",
  "yyyyyyyyy",
  "yyyyyyyyy",
  "yyyyyyyyy",
  "yyyyyyyyy",
  "yyyyyyyyy",
  "yyyyyyyyy",
  "yyyyyyyyy",
  ".yyyyyyy.",
];
