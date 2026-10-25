import * as math from "@mrlyprod/mrly/math";
import type { Pen } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = { math };

/* TYPES */

type Plan = { stack: Float32Array; top: number };

const CODE = 495;
const NUMBER = 3;
const LEVEL = 2;
const BASE = 3;
const COPIES = 6;
const OUT = 256;
const SAMPLES = 2;
const WANT = { fills: 64, order: 4, petals: 12 };

let plan: Plan | null = null;

function study(): Plan {
  if (plan) return plan;
  const cell = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  const side = cell.shape[0];
  const data = Float32Array.from(cell.types);
  const fills = math.two.fills(cell);
  const order = math.spin.turns(math.spin.harmonics(data, side, 64, 16));
  const petals = math.spin.petals(COPIES, order);
  if (fills !== WANT.fills || order !== WANT.order || petals !== WANT.petals)
    throw new Error(
      `radial: the carpet of ${NUMBER} at level ${LEVEL} reads ${fills} fills, order ${order} and ${petals} petals over ${COPIES} copies, want ${WANT.fills}, ${WANT.order} and ${WANT.petals}`,
    );
  const stack = math.spin.radial(data, side, OUT, COPIES, 1 / COPIES, math.spin.Blend.named("mean")!, SAMPLES);
  let top = 0;
  for (const v of stack) top = Math.max(top, v);
  plan = { stack, top };
  return plan;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "mint", 0.16);
  const { stack, top } = study();
  const [cx, cy] = box.center();
  const r = box.radius();
  const colors = new Uint8Array(stack.length * 4);
  for (let i = 0; i < stack.length; i++) {
    const t = stack[i] / top;
    colors[i * 4] = 255;
    colors[i * 4 + 1] = 255;
    colors[i * 4 + 2] = 255;
    colors[i * 4 + 3] = Math.round(255 * t ** 1.4);
  }
  pen.disc(cx, cy, r, paint.deep);
  pen.image(cx - r, cy - r, 2 * r, 2 * r, { shape: [OUT, OUT], colors });
  pen.ring(cx, cy, r, 4, paint.light);
}
