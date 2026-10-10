import * as num from "mrlyjs/num";
import { plot } from "mrlyjs/view";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { num };

const TOP = 30;
const SAMPLES = 1600;

let memo: number[] | undefined;

function sums() {
  if (memo) return memo;
  memo = Array.from({ length: TOP + 1 }, (_, n) => num.series.harmonic(n));
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const harmonic = sums();
  const frame = pen.frame(0.08);
  const left = 1;
  const right = TOP + 1;
  const peak = harmonic[TOP];
  const across = (x: number) => frame.x + (frame.w * (x - left)) / (right - left);
  const up = (v: number) => frame.y + frame.h * (1 - v / peak);

  const strip = ink.fade(ink.blue, 0.28);
  const width = frame.w / SAMPLES;
  for (let k = 0; k < SAMPLES; k++) {
    const x = left + ((right - left) * (k + 0.5)) / SAMPLES;
    const crest = up(harmonic[Math.floor(x)]);
    const foot = up(Math.log(x));
    pen.rect(across(x) - width, crest, width * 2, foot - crest, strip);
  }

  const curve: Point[] = [];
  for (let k = 0; k <= SAMPLES; k++) {
    const x = left + ((right - left) * k) / SAMPLES;
    curve.push([across(x), up(Math.log(x))]);
  }
  pen.polyline(curve, 3.4, ink.blue);

  const stair: Point[] = [];
  for (let n = 1; n <= TOP; n++) {
    const v = harmonic[n];
    stair.push([across(n), up(v)]);
    stair.push([across(n + 1), up(v)]);
  }
  pen.polyline(stair, 3.4, ink.yellow);

  plot.axis(pen, frame, ink.line);
  if (stair.length !== 2 * TOP) throw new Error(`wiki-euler-mascheroni-constant: ${stair.length} stair points, want ${2 * TOP}`);
  if (!(Math.abs(peak - Math.log(TOP) - num.series.EULER()) < 0.02)) throw new Error("wiki-euler-mascheroni-constant: the peak is not within 0.02 of ln 30 plus gamma");
}
