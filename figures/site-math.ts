import * as math from "mrlyjs/math";
import { Grid, frame } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const CODE = 7;
const BITS = 8;
const SIDE = 27;
const WIDE = 594;
const TOP = 140;
const THIN = 3;

export default function draw(pen: Pen, ink: Ink) {
  const left = (pen.width - WIDE) / 2;
  const pitch = WIDE / BITS;
  const row = new Grid(frame(left, TOP, WIDE, pitch), BITS, 1, 0.14);
  let lit = 0;
  for (let bit = 0; bit < BITS; bit++) {
    const [x, y, w, h] = row.cell(bit, 0);
    if ((CODE >> bit) & 1) {
      pen.rect(x, y, w, h, ink.fg);
      lit++;
    } else {
      pen.rect(x, y, w, h, ink.line);
      pen.rect(x + THIN, y + THIN, w - 2 * THIN, h - 2 * THIN, ink.ground);
    }
  }
  const corners = math.bang.code_to_corners(CODE, 2, 2).length;
  if (lit !== corners) throw new Error(`site-math: ${lit} lit bits, want ${corners}`);
  const carpet = math.two.create(CODE, 3, 3, 0, 2);
  if (carpet.shape[0] !== SIDE || carpet.shape[1] !== SIDE) throw new Error(`site-math: carpet shape ${carpet.shape}, want ${SIDE} square`);
  let filled = 0;
  for (const kind of carpet.types) filled += kind;
  if (filled !== 512) throw new Error(`site-math: carpet sums to ${filled}, want 512`);
  const under = pen.height - TOP - WIDE;
  new Grid(frame(left, under, WIDE, WIDE), SIDE, SIDE, 0).paint(pen, carpet, (kind) => (kind !== 0 ? ink.blue : null));
}
