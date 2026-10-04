import * as num from "mrlyjs/num";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { num };

const REACH = 8;
const POINTS = 217;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.07);
  const ring = "Eisenstein";
  const window = new num.gauss.Window(ring, REACH);
  const points = window.points();
  if (points.length !== POINTS) throw new Error(`wiki-eisenstein-integers: ${points.length} points, want ${POINTS}`);
  const associates = num.gauss.Ring.associates(ring, 1, 0);
  if (associates.length !== 6) throw new Error(`wiki-eisenstein-integers: ${associates.length} units, want 6`);
  const scale = frame.w / (2 * REACH + 1);
  const [cx, cy] = frame.center();
  const place = (a: bigint, b: bigint): Point => {
    const [x, y] = num.gauss.Ring.place(ring, a, b);
    return [cx + x * scale, cy - y * scale];
  };
  for (const [a, b] of points) {
    for (const [da, db] of associates.slice(0, 3)) {
      if (window.holds(a + da, b + db)) pen.segment(place(a, b), place(a + da, b + db), 1.2, ink.line);
    }
  }
  window.free();
  for (const [a, b] of points) {
    const [x, y] = place(a, b);
    pen.disc(x, y, scale * 0.16, ink.blue);
  }
  for (const [a, b] of associates) {
    const [x, y] = place(a, b);
    pen.disc(x, y, scale * 0.3, ink.yellow);
  }
  const [x, y] = place(0n, 0n);
  pen.disc(x, y, scale * 0.34, ink.orange);
}
