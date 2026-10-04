import * as math from "mrlyjs/math";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const TILE = 27;
const CELL = 4;
const GUTTER = 17;
const COLS = 7;
const ROWS = 4;
const DESIGNS = [
  15, 30, 57, 85, 102, 108, 325, 31, 79, 103, 115, 122, 173, 341, 119, 111, 125, 187, 231, 245, 189, 127, 191, 239, 254, 351, 367, 381,
];

export default function draw(pen: Pen, ink: Ink) {
  const side = TILE * CELL;
  const width = COLS * side + (COLS - 1) * GUTTER;
  const height = ROWS * side + (ROWS - 1) * GUTTER;
  const left = Math.round((pen.width - width) / 2);
  const top = Math.round((pen.height - height) / 2);
  let painted = 0;
  DESIGNS.forEach((code, place) => {
    const design = math.two.create(code, 3, 3, 0, 3);
    if (design.shape[0] !== TILE || design.shape[1] !== TILE) throw new Error(`site-demos: design ${code} is ${design.shape}, want ${TILE} by ${TILE}`);
    const tone = ink.inks[place % 3];
    const x = left + (place % COLS) * (side + GUTTER);
    const y = top + Math.floor(place / COLS) * (side + GUTTER);
    let filled = 0;
    for (let row = 0; row < TILE; row++) {
      for (let col = 0; col < TILE; col++) {
        if (design.types[row * TILE + col] === 0) continue;
        filled++;
        pen.rect(x + col * CELL, y + row * CELL, CELL, CELL, tone);
      }
    }
    const want = Number(math.counts.fill(code, 3, 2, 3, 3));
    if (filled !== want) throw new Error(`site-demos: design ${code} filled ${filled}, want ${want}`);
    painted += filled;
  });
  if (painted !== 5236) throw new Error(`site-demos: painted ${painted}, want 5236`);
}
