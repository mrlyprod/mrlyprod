import type * as mrly from "@mrlyprod/mrly/math";
import type { bang, Cell } from "@mrlyprod/mrly/math";
import type { Rng } from "../rand.ts";
import { pick, rng } from "../rand.ts";

/* TYPES */

export type Mrly = typeof mrly;

export type Code = string | number | bigint;

export type Option = [number, string];

export type Choice = { key: string; label: string; kind: "segment"; def: number; options: Option[]; group: string };

export type Field = { key: string; label: string; kind: "text"; def: string; group: string };

export type Rows = [Choice, Choice, Field];

export type Orbit = { code: string; orbit: number };

export type Facts = { rep: string; orbit: number; anf: string; degree: number };

export type Run = { upto: number; terms: string[] };

export type Census = {
  fills: number;
  voids: number;
  vertices: number;
  edges: number;
  euler: number;
  perimeter?: number;
  surface?: number;
  faces?: number;
};

export const DESIGN: Rows = [
  {
    key: "dim",
    label: "Dim",
    kind: "segment",
    def: 2,
    options: [
      [2, "2D"],
      [3, "3D"],
    ],
    group: "Design",
  },
  {
    key: "base",
    label: "Base",
    kind: "segment",
    def: 2,
    options: [
      [2, "2"],
      [3, "3"],
    ],
    group: "Design",
  },
  { key: "code", label: "Code", kind: "text", def: "", group: "Design" },
];

export const SIDE = 3;
export const LIMIT = 512;
const BUDGET = 60000;
const ROTATION = 0;
const HOME = "carpet";
const UPTO = { 2: 4, 3: 2 };

export type Base = keyof typeof UPTO;

const worlds = new Map<number, bang.Universe>();
const groups = new Map<string, Uint32Array[]>();
const reps = new Map<string, Orbit[] | null>();
const runs = new Map<number, Run>();

/* SPACE */

export const cap = (math: Mrly, dim: number) => math.bang.level_cap(SIDE, dim, BUDGET);

export function total(math: Mrly, dim: number, base: number): number {
  return Number(math.bang.factory.total_codes(dim, base));
}

export function distinct(math: Mrly, dim: number, base: number): number {
  return Number(math.bang.baseq.distinct_designs(base, dim));
}

export function codes(math: Mrly, dim: number, base: number): string[] | null {
  const count = total(math, dim, base);
  return count <= LIMIT ? Array.from({ length: count }, (_, code) => String(code)) : null;
}

export function roll(math: Mrly, dim: number, base: number, rand: Rng = Math.random): string {
  return math.bang.random_code(dim, base, new math.Rng(Math.floor(rand() * 2 ** 32)), 1);
}

export function settle(math: Mrly, dim: number, base: number, code: Code | null | undefined, rand?: Rng): string {
  const text = String(code ?? "").trim();
  if (text === "" || !Number.isFinite(Number(text))) return roll(math, dim, base, rand);
  const count = total(math, dim, base);
  return String(((Math.floor(Number(text)) % count) + count) % count);
}

export function home(math: Mrly, dim: number, base: number): string {
  const words = math.name.words(dim, base);
  return (words.find(([, name]) => name === HOME) ?? words[0])?.[0] ?? "0";
}

export function canon(math: Mrly, dim: number, base: number, code: Code | null | undefined): string {
  const text = String(code ?? "").trim();
  return text === "" || !Number.isFinite(Number(text)) ? home(math, dim, base) : settle(math, dim, base, text);
}

export function drawn(
  math: Mrly,
  dim: number,
  base: number,
  list: readonly string[] | null | undefined,
  code: Code | null | undefined,
  seed?: number | null,
): string {
  const rand = rng(seed);
  const others = (list ?? []).filter((one) => one !== String(code));
  return others.length ? pick(rand, others) : roll(math, dim, base, rand);
}

export function sequence(math: Mrly, base: Base): Run {
  if (!runs.has(base))
    runs.set(base, { upto: UPTO[base], terms: math.bang.baseq.sequence(base, UPTO[base]).map(String) });
  return runs.get(base)!;
}

/* CLASSES */

function universe(math: Mrly, dim: number): bang.Universe {
  if (!worlds.has(dim)) worlds.set(dim, math.bang.bang(dim));
  return worlds.get(dim)!;
}

function group(math: Mrly, dim: number, base: number): Uint32Array[] {
  const key = `${dim}:${base}`;
  if (!groups.has(key)) groups.set(key, math.bang.baseq.group(base, dim));
  return groups.get(key)!;
}

function orbits(math: Mrly, dim: number, base: number): Orbit[] | null {
  if (base === 2) {
    return math.bang.universe_codes(dim).map((code) => ({ code, orbit: facts(math, dim, base, code).orbit }));
  }
  try {
    return math.bang.baseq.representatives(base, dim).map(([code, orbit]) => ({ code, orbit }));
  } catch {
    return null;
  }
}

export function classes(math: Mrly, dim: number, base: number): Orbit[] | null {
  const key = `${dim}:${base}`;
  if (!reps.has(key)) reps.set(key, orbits(math, dim, base));
  return reps.get(key)!;
}

export function facts(math: Mrly, dim: number, base: number, code: Code): Facts {
  if (base === 2 && dim > 1) {
    const design = universe(math, dim).design(code);
    const read = { rep: design.class_rep, orbit: design.orbit_size, anf: design.anf(), degree: design.degree() };
    design.free();
    return read;
  }
  const maps = group(math, dim, base);
  const two = base === 2;
  return {
    rep: math.bang.baseq.canonical(maps, code),
    orbit: math.bang.baseq.orbit(maps, code).length,
    anf: two ? math.bang.universe.anf_string(code, dim) : "",
    degree: two ? math.bang.universe.degree(code, dim) : -1,
  };
}

export function title(math: Mrly, dim: number, base: number, code: Code): string {
  const bang = new math.name.Bang(code, dim, base);
  const text = bang.to_prose();
  bang.free();
  return text;
}

/* GROWTH */

export function grow(math: Mrly, dim: number, base: number, code: Code, level: number): Cell {
  if (dim === 1) {
    const { shape, data } = math.bang.factory.create(code, SIDE, 1, base, level);
    return { shape: [1, shape[0]], types: data };
  }
  return dim === 2 ? math.two.create(code, SIDE, level, ROTATION, base) : math.three.create(code, SIDE, level, base);
}

export function census(math: Mrly, dim: number, cell: Cell): Census {
  if (dim === 2) {
    const { fills, voids, perimeter, vertices, edges, euler } = math.two.census(cell);
    return { fills, voids, vertices, edges, euler, perimeter: Number(perimeter) };
  }
  const { fills, voids, surface, vertices, edges, faces, euler } = math.three.census(cell);
  return { fills, voids, vertices, edges, euler, surface: Number(surface), faces };
}

export function dimension(math: Mrly, dim: number, base: number, code: Code): number {
  return math.counts.dimension(code, SIDE, dim, base);
}

/* LISTS */

export function gallery(math: Mrly, dim: number, base: number, filter: string | undefined): string[] | null {
  if (filter === "named") return math.name.words(dim, base).map(([code]) => code);
  if (filter === "classes") return classes(math, dim, base)?.map((one) => one.code) ?? null;
  return codes(math, dim, base);
}

export function next(list: readonly string[] | null | undefined, code: Code, by: number): string {
  if (!list?.length) return String(code);
  const at = list.indexOf(String(code));
  if (at < 0) return list[0];
  return list[(at + by + list.length) % list.length];
}
