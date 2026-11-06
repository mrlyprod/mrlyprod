import type { Color, Pen } from "@mrlyprod/draw";
import { frame, Grid } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const MARK = [
  "....111....",
  "....101....",
  "....111....",
  "...........",
  "101.....111",
  "010.....101",
  "101.....111",
  "...........",
  "111.101.111",
  "101.010.101",
  "111.101.111",
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
  const tone: Color = paint.white;
  for (const [row, line] of MARK.entries())
    for (const [col, bit] of [...line].entries()) if (bit === "1") cells.fill(pen, col, row, tone);
}
