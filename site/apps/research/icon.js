import * as math from "mrlyjs/math";
import { Grid, frame } from "mrlyjs/view";

export const units = { math };

const CODE = 495;
const BASE = 3;
const GAPS = [0.16, 0.1, 0.055, 0];

function panel(pen, ink, cell, level, gap) {
  const cells = math.two.create(CODE, BASE, level, 0, BASE);
  const side = cells.shape[1];
  if (side !== BASE ** level) throw new Error(`research: side ${side} at level ${level}, want ${BASE ** level}`);
  new Grid(cell, side, side, gap).paint(pen, cells, (kind) => (kind !== 0 ? ink.yellow : null));
}

export default function draw(pen, ink) {
  const area = pen.frame(0.08);
  const gutter = area.w * 0.036;
  const side = (area.w - gutter) / 2;
  GAPS.forEach((gap, slot) => {
    const x = area.x + (slot % 2) * (side + gutter);
    const y = area.y + Math.floor(slot / 2) * (side + gutter);
    panel(pen, ink, frame(x, y, side, side), slot + 1, gap);
  });
}
