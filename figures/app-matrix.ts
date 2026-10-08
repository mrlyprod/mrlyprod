import * as font from "mrlyjs/font";
import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { font };

const CELL = 20;
const GLYPH = 5;
const ACROSS = 3;
const DOWN = 1;

const RAIN = [
  "7A2C0",
  "3EH81",
  "F49O5",
  "61PFL",
  "2U7IE",
  "O8LE3",
  "C5T9H",
];

const DROPS = [
  [0, 3, 4],
  [1, 5, 6],
  [2, 1, 2],
  [3, 4, 5],
  [4, 6, 4],
];

export default function draw(pen: Pen, ink: Ink) {
  const glyphs = font.map();
  const rows = RAIN.length;
  const cols = RAIN[0].length;
  const wide = cols * GLYPH + (cols - 1) * ACROSS;
  const tall = rows * GLYPH + (rows - 1) * DOWN;
  const cells = new Grid(frame(Math.round((pen.width - CELL * wide) / 2), Math.round((pen.height - CELL * tall) / 2), CELL * wide, CELL * tall), wide, tall, 0);
  for (const [col, head, length] of DROPS) {
    for (let d = 0; d < length; d++) {
      const row = head - d;
      if (row < 0 || row >= rows) continue;
      const c = RAIN[row][col];
      const bits = glyphs[c];
      if (!bits || bits.length !== GLYPH || bits.some((one) => one.length !== GLYPH)) throw new Error(`app-matrix: ${c} is not a ${GLYPH} by ${GLYPH} glyph`);
      const tone = d === 0 ? ink.fg : ink.mix(ink.ground, ink.blue, 1 - (0.7 * (d - 1)) / (length - 1));
      bits.forEach((one, y) => [...one].forEach((bit, x) => bit === "1" && cells.fill(pen, col * (GLYPH + ACROSS) + x, row * (GLYPH + DOWN) + y, tone)));
    }
  }
}
