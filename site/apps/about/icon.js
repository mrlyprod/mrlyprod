import { grid, Grid } from "#view";
import { plate } from "../../lib/icon.js";

export const units = {};

const WANT = { side: 5, holes: 4 };

let memo = null;

function holes() {
  if (memo) return memo;
  const mask = grid.mask(grid.LOGO, 1);
  const found = [];
  mask.forEach((row, r) => row.forEach((on, c) => on || found.push([c, r])));
  if (mask.length !== WANT.side || found.length !== WANT.holes) throw new Error(`about: the mark is ${mask.length} a side with ${found.length} holes, want ${WANT.side} and ${WANT.holes}`);
  memo = { side: mask.length, found };
  return memo;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "indigo", 0.2);
  const { side, found } = holes();
  const cells = new Grid(box, side, side, 0);
  const unit = box.w / side;
  pen.round_rect(box.x, box.y, box.w, box.h, unit * 0.45, paint.white);
  for (const [col, row] of found) {
    const [x, y, w, h] = cells.cell(col, row);
    pen.round_rect(x, y, w, h, unit * 0.18, paint.ink);
  }
}
