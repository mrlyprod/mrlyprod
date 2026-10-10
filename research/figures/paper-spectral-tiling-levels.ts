import type { Ink, Pen } from "mrlyjs/view";

export const units = {};

const BASE = 12;
const DIGITS = [0, 3, 6, 9];
const DEPTH = 5;
const LOW = -1.25;
const HIGH = 2;
const TERMS = 16;

function lattice(): number[] {
  let pts = [0];
  for (let j = 0; j < DEPTH; j++) {
    const w = BASE ** j;
    pts = pts.flatMap((p) => DIGITS.map((l) => p + w * l));
  }
  return pts.sort((a, b) => a - b);
}

function power(xi: number): number {
  let v = 1;
  let t = xi;
  for (let j = 0; j < TERMS; j++) {
    t /= BASE;
    v *= Math.cos(Math.PI * t) * Math.cos(8 * Math.PI * t);
  }
  return v * v;
}

export default function draw(pen: Pen, ink: Ink) {
  const base = lattice();
  if (base.length !== 4 ** DEPTH) throw new Error(`paper-spectral-tiling-levels: ${base.length} lattice points, want ${4 ** DEPTH}`);
  const groups = [base.map((l) => 3 * l), base.map((l) => 3 * l + 0.75)];
  const frame = pen.frame(0.08);
  const cols = Math.round(frame.w);
  const tones = [ink.blue, ink.orange];
  for (let c = 0; c < cols; c++) {
    const xi = LOW + ((c + 0.5) / cols) * (HIGH - LOW);
    const parts = groups.map((g) => g.map((l) => power(xi - l)));
    const total = parts.flat().reduce((a, b) => a + b, 0);
    if (!(total <= 1 + 1e-9 && total > 1 - 1e-2)) throw new Error(`paper-spectral-tiling-levels: Q(${xi}) = ${total}`);
    let y = frame.y + frame.h;
    parts.forEach((bands, g) => {
      let run = 0;
      const flush = () => {
        if (run > 0) pen.rect(frame.x + c, y - run, 1, run, tones[g]);
        y -= run;
        run = 0;
      };
      for (const q of bands) {
        const h = q * frame.h;
        if (h < 4) {
          run += h;
          continue;
        }
        flush();
        pen.rect(frame.x + c, y - h, 1, h - 1.5, tones[g]);
        pen.rect(frame.x + c, y - 1.5, 1, 1.5, ink.ground);
        y -= h;
      }
      flush();
    });
  }
}
