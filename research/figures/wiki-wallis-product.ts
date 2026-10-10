import * as num from "mrlyjs/num";
import { plot } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const TERMS = 40;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const limit = Math.PI / 2;
  const walk = Array.from({ length: TERMS }, (_, k) => num.series.wallis_half_pi(k + 1));
  const lo = walk[0] - 0.06;
  const hi = limit + 0.04;
  const foot = frame.y + frame.h;
  const up = (v: number) => foot - (frame.h * (v - lo)) / (hi - lo);
  const slot = frame.w / TERMS;
  const pad = slot * 0.16;
  walk.forEach((value, i) => {
    const y = up(value);
    pen.rect(frame.x + i * slot + pad, y, slot - 2 * pad, foot - y, ink.blue);
  });
  plot.dashed(pen, frame, up(limit), 2.5, ink.dim);
  plot.axis(pen, frame, ink.line);
  if (walk.filter((value) => value < limit).length !== TERMS) throw new Error("wiki-wallis-product: a partial product reached the limit");
}
