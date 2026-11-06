import type { Color, Pen } from "@mrlyprod/draw";
import { bits, rgb } from "@mrlyprod/ui/kit/glyph.ts";

export const BOX = 16;

const VGA = {
  k: rgb("#000000"),
  w: rgb("#ffffff"),
  s: rgb("#c0c0c0"),
  n: rgb("#000080"),
  y: rgb("#ffff00"),
  r: rgb("#ff0000"),
  e: rgb("#008000"),
  o: rgb("#ff8000"),
  b: rgb("#0080ff"),
  m: rgb("#ff00ff"),
};

export function stamp(pen: Pen, rows: readonly string[], ink?: Color) {
  bits(pen, rows, { ...VGA, "#": ink });
}

export const flip = (rows: readonly string[]) => rows.map((row) => [...row].reverse().join(""));

export const drop = (rows: readonly string[]) => [...rows].reverse();

export const turn = (rows: readonly string[]) => rows.map((_, x) => rows.map((row) => row[x]).join(""));
