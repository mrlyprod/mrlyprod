import type { Image, moire } from "@mrlyprod/mrly/math";
import type { Color, Ink } from "#view";
import { ink } from "#view";
import type { Scalar } from "../app/spec.ts";
import type { Paints } from "../color.ts";
import { hex, rgb } from "../color.ts";
import type { Code, Mrly } from "../designs/engine.ts";
import { canon } from "../designs/engine.ts";
import { rng } from "../rand.ts";
import { dark, light } from "../theme/theme.ts";

/* TYPES */

export type Look = Pick<Paints, "paper" | "accent">;

export type Ramp = "fire" | "heat" | "diverge" | "tint" | "ink" | "bands";

export type Combine = "sum" | "and" | "xor" | "hive";

export type Colorizer = { Bins: { background: string; ramp: string[] } };

export type Entry = { base: Scalar; code: Code | null | undefined; level: Scalar };

export type Recipe = Entry & { limit: number; combine: Combine };

export type Styled = Recipe & { ramp: Ramp; levels: number; invert: Scalar };

export type Design = { base: 2 | 3; code: string; level: number; name: string };

export type Plan = Design & { numbers: number[]; spec: moire.Spec; merge: moire.Combine; lattice: moire.Lattice };

export type Held = { field: moire.Field; data: Float32Array; size: number };

export type Preset = { name: string; base: number; code: string; level: number; combine: Combine };

export type Drawn = Omit<Preset, "name"> & { seed: number; limit: number };

export type Table = { scales: number[]; grid: number[][] };

export type Trial = { row: number; mate: number; r: number | null; witness: moire.pairs.Witness | null; k: number };

export const RAMPS: [Ramp, string][] = [
  ["fire", "Fire"],
  ["heat", "Heat"],
  ["diverge", "Diverge"],
  ["tint", "Tint"],
  ["ink", "Ink"],
  ["bands", "Bands"],
];
export const COMBINES: [Combine, string][] = [
  ["sum", "Sum"],
  ["and", "And"],
  ["xor", "Xor"],
  ["hive", "Hive"],
];
export const CROPS: [string, string][] = [
  ["", "Off"],
  ["ball", "Ball"],
  ["box", "Box"],
  ["diamond", "Diamond"],
  ["triangle", "Triangle"],
  ["octagon", "Octagon"],
];
export const SIZES = [128, 256, 384, 512];
export const TOP = 41;
export const BEAT = 350;
export const DEEPEST = 5;
export const GRAIN = 32;
const DIM = 2;
const MERGE: Record<Combine, moire.Combine> = { sum: "Sum", and: "And", xor: "Xor", hive: "Sum" };

/* SCALES */

export function odd(limit: number): number[] {
  const top = Math.max(1, Math.floor(limit)) | 1;
  return Array.from({ length: (top + 1) / 2 }, (_, i) => 2 * i + 1);
}

export function climb(from: number, beats: number): number {
  const steps = (TOP + 1) / 2;
  const at = Math.min(steps - 1, (Math.max(1, Math.floor(from) | 1) - 1) / 2);
  return 1 + 2 * ((at + beats) % steps);
}

/* COLOUR */

const bright = ([r, g, b]: readonly number[]) => (r * 299 + g * 587 + b * 114) / 255000 > 0.5;

const contrast = (c: readonly number[]) => (bright(c) ? [0, 0, 0] : [255, 255, 255]);

const along = (stops: readonly number[][], t: number) => {
  const k = t * (stops.length - 1);
  const i = Math.min(stops.length - 2, Math.floor(k));
  const f = k - i;
  return stops[i].map((v, c) => v + (stops[i + 1][c] - v) * f);
};

const inks = new Map<string, Ink>();

export function palette({ paper, accent }: Look): Ink {
  const key = `${paper} ${accent}`;
  if (!inks.has(key)) {
    const ground = rgb(paper);
    const roles = { ...(bright(ground) ? light : dark), ground, accent: rgb(accent) };
    inks.set(key, ink(roles));
  }
  return inks.get(key)!;
}

export const css = (c: readonly number[]) => `rgba(${c[0]}, ${c[1]}, ${c[2]}, ${(c[3] ?? 255) / 255})`;

export function heat(ink: Ink, r: number): Color {
  const t = Math.min(1, Math.sqrt(Math.max(0, r)));
  return t < 0.5 ? ink.mix(ink.fg, ink.blue, t * 2) : ink.mix(ink.blue, ink.mix(ink.blue, ink.ground, 0.6), t * 2 - 1);
}

export function stops(ramp: Ramp, look: Look): number[][] {
  const paper = rgb(look.paper);
  const accent = rgb(look.accent);
  if (ramp === "tint" || ramp === "bands") return [paper, accent];
  if (ramp === "ink") return [paper, accent, contrast(paper)];
  const made = palette(look).Ramp;
  return (ramp === "heat" ? made.heat() : ramp === "diverge" ? made.diverge() : made.fire()).stops.map((c) =>
    c.slice(0, 3),
  );
}

export function bins(ramp: Ramp, levels: number, look: Look): string[] {
  const n = Math.max(2, Math.floor(levels));
  const list = stops(ramp, look);
  if (ramp === "bands") return Array.from({ length: n }, (_, i) => hex(list[i % 2]));
  return Array.from({ length: n }, (_, i) => hex(along(list, i / (n - 1))));
}

