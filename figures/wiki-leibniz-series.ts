import * as num from "mrlyjs/num";
import { plot, type Ink, type Pen, type Point } from "mrlyjs/view";

export const units = { num };

const TERMS = 30;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const limit = Math.PI / 4;
  const walk = Array.from({ length: TERMS }, (_, k) => num.series.leibniz(k + 1));
  const lo = Math.min(...walk) - 0.05;
  const hi = Math.max(...walk) + 0.05;
  const slot = frame.w / TERMS;
  const up = (v: number) => frame.y + frame.h * (1 - (v - lo) / (hi - lo));
  const pts: Point[] = walk.map((value, i) => [frame.x + (i + 0.5) * slot, up(value)]);
  plot.axis(pen, frame, ink.line);
  plot.dashed(pen, frame, up(limit), 2.5, ink.dim);
  pen.polyline(pts, 2, ink.line);
  let over = 0;
  let under = 0;
  pts.forEach(([x, y], i) => {
    const high = walk[i] > limit;
    pen.disc(x, y, slot * 0.34, high ? ink.blue : ink.orange);
    if (high) over++;
    else under++;
  });
  if (over !== 15) throw new Error(`wiki-leibniz-series: ${over} partials over the limit, want 15`);
  if (under !== 15) throw new Error(`wiki-leibniz-series: ${under} partials under the limit, want 15`);
}
