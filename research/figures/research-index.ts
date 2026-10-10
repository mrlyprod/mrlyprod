import * as math from "mrlyjs/math";
import { Grid, frame, type Frame, type Ink, type Pen } from "mrlyjs/view";

export const units = { math };

const GAPS = [0.16, 0.1, 0.055, 0];

function panel(pen: Pen, ink: Ink, cell: Frame, level: number, gap: number) {
  const cells = math.two.create(495, 3, level, 0, 3);
  const side = cells.shape[1];
  if (side !== 3 ** level) throw new Error(`research-index: side ${side} at level ${level}, want ${3 ** level}`);
  new Grid(cell, side, side, gap).paint(pen, cells, (kind) => (kind !== 0 ? ink.yellow : null));
}

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const gutter = area.w * 0.036;
  const side = (area.w - gutter) / 2;
  GAPS.forEach((gap, slot) => {
    const x = area.x + (slot % 2) * (side + gutter);
    const y = area.y + Math.floor(slot / 2) * (side + gutter);
    panel(pen, ink, frame(x, y, side, side), slot + 1, gap);
  });
}
