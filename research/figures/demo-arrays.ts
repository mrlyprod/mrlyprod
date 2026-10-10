import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const GENERATOR = [0, 1, 2, 4];
const BASE = 9;
const REACH = 40;
const LAGS = 2 * REACH + 1;
const OUT = 13;
const LOST = [-24, -23, 23, 24];
const BAND = 0.16;
const GAP = 0.07;
const ROD = 0.56;
const BAR = 0.62;

function weights(set: number[]): number[] {
  const marks = Array.from({ length: LAGS }, (_, p) => (set.includes(p) ? 1 : 0));
  const mirror = Array.from({ length: LAGS }, (_, m) => (m <= REACH ? marks[REACH - m] : 0));
  return num.blend.cauchy(mirror, marks).map(Number);
}

function paired(): number[] {
  const w = weights(GENERATOR);
  return GENERATOR.filter((g) => GENERATOR.some((h) => h !== g && w[g - h + REACH] === 1));
}

let memo: { sensors: number[]; essential: boolean[]; whole: number[]; less: number[] } | undefined;

function array() {
  if (memo) return memo;
  const sensors = GENERATOR.flatMap((high) => GENERATOR.map((low) => low + BASE * high)).sort((a, b) => a - b);
  if (sensors.length !== 16 || sensors.at(-1) !== REACH) throw new Error(`demo-arrays: ${sensors.length} sensors to ${sensors.at(-1)}, want 16 to ${REACH}`);
  const u = paired();
  if (u.join() !== "0,1,4") throw new Error(`demo-arrays: U(G) = ${u}, want 0,1,4`);
  const essential = sensors.map((s) => u.includes(s % BASE) && u.includes(Math.floor(s / BASE)));
  const count = essential.filter(Boolean).length;
  if (count !== u.length ** 2) throw new Error(`demo-arrays: ${count} essential, want ${u.length ** 2}`);
  const whole = weights(sensors);
  const less = weights(sensors.filter((s) => s !== OUT));
  if (whole.length !== LAGS || !whole.every((w) => w > 0)) throw new Error("demo-arrays: the coarray has a hole");
  if (whole[REACH] !== 16 || less[REACH] !== 15) throw new Error(`demo-arrays: lag zero ${whole[REACH]} then ${less[REACH]}, want 16 then 15`);
  const lost = less.flatMap((w, i) => (w === 0 ? [i - REACH] : []));
  if (lost.join() !== LOST.join()) throw new Error(`demo-arrays: lost ${lost}, want ${LOST}`);
  if (!essential[sensors.indexOf(OUT)]) throw new Error("demo-arrays: the knocked sensor is inessential");
  memo = { sensors, essential, whole, less };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { sensors, essential, whole, less } = array();
  const frame = pen.frame(0.08);
  const site = frame.w / (REACH + 1);
  const band = frame.h * BAND;
  const rod = site * ROD;
  for (let p = 0; p <= REACH; p++) {
    const x = frame.x + (p + 0.5) * site;
    const i = sensors.indexOf(p);
    if (i < 0) {
      pen.disc(x, frame.y + band / 2, site * 0.12, ink.line);
      continue;
    }
    const color = p === OUT ? ink.dim : essential[i] ? ink.orange : ink.blue;
    pen.round_rect(x - rod / 2, frame.y, rod, band, rod / 2, color);
  }
  const col = frame.w / LAGS;
  const bar = col * BAR;
  const floor = frame.y + frame.h;
  const peak = whole[REACH];
  const unit = (frame.h - band - frame.h * GAP) / peak;
  for (let i = 0; i < LAGS; i++) {
    const x = frame.x + (i + 0.5) * col;
    const lost = less[i] === 0;
    const h = (lost ? peak : less[i]) * unit;
    pen.round_rect(x - bar / 2, floor - h, bar, h, Math.min(bar / 2, h / 2), lost ? ink.orange : ink.blue);
  }
}
