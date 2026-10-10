import * as math from "mrlyjs/math";
import { frame, Grid, plot } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";
import census from "./census/demo-tube.json" with { type: "json" };

const CODE = "495";
const SIDE = 3;
const LEVEL = 5;
const SPAN = 243;
const CELL = 2;
const STEPS = 1200;

export const units = { math };

let memo: ArrayLike<number> | undefined;

function carpet() {
  if (memo) return memo;
  const cells = math.two.create(CODE, SIDE, LEVEL, 0, SIDE);
  if (cells.types.length !== SPAN * SPAN) throw new Error(`demo-tube: ${cells.types.length} cells, want ${SPAN * SPAN}`);
  let filled = 0;
  for (const kind of cells.types) if (kind !== 0) filled++;
  if (filled !== 32768) throw new Error(`demo-tube: ${filled} filled cells, want 32768`);
  memo = cells.types;
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { near, profile } = census as { near: string; profile: number[] };
  if (near.length !== SPAN * SPAN) throw new Error(`demo-tube: ${near.length} near flags, want ${SPAN * SPAN}`);
  if (profile.length !== STEPS + 1) throw new Error(`demo-tube: ${profile.length} samples, want ${STEPS + 1}`);
  const types = carpet();
  const phases = Array.from({ length: STEPS + 1 }, (_, i) => 1 / 3 + ((2 / 3) * i) / STEPS);
  const margin = Math.round(pen.width * 0.08);
  const plate = CELL * SPAN;
  const lattice = new Grid(frame(margin, margin, plate, plate), SPAN, SPAN, 0);
  for (let row = 0; row < SPAN; row++) {
    for (let col = 0; col < SPAN; col++) {
      const at = row * SPAN + col;
      if (types[at] !== 0) lattice.fill(pen, col, row, ink.fg);
      else if (near[at] === "1") lattice.fill(pen, col, row, ink.blue);
    }
  }
  const tall = 320;
  const panel = frame(pen.width - margin - plate, pen.height - margin - tall, plate, tall);
  plot.axis(pen, panel, ink.line);
  plot.curve(pen, panel.inset(24), phases, profile, 4, ink.yellow);
}
