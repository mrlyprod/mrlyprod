import type { Pen } from "@mrlyprod/draw";
import { frame, Grid } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const MARK = [
  "11111011111",
  "10000000001",
  "10111011101",
  "10100000101",
  "10101110101",
  "00001010000",
  "10101110101",
  "10100000101",
  "10111011101",
  "10000000001",
  "11111011111",
];

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "teal", 0.2);
  const side = MARK.length;
  const unit = Math.floor(Math.min(box.w, box.h) / side);
  const [cx, cy] = box.center();
  const cells = new Grid(
    frame(Math.round(cx - (unit * side) / 2), Math.round(cy - (unit * side) / 2), unit * side, unit * side),
    side,
    side,
    0,
  );
  for (const [row, line] of MARK.entries())
    for (const [col, bit] of [...line].entries()) if (bit === "1") cells.fill(pen, col, row, paint.white);
}
