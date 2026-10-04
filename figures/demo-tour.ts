import * as math from "mrlyjs/math";
import { Grid, type Ink, type Pen, type Point } from "mrlyjs/view";

export const units = { math };

const CODE = 7;
const NUMBER = 3;
const BASE = 2;
const LEVEL = 4;
const SIDE = 81;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const carpet = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  if (carpet.shape.join() !== `${SIDE},${SIDE}` || math.two.fills(carpet) !== 4096) throw new Error(`demo-tour: carpet ${carpet.shape} with ${math.two.fills(carpet)} fills, want 81,81 with 4096`);
  if (math.two.perimeter(carpet) !== "3536") throw new Error(`demo-tour: perimeter ${math.two.perimeter(carpet)}, want 3536`);
  const types = carpet.types;
  const on = (row: number, col: number) => row >= 0 && row < SIDE && col >= 0 && col < SIDE && types[row * SIDE + col] !== 0;
  const grid = new Grid(frame, SIDE, SIDE, 0);
  const body = ink.mix(ink.line, ink.dim, 0.35);
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      if (on(row, col)) grid.fill(pen, col, row, body);
    }
  }
  const thick = frame.cell(SIDE) * 0.22;
  let edges = 0;
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      if (!on(row, col)) continue;
      const [x, y, w, h] = grid.cell(col, row);
      const sides: [boolean, Point, Point][] = [
        [!on(row - 1, col), [x, y], [x + w, y]],
        [!on(row + 1, col), [x, y + h], [x + w, y + h]],
        [!on(row, col - 1), [x, y], [x, y + h]],
        [!on(row, col + 1), [x + w, y], [x + w, y + h]],
      ];
      for (const [open, a, b] of sides) {
        if (open) {
          pen.segment(a, b, thick, ink.yellow);
          edges++;
        }
      }
    }
  }
  if (String(edges) !== math.two.perimeter(carpet)) throw new Error(`demo-tour: ${edges} edges, want ${math.two.perimeter(carpet)}`);
}
