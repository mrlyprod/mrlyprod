import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const CODE = 23;
const SIDE = 27;
const CELL = 33;
const DEPTHS = 3;
const WANT = [512, 64, 8];

const kept = (x: number, y: number) => (CODE >> ((x & 1) * 4 + (y & 1) * 2)) & 1;

function lit(level: number, x: number, y: number) {
  for (let l = 0; l < level; l++, x = Math.floor(x / 3), y = Math.floor(y / 3)) if (!kept(x % 3, y % 3)) return false;
  return true;
}

export default function draw(pen: Pen, ink: Ink) {
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, 0);
  const tones = [ink.fg, ink.blue, ink.mix(ink.ground, ink.blue, 0.55)];
  const counts = [0, 0, 0];
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      let depth = 0;
      while (depth + 1 < DEPTHS) {
        const side = SIDE / 3 ** (depth + 1);
        const from = (SIDE - side) / 2;
        if (col < from || row < from || col >= from + side || row >= from + side) break;
        depth++;
      }
      const side = SIDE / 3 ** depth;
      const from = (SIDE - side) / 2;
      if (!lit(DEPTHS - depth, col - from, row - from)) continue;
      counts[depth]++;
      cells.fill(pen, col, row, tones[depth]);
    }
  }
  if (counts.join() !== WANT.join()) throw new Error(`app-zoom: the nested cells light ${counts.join(", ")}, want ${WANT.join(", ")}`);
}
