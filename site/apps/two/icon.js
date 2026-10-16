import * as math from "@mrlyprod/mrly/math";
import { Grid } from "#view";
import { plate } from "../../lib/icon.js";

export const units = { math };

const CODE = 7;
const BASE = 2;
const NUMBER = 3;
const LEVEL = 2;
const COPIES = 3;
const HUE = "green";
const INSET = 0.17;
const SEAM = 0.01;
const WANT = { side: 27, fills: 576 };

let sheet = null;

function joins(side, rows, cols) {
  const out = [];
  for (let col = side; col < cols; col += side) out.push([0, col, 0, rows]);
  for (let row = side; row < rows; row += side) out.push([1, row, 0, cols]);
  return out;
}

function facts() {
  if (sheet) return sheet;
  const tile = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  const block = math.two.merge(Array.from({ length: COPIES * COPIES }, () => tile), COPIES, COPIES);
  const fills = math.two.fills(block);
  const [rows, cols] = block.shape;
  if (cols !== WANT.side || fills !== WANT.fills) throw new Error(`two: a sheet of ${cols} fills ${fills}, want ${WANT.side} and ${WANT.fills}`);
  sheet = { shape: block.shape, types: block.types, lines: joins(tile.shape[0], rows, cols) };
  return sheet;
}

export default function draw(pen) {
  const { box, paint, size } = plate(pen, HUE, INSET);
  const { shape, types, lines } = facts();
  const [rows, cols] = shape;
  new Grid(box, cols, rows, 0).paint(pen, { shape, types }, (type) => (type ? paint.white : null));
  const w = box.w / cols;
  const h = box.h / rows;
  for (const [axis, at, start, len] of lines) {
    const a = axis ? [box.x + start * w, box.y + at * h] : [box.x + at * w, box.y + start * h];
    const b = axis ? [box.x + (start + len) * w, box.y + at * h] : [box.x + at * w, box.y + (start + len) * h];
    pen.segment(a, b, size * SEAM, paint.light);
  }
}
