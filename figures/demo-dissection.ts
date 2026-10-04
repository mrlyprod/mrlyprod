import * as num from "mrlyjs/num";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { num };

const BASE = 10;
const MISSING = 7;
const LEVEL = 3;
const Z = 8;
const COUNTS = [732, 202, 26, 40];
const DECADES = 4;
const SLOTS: Record<string, number> = { A: 0, B: 1, C1: 2, C2: 3 };

type Data = { slots: number[]; weight: Float64Array; scale: number };

let memo: Data | undefined;

function data() {
  if (memo) return memo;
  const digits: number[] = [];
  for (let d = 0; d < BASE; d++) if (d !== MISSING) digits.push(d);
  const cut = num.dissection.regions(BASE, LEVEL, Z);
  const weight = num.dissection.weights(BASE, digits, LEVEL);
  const y = cut.length;
  const scale = digits.length ** LEVEL;
  const slots = cut.map((region) => SLOTS[region]);
  if (slots.some((slot) => slot === undefined)) throw new Error(`demo-dissection: a region outside ${Object.keys(SLOTS)}`);
  const counts = [0, 0, 0, 0];
  for (const slot of slots) counts[slot]++;
  if (counts.some((count, i) => count !== COUNTS[i])) throw new Error(`demo-dissection: regions ${counts}, want ${COUNTS}`);
  if (y !== 1000) throw new Error(`demo-dissection: ${y} frequencies, want 1000`);
  if (weight.length !== y) throw new Error(`demo-dissection: ${weight.length} weights, want ${y}`);
  if (!(Math.abs(weight[0] / scale - 1) < 1e-12)) throw new Error("demo-dissection: the first weight is not the scale");
  memo = { slots, weight, scale };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { slots, weight, scale } = data();
  const y = slots.length;
  const frame = pen.frame(0.08);
  const [cx, cy] = frame.center();
  const outer = frame.radius();
  const inner = 0.36 * outer;
  const inks = [ink.blue, ink.dim, ink.orange, ink.yellow];
  const at = (r: number, t: number): Point => [cx + r * Math.sin(t), cy - r * Math.cos(t)];
  const step = (2 * Math.PI) / y;
  pen.ring(cx, cy, inner - 6, 1, ink.line);
  for (const pass of [1, 0, 2, 3]) {
    for (let a = 0; a < y; a++) {
      if (slots[a] !== pass) continue;
      const w = weight[a] / scale;
      const reach = Math.min(Math.max(1 + Math.log10(Math.max(w, 1e-12)) / DECADES, 0.015), 1);
      const top = inner + (outer - inner) * reach;
      const mid = a * step;
      const half = 0.34 * step;
      pen.polygon([at(inner, mid - half), at(top, mid - half), at(top, mid + half), at(inner, mid + half)], inks[pass]);
    }
  }
}
