import * as num from "mrlyjs/num";
import { frame as box, plot } from "mrlyjs/view";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const SQUARES = 12;
const TERMS = 40;

export default function draw(pen: Pen, ink: Ink) {
  const frame = pen.frame(0.08);
  const top = box(frame.x, frame.y, frame.w, frame.h * 0.38);
  const chart = box(frame.x, frame.y + frame.h * 0.46, frame.w, frame.h * 0.54);
  const reach = num.series.harmonic(SQUARES);
  const unit = top.w / reach;
  const base = top.y + top.h;
  let x = top.x;
  let laid = 0;
  for (let k = 1; k <= SQUARES; k++) {
    const side = unit / k;
    pen.rect(x, base - side, side - 2, side, ink.blue);
    x += side;
    laid++;
  }
  plot.baseline(pen, box(top.x, top.y, top.w, base - top.y), ink.line);
  const limit = num.series.BASEL();
  const walk: number[] = [];
  for (let k = 1; k <= TERMS; k++) walk.push(num.series.basel(k));
  const lo = walk[0] - 0.14;
  const hi = limit + 0.12;
  const foot = chart.y + chart.h;
  const up = (v: number) => foot - (chart.h * (v - lo)) / (hi - lo);
  const slot = chart.w / TERMS;
  const pad = slot * 0.16;
  walk.forEach((value, i) => {
    const y = up(value);
    pen.rect(chart.x + i * slot + pad, y, slot - 2 * pad, foot - y, ink.yellow);
  });
  plot.dashed(pen, chart, up(limit), 2.5, ink.dim);
  plot.axis(pen, chart, ink.line);
  if (laid !== SQUARES) throw new Error(`wiki-basel-problem: ${laid} squares laid, want ${SQUARES}`);
  if (walk.filter((value) => value < limit).length !== TERMS) throw new Error("wiki-basel-problem: a partial sum reached the limit");
}
