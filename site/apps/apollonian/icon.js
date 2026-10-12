import * as num from "mrlyjs/num";

export const units = { num };

const ROOT = "-1,2,2,3";
const CAP = 256;
const WANT = 558;
const SEEDS = 3;
const STRIDE = 4;
const EPS = 1e-9;
const MARGIN = 0.05;
const WASH = 0.08;
const PALE = 0.25;
const RIM = 2;

let plan = null;

function study() {
  if (plan) return plan;
  const data = num.apollonian.flat(ROOT, CAP);
  const count = data.length / STRIDE;
  const outside = Array.from({ length: count }, (_, i) => i * STRIDE).filter((at) => !Number.isInteger(data[at + 3]) || data[at + 3] > CAP || Math.hypot(data[at], data[at + 1]) + data[at + 2] > 1 + EPS).length;
  if (count !== WANT || outside !== 0) throw new Error(`apollonian: ${count} circles, ${outside} off the outer circle, want ${WANT}, 0`);
  const k0 = data[SEEDS * STRIDE + 3];
  const octaves = Math.ceil(Math.log2(CAP / k0)) + 1;
  const bands = new Uint8Array(count);
  for (let i = 0; i < count; i++) bands[i] = Math.min(octaves - 1, Math.max(0, Math.floor(Math.log2(data[i * STRIDE + 3] / k0))));
  plan = { data, bands, octaves, count };
  return plan;
}

export default function draw(pen, ink) {
  const { data, bands, octaves, count } = study();
  const box = pen.frame(MARGIN);
  const [cx, cy] = box.center();
  const R = box.w / 2;
  const shades = Array.from({ length: octaves }, (_, b) => ink.mix(ink.ground, ink.blue, PALE + (1 - PALE) * (b / (octaves - 1))));
  pen.disc(cx, cy, R, ink.mix(ink.ground, ink.blue, WASH));
  for (let i = 0; i < count; i++) pen.disc(cx + data[i * STRIDE] * R, cy - data[i * STRIDE + 1] * R, data[i * STRIDE + 2] * R, shades[bands[i]]);
  pen.ring(cx, cy, R, RIM, ink.dim);
}
