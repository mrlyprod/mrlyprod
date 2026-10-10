import * as num from "mrlyjs/num";
import { plot } from "mrlyjs/view";
import type { Frame, Ink, Pen, Point } from "mrlyjs/view";

export const units = { num };

const LOW = 8;
const HIGH = 14;
const PER_OCTAVE = 16;
const ONE = 1;
const MANY = 30;
const CURVE = 2400;

type Facts = { peak: number; dots: number[]; one: number[]; many: number[] };

let memo: Facts | undefined;

// FACTS

function facts() {
  if (memo) return memo;
  const reach = Math.trunc(2 ** (HIGH + 1));
  const phi = num.factor.totients(reach);
  if (reach !== 32768) throw new Error(`demo-novelty: reach ${reach}, want 32768`);
  const head = Array.from(phi.slice(1, 11), Number);
  if (head.join() !== "1,1,2,2,4,2,6,4,6,4") throw new Error(`demo-novelty: totients ${head}, want 1,1,2,2,4,2,6,4,6,4`);
  const main = num.zeta.novelty_main();
  const samples = Math.round((HIGH - LOW) * PER_OCTAVE) + 1;
  if (samples !== 97) throw new Error(`demo-novelty: ${samples} samples, want 97`);
  const js = Array.from({ length: samples }, (_, k) => LOW + k / PER_OCTAVE);
  const dots = js.map((j) => {
    const y = 2 ** -j;
    return num.zeta.smoothed_novelty(phi, y, main) / y ** 1.5;
  });
  const line = new num.zeta.Line();
  const gammas = line.zeros(MANY);
  if (!(Math.abs(gammas[0] - 14.134725) < 1e-6)) throw new Error(`demo-novelty: first zero ${gammas[0]}, want 14.134725`);
  const coef = line.novelty_coefficients(gammas);
  if (!(Math.abs(coef[0].abs() - 0.1879) < 5e-4)) throw new Error(`demo-novelty: first coefficient ${coef[0].abs()}, want 0.1879`);
  const wave = (count: number, j: number) => num.zeta.novelty_wave(gammas.subarray(0, count), coef.slice(0, count), -j * Math.LN2);
  const peak = dots.reduce((a, v) => Math.max(a, Math.abs(v)), 0);
  const miss = (count: number) => js.reduce((m, j, i) => Math.max(m, Math.abs(dots[i] - wave(count, j))), 0) / peak;
  if (!(miss(MANY) < 1e-2)) throw new Error(`demo-novelty: ${MANY} zeros miss ${miss(MANY)}, want under 0.01`);
  const missOne = miss(ONE);
  if (!(missOne > 0.3 && missOne < 0.7)) throw new Error(`demo-novelty: ${ONE} zero misses ${missOne}, want 0.3 to 0.7`);
  const sweep = (count: number) => Array.from({ length: CURVE + 1 }, (_, k) => wave(count, LOW + ((HIGH - LOW) * k) / CURVE));
  memo = { peak, dots, one: sweep(ONE), many: sweep(MANY) };
  return memo;
}

// DRAW

function trace(area: Frame, span: number, values: number[]): Point[] {
  const last = values.length - 1;
  return values.map((v, i) => [area.x + (area.w * i) / last, area.y + area.h * (0.5 - (0.5 * v) / span)]);
}

export default function draw(pen: Pen, ink: Ink) {
  const { peak, dots, one, many } = facts();
  const frame = pen.frame(0.08);
  plot.axis(pen, frame, ink.line);
  const mid = frame.y + frame.h / 2;
  pen.segment([frame.x, mid], [frame.x + frame.w, mid], 1.5, ink.line);
  const inner = frame.inset(22);
  const span = peak * 1.12;
  pen.polyline(trace(inner, span, one), 4, ink.dim);
  pen.polyline(trace(inner, span, many), 2, ink.orange);
  plot.dots(pen, trace(inner, span, dots), 5, ink.blue);
}
