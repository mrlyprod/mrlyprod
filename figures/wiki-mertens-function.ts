import * as num from "mrlyjs/num";
import { plot } from "mrlyjs/view";
import type { Ink, Pen, Point } from "mrlyjs/view";
import census from "./census/wiki-mertens-function.json" with { type: "json" };

const TOP = 1000;
const LAST = 2;
const LOW = -12;

export const units = { num };

let memo: number[] | undefined;

function walk(bound: number[]) {
  if (memo) return memo;
  const sums = Array.from({ length: TOP }, (_, i) => Number(num.series.mertens(i + 1)));
  if (sums[TOP - 1] !== LAST) throw new Error(`wiki-mertens-function: M(${TOP}) is ${sums[TOP - 1]}, want ${LAST}`);
  if (Math.min(...sums) !== LOW) throw new Error(`wiki-mertens-function: the walk bottoms at ${Math.min(...sums)}, want ${LOW}`);
  if (sums.some((m, i) => Math.abs(m) > bound[i])) throw new Error("wiki-mertens-function: the walk leaves the square root trumpet");
  memo = sums;
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { bound } = census as { bound: number[] };
  if (bound.length !== TOP) throw new Error(`wiki-mertens-function: ${bound.length} bounds, want ${TOP}`);
  const sums = walk(bound);
  const frame = pen.frame(0.08);
  const reach = bound[TOP - 1] * 1.1;
  const across = (n: number) => frame.x + (frame.w * (n - 1)) / (TOP - 1);
  const up = (v: number) => frame.y + frame.h * (1 - (v + reach) / (2 * reach));

  const over: Point[] = [];
  const under: Point[] = [];
  for (let n = 1; n <= TOP; n++) {
    over.push([across(n), up(bound[n - 1])]);
    under.push([across(n), up(-bound[n - 1])]);
  }
  pen.polyline(over, 2.6, ink.dim);
  pen.polyline(under, 2.6, ink.dim);
  pen.segment([frame.x, up(0)], [frame.x + frame.w, up(0)], 1.6, ink.line);

  const steps: Point[] = [];
  sums.forEach((m, i) => {
    const n = i + 1;
    steps.push([across(n), up(m)], [across(n + 1), up(m)]);
  });
  pen.polyline(steps, 2.6, ink.blue);

  plot.axis(pen, frame, ink.line);
}
