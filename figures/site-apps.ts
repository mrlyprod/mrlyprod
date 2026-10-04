import * as math from "mrlyjs/math";
import { Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const SIDE = 81;

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const sheet = math.two.create(7, 3, 4, 0, 2);
  if (sheet.shape[0] !== SIDE || sheet.shape[1] !== SIDE) throw new Error(`site-apps: sheet shape ${sheet.shape}, want ${SIDE} square`);
  let filled = 0;
  for (const kind of sheet.types) filled += kind;
  if (filled !== 4096) throw new Error(`site-apps: sheet sums to ${filled}, want 4096`);
  new Grid(area, SIDE, SIDE, 0).paint(pen, sheet, (kind) => (kind !== 0 ? ink.dim : null));
}
