import { Grid } from "#view";
import { plate } from "../../lib/icon.js";

export const units = {};

const ART = ["0000011100000", "0011011101100", "0111111111110", "0011111111100", "0111000001110", "1110022200111", "1110022200111", "1110022200111", "0111000001110", "0011111111100", "0111111111110", "0011011101100", "0000011100000"];

export default function draw(pen) {
  const { box, paint } = plate(pen, "gray", 0.22);
  const cols = ART[0].length;
  const rows = ART.length;
  const tone = { 1: paint.white, 2: paint.deep };
  for (const line of ART) {
    if (line.length !== cols) throw new Error(`settings: row ${line} is not ${cols} wide`);
    for (const c of line) if (c !== "0" && !tone[c]) throw new Error(`settings: no tone for ${c}`);
  }
  const cells = new Grid(box, cols, rows, 0);
  ART.forEach((line, row) => [...line].forEach((c, col) => c !== "0" && cells.fill(pen, col, row, tone[c])));
}
