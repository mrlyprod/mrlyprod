import * as math from "@mrlyprod/mrly/math";
import { iso } from "#view";
import { plate } from "../../lib/icon.js";

export const units = { math };

const CODE = 495;
const BASE = 3;
const DIM = 2;
const LEVEL = 1;
const SCALES = [27, 9, 3];
const WANT = [648, 72, 8];
const RISE = 0.3;
const SEAM = 0.6;
const THICK = 0.04;

let held = null;

function runs(mask, side) {
  const out = [];
  let open = new Map();
  for (let row = 0; row <= side; row++) {
    const next = new Map();
    for (let col = 0; row < side && col < side; col++) {
      if (!mask[row * side + col]) continue;
      const from = col;
      while (col + 1 < side && mask[row * side + col + 1]) col++;
      const key = `${from}:${col + 1}`;
      next.set(key, open.get(key) ?? { top: row, from, to: col + 1 });
    }
    for (const [key, run] of open) if (!next.has(key)) out.push({ ...run, bottom: row });
    open = next;
  }
  return out;
}

function layers() {
  if (held) return held;
  const spec = math.moire.Spec.new(CODE, BASE, DIM);
  held = SCALES.map((number, i) => {
    const side = number * BASE ** (LEVEL - 1);
    const mask = math.moire.layer({ spec, number, level: LEVEL, lattice: "Square", size: side, slices: [] });
    const fills = mask.reduce((n, on) => n + (on ? 1 : 0), 0);
    if (mask.length !== side * side || fills !== WANT[i]) throw new Error(`moire: the carpet at scale ${number} lights ${fills} of ${mask.length} cells, want ${WANT[i]} of ${side * side}`);
    return { side, runs: runs(mask, side) };
  });
  return held;
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "purple", 0.16);
  const tones = [paint.deep, paint.light, paint.white];
  const edges = [paint.ink, paint.deep, paint.light];
  const top = RISE * (SCALES.length - 1);
  const [wide] = iso.project(1, 0, 0);
  const scale = Math.min(box.w / (2 * wide), box.h / (1 + top));
  const [cx, cy] = box.center();
  const oy = cy - (scale * (1 - top)) / 2;
  const at = (x, y, z) => {
    const [u, v] = iso.project(x, y, z);
    return [cx + u * scale, oy + v * scale];
  };
  const pad = SEAM / scale;
  const sheet = ({ side, runs: list }, z, color) => {
    for (const { top: r0, bottom: r1, from, to } of list) {
      const x0 = from / side - pad;
      const x1 = to / side + pad;
      const y0 = r0 / side - pad;
      const y1 = r1 / side + pad;
      pen.polygon([at(x0, y0, z), at(x1, y0, z), at(x1, y1, z), at(x0, y1, z)], color);
    }
  };
  layers().forEach((layer, i) => {
    sheet(layer, RISE * i - THICK, edges[i]);
    sheet(layer, RISE * i, tones[i]);
  });
}
