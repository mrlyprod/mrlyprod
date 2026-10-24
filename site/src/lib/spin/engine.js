import { rgb } from "../color.js";
import { canon, roll } from "../designs/engine.js";
import { pick } from "../rand.js";
import { palette } from "../theme/palette.gen.js";

export const SOURCES = [
  ["design", "Design"],
  ["moire", "Moire"],
];
export const DIMS = [
  [2, "2D"],
  [3, "3D"],
];
export const BASES = [
  [2, "2"],
  [3, "3"],
];
export const NUMBERS = [3, 5, 7, 9];
export const TILES = [3, 5];
export const PRESETS = ["heatmap", "weave", "hive", "carpet"];
export const BLENDS = [
  ["mean", "Mean"],
  ["sum", "Sum"],
  ["union", "Union"],
  ["meet", "Meet"],
  ["parity", "Parity"],
  ["difference", "Difference"],
];
export const RAMPS = [
  ["heat", "Heat"],
  ["fire", "Fire"],
  ["tint", "Tint"],
];
export const SIDE = 256;
export const CUT = 27;
export const FIELD = 256;
export const HEX = 384;
export const LIMIT = 41;
export const STEPS = 512;
export const RINGS = 160;
export const ORDERS = 48;
export const OUT = 512;
export const LEAD = 3;
export const BEAT = 500;
export const WHEEL = 1024;
export const RPM = 1800;
export const COPIES = 36;
export const NEEDLES = [33, 45, 78, 899, 900];
const ROTATION = 0;
const EASE = 0.1;

let kept = null;
let rose = null;
let bull = null;

/* SOURCE */

export function cap(number, budget = SIDE) {
  let level = 1;
  while (number ** (level + 1) <= budget) level++;
  return level;
}

export const cube = (value) => Number(value.dim) === 3;

export function key(value) {
  if (value.source === "moire") return ["moire", value.preset, value.limit].join(":");
  return cube(value)
    ? ["slice", value.base, value.code, value.tile, value.slevel].join(":")
    : ["flat", value.base, value.code, value.number, value.level].join(":");
}

export function field(math, name, limit, side) {
  const preset = math.moire.named(name, limit);
  const out = preset.field(side);
  const data = Float32Array.from(out.data);
  out.free();
  preset.free();
  return data;
}

export function source(math, value) {
  if (value.source === "moire")
    return {
      kind: "moire",
      code: value.preset,
      name: value.preset,
      side: FIELD,
      data: field(math, value.preset, value.limit, FIELD),
      fills: null,
      binary: false,
    };
  if (cube(value)) {
    const code = canon(math, 3, value.base, value.code);
    const cut = math.six.cut_design(code, value.tile, Math.min(value.slevel, cap(value.tile, CUT)), value.base);
    return {
      kind: "slice",
      code,
      name: math.name.of(code, 3, value.base) ?? "",
      side: HEX,
      data: math.six.raster(cut, HEX),
      fills: math.six.fills(cut),
      binary: true,
    };
  }
  const code = canon(math, 2, value.base, value.code);
  const cell = math.two.create(code, value.number, Math.min(value.level, cap(value.number)), ROTATION, value.base);
  return {
    kind: "flat",
    code,
    name: math.name.of(code, 2, value.base) ?? "",
    side: cell.shape[0],
    data: Float32Array.from(cell.types),
    fills: math.two.fills(cell),
    binary: true,
  };
}

