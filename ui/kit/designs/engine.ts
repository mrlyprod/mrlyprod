import type * as bang from "@mrlyprod/bang";
import type { Cell } from "@mrlyprod/math";
import type { Had } from "../hooks.ts";
import type { Rng } from "../rand.ts";
import { pick, rng } from "../rand.ts";

export const CRATES = [
  "arcs",
  "automaton",
  "bang",
  "chaos",
  "chladni",
  "crop",
  "graph",
  "math",
  "moire",
  "radix",
  "roulette",
  "sequences",
  "six",
  "spin",
  "star",
  "thue",
  "three",
  "tourbillon",
  "two",
] as const;

export type Crate = (typeof CRATES)[number];

export type Mrly = Had<Crate>;

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

export const cap = (mrly: Mrly, dim: number) => mrly.math.factory.level_cap(SIDE, dim, BUDGET);

export function total(mrly: Mrly, dim: number, base: number): number {
  return Number(mrly.math.factory.total_codes(dim, base));
}

export function distinct(mrly: Mrly, dim: number, base: number): number {
  return Number(mrly.bang.baseq.distinct_designs(base, dim));
}

export function codes(mrly: Mrly, dim: number, base: number): string[] | null {
  const count = total(mrly, dim, base);
  return count <= LIMIT ? Array.from({ length: count }, (_, code) => String(code)) : null;
}

export function roll(mrly: Mrly, dim: number, base: number, rand: Rng): string {
  return mrly.math.factory.random_code(dim, base, new mrly.math.Rng(Math.floor(rand() * 2 ** 32)), 1);
}

const wrap = (mrly: Mrly, dim: number, base: number, text: string) => {
  const count = total(mrly, dim, base);
  return String(((Math.floor(Number(text)) % count) + count) % count);
};

export function settle(mrly: Mrly, dim: number, base: number, code: Code | null | undefined, rand: Rng): string {
  const text = String(code ?? "").trim();
  return text === "" || !Number.isFinite(Number(text)) ? roll(mrly, dim, base, rand) : wrap(mrly, dim, base, text);
}

export function home(mrly: Mrly, dim: number, base: number): string {
  const words = mrly.math.name.words(dim, base);
  return (words.find(([, name]) => name === HOME) ?? words[0])?.[0] ?? "0";
}

export function canon(mrly: Mrly, dim: number, base: number, code: Code | null | undefined): string {
  const text = String(code ?? "").trim();
  return text === "" || !Number.isFinite(Number(text)) ? home(mrly, dim, base) : wrap(mrly, dim, base, text);
}

export function drawn(
  mrly: Mrly,
  dim: number,
  base: number,
  list: readonly string[] | null | undefined,
  code: Code | null | undefined,
  seed: number,
): string {
  const rand = rng(seed);
  const others = (list ?? []).filter((one) => one !== String(code));
  return others.length ? pick(rand, others) : roll(mrly, dim, base, rand);
}

export function sequence(mrly: Mrly, base: Base): Run {
  if (!runs.has(base))
    runs.set(base, { upto: UPTO[base], terms: mrly.bang.baseq.sequence(base, UPTO[base]).map(String) });
  return runs.get(base)!;
}

function universe(mrly: Mrly, dim: number): bang.Universe {
  if (!worlds.has(dim)) worlds.set(dim, mrly.bang.bang(dim));
  return worlds.get(dim)!;
}

function group(mrly: Mrly, dim: number, base: number): Uint32Array[] {
  const key = `${dim}:${base}`;
  if (!groups.has(key)) groups.set(key, mrly.bang.baseq.group(base, dim));
  return groups.get(key)!;
}

function orbits(mrly: Mrly, dim: number, base: number): Orbit[] | null {
  if (base === 2) {
    return mrly.bang.universe_codes(dim).map((code) => ({ code, orbit: facts(mrly, dim, base, code).orbit }));
  }
  try {
    return mrly.bang.baseq.representatives(base, dim).map(([code, orbit]) => ({ code, orbit }));
  } catch {
    return null;
  }
}

export function classes(mrly: Mrly, dim: number, base: number): Orbit[] | null {
  const key = `${dim}:${base}`;
  if (!reps.has(key)) reps.set(key, orbits(mrly, dim, base));
  return reps.get(key)!;
}

export function facts(mrly: Mrly, dim: number, base: number, code: Code): Facts {
  if (base === 2 && dim > 1) {
    const design = universe(mrly, dim).design(code);
    const read = { rep: design.class_rep, orbit: design.orbit_size, anf: design.anf(), degree: design.degree() };
    design.free();
    return read;
  }
  const maps = group(mrly, dim, base);
  const two = base === 2;
  return {
    rep: mrly.bang.baseq.canonical(maps, code),
    orbit: mrly.bang.baseq.orbit(maps, code).length,
    anf: two ? mrly.bang.universe.anf_string(code, dim) : "",
    degree: two ? mrly.bang.universe.degree(code, dim) : -1,
  };
}

export function title(mrly: Mrly, dim: number, base: number, code: Code): string {
  const bang = new mrly.math.name.Bang(code, dim, base);
  const text = bang.to_prose();
  bang.free();
  return text;
}

export function grow(mrly: Mrly, dim: number, base: number, code: Code, level: number): Cell {
  if (dim === 1) {
    const { shape, data } = mrly.math.factory.create(code, SIDE, 1, base, level);
    return { shape: [1, shape[0]], types: data };
  }
  return dim === 2 ? mrly.two.create(code, SIDE, level, ROTATION, base) : mrly.three.create(code, SIDE, level, base);
}

export function census(mrly: Mrly, dim: number, cell: Cell): Census {
  if (dim === 2) {
    const { fills, voids, perimeter, vertices, edges, euler } = mrly.two.census(cell);
    return { fills, voids, vertices, edges, euler, perimeter: Number(perimeter) };
  }
  const { fills, voids, surface, vertices, edges, faces, euler } = mrly.three.census(cell);
  return { fills, voids, vertices, edges, euler, surface: Number(surface), faces };
}

export function dimension(mrly: Mrly, dim: number, base: number, code: Code): number {
  return mrly.sequences.counts.dimension(code, SIDE, dim, base);
}

export function gallery(mrly: Mrly, dim: number, base: number, filter: string | undefined): string[] | null {
  if (filter === "named") return mrly.math.name.words(dim, base).map(([code]) => code);
  if (filter === "classes") return classes(mrly, dim, base)?.map((one) => one.code) ?? null;
  return codes(mrly, dim, base);
}

export function next(list: readonly string[] | null | undefined, code: Code, by: number): string {
  if (!list?.length) return String(code);
  const at = list.indexOf(String(code));
  if (at < 0) return list[0];
  return list[(at + by + list.length) % list.length];
}
