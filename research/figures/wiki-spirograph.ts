import * as math from "mrlyjs/math";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { math };

const RING = 7;
const WHEEL = 3;
const REACH = 0.8;
const SAMPLES = 4000;

let memo: { curve: Point[]; box: Float64Array } | undefined;

function roulette() {
  if (memo) return memo;
  const path = math.spirograph.track("in", RING, WHEEL, 4, 1);
  if (path.ratio[0] !== RING || path.ratio[1] !== WHEEL || path.orbits !== 3 || path.fold !== 7) throw new Error(`wiki-spirograph: ratio ${path.ratio}, ${path.orbits} orbits, fold ${path.fold}, want ${RING},${WHEEL}, 3, 7`);
  const pencil: math.spirograph.Pencil = { x: REACH, y: 0, seat: [0, 0], kind: "Fill" };
  const curve: Point[] = [];
  for (let k = 0; k < SAMPLES; k++) curve.push(math.spirograph.point(path, pencil, (path.total * k) / (SAMPLES - 1)));
  if (curve.length !== SAMPLES) throw new Error(`wiki-spirograph: ${curve.length} samples, want ${SAMPLES}`);
  memo = { curve, box: math.spirograph.frame(path, [pencil]) };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { curve, box } = roulette();
  const frame = pen.frame(0.08);
  const [x0, y0, x1, y1] = box;
  const scale = Math.min(frame.w / (x1 - x0), frame.h / (y1 - y0));
  const [cx, cy] = frame.center();
  pen.ring(cx, cy, RING * scale, 1.5, ink.dim);
  pen.polyline(curve.map(([x, y]): Point => [cx + x * scale, cy - y * scale]), 2.4, ink.blue);
}
