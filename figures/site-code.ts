import { frame, grid, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const CELL = 51;

const CODE = [
  "000100000101000",
  "001100001101100",
  "011000001000110",
  "110000011000011",
  "100000010000001",
  "110000110000011",
  "011000100000110",
  "001101100001100",
  "000101000001000",
];

export default function draw(pen: Pen, ink: Ink) {
  const cols = CODE[0].length;
  const rows = CODE.length;
  const w = CELL * cols;
  const h = CELL * rows;
  const x = Math.round((pen.width - w) / 2);
  const y = Math.round((pen.height - h) / 2);
  const cells = new Grid(frame(x, y, w, h), cols, rows, 0);
  cells.carpet(pen, grid.mask(CODE, 1), ink.fg);
  for (let row = 0; row < rows; row++) {
    const col = 9 - Math.floor(row / 2);
    cells.fill(pen, col, row, ink.blue);
    if (row % 2 === 1) cells.fill(pen, col - 1, row, ink.blue);
  }
}
