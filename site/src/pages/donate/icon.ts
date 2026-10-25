import type { Pen, Point } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = {};

const LOBE = 0.27;
const DROP = 0.62;
const LIFT = 0.1;
const GLINT = 0.11;

function tangent(center: Point, tip: Point, r: number, side: number): Point {
  const dx = tip[0] - center[0];
  const dy = tip[1] - center[1];
  const d = Math.hypot(dx, dy);
  if (d <= r)
    throw new Error(`donate: the tip sits inside a lobe, ${d.toFixed(1)} from its centre at radius ${r.toFixed(1)}`);
  const turn = Math.atan2(dy, dx) + side * Math.acos(r / d);
  return [center[0] + r * Math.cos(turn), center[1] + r * Math.sin(turn)];
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "red", 0.18);
  const [cx, cy] = box.center();
  const r = box.w * LOBE;
  const ty = cy - box.h * LIFT;
  const left: Point = [cx - r, ty];
  const right: Point = [cx + r, ty];
  const tip: Point = [cx, ty + box.h * DROP];
  const a = tangent(left, tip, r, 1);
  const b = tangent(right, tip, r, -1);
  pen.disc(left[0], left[1], r, paint.white);
  pen.disc(right[0], right[1], r, paint.white);
  pen.polygon([a, tip, b, [cx, ty]], paint.white);
  pen.arc([left[0], left[1]], r * 0.62, [Math.PI * 1.05, Math.PI * 1.55], box.w * GLINT, paint.light);
}
