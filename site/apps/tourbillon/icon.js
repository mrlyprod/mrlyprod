import * as core from "mrlyjs/core";
import * as math from "mrlyjs/math";
import { plate } from "../../lib/icon.js";

export const units = { core, math };

const TOP = 41;
const SIZE = 256;
const SPREAD = 8;
const FLOOR = 0.54;
const CEIL = 0.82;
const RIM = 0.016;
const WANT = { layers: 21, centre: 10 / 21 };

let held = null;

function spun() {
  if (held) return held;
  const data = math.tourbillon.field(TOP, SIZE, "golden", 0, "odd", "plain", "cells", "mean", 1);
  const read = math.tourbillon.stats(data, SIZE, TOP, "golden", 0, "odd", "plain", "mean", 1);
  if (read.layers !== WANT.layers || Math.abs(read.centre - WANT.centre) > 1e-6 || data.length !== SIZE * SIZE) throw new Error(`tourbillon: ${read.layers} layers, centre ${read.centre}, want ${WANT.layers} and ${WANT.centre}`);
  const span = read.high - read.low || 1;
  const types = Uint8Array.from(data, (v) => (Number.isNaN(v) ? 0 : Math.round((255 * (v - read.low)) / span)));
  const side = 2 * SPREAD + 1;
  const run = new Uint8Array(side).fill(1);
  const across = core.cell.blur({ shape: [SIZE, SIZE], types }, { shape: [1, side], data: run }, false);
  const soft = core.cell.blur(across, { shape: [side, 1], data: run }, false).types;
  let low = 255;
  let high = 0;
  for (let i = 0; i < data.length; i++) {
    if (Number.isNaN(data[i])) continue;
    low = Math.min(low, soft[i]);
    high = Math.max(high, soft[i]);
  }
  const colors = new Uint8ClampedArray(SIZE * SIZE * 4);
  for (let i = 0; i < data.length; i++) {
    if (Number.isNaN(data[i])) continue;
    const t = Math.min(1, Math.max(0, ((soft[i] - low) / (high - low || 1) - FLOOR) / (CEIL - FLOOR)));
    colors.set([255, 255, 255, Math.round(255 * t * t * (3 - 2 * t))], i * 4);
  }
  held = { shape: [SIZE, SIZE], colors };
  return held;
}

export default function draw(pen) {
  const { box, paint, size } = plate(pen, "cyan", 0.17);
  const [cx, cy] = box.center();
  const r = box.radius();
  pen.disc(cx, cy, r, [...paint.deep.slice(0, 3), 110]);
  pen.image(cx - r, cy - r, 2 * r, 2 * r, spun());
  pen.ring(cx, cy, r, size * RIM, paint.light);
}
