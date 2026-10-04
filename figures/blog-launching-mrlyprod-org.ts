import { Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const SIDE = 8;
const GUTTER = 3;
const EDGES = [28, 40, 60, 64];

function band(index: number) {
  const slot = EDGES.findIndex((edge) => index < edge);
  return slot < 0 ? 3 : slot;
}

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  pen.rect(frame.x, frame.y, frame.w, frame.h, ink.line);
  const grid = new Grid(frame, SIDE, SIDE, 0);
  const tones = [ink.blue, ink.yellow, ink.green, ink.ground];
  const counts = [0, 0, 0, 0];
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const slot = band(row * SIDE + col);
      const [x, y, w, h] = grid.cell(col, row);
      pen.rect(x + GUTTER / 2, y + GUTTER / 2, w - GUTTER, h - GUTTER, tones[slot]);
      counts[slot]++;
    }
  }
  if (counts.join() !== "28,12,20,4") throw new Error(`blog-launching-mrlyprod-org: bands ${counts}, want 28,12,20,4`);
}
