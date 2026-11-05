import type { Color, Pen } from "@mrlyprod/draw";
import { Grid } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";
import * as num from "@mrlyprod/num";

export const units = { num };

const SIDE = 8;
const GAP = 0.1;
const CORNER = 0.16;
const DIM: Color = [255, 255, 255, 82];
const WANT = { order: "1,-1,-1,1,-1,1,1,-1", squares: "1,-1,-1,1,-1,-1,1,-1", above: 20, below: 9 };

let cells: Int8Array | null = null;

function signs() {
  if (cells) return cells;
  const thue = num.readings.Design.thue();
  const order = num.readings.stream(thue, "N", SIDE * SIDE);
  const squares = num.readings.stream(thue, "Square", SIDE * SIDE);
  const got = [Array.from(order.slice(0, 8)).join(), Array.from(squares.slice(0, 8)).join()];
  if (got[0] !== WANT.order || got[1] !== WANT.squares)
    throw new Error(`readings: Thue-Morse reads ${got.join(" and ")}, want ${WANT.order} and ${WANT.squares}`);
  const out = new Int8Array(SIDE * SIDE);
  let above = 0;
  let below = 0;
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const n = SIDE * row + col;
      const sign = col >= row ? order[n] : squares[n];
      out[n] = sign;
      if (sign > 0) {
        if (col >= row) above++;
        else below++;
      }
    }
  }
  if (above !== WANT.above || below !== WANT.below)
    throw new Error(
      `readings: ${above} plus cells above the diagonal and ${below} below, want ${WANT.above} and ${WANT.below}`,
    );
  cells = out;
  return out;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "teal", 0.18);
  const grid = new Grid(box, SIDE, SIDE, GAP);
  const read = signs();
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const [x, y, w, h] = grid.cell(col, row);
      pen.round_rect(x, y, w, h, w * CORNER, read[row * SIDE + col] > 0 ? paint.white : DIM);
    }
  }
}
