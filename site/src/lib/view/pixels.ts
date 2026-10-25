import type { Pen } from "./frame.ts";
import { type Color, clamp } from "./ink.ts";

/* TYPES */

export type Pixels = {
  shape: [number, number];
  colors: Uint8Array<ArrayBuffer> | Uint8ClampedArray<ArrayBuffer>;
  cover?: ArrayLike<number>;
};

export type Patch = Pixels & {
  colors: Uint8ClampedArray<ArrayBuffer>;
  cover: Float64Array;
  blend(px: number, py: number, color: Color, cover?: number): void;
  paint(pen: Pen): void;
};

/* COVER */

export function fold(pixels: Pixels): Uint8ClampedArray<ArrayBuffer> {
  const [h, w] = pixels.shape;
  const n = w * h;
  const out = new Uint8ClampedArray(n * 4);
  out.set(pixels.colors.subarray(0, n * 4));
  const cover = pixels.cover;
  if (cover) for (let i = 0; i < n; i++) out[i * 4 + 3] = Math.round(out[i * 4 + 3] * clamp(cover[i]));
  return out;
}

export function patch(x: number, y: number, w: number, h: number): Patch {
  if (![x, y, w, h].every(Number.isInteger) || w < 0 || h < 0)
    throw new Error(`patch: ${x}, ${y}, ${w}, ${h} is not a box of whole pixels`);
  const colors = new Uint8ClampedArray(w * h * 4);
  const cover = new Float64Array(w * h);
  return {
    shape: [h, w],
    colors,
    cover,
    blend(px, py, color, amount = 1) {
      const col = px - x;
      const row = py - y;
      if (col < 0 || row < 0 || col >= w || row >= h) return;
      colors.set(color, (row * w + col) * 4);
      cover[row * w + col] = amount;
    },
    paint(pen) {
      pen.image(x, y, w, h, this);
    },
  };
}
