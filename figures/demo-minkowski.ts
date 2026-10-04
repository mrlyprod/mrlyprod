import * as math from "mrlyjs/math";
import type { Ink, Pen, Point } from "mrlyjs/view";

const PERIODS = 8;
const STEPS = 144;
const OPEN = 45;
const TWELFTH = [2.122718, 2.122723];
const SIXTH = [2.135019, 2.136794];

export const units = { math };

type Series = { us: number[]; waves: (number | undefined)[]; reads: (number | undefined)[]; once: number[]; start: number };

let memo: Series | undefined;

function series() {
  if (memo) return memo;
  const { profile, reading } = math.three.sponge;
  const cover = math.three.sponge.COVER();
  const edge = math.three.sponge.EDGE();
  const start = Math.log(1 / cover);
  const period = Math.log(3);
  const count = PERIODS * STEPS;
  const us = Array.from({ length: count }, (_, i) => start + (period * (i + 0.5)) / STEPS);
  const first = us.slice(0, STEPS).map((u) => profile(Math.exp(-u)));
  const waves = Array.from({ length: count }, (_, i) => first[i % STEPS]);
  const reads = us.map((u) => reading(Math.exp(-u)));
  const gaps = waves.filter((v) => v === undefined).length;
  if (gaps !== PERIODS * OPEN) throw new Error(`demo-minkowski: ${gaps} gaps, want ${PERIODS * OPEN}`);
  if (!waves.every((p, i) => (p === undefined) === (reads[i] === undefined))) throw new Error("demo-minkowski: the profile and the reading gap apart");
  if (!waves.every((p, i) => p === undefined || reads[i]! < p)) throw new Error("demo-minkowski: a reading is not below the profile");
  const twelfth = profile(1 / 12) ?? 0;
  const sixth = profile(edge) ?? 0;
  if (!(TWELFTH[0] <= twelfth && twelfth <= TWELFTH[1])) throw new Error(`demo-minkowski: profile at 1/12 is ${twelfth}`);
  if (!(SIXTH[0] <= sixth && sixth <= SIXTH[1])) throw new Error(`demo-minkowski: profile at 1/6 is ${sixth}`);
  const once = first.filter((v): v is number => v !== undefined);
  memo = { us, waves, reads, once, start };
  return memo;
}

function runs(values: (number | undefined)[], place: (i: number, v: number) => Point | undefined) {
  const out: Point[][] = [];
  let run: Point[] = [];
  values.forEach((value, i) => {
    const point = value === undefined ? undefined : place(i, value);
    if (point) run.push(point);
    else {
      if (run.length > 1) out.push(run);
      run = [];
    }
  });
  if (run.length > 1) out.push(run);
  return out;
}

export default function draw(pen: Pen, ink: Ink) {
  const { us, waves, reads, once, start } = series();
  const cover = math.three.sponge.COVER();
  const edge = math.three.sponge.EDGE();
  const period = Math.log(3);
  const low = Math.min(...once);
  const high = Math.max(...once);
  const spread = high - low;
  const floor = low - 1.6 * spread;
  const roof = high + 0.35 * spread;

  const plot = pen.frame(0.08);
  const u0 = start;
  const u1 = start + PERIODS * period;
  const xOf = (u: number) => plot.x + (plot.w * (u - u0)) / (u1 - u0);
  const yOf = (v: number) => plot.y + (plot.h * (roof - v)) / (roof - floor);
  const gap = Math.log(cover / edge);
  for (let k = 0; k < PERIODS; k++) {
    const left = xOf(u0 + k * period);
    const right = xOf(u0 + k * period + gap);
    pen.rect(left, plot.y, right - left, plot.h, ink.panel);
  }
  const hair = Math.max(plot.w / 512, 1);
  for (let k = 1; k < PERIODS; k++) {
    const x = xOf(u0 + k * period);
    pen.segment([x, plot.y], [x, plot.y + plot.h], hair, ink.line);
  }
  const place = (i: number, v: number): Point | undefined => (v >= floor ? [xOf(us[i]), yOf(Math.min(v, roof))] : undefined);
  for (const line of runs(waves, place)) pen.polyline(line, 7, ink.yellow);
  for (const line of runs(reads, place)) pen.polyline(line, 5, ink.blue);
}