export function drawn(math, value, rand) {
  if (value.source === "moire")
    return { preset: pick(rand, PRESETS), limit: 2 * Math.floor(rand() * ((LIMIT + 1) / 2)) + 1 };
  return { code: roll(math, cube(value) ? 3 : 2, value.base, rand) };
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

export function study(math, value) {
  const at = key(value);
  if (kept?.at === at) return kept.plan;
  const made = source(math, value);
  const { data, side } = made;
  const profile = math.spin.profile(data, side, STEPS);
  const power = math.spin.harmonics(data, side, RINGS, ORDERS);
  const reach = math.spin.reach(side);
  const plan = {
    ...made,
    profile,
    power,
    reach,
    top: peak(data) || 1,
    order: math.spin.turns(power),
    mass: math.spin.mass(profile, side),
    disc: disc(profile, reach),
    peak: peak(profile),
    share: shares(power),
    leading: leading(power),
  };
  kept = { at, plan };
  return plan;
}

export const title = ({ name, code, kind }) => (kind === "moire" || !name ? code : `${name} ${code}`);

export function rings(math, plan) {
  if (bull?.plan !== plan)
    bull = { plan, values: math.spin.wheel(plan.profile, WHEEL), top: plan.peak || 1, side: WHEEL };
  return bull;
}

/* STACK */

export const turn = (value) => (Number(value.full) === 1 ? 1 / value.copies : value.step / 360);

export const degrees = (value) => +(turn(value) * 360).toFixed(3);

export function petals(math, plan, value) {
  if (Number(value.full) !== 1) return "partial orbit";
  return plan.order ? math.spin.petals(value.copies, plan.order) : "none";
}

export function sketch(math, plan, value, side) {
  const values = math.spin.radial(
    plan.data,
    plan.side,
    side,
    value.copies,
    turn(value),
    math.spin.Blend.named(value.blend),
    value.samples,
  );
  return { values, top: peak(values) || 1, side };
}

export function rosette(math, plan, value) {
  const at = [value.copies, turn(value), value.blend, value.samples].join(":");
  if (rose?.plan !== plan || rose.at !== at) rose = { plan, at, ...sketch(math, plan, value, OUT) };
  return rose;
}

/* PAINT */

const contrast = ([r, g, b]) => ((r * 299 + g * 587 + b * 114) / 255000 > 0.5 ? [0, 0, 0] : [255, 255, 255]);

export const edge = ({ paper }) => `rgba(${contrast(rgb(paper)).join(", ")}, 0.2)`;

export function stops(ramp, { paper, accent }) {
  const ground = rgb(paper);
  const top = contrast(ground);
  if (ramp === "tint") return [ground, rgb(accent)];
  if (ramp === "fire") return [ground, rgb(palette.orange), rgb(palette.yellow), top];
  return [ground, rgb(accent), top];
}

const along = (list, t) => {
  const k = t * (list.length - 1);
  const i = Math.min(list.length - 2, Math.floor(k));
  const f = k - i;
  return list[i].map((v, c) => Math.round(v + (list[i + 1][c] - v) * f));
};

export function paint(values, top, list, levels, invert) {
  const n = Math.max(2, Math.floor(levels));
  const lut = Array.from({ length: n }, (_, i) => along(list, Number(invert) === 1 ? 1 - i / (n - 1) : i / (n - 1)));
  const k = top > 0 ? n / top : 0;
  const out = new Uint8ClampedArray(values.length * 4);
  for (let i = 0; i < values.length; i++) {
    const bin = Math.floor(values[i] * k);
    const [r, g, b] = lut[bin > 0 ? Math.min(n - 1, bin) : 0];
    out[i * 4] = r;
    out[i * 4 + 1] = g;
    out[i * 4 + 2] = b;
    out[i * 4 + 3] = 255;
  }
  return out;
}

export function cells(data, [r, g, b], ground) {
  const out = new Uint8ClampedArray(data.length * 4);
  for (let i = 0; i < data.length; i++) {
    out[i * 4] = r;
    out[i * 4 + 1] = g;
    out[i * 4 + 2] = b;
    out[i * 4 + 3] = data[i] > 0 ? 255 : ground;
  }
  return out;
}

/* TIME */

export function step(rpm, dt) {
  return (rpm * 6 * dt) / 1000;
}

export const spun = (phase, rpm, dt) => (phase + step(rpm, dt)) % 360;

export function seen(deg, order) {
  if (!order) return 0;
  const period = 360 / order;
  return deg - Math.round(deg / period) * period;
}

export function pace(dt, was) {
  return was ? was + (dt - was) * EASE : dt;
}

/* STAGE */

export const layout = (w, h, pad) => ({ x: w / 2, y: h / 2, r: Math.max(1, Math.min(w, h) / 2 - pad) });
