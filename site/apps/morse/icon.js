import * as num from "mrlyjs/num";
import { Grid } from "mrlyjs/view";
import { plate } from "../../lib/icon.js";

export const units = { num };

const LEVEL = 2;
const SIDE = 2 ** LEVEL;
const GAP = 0.12;
const CORNER = 0.24;
const WANT = { tile: "0110", level: LEVEL, plus: SIDE * SIDE / 2 };

let lift = null;

function signs() {
  if (lift) return lift;
  const grid = num.morse.lift("Parity", SIDE);
  const read = num.morse.fold(grid, SIDE, 2);
  let plus = 0;
  for (const bit of grid) if (!bit) plus++;
  const got = [read.tile.join(""), read.level, plus].join();
  const want = [WANT.tile, WANT.level, WANT.plus].join();
  if (!read.folds || got !== want) throw new Error(`morse: the parity lift of ${SIDE} reads ${got}, want ${want}`);
  lift = grid;
  return grid;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "yellow", 0.18);
  const grid = signs();
  const cells = new Grid(box, SIDE, SIDE, GAP);
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const [x, y, w, h] = cells.cell(col, row);
      pen.round_rect(x, y, w, h, w * CORNER, grid[row * SIDE + col] ? paint.deep : paint.white);
    }
  }
}
