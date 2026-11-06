import type { Color, Pen } from "@mrlyprod/draw";
import { frame, Grid } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";

export const units = {};

const ART = [
  "1111111110",
  "1000000012",
  "1033333012",
  "1030303012",
  "1033333012",
  "1030303012",
  "1033333012",
  "1000000012",
  "1000000012",
  "1111111112",
  "0222222222",
];

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "teal", 0.2);
  const cols = ART[0].length;
  const rows = ART.length;
  const unit = Math.floor(Math.min(box.w / cols, box.h / rows));
  const [cx, cy] = box.center();
  const cells = new Grid(
    frame(Math.round(cx - (unit * cols) / 2), Math.round(cy - (unit * rows) / 2), unit * cols, unit * rows),
    cols,
    rows,
    0,
  );
  const tones: Record<string, Color> = { 1: paint.white, 2: paint.deep, 3: paint.deep };
  for (const [row, line] of ART.entries())
    for (const [col, bit] of [...line].entries()) if (tones[bit]) cells.fill(pen, col, row, tones[bit]);
}
