import type * as crop from "@mrlyprod/crop";
import type * as math from "@mrlyprod/math";
import type { Code, Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Run } from "@mrlyprod/ui/kit/svg.ts";
import { runs } from "@mrlyprod/ui/kit/svg.ts";
import { cell, DIM, tensor } from "../shared/two/engine.ts";

export type Point = [number, number];

type Round = Extract<crop.Shape, { Ball: unknown }>;

type Walled = Extract<crop.Shape, { Polytope: unknown }>;

export type Plane = (Round & { Polytope?: undefined }) | (Walled & { Ball?: undefined });

export type Masked = Plane | { Anti: Plane };

export type Outline =
  | { circle: [number, number, number]; points?: undefined }
  | { circle?: undefined; points: Point[] };

export type Census = { cells: number[]; filled: number[]; after: number };

export type Kept = { grid: math.Tensor; extra: number; over: boolean };

export type Look = { faint: Run[]; lit: Run[]; cut: Run[]; coarse: number; side: number };

export type Room = { deep: number; side: number; most: number };

export type Circle = { seen: number[]; inside: number[]; cut: number[]; side: number; mass: number; d: number };

export const RDEN = 120;
export const RMAX = 108;
export const SIDE = 243;
export const CELLS = 600000;
export const REFINE = 2 ** 21;
export const OUT = 0;
export const CUT = 1;
export const IN = 2;
export const SHAPES: [string, string][] = [
  ["ball", "Circle"],
  ["box", "Square"],
  ["diamond", "Diamond"],
  ["triangle", "Triangle"],
  ["octagon", "Octagon"],
];
export const POLICIES: [string, string][] = [
  ["inside", "Inside"],
  ["touching", "Touching"],
  ["refined1", "Refined 1"],
  ["refined2", "Refined 2"],
];
const EXTRA: Partial<Record<string, number>> = { refined1: 1, refined2: 2 };

export const top = (mrly: Mrly, number: number) => mrly.math.factory.level_cap(number, 1, SIDE);

export const counted = (mrly: Mrly, number: number) => mrly.math.factory.level_cap(number, DIM, CELLS);

export function core(mrly: Mrly, name: string, num: number, den = RDEN): Plane {
  const r = new mrly.crop.Frac(num, den);
  const made = mrly.crop.named(name, DIM, r) as Plane;
  r.free();
  return made;
}

export const shape = (mrly: Mrly, name: string, num: number, anti: boolean, den = RDEN): Masked =>
  anti ? { Anti: core(mrly, name, num, den) } : core(mrly, name, num, den);

const real = ({ num, den }: crop.FracData) => Number(num) / Number(den);

export function outline(made: Plane): Outline {
  if (made.Ball) {
    const [row, col] = made.Ball.center.map(real);
    return { circle: [col, row, real(made.Ball.radius)] };
  }
  const walls = made.Polytope.walls
    .map(({ normal: [a, b], offset }) => ({ a, b, o: real(offset), turn: Math.atan2(a, b) }))
    .sort((p, q) => p.turn - q.turn);
  const points = walls.map((p, i): Point => {
    const q = walls[(i + 1) % walls.length];
    const det = p.a * q.b - p.b * q.a;
    return [(p.a * q.o - p.o * q.a) / det, (p.o * q.b - p.b * q.o) / det];
  });
  return { points };
}

export const grid = (mrly: Mrly, code: Code, number: number, level: number, base: number) =>
  tensor(mrly.two.create(code, number, level, 0, base));

export const exposed = (mrly: Mrly, types: math.Tensor) => Number(mrly.two.perimeter(cell(types)));

export function census(mrly: Mrly, types: math.Tensor, made: crop.Shape): Census {
  const { cells, filled } = mrly.crop.census(made, types);
  return { cells, filled, after: exposed(mrly, mrly.crop.crop(types, made, true)) };
}

export function keep(mrly: Mrly, types: math.Tensor, made: crop.Shape, policy: string, number: number): Kept {
  const extra = EXTRA[policy] ?? 0;
  if (!extra) return { grid: mrly.crop.crop(types, made, policy === "touching"), extra: 0, over: false };
  const side = types.shape[0] * number ** extra;
  if (side * side > REFINE) return { grid: mrly.crop.crop(types, made, true), extra: 0, over: true };
  return { grid: mrly.crop.refine(types, made, number, extra, false), extra, over: false };
}

export function layers(types: math.Tensor, regions: math.Tensor["data"], kept: Kept, edge: boolean): Look {
  const fill = types.data;
  const own = kept.grid.data;
  const fine = kept.extra > 0;
  const pick = (test: (on: number, i: number) => unknown) =>
    runs({ shape: types.shape, types: fill.map((on, i) => (test(on, i) ? 1 : 0)) });
  const coarse = types.shape[0];
  if (edge || fine)
    return { faint: pick((on) => on), lit: runs(cell(kept.grid)), cut: [], coarse, side: kept.grid.shape[0] };
  return {
    faint: pick((on, i) => on && !own[i]),
    lit: pick((_on, i) => own[i] && regions[i] === IN),
    cut: pick((_on, i) => own[i] && regions[i] !== IN),
    coarse,
    side: coarse,
  };
}

export function room(mrly: Mrly, number: number, count: number, centre: string): Room {
  const deep = Math.min(count, counted(mrly, number));
  const side = Number(mrly.sequences.counts.grid(number, 1, deep));
  return { deep, side, most: centre === "centre" ? Math.floor((side - 1) / 2) : side - 1 };
}

export function circle(mrly: Mrly, code: Code, number: number, count: number, base: number, centre: string): Circle {
  const types = grid(mrly, code, number, count, base);
  const side = types.shape[0];
  const [origin, most]: [number[], number] =
    centre === "centre" ? [[side, side], Math.floor((side - 1) / 2)] : [[0, 0], side - 1];
  const table = mrly.crop.radial_census(types, origin, most);
  return {
    seen: table.map((row) => row.seen),
    inside: table.map((row) => row.inside),
    cut: table.map((row) => row.cut),
    side,
    mass: Number(mrly.sequences.counts.fill(code, number, DIM, 1, base)),
    d: mrly.sequences.counts.dimension(code, number, DIM, base),
  };
}
