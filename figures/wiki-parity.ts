import * as math from "mrlyjs/math";
import { Grid, type Ink, type Pen } from "mrlyjs/view";

export const units = { math };

const SIDE = 6;
const CORNERS = 4;

export default function draw(pen: Pen, ink: Ink) {
  const inks = [ink.blue, ink.yellow, ink.orange, ink.green];
  const classes = Array.from({ length: CORNERS }, (_, corner) => math.two.create(1 << corner, SIDE, 1, 0, 2));
  for (const [corner, design] of classes.entries()) {
    if (design.shape[0] !== SIDE || design.shape[1] !== SIDE) throw new Error(`wiki-parity: class ${corner} is ${design.shape}, want ${SIDE} by ${SIDE}`);
  }
  const grid = new Grid(pen.frame(0.08), SIDE, SIDE, 0.05);
  const counts = [0, 0, 0, 0];
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const held = classes.flatMap((design, corner) => (design.types[row * SIDE + col] !== 0 ? [corner] : []));
      if (held.length !== 1) throw new Error(`wiki-parity: cell ${row},${col} is in ${held.length} classes, want 1`);
      grid.fill(pen, col, row, inks[held[0]]);
      counts[held[0]]++;
    }
  }
  if (counts.some((n) => n !== 9)) throw new Error(`wiki-parity: classes of ${counts}, want 9 each`);
}
