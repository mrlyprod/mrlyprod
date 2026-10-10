import { frame, Grid } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const SIDE = 63;
const CELL = 13;
const SUB = 3;
const HOME = { xMin: -1.5, xMax: 1.5, yMin: -1.5, yMax: 1.5 };
const SEED = [-0.8, 0.156];
const MAX = 100;
const BANDS: [number, number][] = [[24, 1], [10, 0.55], [5, 0.3], [3, 0.14]];

function escape(x: number, y: number) {
  let zr = x;
  let zi = y;
  let n = 0;
  while (zr * zr + zi * zi <= 4 && n < MAX) {
    const t = zr * zr - zi * zi + SEED[0];
    zi = 2 * zr * zi + SEED[1];
    zr = t;
    n++;
  }
  return n;
}

function slowest(col: number, row: number) {
  let most = 0;
  for (let j = 0; j < SUB; j++) {
    for (let i = 0; i < SUB; i++) {
      const x = HOME.xMin + ((col + (i + 0.5) / SUB) / SIDE) * (HOME.xMax - HOME.xMin);
      const y = HOME.yMax - ((row + (j + 0.5) / SUB) / SIDE) * (HOME.yMax - HOME.yMin);
      most = Math.max(most, escape(x, y));
    }
  }
  return most;
}

let memo: Uint8Array | undefined;

function counts() {
  if (memo) return memo;
  const out = new Uint8Array(SIDE * SIDE);
  for (let row = 0; row < SIDE; row++) for (let col = 0; col < SIDE; col++) out[row * SIDE + col] = slowest(col, row);
  for (let at = 0; at < out.length; at++) if (out[at] !== out[out.length - 1 - at]) throw new Error(`app-julia: cell ${at} breaks the symmetry z to -z`);
  const inside = out.filter((n) => n === MAX).length;
  if (inside !== 761) throw new Error(`app-julia: ${inside} cells hold the set, want 761`);
  memo = out;
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const w = CELL * SIDE;
  const cells = new Grid(frame(Math.round((pen.width - w) / 2), Math.round((pen.height - w) / 2), w, w), SIDE, SIDE, 0);
  const tones = BANDS.map(([from, share]) => [from, ink.mix(ink.ground, ink.blue, share)] as const);
  const steps = counts();
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      const n = steps[row * SIDE + col];
      if (n === MAX) cells.fill(pen, col, row, ink.fg);
      else {
        const band = tones.find(([from]) => n >= from);
        if (band) cells.fill(pen, col, row, band[1]);
      }
    }
  }
}
