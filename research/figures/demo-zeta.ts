import * as num from "mrlyjs/num";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { num };

const REACH = 50;
const STEPS = 8000;

// PATH

let memo: Point[] | undefined;

function path() {
  if (memo) return memo;
  const line = new num.zeta.Line();
  const zeros = line.zeros(10);
  if (line.count(REACH) !== 10) throw new Error(`demo-zeta: ${line.count(REACH)} zeros below ${REACH}, want 10`);
  if (!(Math.abs(zeros[0] - 14.134725) < 1e-5)) throw new Error(`demo-zeta: first zero ${zeros[0]}, want 14.134725`);
  if (!(zeros[zeros.length - 1] < REACH)) throw new Error(`demo-zeta: last zero ${zeros[zeros.length - 1]}, want below ${REACH}`);
  const points: Point[] = [];
  for (let step = 0; step <= STEPS; step++) {
    const [value] = line.point((REACH * step) / STEPS);
    points.push([value.re, value.im]);
    value.free();
  }
  line.free();
  if (points.length !== STEPS + 1) throw new Error(`demo-zeta: ${points.length} points, want ${STEPS + 1}`);
  memo = points;
  return memo;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const points = path();
  const frame = pen.frame(0.08);
  const left = Math.min(0, ...points.map((p) => p[0]));
  const right = Math.max(0, ...points.map((p) => p[0]));
  const under = Math.min(0, ...points.map((p) => p[1]));
  const over = Math.max(0, ...points.map((p) => p[1]));
  const unit = Math.min(frame.w, frame.h) / Math.max(right - left, over - under);
  const [midX, midY] = frame.center();
  const cx = midX - 0.5 * (left + right) * unit;
  const cy = midY + 0.5 * (under + over) * unit;
  pen.segment([frame.x, cy], [frame.x + frame.w, cy], 2, ink.line);
  pen.segment([cx, frame.y], [cx, frame.y + frame.h], 2, ink.line);
  pen.ring(cx, cy, unit, 2, ink.line);
  pen.polyline(points.map(([re, im]): Point => [cx + re * unit, cy - im * unit]), 2.5, ink.blue);
  pen.ring(cx, cy, unit * 0.07, 7, ink.yellow);
}
