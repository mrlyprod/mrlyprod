import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const SIDE = 63;
const CELL = 13;
const HOME = { xMin: -2, xMax: 1, yMin: -1.5, yMax: 1.5 };
const MAX = 100;
const BANDS: [number, number][] = [[24, 1], [10, 0.55], [5, 0.3], [3, 0.14]];

function escape(cr: number, ci: number) {
  let zr = 0;
  let zi = 0;
  let n = 0;
  while (zr * zr + zi * zi <= 4 && n < MAX) {
    const t = zr * zr - zi * zi + cr;
    zi = 2 * zr * zi + ci;
    zr = t;
    n++;
  }
  return n;
}

export default function draw(pen: Pen, ink: Ink) {
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, 0);
  const tones = BANDS.map(([from, share]) => [from, ink.mix(ink.ground, ink.blue, share)] as const);
  let inside = 0;
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const x = HOME.xMin + ((col + 0.5) / SIDE) * (HOME.xMax - HOME.xMin);
      const y = HOME.yMax - ((row + 0.5) / SIDE) * (HOME.yMax - HOME.yMin);
      const n = escape(x, y);
      if (n === MAX) {
        inside++;
        cells.fill(pen, col, row, ink.fg);
      } else {
        const band = tones.find(([from]) => n >= from);
        if (band) cells.fill(pen, col, row, band[1]);
      }
    }
  }
  if (inside !== 695) throw new Error(`app-mandelbrot: ${inside} cells hold the set, want 695`);
}
