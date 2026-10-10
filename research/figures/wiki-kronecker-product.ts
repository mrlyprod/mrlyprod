import * as math from "mrlyjs/math";
import { Grid, frame, type Ink, type Pen } from "mrlyjs/view";

export const units = { math };

const CODE = 7;
const LEVELS = [1, 2, 3];
const GUTTER = 60;
const FILLS = [8, 64, 512];

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.08);
  const tile = (area.w - 2 * GUTTER) / 3;
  const top = area.y + (area.h - tile) / 2;
  LEVELS.forEach((level, index) => {
    const design = math.two.create(CODE, 3, level, 0, 2);
    const side = design.shape[1];
    if (side !== 3 ** level) throw new Error(`wiki-kronecker-product: side ${side} at level ${level}, want ${3 ** level}`);
    if (design.shape[0] !== side) throw new Error(`wiki-kronecker-product: ${design.shape[0]} rows at level ${level}, want ${side}`);
    let fills = 0;
    for (const kind of design.types) fills += kind;
    if (fills !== FILLS[index]) throw new Error(`wiki-kronecker-product: ${fills} cells at level ${level}, want ${FILLS[index]}`);
    const x = area.x + index * (tile + GUTTER);
    const tone = index === 0 ? ink.yellow : ink.blue;
    new Grid(frame(x, top, tile, tile), side, side, 0).paint(pen, design, (kind) => (kind !== 0 ? tone : null));
  });
}
