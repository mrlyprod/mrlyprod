import type { Ink, Pen } from "mrlyjs/view";
import census from "./census/wiki-stern-diatomic-sequence.json" with { type: "json" };

const LEVEL = 8;

export const units = {};

export default function draw(pen: Pen, ink: Ink) {
  const { rows } = census as { rows: number[][] };
  if (rows.length !== LEVEL) throw new Error(`wiki-stern-diatomic-sequence: ${rows.length} rows, want ${LEVEL}`);
  rows.forEach((row, k) => {
    if (row.length !== 2 ** k + 1) throw new Error(`wiki-stern-diatomic-sequence: row ${k} holds ${row.length} terms, want ${2 ** k + 1}`);
    if (row.some((value) => !Number.isInteger(value) || value < 1)) throw new Error(`wiki-stern-diatomic-sequence: row ${k} holds a term below one`);
  });
  const frame = pen.frame(0.08);
  const peaks = rows.map((row) => Math.max(...row));
  const finest = frame.w / 2 ** (LEVEL - 1);
  const width = finest * 0.66;
  const span = frame.w - width;
  const band = frame.h / LEVEL;
  const tall = band * 0.8;
  rows.forEach((row, k) => {
    const foot = frame.y + band * (k + 1) - (band - tall) / 2;
    row.forEach((value, i) => {
      const height = (tall * value) / peaks[k];
      const x = frame.x + (span * i) / 2 ** k;
      pen.rect(x, foot - height, width, height, value === peaks[k] ? ink.yellow : ink.blue);
    });
  });
}
