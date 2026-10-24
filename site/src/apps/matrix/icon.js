import * as font from "@mrlyprod/mrly/font";
import { frame, Grid, mix } from "#view";
import { plate } from "../../lib/plate.js";

export const units = { font };

const GLYPH = 5;
const ACROSS = 2;
const DOWN = 1;

const RAIN = ["7A2", "3EH", "F49", "61P"];

const DROPS = [
  [0, 2, 3],
  [1, 3, 4],
  [2, 3, 3],
];

export default function draw(pen) {
  const { box, paint } = plate(pen, "green", 0.16);
  const glyphs = font.map();
  const rows = RAIN.length;
  const cols = RAIN[0].length;
  for (const line of RAIN) if (line.length !== cols) throw new Error(`matrix: row ${line} is not ${cols} wide`);
  const wide = cols * GLYPH + (cols - 1) * ACROSS;
  const tall = rows * GLYPH + (rows - 1) * DOWN;
  const cell = Math.min(box.w / wide, box.h / tall);
  const [cx, cy] = box.center();
  const cells = new Grid(
    frame(cx - (cell * wide) / 2, cy - (cell * tall) / 2, cell * wide, cell * tall),
    wide,
    tall,
    0,
  );
  for (const [col, head, length] of DROPS) {
    for (let d = 0; d < length; d++) {
      const row = head - d;
      if (row < 0 || row >= rows) continue;
      const c = RAIN[row][col];
      const bits = glyphs[c];
      if (!bits || bits.length !== GLYPH || bits.some((one) => one.length !== GLYPH))
        throw new Error(`matrix: ${c} is not a ${GLYPH} by ${GLYPH} glyph`);
      const tone =
        d === 0 ? paint.white : length < 3 ? paint.light : mix(paint.light, paint.deep, (d - 1) / (length - 2));
      bits.forEach((one, y) => {
        [...one].forEach((bit, x) => {
          if (bit === "1") cells.fill(pen, col * (GLYPH + ACROSS) + x, row * (GLYPH + DOWN) + y, tone);
        });
      });
    }
  }
}
