import * as life from "mrlyjs/life";
import { Grid } from "mrlyjs/view";
import { plate } from "../../lib/icon.js";

export const units = { life };

const GLIDER = [[1, 0], [2, 1], [0, 2], [1, 2], [2, 2]];
const SIDE = 9;
const START = 1;
const PERIOD = 4;
const GHOSTS = [128, 88, 56, 28];
const FIELD = 3 + GHOSTS.length;
const GAP = 0.1;
const CORNER = 0.26;
const MOORE = { shape: [3, 3], data: Uint8Array.from([1, 1, 1, 1, 0, 1, 1, 1, 1]) };

let proven = false;

function prove() {
  if (proven) return;
  const seed = new Uint8Array(SIDE * SIDE);
  for (const [c, r] of GLIDER) seed[(START + r) * SIDE + START + c] = 1;
  let cell = { shape: [SIDE, SIDE], types: seed };
  for (let g = 0; g < PERIOD; g++) {
    cell = life.next_grid(cell, [3], [2, 3], MOORE, "Constant");
    const alive = cell.types.reduce((a, b) => a + b, 0);
    if (alive !== GLIDER.length) throw new Error(`life: generation ${g + 1} has ${alive} live cells, the glider has ${GLIDER.length}`);
  }
  for (const [c, r] of GLIDER) if (!cell.types[(START + 1 + r) * SIDE + START + 1 + c]) throw new Error(`life: after ${PERIOD} generations the glider is not one step down the diagonal`);
  proven = true;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "green", 0.16);
  prove();
  const cells = new Grid(box, FIELD, FIELD, GAP);
  const alpha = new Map();
  const stamp = (off, a) => {
    for (const [c, r] of GLIDER) {
      const key = (off + r) * FIELD + off + c;
      alpha.set(key, Math.max(alpha.get(key) ?? 0, a));
    }
  };
  GHOSTS.forEach((a, i) => stamp(GHOSTS.length - 1 - i, a));
  stamp(GHOSTS.length, 255);
  for (const [key, a] of alpha) {
    const [x, y, w, h] = cells.cell(key % FIELD, Math.floor(key / FIELD));
    pen.round_rect(x, y, w, h, w * CORNER, a === 255 ? paint.white : [255, 255, 255, a]);
  }
}
