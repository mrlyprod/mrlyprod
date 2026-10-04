import * as num from "mrlyjs/num";
import { plot } from "mrlyjs/view";
import type { Ink, Pen, Point } from "mrlyjs/view";
import census from "./census/wiki-prime-counting-function.json" with { type: "json" };

const TOP = 200;
const SAMPLES = 1200;
const COUNT = 46;

export const units = { num };

export default function draw(pen: Pen, ink: Ink) {
  const { guess } = census as { guess: number[] };
  if (guess.length !== SAMPLES + 1) throw new Error(`wiki-prime-counting-function: ${guess.length} estimates, want ${SAMPLES + 1}`);
  const frame = pen.frame(0.08);
  const left = 2;
  const right = TOP + 1;
  const roll = num.prime.primes(TOP);
  const counts = Array.from({ length: TOP - 1 }, (_, i) => num.prime.prime_count(i + 2));
  const peak = counts[TOP - 2];
  if (peak !== COUNT) throw new Error(`wiki-prime-counting-function: ${peak} primes up to ${TOP}, want ${COUNT}`);
  if (roll.length !== peak) throw new Error(`wiki-prime-counting-function: ${roll.length} primes listed, want ${peak}`);
  const across = (x: number) => frame.x + (frame.w * (x - left)) / (right - left);
  const up = (v: number) => frame.y + frame.h * (1 - v / peak);

  const hair = ink.fade(ink.dim, 0.3);
  for (const p of roll) {
    const x = across(p);
    pen.segment([x, frame.y], [x, frame.y + frame.h], 1.4, hair);
  }

  const curve: Point[] = guess.map((v, k) => [across(left + ((right - left) * k) / SAMPLES), up(v)]);
  pen.polyline(curve, 3.4, ink.blue);

  const stair: Point[] = [];
  counts.forEach((seen, i) => {
    const n = i + 2;
    stair.push([across(n), up(seen)], [across(n + 1), up(seen)]);
  });
  pen.polyline(stair, 3.4, ink.yellow);

  plot.axis(pen, frame, ink.line);
}
