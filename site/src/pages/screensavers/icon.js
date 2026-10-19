import { Grid } from "#view";
import { plate } from "../../lib/plate.js";

export const units = {};

const ART = ["1111111111111", "1222222222221", "1222221222221", "1222211122221", "1221111111221", "1222111112221", "1222211122221", "1222112112221", "1222122212221", "1222222222221", "1111111111111", "0000011100000", "0001111111000"];

export default function draw(pen) {
  const { box, paint } = plate(pen, "teal", 0.22);
  const cols = ART[0].length;
  const rows = ART.length;
  const tone = { 1: paint.white, 2: paint.deep };
  for (const line of ART) {
    if (line.length !== cols) throw new Error(`screensavers: row ${line} is not ${cols} wide`);
    for (const c of line) if (c !== "0" && !tone[c]) throw new Error(`screensavers: no tone for ${c}`);
  }
  const cells = new Grid(box, cols, rows, 0);
  ART.forEach((line, row) => [...line].forEach((c, col) => c !== "0" && cells.fill(pen, col, row, tone[c])));
}
