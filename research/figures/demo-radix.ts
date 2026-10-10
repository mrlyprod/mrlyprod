import * as num from "mrlyjs/num";
import { plot, type Frame, type Ink, type Pen, type Point } from "mrlyjs/view";

export const units = { num };

const LEVEL = 6;
const WORDS = 117649;
const DIGITS = 7;
const NORM = 7n;
const MARGIN = 0.08;
const DOT = 1.2;

let memo: Point[] | undefined;

// CLOUD

function cloud() {
  if (memo) return memo;
  const design = num.radix.flowsnake();
  if (design.size() !== DIGITS) throw new Error(`demo-radix: ${design.size()} digits, want ${DIGITS}`);
  if (design.base().norm() !== NORM) throw new Error(`demo-radix: base norm ${design.base().norm()}, want ${NORM}`);
  if (BigInt(design.fill(LEVEL)) !== BigInt(WORDS)) throw new Error(`demo-radix: ${design.fill(LEVEL)} words, want ${WORDS}`);
  if (design.distinct(LEVEL) !== WORDS) throw new Error(`demo-radix: ${design.distinct(LEVEL)} distinct points, want ${WORDS}`);
  const points = design.plane(LEVEL);
  if (points.length !== WORDS) throw new Error(`demo-radix: ${points.length} points, want ${WORDS}`);
  memo = points;
  return memo;
}

// THE LAYOUT

function fit(pts: Point[], frame: Frame): Point[] {
  let lowX = Number.MAX_VALUE;
  let lowY = Number.MAX_VALUE;
  let highX = -Number.MAX_VALUE;
  let highY = -Number.MAX_VALUE;
  for (const [x, y] of pts) {
    lowX = Math.min(lowX, x);
    lowY = Math.min(lowY, y);
    highX = Math.max(highX, x);
    highY = Math.max(highY, y);
  }
  const scale = Math.min(frame.w / (highX - lowX), frame.h / (highY - lowY));
  const midX = (lowX + highX) / 2;
  const midY = (lowY + highY) / 2;
  const [cx, cy] = frame.center();
  return pts.map(([x, y]) => [cx + (x - midX) * scale, cy - (y - midY) * scale]);
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const laid = fit(cloud(), pen.frame(MARGIN));
  plot.dots(pen, laid, DOT, ink.blue);
}
