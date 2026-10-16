import * as num from "@mrlyprod/mrly/num";
import { plate } from "../../lib/icon.js";

export const units = { num };

const ROOT = "-1,2,2,3";
const CAP = 256;
const WANT = 558;
const HALVES = 2;
const SEEDS = 3;
const STRIDE = 4;
const EPS = 1e-9;
const FLOOR = 110;
const RIM = 0.016;
const SHRINK = 0.93;

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

export default function draw(pen) {
  const { box, paint, size } = plate(pen, "cyan", 0.16);
  const { data, bands, octaves, count } = study();
  const [cx, cy] = box.center();
  const R = box.radius();
  const tones = Array.from({ length: octaves }, (_, b) => [255, 255, 255, Math.round(255 - (255 - FLOOR) * (b / (octaves - 1)))]);
  for (let i = 0; i < count; i++) pen.disc(cx + data[i * STRIDE] * R, cy - data[i * STRIDE + 1] * R, data[i * STRIDE + 2] * R * SHRINK, i < HALVES ? paint.light : tones[bands[i]]);
  pen.ring(cx, cy, R, size * RIM, paint.white);
}
