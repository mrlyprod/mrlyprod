import { frame, grid, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const CELL = 55;

const LETTER = [
  "11111111111111",
  "10000000000001",
  "10000000000001",
  "10000000000001",
  "10000000000001",
  "10000000000001",
  "10000000000001",
  "10000000000001",
  "10000000000001",
  "11111111111111",
];

export default function draw(pen: Pen, ink: Ink) {
  const cols = LETTER[0].length;
  const rows = LETTER.length;
  const w = CELL * cols;
  const h = CELL * rows;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - h) / 2), w, h), cols, rows, 0);
  cells.carpet(pen, grid.mask(LETTER, 1), ink.fg);
  for (let step = 0; step < 6; step++) {
    cells.fill(pen, 1 + step, 1 + step, ink.blue);
    cells.fill(pen, 12 - step, 1 + step, ink.blue);
  }
}
