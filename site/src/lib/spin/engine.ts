import type { Scalar } from "../app/spec.ts";
import type { Paints } from "../color.ts";
import { rgb } from "../color.ts";
import type { Mrly } from "../designs/engine.ts";
import { canon, roll } from "../designs/engine.ts";
import type { Rng } from "../rand.ts";
import { pick } from "../rand.ts";
import { palette } from "../theme/palette.gen.js";

/* TYPES */

export type Kind = "flat" | "slice" | "moire";

export type SourceName = "design" | "moire";

export type Blend = "mean" | "sum" | "union" | "meet" | "parity" | "difference";

export type Ramp = "heat" | "fire" | "tint";

export type Src = {
  source: SourceName;
  dim: number;
  base: number;
  code: string;
  number: number;
  level: number;
  preset: string;
  limit: number;
  tile: number;
  slevel: number;
};

export type Made = {
  kind: Kind;
  code: string;
  name: string;
  side: number;
  data: Float32Array;
  fills: number | null;
  binary: boolean;
};

export type Lead = { order: number; share: number };

export type Plan = Made & {
  profile: Float32Array;
  power: Float64Array;
  reach: number;
  top: number;
  order: number;
  mass: number;
  disc: number;
  peak: number;
  share: number[];
  leading: Lead[];
};

export type Kept = { at: string; plan: Plan };

export type Drawn = Pick<Src, "preset" | "limit"> | Pick<Src, "code">;

export type Titled = { name: string; code: string; kind: string };

export type Stack = { copies: number; full: Scalar; step: number; blend: Blend; samples: number };

export type Raster = { values: Float32Array; top: number; side: number };

export type Rings = Raster & { plan: Plan };

export type Rose = Raster & { plan: Plan; at: string };

export type Stops = readonly (readonly number[])[];

export type Disc = { x: number; y: number; r: number };

