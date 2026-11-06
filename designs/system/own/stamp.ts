import type { Color, Pen } from "@mrlyprod/draw";
import { bits } from "@mrlyprod/ui/kit/glyph.ts";

export const BOX = 7;

export function stamp(pen: Pen, rows: readonly string[], ink: Color) {
  bits(pen, rows, { "#": ink });
}

export const flip = (rows: readonly string[]) => rows.map((row) => [...row].reverse().join(""));

export const drop = (rows: readonly string[]) => [...rows].reverse();

export const turn = (rows: readonly string[]) => rows.map((_, x) => rows.map((row) => row[x]).join(""));

export const over = (...layers: (readonly string[])[]) =>
  layers[0]!.map((_, y) =>
    [...layers[0]![y]!].map((_, x) => (layers.some((layer) => layer[y]![x] !== ".") ? "#" : ".")).join(""),
  );
