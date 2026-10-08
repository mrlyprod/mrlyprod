import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const CELL = 64;

const ART = [
  "0003333333333",
  "0003000000003",
  "0003000000003",
  "1111111111003",
  "1000000001003",
  "1022222201003",
  "1000000001003",
  "1022222201003",
  "1000000001003",
  "1022220001333",
  "1000000001000",
  "1000000001000",
  "1111111111000",
];

export default function draw(pen: Pen, ink: Ink) {
  const cols = ART[0].length;
  const rows = ART.length;
  const tone: Record<string, typeof ink.fg> = { "1": ink.fg, "2": ink.blue, "3": ink.dim };
  for (const line of ART) {
    if (line.length !== cols) throw new Error(`site-pages: row ${line} is not ${cols} wide`);
    for (const c of line) if (c !== "0" && !tone[c]) throw new Error(`site-pages: no tone for ${c}`);
  }
  const w = CELL * cols;
  const h = CELL * rows;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - h) / 2), w, h), cols, rows, 0);
  ART.forEach((line, row) => [...line].forEach((c, col) => c !== "0" && cells.fill(pen, col, row, tone[c])));
}