export function colorizer(ramp: Ramp, levels: number, look: Look): Colorizer {
  return { Bins: { background: hex(rgb(look.paper)), ramp: bins(ramp, levels, look) } };
}

export function paint(
  math: Mrly,
  { field: made, data }: Pick<Held, "field" | "data">,
  colors: Colorizer,
  levels: number,
  invert: Scalar,
  symmetric = false,
): Image {
  const image = math.moire.render(made, colors, Math.max(2, Math.floor(levels)), symmetric, Number(invert) === 1, 1);
  const out = image.colors;
  for (let i = 0; i < data.length; i++) if (Number.isNaN(data[i])) out[i * 4 + 3] = 0;
  return { shape: image.shape, colors: out };
}

/* DESIGN */

export function design(math: Mrly, value: Entry): Design {
  const base = Number(value.base) === 2 ? 2 : 3;
  const code = canon(math, DIM, base, value.code);
  const level = Math.min(DEEPEST, Math.max(1, Math.round(Number(value.level) || 1)));
  return { base, code, level, name: math.name.of(code, DIM, base) ?? "" };
}

const tables = new WeakMap<Mrly, Preset[]>();

export function presets(math: Mrly): Preset[] {
  if (!tables.has(math)) {
    tables.set(
      math,
      math.moire.all(1).map((one): Preset => {
        const { name, spec, level, lattice, combine } = one.toJSON();
        one.free();
        return {
          name,
          base: spec.base,
          code: String(spec.code),
          level,
          combine: lattice === "Hex" ? "hive" : (combine.toLowerCase() as Lowercase<moire.Combine>),
        };
      }),
    );
  }
  return tables.get(math)!;
}

export function presetOf(list: readonly Preset[], value: Pick<Recipe, "base" | "code" | "level" | "combine">): string {
  return (
    list.find(
      (one) =>
        one.base === Number(value.base) &&
        one.code === String(value.code) &&
        one.level === Number(value.level) &&
        one.combine === value.combine,
    )?.name ?? ""
  );
}

export function drawn(math: Mrly, seed: number): Drawn {
  const rand = rng(seed);
  const list = presets(math);
  const { name, ...picked } = list[Math.floor(rand() * list.length)];
  return { seed, ...picked, limit: 1 + 2 * Math.floor(rand() * ((TOP + 1) / 2)) };
}

/* LAW */

function memo<K, V>(size: number) {
  const held = new Map<K, V>();
  return (key: K, make: () => V): V => {
    if (held.has(key)) return held.get(key)!;
    const made = make();
    held.set(key, made);
    if (held.size > size) held.delete(held.keys().next().value!);
    return made;
  };
}

const witnesses = memo<number, moire.pairs.Witness>(128);

const matrices = memo<number, Table>(8);

const witness = (math: Mrly, scale: number) => witnesses(scale, () => math.moire.pairs.witness(scale));

export function matrix(math: Mrly, limit: number): Table {
  return matrices(limit, () => {
    const scales = odd(limit).filter((n) => n >= 3);
    return { scales, grid: scales.map((a) => scales.map((b) => math.moire.pairs.correlation(a, b))) };
  });
}

export function law(math: Mrly, limit: number, scale: number, mate: number): Trial {
  const { scales } = matrix(math, limit);
  const row = scales.filter((n) => n <= scale).at(-1) ?? scales[0] ?? 0;
  const seen = row ? witness(math, row) : null;
  const pair = seen?.scales.includes(mate) ? mate : (seen?.scales.at(-1) ?? 0);
  const r = seen && pair ? seen.row[seen.scales.indexOf(pair)] : null;
  return { row, mate: pair, r, witness: seen, k: scales.length };
}

/* STUDY */

export function study(math: Mrly, value: Recipe): Plan {
  const made = design(math, value);
  const numbers = odd(value.limit);
  return {
    ...made,
    numbers,
    spec: math.moire.Spec.new(made.code, made.base, DIM),
    merge: MERGE[value.combine] ?? "Sum",
    lattice: value.combine === "hive" ? "Hex" : "Square",
  };
}

export function swatch(math: Mrly, value: Styled, size: number, look: Look) {
  const made = field(math, study(math, value), size);
  const out = paint(math, made, colorizer(value.ramp, value.levels, look), value.levels, value.invert);
  made.field.free();
  return out;
}

export function crop(math: Mrly, name: string, radius: number, anti: Scalar, size: number): Uint8Array | null {
  if (!name) return null;
  const frac = new math.shape.Frac(radius, GRAIN);
  const shape = math.shape.named(name, DIM, frac);
  frac.free();
  return Uint8Array.from(math.shape.regions(shape, [size, size]).data, (region) =>
    (Number(anti) === 1 ? region === 0 : region > 0) ? 1 : 0,
  );
}

export function field(math: Mrly, plan: Plan, size: number, kept: Uint8Array | null = null): Held {
  const made = math.moire.stack(plan.spec, plan.numbers, plan.merge, plan.level, plan.lattice, size, []);
  const data = made.data;
  if (!kept) return { field: made, data, size };
  made.free();
  for (let i = 0; i < data.length; i++) if (!kept[i]) data[i] = NaN;
  return { field: math.moire.Field.from_data(data, size), data, size };
}