export const SOURCES: [SourceName, string][] = [
  ["design", "Design"],
  ["moire", "Moire"],
];
export const DIMS: [number, string][] = [
  [2, "2D"],
  [3, "3D"],
];
export const BASES: [number, string][] = [
  [2, "2"],
  [3, "3"],
];
export const NUMBERS = [3, 5, 7, 9];
export const TILES = [3, 5];
export const PRESETS = ["heatmap", "weave", "hive", "carpet"];
export const BLENDS: [Blend, string][] = [
  ["mean", "Mean"],
  ["sum", "Sum"],
  ["union", "Union"],
  ["meet", "Meet"],
  ["parity", "Parity"],
  ["difference", "Difference"],
];
export const RAMPS: [Ramp, string][] = [
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

let kept: Kept | null = null;
let rose: Rose | null = null;
let bull: Rings | null = null;

/* SOURCE */

export function cap(number: number, budget = SIDE): number {
  let level = 1;
  while (number ** (level + 1) <= budget) level++;
  return level;
}

export const cube = (value: Pick<Src, "dim">) => Number(value.dim) === 3;

export function key(value: Src): string {
  if (value.source === "moire") return ["moire", value.preset, value.limit].join(":");
  return cube(value)
    ? ["slice", value.base, value.code, value.tile, value.slevel].join(":")
    : ["flat", value.base, value.code, value.number, value.level].join(":");
}

export function field(math: Mrly, name: string, limit: number, side: number): Float32Array {
  const preset = math.moire.named(name, limit);
  const out = preset.field(side);
  const data = Float32Array.from(out.data);
  out.free();
  preset.free();
  return data;
}

export function source(math: Mrly, value: Src): Made {
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

export function drawn(math: Mrly, value: Src, rand: Rng): Drawn {
  if (value.source === "moire")
    return { preset: pick(rand, PRESETS), limit: 2 * Math.floor(rand() * ((LIMIT + 1) / 2)) + 1 };
  return { code: roll(math, cube(value) ? 3 : 2, value.base, rand) };
}

/* FACTS */

export function shares(power: ArrayLike<number> & Iterable<number>): number[] {
  let sum = 0;
  for (const p of power) sum += p;
  return Array.from(power, (p) => (sum > 0 ? p / sum : 0));
}

export function leading(power: ArrayLike<number> & Iterable<number>, n = LEAD): Lead[] {
  return shares(power)
    .map((share, order) => ({ order, share }))
    .slice(1)
    .filter((one) => one.share > 0)
    .sort((a, b) => b.share - a.share)
    .slice(0, n);
}

export function peak(values: Iterable<number>): number {
  let top = 0;
  for (const v of values) if (v > top) top = v;
  return top;
}

export function disc(profile: Float32Array, reach: number): number {
  const lit = profile.findIndex((v) => v > 0);
  return lit < 0 ? reach : (lit / (profile.length - 1)) * reach;
}

export function study(math: Mrly, value: Src): Plan {
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

export const title = ({ name, code, kind }: Titled) => (kind === "moire" || !name ? code : `${name} ${code}`);

export function rings(math: Mrly, plan: Plan): Rings {
  if (bull?.plan !== plan)
    bull = { plan, values: math.spin.wheel(plan.profile, WHEEL), top: plan.peak || 1, side: WHEEL };
  return bull;
}

/* STACK */

export const turn = (value: Pick<Stack, "full" | "copies" | "step">) =>
  Number(value.full) === 1 ? 1 / value.copies : value.step / 360;

export const degrees = (value: Pick<Stack, "full" | "copies" | "step">) => +(turn(value) * 360).toFixed(3);

export function petals(math: Mrly, plan: Pick<Plan, "order">, value: Pick<Stack, "full" | "copies">): number | string {
  if (Number(value.full) !== 1) return "partial orbit";
  return plan.order ? math.spin.petals(value.copies, plan.order) : "none";
}

export function sketch(math: Mrly, plan: Pick<Plan, "data" | "side">, value: Stack, side: number): Raster {
  const values = math.spin.radial(
    plan.data,
    plan.side,
    side,
    value.copies,
    turn(value),
    math.spin.Blend.named(value.blend)!,
    value.samples,
  );
  return { values, top: peak(values) || 1, side };
}

export function rosette(math: Mrly, plan: Plan, value: Stack): Rose {
  const at = [value.copies, turn(value), value.blend, value.samples].join(":");
  if (rose?.plan !== plan || rose.at !== at) rose = { plan, at, ...sketch(math, plan, value, OUT) };
  return rose;
}

/* PAINT */

const contrast = ([r, g, b]: readonly number[]) =>
  (r * 299 + g * 587 + b * 114) / 255000 > 0.5 ? [0, 0, 0] : [255, 255, 255];

export const edge = ({ paper }: Pick<Paints, "paper">) => `rgba(${contrast(rgb(paper)).join(", ")}, 0.2)`;

export function stops(ramp: Ramp, { paper, accent }: Pick<Paints, "paper" | "accent">): number[][] {
  const ground = rgb(paper);
  const top = contrast(ground);
  if (ramp === "tint") return [ground, rgb(accent)];
  if (ramp === "fire") return [ground, rgb(palette.orange), rgb(palette.yellow), top];
  return [ground, rgb(accent), top];
}

const along = (list: Stops, t: number) => {
  const k = t * (list.length - 1);
  const i = Math.min(list.length - 2, Math.floor(k));
  const f = k - i;
  return list[i].map((v, c) => Math.round(v + (list[i + 1][c] - v) * f));
};

export function paint(
  values: ArrayLike<number>,
  top: number,
  list: Stops,
  levels: number,
  invert: Scalar,
): Uint8ClampedArray<ArrayBuffer> {
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

export function cells(
  data: ArrayLike<number>,
  [r, g, b]: readonly number[],
  ground: number,
): Uint8ClampedArray<ArrayBuffer> {
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

export function step(rpm: number, dt: number): number {
  return (rpm * 6 * dt) / 1000;
}

export const spun = (phase: number, rpm: number, dt: number) => (phase + step(rpm, dt)) % 360;

export function seen(deg: number, order: number): number {
  if (!order) return 0;
  const period = 360 / order;
  return deg - Math.round(deg / period) * period;
}

export function pace(dt: number, was: number): number {
  return was ? was + (dt - was) * EASE : dt;
}

/* STAGE */

export const layout = (w: number, h: number, pad: number): Disc => ({
  x: w / 2,
  y: h / 2,
  r: Math.max(1, Math.min(w, h) / 2 - pad),
});
