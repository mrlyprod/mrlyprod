import { Grid, frame, grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const CELL = 70;

const SHEET = [
  "11111111000",
  "10000000100",
  "10000000010",
  "10000000001",
  "10000000001",
  "10000000001",
  "10000000001",
  "10000000001",
  "10000000001",
  "10000000001",
  "10000000001",
  "10000000001",
  "11111111111",
];

const RUNS = [
  [4, 9],
  [6, 9],
  [8, 7],
];

export default function draw(pen: Pen, ink: Ink) {
  const cols = SHEET[0].length;
  const rows = SHEET.length;
  const [w, h] = [CELL * cols, CELL * rows];
  const x = Math.round((pen.width - w) / 2);
  const y = Math.round((pen.height - h) / 2);
  const cells = new Grid(frame(x, y, w, h), cols, rows, 0);
  cells.carpet(pen, grid.mask(SHEET, 1), ink.fg);
  for (const [row, end] of RUNS) {
    for (let col = 2; col < end; col++) cells.fill(pen, col, row, ink.blue);
  }
}
