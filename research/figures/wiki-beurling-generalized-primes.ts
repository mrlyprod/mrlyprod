import * as num from "mrlyjs/num";
import { frame, plot, type Color, type Frame, type Ink, type Pen, type Point } from "mrlyjs/view";
import census from "./census/wiki-beurling-generalized-primes.json" with { type: "json" };

const TOP = 100_000;
const SAMPLES = 1000;
const PRIMES = 9592;

export const units = { num };

let memo: number[] | undefined;

function counted(xs: number[]) {
  if (memo) return memo;
  const counts = xs.map((x) => num.prime.prime_count(x));
  if (counts[SAMPLES] !== PRIMES) throw new Error(`wiki-beurling-generalized-primes: ${counts[SAMPLES]} primes up to ${TOP}, want ${PRIMES}`);
  const listed = num.prime.primes(TOP).length;
  if (listed !== PRIMES) throw new Error(`wiki-beurling-generalized-primes: ${listed} primes listed, want ${PRIMES}`);
  memo = counts;
  return memo;
}

function trace(pen: Pen, box: Frame, xs: number[], counts: number[], peak: number, color: Color) {
  const pts: Point[] = xs.map((x, k) => [box.x + (box.w * x) / TOP, box.y + box.h * (1 - counts[k] / peak)]);
  pen.polyline(pts, 3.4, color);
}

export default function draw(pen: Pen, ink: Ink) {
  const { toy_integers, toy_primes } = census as { toy_integers: number[]; toy_primes: number[] };
  if (toy_integers.length !== SAMPLES + 1) throw new Error(`wiki-beurling-generalized-primes: ${toy_integers.length} integer counts, want ${SAMPLES + 1}`);
  if (toy_primes.length !== SAMPLES + 1) throw new Error(`wiki-beurling-generalized-primes: ${toy_primes.length} prime counts, want ${SAMPLES + 1}`);
  const xs = Array.from({ length: SAMPLES + 1 }, (_, k) => Math.floor((TOP * k) / SAMPLES));
  const all_primes = counted(xs);
  const peak = all_primes[SAMPLES];
  const area = pen.frame(0.08);
  const gap = area.h * 0.08;
  const half = (area.h - gap) / 2;
  const upper = frame(area.x, area.y, area.w, half);
  const lower = frame(area.x, area.y + half + gap, area.w, half);
  trace(pen, upper, xs, xs, TOP, ink.blue);
  trace(pen, upper, xs, toy_integers, TOP, ink.yellow);
  trace(pen, lower, xs, all_primes, peak, ink.blue);
  trace(pen, lower, xs, toy_primes, peak, ink.yellow);
  plot.axis(pen, upper, ink.line);
  plot.axis(pen, lower, ink.line);
}
