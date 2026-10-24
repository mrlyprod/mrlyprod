import { runs } from "../../lib/app/svg.js";
import { cell, DIM, tensor } from "../../lib/two/engine.js";

export const RDEN = 120;
export const RMAX = 108;
export const SIDE = 243;
export const CELLS = 600000;
export const REFINE = 2 ** 21;
export const OUT = 0;
export const CUT = 1;
export const IN = 2;
export const SHAPES = [
  ["ball", "Circle"],
  ["box", "Square"],
  ["diamond", "Diamond"],
  ["triangle", "Triangle"],
  ["octagon", "Octagon"],
];
export const POLICIES = [
  ["inside", "Inside"],
  ["touching", "Touching"],
  ["refined1", "Refined 1"],
  ["refined2", "Refined 2"],
];
const EXTRA = { refined1: 1, refined2: 2 };

/* ROOM */

export const top = (math, number) => math.bang.level_cap(number, 1, SIDE);

export const counted = (math, number) => math.bang.level_cap(number, DIM, CELLS);

/* SHAPE */

export function core(math, name, num, den = RDEN) {
  const r = new math.shape.Frac(num, den);
  const made = math.shape.named(name, DIM, r);
  r.free();
  return made;
}

export const shape = (math, name, num, anti, den = RDEN) =>
  anti ? { Anti: core(math, name, num, den) } : core(math, name, num, den);

const real = ({ num, den }) => Number(num) / Number(den);

export function outline(made) {
  if (made.Ball) {
    const [row, col] = made.Ball.center.map(real);
    return { circle: [col, row, real(made.Ball.radius)] };
  }
  const walls = made.Polytope.walls
    .map(({ normal: [a, b], offset }) => ({ a, b, o: real(offset), turn: Math.atan2(a, b) }))
    .sort((p, q) => p.turn - q.turn);
  const points = walls.map((p, i) => {
    const q = walls[(i + 1) % walls.length];
    const det = p.a * q.b - p.b * q.a;
    return [(p.a * q.o - p.o * q.a) / det, (p.o * q.b - p.b * q.o) / det];
  });
  return { points };
}

/* DESIGN */

export const grid = (math, code, number, level, base) => tensor(math.two.create(code, number, level, 0, base));

export const exposed = (math, types) => Number(math.two.perimeter(cell(types)));

/* CENSUS */

export function census(math, types, made) {
  const { cells, filled } = math.shape.census(made, types);
  return { cells, filled, after: exposed(math, math.shape.crop(types, made, true)) };
}

export function keep(math, types, made, policy, number) {
  const extra = EXTRA[policy] ?? 0;
  if (!extra) return { grid: math.shape.crop(types, made, policy === "touching"), extra: 0, over: false };
  const side = types.shape[0] * number ** extra;
  if (side * side > REFINE) return { grid: math.shape.crop(types, made, true), extra: 0, over: true };
  return { grid: math.shape.refine(types, made, number, extra, false), extra, over: false };
}

export function layers(types, regions, kept, edge) {
  const fill = types.data;
  const own = kept.grid.data;
  const fine = kept.extra > 0;
  const pick = (test) => runs({ shape: types.shape, types: fill.map((on, i) => (test(on, i) ? 1 : 0)) });
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

/* CIRCLE */

export function room(math, number, count, centre) {
  const deep = Math.min(count, counted(math, number));
  const side = Number(math.counts.grid(number, 1, deep));
  return { deep, side, most: centre === "centre" ? Math.floor((side - 1) / 2) : side - 1 };
}

export function circle(math, code, number, count, base, centre) {
  const types = grid(math, code, number, count, base);
  const side = types.shape[0];
  const [origin, most] = centre === "centre" ? [[side, side], Math.floor((side - 1) / 2)] : [[0, 0], side - 1];
  const table = math.shape.radial_census(types, origin, most);
  return {
    seen: table.map((row) => row.seen),
    inside: table.map((row) => row.inside),
    cut: table.map((row) => row.cut),
    side,
    mass: Number(math.counts.fill(code, number, DIM, 1, base)),
    d: math.counts.dimension(code, number, DIM, base),
  };
}
