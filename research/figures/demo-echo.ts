import * as num from "mrlyjs/num";
import { plot, type Color, type Frame, type Ink, type Pen } from "mrlyjs/view";

export const units = { num };

const BASE = 10;
const DIGITS = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const DEPTH = 5;
const SAMPLES = 4096;
const FLOOR = 101;
const BAND: [number, number] = [4, 60];
const THRESHOLD = 8;

type Echo = { gamma: Float64Array; score: Float64Array; found: Uint32Array; zeros: number[]; lattice: number[] };

let memo: Echo | undefined;

function echo() {
  if (memo) return memo;
  const values = num.design.elements(BASE, DIGITS, DEPTH);
  const sieve = num.factor.mobius_sieve(Number(values[values.length - 1]));
  const mu = Int8Array.from(values, (v) => sieve[Number(v)]);
  const running = num.design.meter(mu);
  if (values.length !== 59048) throw new Error(`demo-echo: ${values.length} elements, want 59048`);
  if (running[values.length - 1] !== 201n) throw new Error(`demo-echo: meter ends at ${running[values.length - 1]}, want 201`);

  const alpha = Math.log(DIGITS.length) / Math.log(BASE);
  const logx = num.design.log_grid(values, SAMPLES);
  const series = num.design.resample(values, running, alpha / 2, logx);
  const [gamma, power] = num.design.spectrum(logx, series);
  const score = num.design.score(power, FLOOR);
  const found = num.design.peaks(gamma, score, BAND, THRESHOLD);
  const bin = gamma[1];
  const zeros = Array.from(num.design.ZETA_ORDINATES()).filter((g) => g > BAND[0] && g < BAND[1]);
  const lattice = Array.from(num.design.pole_lattice(BASE, BAND[1])).filter((g) => g > BAND[0]);
  if (found.length !== 5) throw new Error(`demo-echo: ${found.length} peaks, want 5`);
  if (zeros.length !== 13) throw new Error(`demo-echo: ${zeros.length} zeros in the band, want 13`);
  if (lattice.length !== 20) throw new Error(`demo-echo: ${lattice.length} lattice lines, want 20`);
  const near = Array.from(found).filter((at) => num.design.nearest(gamma[at], zeros) <= bin).length;
  if (near !== 5) throw new Error(`demo-echo: ${near} peaks on a zero, want 5`);
  memo = { gamma, score, found, zeros, lattice };
  return memo;
}

function column(pen: Pen, frame: Frame, x: number, thick: number, color: Color, dash: number) {
  if (dash <= 0) {
    pen.segment([x, frame.y], [x, frame.y + frame.h], thick, color);
    return;
  }
  let y = frame.y;
  while (y < frame.y + frame.h) {
    const end = Math.min(y + dash, frame.y + frame.h);
    pen.segment([x, y], [x, end], thick, color);
    y = end + dash * 0.8;
  }
}

export default function draw(pen: Pen, ink: Ink) {
  const { gamma, score, found, zeros, lattice } = echo();
  const frame = pen.frame(0.08);
  const peak = Array.from(found).reduce((top, at) => Math.max(top, score[at]), 0);
  const x = (g: number) => frame.x + (frame.w * (g - BAND[0])) / (BAND[1] - BAND[0]);
  const y = (v: number) => frame.y + frame.h * (1 - Math.sqrt(Math.min(Math.max(v / peak, 0), 1)));

  for (const line of lattice) column(pen, frame, x(line), 3, ink.fade(ink.pink, 0.34), 13);
  for (const zero of zeros) column(pen, frame, x(zero), 7, ink.fade(ink.yellow, 0.46), 0);
  plot.baseline(pen, frame, ink.line);
  for (let at = 0; at < gamma.length; at++) {
    if (gamma[at] <= BAND[0] || gamma[at] >= BAND[1]) continue;
    pen.segment([x(gamma[at]), frame.y + frame.h], [x(gamma[at]), y(score[at])], 4, ink.blue);
  }
  plot.dots(pen, Array.from(found, (at): [number, number] => [x(gamma[at]), y(score[at])]), 11, ink.yellow);
}
