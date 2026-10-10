import { resolve, total } from '../designs/engine.js';

export const NUMBERS = [3, 5, 7, 9];
export const BLENDS = [['mean', 'Mean'], ['sum', 'Sum'], ['union', 'Union'], ['meet', 'Meet'], ['parity', 'Parity'], ['difference', 'Difference']];
export const SHOWS = [['both', 'Turntable and wheel'], ['turn', 'Turntable'], ['wheel', 'Wheel'], ['radial', 'Rosette']];
export const SIDE = 256;
export const STEPS = 512;
export const RINGS = 160;
export const ORDERS = 48;
export const OUT = 384;
export const SAMPLES = 2;
export const LEAD = 3;
export const BEAT = 500;
export const WHEEL = 1024;
export const DIM = 2;
export const NEEDLES = [33, 45, 78, 899, 900];
const ROTATION = 0;
const EASE = 0.1;
const TAU = Math.PI * 2;

let kept = null;
let rose = null;
let bull = null;

/* DESIGN */

export function cap(number) {
  let level = 1;
  while (number ** (level + 1) <= SIDE) level++;
  return level;
}

export function design(math, value, rand) {
  return String(resolve(value.code, total(math, DIM, value.base), rand));
}

export function grow(math, value, code) {
  return math.two.create(code, value.number, Math.min(value.level, cap(value.number)), ROTATION, value.base);
}

/* FACTS */

export function shares(power) {
  let sum = 0;
  for (const p of power) sum += p;
  return Array.from(power, (p) => (sum > 0 ? p / sum : 0));
}

export function leading(power, n = LEAD) {
  return shares(power)
    .map((share, order) => ({ order, share }))
    .slice(1)
    .filter((one) => one.share > 0)
    .sort((a, b) => b.share - a.share)
    .slice(0, n);
}

export function peak(values) {
  let top = 0;
  for (const v of values) if (v > top) top = v;
  return top;
}

export function disc(profile, reach) {
  const lit = profile.findIndex((v) => v > 0);
  return lit < 0 ? reach : (lit / (profile.length - 1)) * reach;
}

const key = (value) => [value.base, value.code, value.number, value.level, value.seed].join(':');

function read(math, value, rand) {
  const code = design(math, value, rand);
  const cell = grow(math, value, code);
  const side = cell.shape[0];
  const data = Float32Array.from(cell.types);
  const profile = math.spin.profile(data, side, STEPS);
  const power = math.spin.harmonics(data, side, RINGS, ORDERS);
  const reach = math.spin.reach(side);
  return { code, level: Math.min(value.level, cap(value.number)), side, cell, data, profile, power, order: math.spin.turns(power), fills: math.two.fills(cell), mass: math.spin.mass(profile, side), reach, inner: side / 2, disc: disc(profile, reach), peak: peak(profile), share: shares(power), leading: leading(power) };
}

export function study(math, value, rand) {
  const at = key(value);
  if (kept?.at !== at) kept = { at, plan: read(math, value, rand) };
  return kept.plan;
}

export function petals(math, plan, copies) {
  return plan.order ? math.spin.petals(copies, plan.order) : 0;
}

export function rings(math, plan) {
  if (bull?.plan !== plan) bull = { plan, values: math.spin.wheel(plan.profile, WHEEL), top: plan.peak || 1, side: WHEEL };
  return bull;
}

export function rosette(math, plan, copies, blend) {
  if (rose?.plan !== plan || rose.copies !== copies || rose.blend !== blend) {
    const values = math.spin.radial(plan.data, plan.side, OUT, copies, 1 / copies, math.spin.Blend.named(blend), SAMPLES);
    rose = { plan, copies, blend, values, top: peak(values) || 1, side: OUT };
  }
  return rose;
}

/* PIXELS */

export function pixels(values, top, [r, g, b]) {
  const out = new Uint8ClampedArray(values.length * 4);
  for (let i = 0; i < values.length; i++) {
    const v = values[i];
    if (v <= 0) continue;
    const at = i * 4;
    out[at] = r;
    out[at + 1] = g;
    out[at + 2] = b;
    out[at + 3] = Math.round(255 * Math.min(1, v / top));
  }
  return out;
}

/* TIME */

export function turns(rpm, t) {
  return (rpm * t) / 60000;
}

export function angle(rpm, t) {
  const k = turns(rpm, t);
  return (k - Math.floor(k)) * TAU;
}

export function step(rpm, dt) {
  return (rpm * 6 * dt) / 1000;
}

export function seen(deg, order) {
  if (!order) return 0;
  const period = 360 / order;
  return deg - Math.round(deg / period) * period;
}

export function pace(dt, was) {
  return was ? was + (dt - was) * EASE : dt;
}

/* STAGE */

export function layout(w, h, show, pad) {
  if (show !== 'both') return { [show]: { x: w / 2, y: h / 2, r: Math.max(1, Math.min(w, h) / 2 - pad) } };
  const wide = w >= h;
  const long = wide ? w : h;
  const short = wide ? h : w;
  const r = Math.max(1, Math.min((long - 3 * pad) / 4, short / 2 - pad));
  const off = r + pad / 2;
  return wide ? { turn: { x: w / 2 - off, y: h / 2, r }, wheel: { x: w / 2 + off, y: h / 2, r } } : { turn: { x: w / 2, y: h / 2 - off, r }, wheel: { x: w / 2, y: h / 2 + off, r } };
}
