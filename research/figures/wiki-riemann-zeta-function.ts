import * as num from "mrlyjs/num";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { num };

const TERMS = 40;
const POWERS = [2, 3, 4];
const FLOOR = 0.94;
const ROOF = 1.72;
const TOLERANCE = [0.026, 0.0004, 0.00002];

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.09);
  const place = (n: number, value: number): Point => [frame.x + (frame.w * (n - 1)) / (TERMS - 1), frame.y + frame.h * (1 - (value - FLOOR) / (ROOF - FLOOR))];
  const limits = POWERS.map((s) => num.lattice.zeta_whole(s));
  for (const limit of limits) {
    const [, y] = place(1, limit);
    pen.segment([frame.x, y], [frame.x + frame.w, y], 1.5, ink.line);
  }
  const inks = [ink.blue, ink.orange, ink.green];
  const ends: number[] = [];
  POWERS.forEach((s, k) => {
    const path: Point[] = [];
    let running = 0;
    for (let n = 1; n <= TERMS; n++) {
      running = num.series.dirichlet(s, [1], n);
      path.push(place(n, running));
    }
    if (path.length !== TERMS) throw new Error(`wiki-riemann-zeta-function: ${path.length} points, want ${TERMS}`);
    pen.polyline(path, 4, inks[k]);
    pen.disc(path[TERMS - 1][0], path[TERMS - 1][1], 9, inks[k]);
    ends.push(running);
  });
  if (ends.length !== POWERS.length) throw new Error(`wiki-riemann-zeta-function: ${ends.length} ends, want ${POWERS.length}`);
  ends.forEach((end, k) => {
    if (!(Math.abs(end - limits[k]) < TOLERANCE[k])) throw new Error(`wiki-riemann-zeta-function: power ${POWERS[k]} ends at ${end}, limit ${limits[k]}`);
  });
}
