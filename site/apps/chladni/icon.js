import * as life from "mrlyjs/life";
import { plate } from "../../lib/icon.js";

export const units = { life };

const CODE = 7;
const LEVEL = 2;
const SIZE = 32;
const BRIGHT = 0.8;
const DIM = 0.6;

let glow = null;

function spectrum() {
  if (glow) return glow;
  const mask = life.design_mask(2, CODE, 3, LEVEL);
  const picture = life.chladni.kernel(mask, SIZE);
  const ring = life.chladni.profile(picture);
  const values = life.chladni.spectrum(picture);
  const top = Math.max(...values);
  const low = Math.min(...values);
  if (mask.shape[0] !== 3 ** LEVEL || top > 1 || top <= low || ring.peak_ring < 1) throw new Error(`chladni: the level-${LEVEL} carpet mask is ${mask.shape[0]} across with a spectrum from ${low} to ${top}, want ${3 ** LEVEL} and a span inside 1`);
  const order = Array.from(values.keys()).sort((a, b) => values[a] - values[b]);
  glow = new Float64Array(values.length);
  order.forEach((at, rank) => (glow[at] = rank / (order.length - 1)));
  return glow;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "mint", 0.15);
  const values = spectrum();
  const colors = new Uint8ClampedArray(SIZE * SIZE * 4);
  const mid = (SIZE - 1) / 2;
  for (let r = 0; r < SIZE; r++) {
    for (let c = 0; c < SIZE; c++) {
      if (Math.hypot(r - mid, c - mid) > SIZE / 2) continue;
      const v = values[r * SIZE + c];
      const tone = v >= BRIGHT ? paint.white : v >= DIM ? [255, 255, 255, 80] : null;
      if (tone) colors.set(tone, (r * SIZE + c) * 4);
    }
  }
  pen.image(box.x, box.y, box.w, box.h, { shape: [SIZE, SIZE], colors });
}
