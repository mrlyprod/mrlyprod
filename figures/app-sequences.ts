import * as math from "mrlyjs/math";
import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { math };

const CODE = "7";
const SIDE = 25;
const CELL = 36;
const PINS = 8;
const GAP = 3;
const LEFT = 1;
const FLOOR = SIDE - 2;
const RISE = 19;
const OCTAGONAL = [8, 21, 40, 65, 96, 133, 176, 225];

let heights: number[] | null = null;

function terms() {
  if (heights) return heights;
  const read = Array.from({ length: PINS }, (_, k) => Number(math.counts.fill(CODE, 2 * (k + 2) - 1, 2, 1, 2)));
  if (read.join() !== OCTAGONAL.join()) throw new Error(`app-sequences: the carpet fills by odd side read ${read.join(", ")}, want the octagonal numbers ${OCTAGONAL.join(", ")}`);
  heights = read.map((v) => Math.max(1, Math.round((RISE * v) / OCTAGONAL[PINS - 1])));
  if (heights[PINS - 1] !== RISE || heights[0] < 1) throw new Error("app-sequences: the tallest pin does not reach the rise");
  return heights;
}

export default function draw(pen: Pen, ink: Ink) {
  const tall = terms();
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, 0);
  for (let col = 0; col < SIDE; col++) cells.fill(pen, col, FLOOR + 1, ink.dim);
  const stem = ink.mix(ink.ground, ink.blue, 0.55);
  let lit = 0;
  tall.forEach((h, k) => {
    const col = LEFT + k * GAP;
    for (let i = 1; i < h; i++, lit++) cells.fill(pen, col, FLOOR - i + 1, stem);
    cells.fill(pen, col, FLOOR - h + 1, ink.blue);
    cells.fill(pen, col + 1, FLOOR - h + 1, ink.blue);
    cells.fill(pen, col, FLOOR - h, ink.fg);
    cells.fill(pen, col + 1, FLOOR - h, ink.fg);
    lit += 4;
  });
  const want = tall.reduce((sum, h) => sum + h - 1, 0) + 4 * PINS;
  if (lit !== want) throw new Error(`app-sequences: ${lit} cells lit, want ${want}`);
}
