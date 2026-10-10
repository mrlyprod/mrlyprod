import * as num from "mrlyjs/num";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { num };

const SPAN = 60;
const PER = 40;
const TRAIL = 9;
const MARGIN = 0.08;
const THIN = 4;
const THICK = 8;
const RING = 3;
const HEAD = 11;
const GHOST = 0.45;
const WANT = 13;

type Plan = { xs: Float64Array; ys: Float64Array; frame: number[]; zeros: number };

let plan: Plan | null = null;

function study(): Plan {
  if (plan) return plan;
  const line = new num.zeta.Line();
  const n = SPAN * PER + 1;
  const xs = new Float64Array(n);
  const ys = new Float64Array(n);
  const frame = [-1, -1, 1, 1];
  let zeros = 0;
  let last = 0;
  for (let i = 0; i < n; i++) {
    const [p, z] = line.point(i / PER);
    const x = p.re;
    const y = p.im;
    p.free();
    xs[i] = x;
    ys[i] = y;
    frame[0] = Math.min(frame[0], x);
    frame[1] = Math.min(frame[1], y);
    frame[2] = Math.max(frame[2], x);
    frame[3] = Math.max(frame[3], y);
    if (i && Math.sign(z) !== Math.sign(last)) zeros++;
    last = z;
  }
  const counted = line.count(SPAN);
  line.free();
  if (zeros !== WANT || counted !== WANT) throw new Error(`app-zeta: the walk to ${SPAN} crosses the origin ${zeros} times and the line counts ${counted}, want ${WANT}`);
  plan = { xs, ys, frame, zeros };
  return plan;
}

export default function draw(pen: Pen, ink: Ink) {
  const { xs, ys, frame } = study();
  const box = pen.frame(MARGIN);
  const [x0, y0, x1, y1] = frame;
  const k = Math.min(box.w / (x1 - x0), box.h / (y1 - y0));
  const ox = box.x + box.w / 2 - ((x0 + x1) / 2) * k;
  const oy = box.y + box.h / 2 + ((y0 + y1) / 2) * k;
  const at = (i: number): Point => [ox + xs[i] * k, oy - ys[i] * k];
  const n = xs.length;
  const kept = TRAIL * PER;
  pen.ring(ox, oy, k, RING, ink.line);
  pen.segment([box.x, oy], [box.x + box.w, oy], RING, ink.line);
  pen.segment([ox, box.y], [ox, box.y + box.h], RING, ink.line);
  pen.polyline(Array.from({ length: n }, (_, i) => at(i)), THIN, ink.mix(ink.ground, ink.blue, GHOST));
  pen.polyline(Array.from({ length: kept + 1 }, (_, i) => at(n - 1 - kept + i)), THICK, ink.blue);
  pen.ring(ox, oy, HEAD * 2, RING, ink.dim);
  pen.disc(...at(n - 1), HEAD, ink.fg);
}
