import { frame, Grid, type Ink, type Pen } from "mrlyjs/view";
import census from "./census/demo-integers.json" with { type: "json" };

const WINDOW = 1000;
const COLS = 40;
const LINES = 25;

export const units = {};

export default function draw(pen: Pen, ink: Ink) {
  const counts = [census.d8, census.d16, census.d32, census.d48];
  if (counts.some((tally) => tally?.length !== WINDOW + 1)) throw new Error(`demo-integers: tallies of ${counts.map((tally) => tally?.length)}, want ${WINDOW + 1} each`);
  const panels = new Grid(pen.frame(0.08), 2, 2, 0.06);
  let drawn = 0;
  counts.forEach((tally, slot) => {
    const [x, y, w, h] = panels.cell(slot % 2, Math.floor(slot / 2));
    const cells = new Grid(frame(x, y, w, h), COLS, LINES, 0.1);
    for (let row = 0; row < LINES; row++) {
      for (let col = 0; col < COLS; col++) {
        const count = tally[row * COLS + col + 1];
        cells.fill(pen, col, row, count === 0 ? ink.orange : count === 1 ? ink.dim : ink.blue);
        drawn++;
      }
    }
  });
  if (drawn !== 4 * WINDOW) throw new Error(`demo-integers: drew ${drawn} cells, want ${4 * WINDOW}`);
}
