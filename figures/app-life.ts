import * as life from "mrlyjs/life";
import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { life };

const SIDE = 21;
const CELL = 44;
const CODE = 7;
const NUMBER = 3;
const LEVEL = 2;
const GEN = 4;
const WANT = [64, 56];

let grown: { seed: Uint8Array; now: Uint8Array } | null = null;

function grow() {
  if (grown) return grown;
  const design = life.design_mask(2, CODE, NUMBER, LEVEL);
  const span = design.shape[0];
  const off = (SIDE - span) >> 1;
  const seed = new Uint8Array(SIDE * SIDE);
  for (let r = 0; r < span; r++) for (let c = 0; c < span; c++) seed[(off + r) * SIDE + off + c] = design.data[r * span + c];
  const mask = life.design_mask(2, CODE, NUMBER, 1);
  let cell = { shape: [SIDE, SIDE], types: seed };
  for (let g = 0; g < GEN; g++) cell = life.next_grid(cell, [3], [2, 3], mask, "Constant");
  const now = Uint8Array.from(cell.types);
  const count = (cells: Uint8Array) => cells.reduce((a, b) => a + b, 0);
  const counts = [count(seed), count(now)];
  if (counts.join() !== WANT.join()) throw new Error(`app-life: the carpet runs ${counts.join(", ")} live, want ${WANT.join(", ")}`);
  for (let r = 0; r < SIDE; r++) for (let c = 0; c < SIDE; c++) if (now[r * SIDE + c] !== now[c * SIDE + r] || now[r * SIDE + c] !== now[(SIDE - 1 - r) * SIDE + c]) throw new Error("app-life: generation 4 lost the carpet's symmetry");
  grown = { seed, now };
  return grown;
}

export default function draw(pen: Pen, ink: Ink) {
  const { seed, now } = grow();
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, 0.12);
  const ghost = ink.mix(ink.ground, ink.dim, 0.45);
  for (let i = 0; i < SIDE * SIDE; i++) {
    if (now[i]) cells.fill(pen, i % SIDE, Math.floor(i / SIDE), ink.blue);
    else if (seed[i]) cells.fill(pen, i % SIDE, Math.floor(i / SIDE), ghost);
  }
}
