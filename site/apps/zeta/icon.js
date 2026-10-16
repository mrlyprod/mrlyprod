import * as num from "@mrlyprod/mrly/num";
import { plate } from "../../lib/icon.js";

export const units = { num };

const SPAN = 60;
const PER = 40;
const WANT = 13;
const INSET = 0.16;
const THIN = 0.009;
const THICK = 0.034;
const RING = 0.022;
const HOLE = 0.055;
const GHOST = 120;

let plan = null;

function study() {
  if (plan) return plan;
  const line = new num.zeta.Line();
  const n = SPAN * PER + 1;
  const xs = new Float64Array(n);
  const ys = new Float64Array(n);
  const zeros = [];
  let last = 0;
  for (let i = 0; i < n; i++) {
    const [p, z] = line.point(i / PER);
    const x = p.re;
    const y = p.im;
    xs[i] = x;
    ys[i] = y;
    if (i && Math.sign(z) !== Math.sign(last)) zeros.push(i);
    last = z;
  }
  const counted = line.count(SPAN);
  line.free();
  if (zeros.length !== WANT || counted !== WANT) throw new Error(`zeta: the walk to ${SPAN} crosses the origin ${zeros.length} times and the line counts ${counted}, want ${WANT}`);
  const first = zeros[0];
  const end = zeros[zeros.length - 1];
  const frame = [Infinity, Infinity, -Infinity, -Infinity];
  let loop = [first, first];
  let reach = 0;
  for (let j = 1; j < zeros.length; j++) {
    for (let i = zeros[j - 1]; i <= zeros[j]; i++) {
      frame[0] = Math.min(frame[0], xs[i]);
      frame[1] = Math.min(frame[1], ys[i]);
      frame[2] = Math.max(frame[2], xs[i]);
      frame[3] = Math.max(frame[3], ys[i]);
      const r = Math.hypot(xs[i], ys[i]);
      if (r > reach) [reach, loop] = [r, [zeros[j - 1], zeros[j]]];
    }
  }
  plan = { xs, ys, frame, walk: [first, end], loop };
  return plan;
}

export default function draw(pen) {
  const { box, paint, size } = plate(pen, "cyan", INSET);
  const { xs, ys, frame, walk, loop } = study();
  const [x0, y0, x1, y1] = frame;
  const k = Math.min(box.w / (x1 - x0), box.h / (y1 - y0));
  const ox = box.x + box.w / 2 - ((x0 + x1) / 2) * k;
  const oy = box.y + box.h / 2 + ((y0 + y1) / 2) * k;
  const at = (i) => [ox + xs[i] * k, oy - ys[i] * k];
  const path = ([from, to]) => Array.from({ length: to - from + 1 }, (_, i) => at(from + i));
  pen.polyline(path(walk), size * THIN, [255, 255, 255, GHOST]);
  pen.polyline(path(loop), size * THICK, paint.white);
  pen.ring(ox, oy, size * HOLE, size * RING, paint.light);
}
