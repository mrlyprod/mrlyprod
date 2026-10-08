import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const CELL = 64;

const ART = [
  "1110011100111",
  "1110011100111",
  "1110011100111",
  "0000000000000",
  "0000000000000",
  "1110022200111",
  "1110022200111",
  "1110022200111",
  "0000000000000",
  "0000000000000",
  "1110011100111",
  "1110011100111",
  "1110011100111",
];

export default function draw(pen: Pen, ink: Ink) {
  const cols = ART[0].length;
  const rows = ART.length;
  const tone: Record<string, typeof ink.fg> = { "1": ink.fg, "2": ink.blue };
  for (const line of ART) {
    if (line.length !== cols) throw new Error(`site-menu: row ${line} is not ${cols} wide`);
    for (const c of line) if (c !== "0" && !tone[c]) throw new Error(`site-menu: no tone for ${c}`);
  }
  const w = CELL * cols;
  const h = CELL * rows;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - h) / 2), w, h), cols, rows, 0);
  ART.forEach((line, row) => [...line].forEach((c, col) => c !== "0" && cells.fill(pen, col, row, tone[c])));
}
