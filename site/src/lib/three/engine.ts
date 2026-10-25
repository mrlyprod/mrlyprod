import type { Cell, moire, shape, Tensor, three } from "@mrlyprod/mrly/math";
import type { Ink } from "#view";
import type { Scalar } from "../app/spec.ts";
import { hex } from "../color.ts";
import type { Code, Mrly } from "../designs/engine.ts";
import { canon } from "../designs/engine.ts";
import type { Colorizer } from "../moire/engine.ts";

/* TYPES */

export type Memo = <T>(key: string | number, make: () => T) => T;

export type Axis = "x" | "y" | "z" | "d";

export type Normal = [number, number, number];

export type RampName = "fire" | "heat" | "diverge";

export type Policy = "inside" | "touching";

export type Centre = "corner" | "centre";

export type Merge = "sum" | "xor" | "and";

export type Made = { base: number; code: string };

export type Coded = { base: Scalar; code?: Code | null };

export type Crop = "ball" | "box" | "diamond" | "octahedron" | "tetrahedron" | "pyramid";

export type Region = shape.Shape | { Anti: shape.Shape };

export type Wall = { normal: number[]; constant: number };

export type Cropping = { crop: Crop; radius: number; anti: Scalar; policy: Policy };

export type Whole = { crop: ""; radius?: number; anti?: Scalar; policy?: Policy };

export type Cut = { cell: Cell; out: number; cut: number; in: number; after: number };

export type Counting = { number: number; count: number; centre: Centre };

export type Count = {
  side: number;
  top: number;
  seen: number[];
  inside: number[];
  cut: number[];
  mass: number;
  d: number;
};

export type Reading = { at: number; seen: number; inside: number; cut: number };

export type Stacking = { limit: number; combine: Merge; depth: number; size: number };

export type Field = {
  volume: moire.Volume;
  layers: number;
  voxels: number;
  min: number;
  max: number;
  free: () => void;
};

export type { Colorizer };

export type Paint = { colorizer: Colorizer; levels: number; invert: Scalar };

export type Plane = {
  key: Axis;
  offset: number;
  frame: moire.Frame;
  pixels: { shape: [number, number]; colors: Uint8Array };
};

export type Skin = { threshold: number } & (Whole | Cropping);

export type Shell = { cell: Cell; count: number; faces: number };

type Free = { free: () => void };

export const DIM = 3;
export const CARPET = "23";
export const NUMBERS = [2, 3, 4, 5];
export const SHAPES: Crop[] = ["ball", "box", "diamond", "octahedron", "tetrahedron", "pyramid"];
export const DEN = 120;
export const REACH = 108;
export const SIDE = 128;
export const CROPPED = 81;
export const FACES = 400000;
export const CUBES = 250000;
export const TALLY = 600000;
export const COUNTED = 600000;
export const LAYERS = 21;
export const COMBINES: [Merge, string][] = [
  ["sum", "Sum"],
  ["xor", "Xor"],
  ["and", "And"],
];
export const SIZES = [48, 64, 80];
export const RAMPS: [RampName, string][] = [
  ["fire", "Fire"],
  ["heat", "Heat"],
  ["diverge", "Diverge"],
];
export const PLANES: [Axis, string, Normal][] = [
  ["x", "X plane", [1, 0, 0]],
  ["y", "Y plane", [0, 1, 0]],
  ["z", "Z plane", [0, 0, 1]],
  ["d", "Diagonal", [1, 1, 1]],
];
export const WIDE: Record<Axis, number> = { x: 256, y: 256, z: 256, d: 384 };
const MERGE: Partial<Record<string, moire.Combine>> & { sum: moire.Combine } = { sum: "Sum", xor: "Xor", and: "And" };

const n = (v: string | number | bigint) => Number(v);

const frees = (one: unknown): one is Free =>
  typeof one === "object" && one !== null && "free" in one && typeof one.free === "function";

/* MEMO */

