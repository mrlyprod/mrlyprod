import { frame, grid, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const CELL = 70;

const HEART = [
  "01100000110",
  "11110001111",
  "11111111111",
  "11111111111",
  "11111111111",
  "01111111110",
  "00111111100",
  "00011111000",
  "00001110000",
  "00000100000",
];

export default function draw(pen: Pen, ink: Ink) {
  const cols = HEART[0].length;
  const rows = HEART.length;
  const w = CELL * cols;
  const h = CELL * rows;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - h) / 2), w, h), cols, rows, 0);
  cells.carpet(pen, grid.mask(HEART, 1), ink.fg);
  for (const [col, row] of [[2, 2], [3, 2], [2, 3]]) cells.fill(pen, col, row, ink.red);
}
