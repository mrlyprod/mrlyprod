import * as math from "mrlyjs/math";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { math };

const CODE = 495;
const NUMBER = 3;
const LEVEL = 2;
const BASE = 3;
const STEPS = 96;
const WHEEL = 256;
const MARGIN = 0.08;
const TURN = 0.35;
const GHOSTS: [number, number][] = [[0.3, 0.3], [0.15, 0.55]];
const GAP = 0.08;
const SHADE = 0.45;
const WANT = { fills: 64 };

type Plan = { side: number; types: ArrayLike<number>; wheel: Float32Array };

let plan: Plan | null = null;

function study(): Plan {
  if (plan) return plan;
  const cell = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  const side = cell.shape[0];
  const fills = math.two.fills(cell);
  if (side !== NUMBER ** LEVEL || fills !== WANT.fills) throw new Error(`app-spin: the carpet of ${NUMBER} at level ${LEVEL} is ${side} a side with ${fills} fills, want ${NUMBER ** LEVEL} and ${WANT.fills}`);
  const profile = math.spin.profile(Float32Array.from(cell.types), side, STEPS);
  const mass = math.spin.mass(profile, side);
  if (Math.abs(mass - fills) > 0.5) throw new Error(`app-spin: the rings carry a mass of ${mass.toFixed(2)}, the design ${fills} fills`);
  plan = { side, types: cell.types, wheel: math.spin.wheel(profile, WHEEL) };
  return plan;
}

function stamp(pen: Pen, cx: number, cy: number, span: number, turn: number, color: Ink["blue"]) {
  const { side, types } = study();
  const unit = span / side;
  const inset = unit * GAP;
  const c = Math.cos(turn);
  const s = Math.sin(turn);
  const at = (u: number, v: number): Point => [cx + u * c - v * s, cy + u * s + v * c];
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

export default function draw(pen: Pen, ink: Ink) {
  const { wheel } = study();
  const box = pen.frame(MARGIN);
  const [cx, cy] = box.center();
  const r = box.radius();
  const [br, bg, bb] = ink.blue;
  const colors = new Uint8Array(wheel.length * 4);
  for (let i = 0; i < wheel.length; i++) {
    colors[i * 4] = br;
    colors[i * 4 + 1] = bg;
    colors[i * 4 + 2] = bb;
    colors[i * 4 + 3] = Math.round(255 * SHADE * wheel[i]);
  }
  pen.image(cx - r, cy - r, 2 * r, 2 * r, { shape: [WHEEL, WHEEL], colors });
  const span = r * Math.SQRT2;
  for (const [lag, tone] of GHOSTS) stamp(pen, cx, cy, span, TURN - lag, ink.mix(ink.ground, ink.blue, tone));
  stamp(pen, cx, cy, span, TURN, ink.fg);
  pen.ring(cx, cy, r, 3, ink.dim);
}