export function memo(limit = 8): Memo {
  const kept = new Map<string | number, unknown>();
  return <T>(key: string | number, make: () => T): T => {
    if (kept.has(key)) {
      const one = kept.get(key) as T;
      kept.delete(key);
      kept.set(key, one);
      return one;
    }
    const one = make();
    kept.set(key, one);
    while (kept.size > limit) {
      const [first, old] = kept.entries().next().value!;
      kept.delete(first);
      if (frees(old)) old.free();
    }
    return one;
  };
}

const cells = memo(6);
const derived = memo(16);
const fields = memo(3);
const meshes = memo(2);
const ids = new WeakMap<WeakKey, number>();
let next = 0;

function keyOf(one: WeakKey): number {
  if (!ids.has(one)) ids.set(one, ++next);
  return ids.get(one)!;
}

/* DESIGN */

export const cap = (math: Mrly, number: number, crop?: Crop | "") =>
  math.bang.level_cap(number, 1, crop ? CROPPED : SIDE);

export const counted = (math: Mrly, number: number) => math.bang.level_cap(number, DIM, COUNTED);

export const side = (math: Mrly, number: number, level: number) => n(math.counts.grid(number, 1, level));

export function top(math: Mrly, number: number, count: number, centre: Centre): number {
  const edge = side(math, number, Math.min(count, counted(math, number)));
  return centre === "centre" ? Math.floor((edge - 1) / 2) : edge - 1;
}

export function design(math: Mrly, value: Coded): Made {
  const base = Number(value.base) === 3 ? 3 : 2;
  const text = String(value.code ?? "").trim();
  return { base, code: canon(math, DIM, base, text) };
}

export function named(math: Mrly, code: Code, base: number): string {
  return math.name.of(code, DIM, base) ?? "";
}

export function tag(name: string, code: Code, base: number): string {
  return `${base === 3 ? "b3 " : ""}${name || "code"} ${code}`;
}

export function grown(math: Mrly, code: Code, base: number, number: number, level: number): Cell {
  return cells(`${code} ${base} ${number} ${level}`, () => math.three.create(code, number, level, base));
}

export const tensor = (cell: Cell): Tensor => ({ shape: cell.shape, data: cell.types });

export const flat = (kept: Tensor): Cell => ({ shape: kept.shape, types: kept.data });

export function faces(math: Mrly, cell: Cell): three.Quad[] {
  return meshes(keyOf(cell), () => math.three.quads(cell));
}

export function tally(math: Mrly, cell: Cell): three.Census | null {
  return derived(`tally ${keyOf(cell)}`, () => (cell.types.length <= TALLY ? math.three.census(cell) : null));
}

export function load(math: Mrly, cell: Cell, cubes: boolean): number {
  return derived(`load ${keyOf(cell)} ${cubes}`, () => (cubes ? math.three.fills(cell) : n(math.three.surface(cell))));
}

/* CROP */

export function region(math: Mrly, name: string, radius: number, anti: boolean, den = DEN): Region {
  const frac = new math.shape.Frac(radius, den);
  const core = math.shape.named(name, DIM, frac);
  frac.free();
  return anti ? { Anti: core } : core;
}

export function walls(core: Region | null | undefined): Wall[] | null {
  const list = core && "Polytope" in core ? core.Polytope.walls : null;
  if (!list) return null;
  return list.map(({ normal, offset }) => {
    const size = Math.hypot(...normal);
    const reach = n(offset.num) / n(offset.den);
    const sum = normal.reduce((a, b) => a + b, 0);
    return { normal: normal.map((one) => -one / size), constant: (2 * reach - sum) / size };
  });
}

export function cropped(math: Mrly, cell: Cell, value: Cropping): Cut {
  const { crop, radius, anti, policy } = value;
  return derived(`crop ${keyOf(cell)} ${crop} ${radius} ${anti} ${policy}`, () => {
    const shape = region(math, crop, radius, Number(anti) === 1);
    const kept = math.shape.crop(tensor(cell), shape, policy !== "inside");
    const { filled } = math.shape.census(shape, tensor(cell));
    const out = flat(kept);
    return { cell: out, out: n(filled[0]), cut: n(filled[1]), in: n(filled[2]), after: n(math.three.surface(out)) };
  });
}

