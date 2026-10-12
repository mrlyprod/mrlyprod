import * as math from "mrlyjs/math";
import { plate } from "../../lib/icon.js";

export const units = { math };

const CODE = 495;
const NUMBER = 3;
const LEVEL = 2;
const BASE = 3;
const STEPS = 96;
const WHEEL = 256;
const TURN = 0.35;
const GHOSTS = [[0.3, 0.3], [0.15, 0.55]];
const GAP = 0.08;
const WANT = { fills: 64 };

let plan = null;

function study() {
  if (plan) return plan;
  const cell = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  const side = cell.shape[0];
  const fills = math.two.fills(cell);
  if (side !== NUMBER ** LEVEL || fills !== WANT.fills) throw new Error(`spin: the carpet of ${NUMBER} at level ${LEVEL} is ${side} a side with ${fills} fills, want ${NUMBER ** LEVEL} and ${WANT.fills}`);
  const profile = math.spin.profile(Float32Array.from(cell.types), side, STEPS);
  const mass = math.spin.mass(profile, side);
  if (Math.abs(mass - fills) > 0.5) throw new Error(`spin: the rings carry a mass of ${mass.toFixed(2)}, the design ${fills} fills`);
  plan = { side, types: cell.types, wheel: math.spin.wheel(profile, WHEEL) };
  return plan;
}

function stamp(pen, cx, cy, span, turn, color) {
  const { side, types } = study();
  const unit = span / side;
  const inset = unit * GAP;
  const c = Math.cos(turn);
  const s = Math.sin(turn);
  const at = (u, v) => [cx + u * c - v * s, cy + u * s + v * c];
  for (let row = 0; row < side; row++) {
    for (let col = 0; col < side; col++) {
      if (!types[row * side + col]) continue;
      const u0 = (col - side / 2) * unit + inset;
      const v0 = (row - side / 2) * unit + inset;
      const u1 = u0 + unit - 2 * inset;
      const v1 = v0 + unit - 2 * inset;
      pen.polygon([at(u0, v0), at(u1, v0), at(u1, v1), at(u0, v1)], color);
    }
  }
}

export default function draw(pen) {
  const { box, paint } = plate(pen, "orange", 0.14);
  const { wheel } = study();
  const [cx, cy] = box.center();
  const r = box.radius();
  const colors = new Uint8Array(wheel.length * 4);
  for (let i = 0; i < wheel.length; i++) {
    colors[i * 4] = 255;
    colors[i * 4 + 1] = 255;
    colors[i * 4 + 2] = 255;
    colors[i * 4 + 3] = Math.round(110 * wheel[i]);
  }
  pen.image(cx - r, cy - r, 2 * r, 2 * r, { shape: [WHEEL, WHEEL], colors });
  const span = r * Math.SQRT2;
  for (const [lag, tone] of GHOSTS) stamp(pen, cx, cy, span, TURN - lag, [255, 255, 255, Math.round(255 * tone * 0.5)]);
  stamp(pen, cx, cy, span, TURN, paint.white);
  pen.ring(cx, cy, r, 4, paint.light);
}
