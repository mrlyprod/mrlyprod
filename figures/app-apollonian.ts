import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const ROOT = "-1,2,2,3";
const CAP = 256;
const WANT = 555;
const MARGIN = 0.05;
const WASH = 0.08;
const PALE = 0.25;
const RIM = 2;

type Plan = { data: Float64Array; bands: Uint8Array; octaves: number; count: number };

let plan: Plan | null = null;

function study(): Plan {
  if (plan) return plan;
  const packing = num.apollonian.grow(ROOT, CAP);
  if (packing.circles.length !== WANT || packing.broken !== 0 || packing.strayed !== 0) throw new Error(`app-apollonian: ${packing.circles.length} circles, ${packing.broken} broken, ${packing.strayed} strayed, want ${WANT}, 0, 0`);
  const rows = [...packing.root.filter((c) => c.k > 0), ...packing.circles];
  const k0 = packing.circles[0].k;
  const octaves = Math.ceil(Math.log2(CAP / k0)) + 1;
  const data = new Float64Array(rows.length * 3);
  const bands = new Uint8Array(rows.length);
  rows.forEach((c, i) => {
    data[i * 3] = c.x / c.k;
    data[i * 3 + 1] = c.y / c.k;
    data[i * 3 + 2] = 1 / c.k;
    bands[i] = Math.min(octaves - 1, Math.max(0, Math.floor(Math.log2(c.k / k0))));
  });
  plan = { data, bands, octaves, count: rows.length };
  return plan;
}

export default function draw(pen: Pen, ink: Ink) {
  const { data, bands, octaves, count } = study();
  const box = pen.frame(MARGIN);
  const [cx, cy] = box.center();
  const R = box.w / 2;
  const shades = Array.from({ length: octaves }, (_, b) => ink.mix(ink.ground, ink.blue, PALE + (1 - PALE) * (b / (octaves - 1))));
  pen.disc(cx, cy, R, ink.mix(ink.ground, ink.blue, WASH));
  for (let i = 0; i < count; i++) pen.disc(cx + data[i * 3] * R, cy - data[i * 3 + 1] * R, data[i * 3 + 2] * R, shades[bands[i]]);
  pen.ring(cx, cy, R, RIM, ink.dim);
}
