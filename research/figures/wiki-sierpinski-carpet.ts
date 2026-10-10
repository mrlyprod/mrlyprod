import * as math from "mrlyjs/math";
import { Grid, type Ink, type Pen } from "mrlyjs/view";

const CODE = "495";
const LEVELS = 4;

export const units = { math };
export const still = (LEVELS - 1) / LEVELS;
export const loop = 8;
export const frames = LEVELS;

export default function draw(pen: Pen, ink: Ink, t: number) {
  const level = 1 + Math.min(Math.max(Math.floor(t * LEVELS), 0), LEVELS - 1);
  const cells = math.two.create(CODE, 3, level, 0, 3);
  const side = cells.shape[1];
  let filled = 0;
  for (const kind of cells.types) if (kind !== 0) filled++;
  if (side !== 3 ** level) throw new Error(`wiki-sierpinski-carpet: side ${side} at level ${level}, want ${3 ** level}`);
  if (filled !== 8 ** level) throw new Error(`wiki-sierpinski-carpet: ${filled} cells at level ${level}, want ${8 ** level}`);
  new Grid(pen.frame(0.08), side, side, 0).paint(pen, cells, (kind) => (kind !== 0 ? ink.blue : null));
}
