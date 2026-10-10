import * as num from "mrlyjs/num";
import { plot, type Ink, type Pen, type Point } from "mrlyjs/view";

export const units = { num };

const TERMS = 60;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const limit = Math.E;
  const walk = Array.from({ length: TERMS }, (_, i) => num.series.e_partial(i + 1));
  const lo = walk[0] - 0.07;
  const hi = limit + 0.05;
  const slot = frame.w / TERMS;
  const up = (v: number) => frame.y + frame.h * (1 - (v - lo) / (hi - lo));
  const pts: Point[] = walk.map((value, i) => [frame.x + (i + 0.5) * slot, up(value)]);
  plot.axis(pen, frame, ink.line);
  plot.dashed(pen, frame, up(limit), 2.5, ink.dim);
  pen.polyline(pts, 3, ink.blue);
  plot.dots(pen, pts, slot * 0.22, ink.yellow);
  if (walk.filter((value) => value < limit).length !== TERMS) throw new Error("wiki-eulers-number: a partial reaches the limit");
}
