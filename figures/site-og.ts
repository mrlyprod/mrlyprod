import { Grid, frame, grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};
export const size = [1200, 630];

const CELL = 18;
const ACROSS = 10;
const DOWN = 5;

export default function draw(pen: Pen, ink: Ink) {
  const seed = grid.mask(grid.LOGO, 1);
  const cols = 6 * ACROSS - 1;
  const rows = 6 * DOWN - 1;
  const mask = Array.from({ length: rows }, (_, row) => Array.from({ length: cols }, (_, col) => row % 6 < 5 && col % 6 < 5 && seed[row % 6][col % 6]));
  const w = CELL * cols;
  const h = CELL * rows;
  const area = frame(Math.round((pen.width - w) / 2), Math.round((pen.height - h) / 2), w, h);
  new Grid(area, cols, rows, 0).carpet(pen, mask, ink.fg);
}
