import type { Color, Pen } from "#view";
import { Grid } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = {};

const GAP = 0.2;
const CORNER = 0.3;

const ART = ["111", "121", "111"];

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "gray", 0.2);
  const cols = ART[0].length;
  const rows = ART.length;
  const tone: Record<string, Color> = { 1: paint.white, 2: paint.light };
  for (const line of ART) {
    if (line.length !== cols) throw new Error(`menu: row ${line} is not ${cols} wide`);
    for (const c of line) if (c !== "0" && !tone[c]) throw new Error(`menu: no tone for ${c}`);
  }
  const cells = new Grid(box, cols, rows, GAP);
  ART.forEach((line, row) => {
    [...line].forEach((c, col) => {
      if (c === "0") return;
      const [x, y, w, h] = cells.cell(col, row);
      pen.round_rect(x, y, w, h, Math.min(w, h) * CORNER, tone[c]);
    });
  });
}
