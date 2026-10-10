import * as num from "mrlyjs/num";
import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const LEVEL = 5;
const SIDE = 2 ** LEVEL;
const CELL = 30;
const GAP = 0.12;
const FAINT = 0.3;
const WANT = { tile: "0110", level: LEVEL, plus: SIDE * SIDE / 2 };

let lift: Uint8Array | null = null;

function signs(): Uint8Array {
  if (lift) return lift;
  const grid = num.morse.lift("Parity", SIDE);
  const read = num.morse.fold(grid, SIDE, 2);
  let plus = 0;
  for (const bit of grid) if (!bit) plus++;
  const got = [read.tile.join(""), read.level, plus].join();
  const want = [WANT.tile, WANT.level, WANT.plus].join();
  if (!read.folds || got !== want) throw new Error(`app-morse: the parity lift of ${SIDE} reads ${got}, want ${want}`);
  lift = grid;
  return grid;
}

export default function draw(pen: Pen, ink: Ink) {
  const grid = signs();
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, GAP);
  const faint = ink.mix(ink.ground, ink.dim, FAINT);
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) cells.fill(pen, col, row, grid[row * SIDE + col] ? faint : ink.blue);
  }
}
