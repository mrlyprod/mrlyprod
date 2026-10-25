import * as num from "@mrlyprod/mrly/num";
import type { Pen } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = { num };

const N = 2;
const M = 5;
const SIGN = -1;
const SIDE = 64;
const ROWS = 72;
const STEPS = 160;
const CHUNK = 64;
const SEED = 7;
const DOTS = 48;
const GAIN = 40;

let sand: Uint8ClampedArray<ArrayBuffer> | null = null;

function settle(): Uint8ClampedArray<ArrayBuffer> {
  if (sand) return sand;
  const field = num.chladni.plate(N, M, SIGN, SIDE);
  let grains: Float32Array = new Float32Array(2 * ROWS * ROWS);
  for (let i = 0; i < ROWS * ROWS; i++) {
    grains[2 * i] = ((i % ROWS) + 0.5) / ROWS;
    grains[2 * i + 1] = (Math.floor(i / ROWS) + 0.5) / ROWS;
  }
  for (let k = 0; k < STEPS; k += CHUNK)
    grains = num.chladni.shake(field, SIDE, grains, k, Math.min(CHUNK, STEPS - k), SEED);
  const counts = new Uint16Array(DOTS * DOTS);
  for (let i = 0; i < grains.length; i += 2) {
    if (Number.isNaN(grains[i])) continue;
    counts[
      Math.min(DOTS - 1, Math.floor(grains[i + 1] * DOTS)) * DOTS + Math.min(DOTS - 1, Math.floor(grains[i] * DOTS))
    ]++;
  }
  sand = new Uint8ClampedArray(DOTS * DOTS * 4);
  counts.forEach((n, i) => {
    sand!.set([255, 255, 255, n * GAIN], 4 * i);
  });
  return sand;
}

export default function draw(pen: Pen) {
  const { box } = plate(pen, "brown", 0.18);
  pen.image(box.x, box.y, box.w, box.h, { shape: [DOTS, DOTS], colors: settle() });
}