/* CIRCLE */

export function circle(math: Mrly, made: Made, value: Counting): Count {
  const { code, base } = made;
  const { number, centre } = value;
  const level = Math.min(value.count, counted(math, number));
  return derived(`circle ${code} ${base} ${number} ${level} ${centre}`, () => {
    const cell = grown(math, code, base, number, level);
    const edge = cell.shape[0];
    const last = top(math, number, level, centre);
    const origin = centre === "centre" ? [edge, edge, edge] : [0, 0, 0];
    const rows = math.shape.radial_census(tensor(cell), origin, last);
    return {
      side: edge,
      top: last,
      seen: rows.map((row) => n(row.seen)),
      inside: rows.map((row) => n(row.inside)),
      cut: rows.map((row) => n(row.cut)),
      mass: n(math.counts.fill(code, number, DIM, 1, base)),
      d: math.counts.dimension(code, number, DIM, base),
    };
  });
}

export function reading(count: Count, r: number): Reading {
  const at = Math.max(1, Math.min(r, count.top));
  return { at, seen: count.seen[at], inside: count.inside[at], cut: count.cut[at] };
}

/* FIELD */

export const odd = (limit: number) =>
  Array.from({ length: (Math.max(1, Math.floor(limit)) + 1) >> 1 }, (_, i) => 2 * i + 1);

export function field(math: Mrly, made: Made, value: Stacking): Field {
  const { code, base } = made;
  const { limit, combine, depth, size } = value;
  return fields(`${code} ${base} ${limit} ${combine} ${depth} ${size}`, () => {
    const spec = math.moire.Spec.new(code, base, DIM);
    const scales = Uint32Array.from(odd(limit));
    const volume = math.moire.volume(spec, scales, MERGE[combine] ?? MERGE.sum, depth, size);
    return {
      volume,
      layers: scales.length,
      voxels: volume.data.length,
      min: volume.min(),
      max: volume.max(),
      free: () => volume.free(),
    };
  });
}

export function most(math: Mrly, value: Coded & Stacking): number {
  return field(math, design(math, value), value).max;
}

export function colorizer(ink: Ink, ramp: RampName, levels: number): Colorizer {
  const line = (ink.Ramp[ramp] ?? ink.Ramp.fire)();
  const count = Math.max(2, Math.floor(levels));
  return {
    Bins: { background: hex(line.at(0)), ramp: Array.from({ length: count }, (_, i) => hex(line.at(i / (count - 1)))) },
  };
}

export function plane(math: Mrly, made: Field, key: Axis, offset: number, paint: Paint): Plane {
  return derived(
    `plane ${keyOf(made)} ${key} ${offset} ${paint.levels} ${paint.invert} ${JSON.stringify(paint.colorizer)}`,
    () => {
      const [, , normal] = PLANES.find(([one]) => one === key)!;
      const frame = math.moire.frame(normal, offset);
      const wide = WIDE[key];
      const [values, mask] = made.volume.plane(frame, wide);
      const sheet = math.moire.Field.from_data(values, wide);
      const image = math.moire.render(
        sheet,
        paint.colorizer,
        Math.max(2, Math.floor(paint.levels)),
        false,
        Number(paint.invert) === 1,
        1,
      );
      sheet.free();
      const colors = Uint8Array.from(image.colors);
      for (let i = 0; i < mask.length; i++) if (!mask[i]) colors[i * 4 + 3] = 0;
      return { key, offset, frame, pixels: { shape: image.shape, colors } };
    },
  );
}

export function shell(math: Mrly, made: Field, value: Skin): Shell {
  return derived(
    `shell ${keyOf(made)} ${value.threshold} ${value.crop} ${value.radius} ${value.anti} ${value.policy}`,
    () => {
      const solid = flat(made.volume.solid(value.threshold));
      const cell = value.crop ? cropped(math, solid, value).cell : solid;
      return { cell, count: made.volume.count(value.threshold), faces: n(math.three.surface(cell)) };
    },
  );
}
