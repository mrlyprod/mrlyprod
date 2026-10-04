import { frame, grid, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const CELL = 55;

const CART = [
  "11000000000000",
  "00100000000000",
  "00010000000000",
  "00001111111111",
  "00001000000001",
  "00001000000001",
  "00000100000010",
  "00000011111100",
  "00000010000100",
  "00000111001110",
  "00000010000100",
];

export default function draw(pen: Pen, ink: Ink) {
  const cols = CART[0].length;
  const rows = CART.length;
  const w = CELL * cols;
  const h = CELL * rows;
  const x = Math.round((pen.width - w) / 2);
  const y = Math.round((pen.height - h) / 2);
  const cells = new Grid(frame(x, y, w, h), cols, rows, 0);
  cells.carpet(pen, grid.mask(CART, 1), ink.fg);
  for (const col of [5, 6, 7, 10, 11, 12]) cells.fill(pen, col, 9, ink.orange);
  for (const row of [8, 10]) {
    cells.fill(pen, 6, row, ink.orange);
    cells.fill(pen, 11, row, ink.orange);
  }
}
