import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const CELL = 19;
const STEPS = 16;
const PAD = [2, 4];
const GAP = [1, 2];
const LIT = 4;

const ROWS = [
  "x...x...x...x...",
  "..xx..x...xx..x.",
  "....x.......x...",
  "x.xx.x..x.x..xx.",
];

export default function draw(pen: Pen, ink: Ink) {
  for (const row of ROWS) if (row.length !== STEPS) throw new Error(`app-techno: ${row} is not ${STEPS} steps`);
  if (![0, 4, 8, 12].every((s) => ROWS[0][s] === "x")) throw new Error("app-techno: the kick is not four on the floor");
  if (ROWS[0][LIT] !== "x" || ROWS[2][LIT] !== "x") throw new Error("app-techno: the lit step holds no kick and clap");
  const col = (s: number) => s * (PAD[0] + GAP[0]) + Math.floor(s / 4);
  const cols = col(STEPS - 1) + PAD[0];
  const rows = ROWS.length * PAD[1] + (ROWS.length - 1) * GAP[1];
  const [w, h] = [CELL * cols, CELL * rows];
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - h) / 2), w, h), cols, rows, 0);
  ROWS.forEach((line, r) => {
    [...line].forEach((c, s) => {
      const on = c === "x";
      const tone = s === LIT ? (on ? ink.fg : ink.dim) : on ? ink.blue : ink.mix(ink.ground, ink.dim, 0.35);
      for (let x = 0; x < PAD[0]; x++) for (let y = 0; y < PAD[1]; y++) cells.fill(pen, col(s) + x, r * (PAD[1] + GAP[1]) + y, tone);
    });
  });
}
