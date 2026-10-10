import * as num from "mrlyjs/num";
import { frame, Grid } from "mrlyjs/view";
import type { Color, Ink, Pen } from "mrlyjs/view";

export const units = { num };

const REACH = 15;
const SIDE = 2 * REACH + 1;
const CELL = 33;
const GAP = 0.14;
const FAINT = 0.3;
const HALF = 0.5;
const WANT = { points: 961, split: 232, inert: 12, ramified: 4, units: 4 };
const CLASSES: Record<string, number> = { Composite: 0, Split: 1, Inert: 2, Ramified: 3, Unit: 4, Zero: 5 };

let fates: Uint8Array | null = null;

function facts(): Uint8Array {
  if (fates) return fates;
  const window = new num.gauss.Window("Gaussian", REACH);
  const census = window.census();
  const grid = new Uint8Array(SIDE * SIDE);
  const seen = [0, 0, 0, 0, 0, 0];
  for (let b = -REACH; b <= REACH; b++) {
    for (let a = -REACH; a <= REACH; a++) {
      const fate = CLASSES[window.class(a, b)];
      grid[(REACH - b) * SIDE + (a + REACH)] = fate;
      seen[fate]++;
    }
  }
  window.free();
  const read = { points: census.points, split: seen[1], inert: seen[2], ramified: seen[3], units: seen[4] };
  const got = Object.values(read).join();
  if (got !== Object.values(WANT).join() || census.split !== seen[1] || census.inert !== seen[2]) throw new Error(`app-gaussian: the window of ${REACH} reads ${got}, want ${Object.values(WANT).join()}`);
  fates = grid;
  return grid;
}

export default function draw(pen: Pen, ink: Ink) {
  const grid = facts();
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, GAP);
  const tones: (Color | null)[] = [ink.mix(ink.ground, ink.dim, FAINT), ink.blue, ink.mix(ink.ground, ink.blue, HALF), ink.fg, ink.dim, null];
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const tone = tones[grid[row * SIDE + col]];
      if (tone) cells.fill(pen, col, row, tone);
    }
  }
}
