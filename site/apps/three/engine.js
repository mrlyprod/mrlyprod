import { tidy } from '../../lib/knobs.js';
import { named, resolve, title, word } from '../designs/engine.js';
import { DESIGN } from '../designs/scene.js';

export const DIM = 3;
export const DEN = 32;
export const BUDGET = 60000;
export const NUMBERS = [2, 3, 4, 5];
export const SHAPES = ['ball', 'box', 'diamond', 'octahedron', 'tetrahedron', 'pyramid'];
export const VIEWS = [['solid', 'Solid'], ['slice', 'Slice'], ['diagonal', 'Diagonal'], ['hex', 'Hexagon']];
export const AXES = ['x', 'y', 'z'];
export const USES = { solid: [], slice: ['axis', 'at'], diagonal: ['at'], hex: [] };

const uses = (view, key) => USES[view]?.includes(key) ?? false;

export const GROW = [
  { key: 'number', label: 'Number', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: 'Grow' },
  { key: 'level', label: 'Level', kind: 'slider', def: 3, min: 1, max: (value) => cap(value.number), step: 1, group: 'Grow' },
];

export const CUT = [
  { key: 'view', label: 'View', kind: 'pick', def: 'solid', options: VIEWS, group: 'Cut' },
  { key: 'axis', label: 'Axis', kind: 'segment', def: 2, options: AXES.map((axis, i) => [i, axis.toUpperCase()]), group: 'Cut', when: (value) => uses(value.view, 'axis') },
  { key: 'at', label: 'Plane', kind: 'slider', def: 0.5, min: 0, max: 1, step: 0.01, group: 'Cut', when: (value) => uses(value.view, 'at') },
];

const OWN = [...GROW, ...CUT].map((row) => row.key);

const count = (v) => Number(v);

/* DESIGN */

export function cap(number) {
  let level = 1;
  while (number ** (DIM * (level + 1)) <= BUDGET) level++;
  return level;
}

export function total(math, base) {
  return Number(math.bang.factory.total_codes(DIM, base));
}

export function tag(facts) {
  return `${facts.base === 3 ? 'b3 ' : ''}${facts.name || 'code'} ${facts.code}`;
}

export function grow(value) {
  return tidy(GROW, value);
}

export function rows(view) {
  return CUT.filter((row) => !row.when || row.when({ view }));
}

export function cutting(value) {
  const all = tidy(CUT, value);
  return Object.fromEntries(rows(all.view).map((row) => [row.key, all[row.key]]));
}

export function staged(value) {
  const kept = Object.entries(value).filter(([key]) => !OWN.includes(key));
  return { ...Object.fromEntries(kept), ...grow(value), ...cutting(value) };
}

export function design(math, value, rand) {
  const { base, code } = tidy(DESIGN, { ...value, dim: DIM });
  return { base, code: String(resolve(code, total(math, base), rand)) };
}

export function crop(math, grown, name, radius) {
  const frac = new math.shape.Frac(radius, DEN);
  const shape = math.shape.named(name, DIM, frac);
  frac.free();
  const tensor = { shape: grown.shape, data: grown.types };
  const kept = math.shape.crop(tensor, shape, true);
  const { filled } = math.shape.census(shape, tensor);
  return { cell: { shape: kept.shape, types: kept.data }, out: filled[0], cut: filled[1], in: filled[2] };
}

/* CUTS */

export function slice(math, plan, value) {
  const side = plan.cell.shape[0];
  const index = Math.round(value.at * (side - 1));
  const flat = math.three.slice(plan.cell, value.axis, index);
  const read = math.two.census(flat);
  return { kind: 'slice', flat, side, facts: { reading: `${AXES[value.axis]} ${index + 1} of ${side}`, fills: read.fills, voids: read.voids, euler: count(read.euler) } };
}

export function diagonal(math, plan, value) {
  const { code, number, level, base, cell } = plan;
  const counts = math.three.profile(code, number, level, base).map(count);
  const support = math.three.support(counts) ?? null;
  const [low, high] = support ?? [0, 0];
  const height = low + Math.round(value.at * (high - low));
  const side = cell.shape[0];
  const listed = support ? math.three.diagonal_slice(code, number, level, base, height) : [];
  const kept = listed.filter(([x, y, z]) => cell.types[(x * side + y) * side + z]);
  const points = kept.map((at) => math.three.project(at));
  const span = support ? counts.slice(low, high + 1) : [0];
  const facts = { reading: `height ${height} of ${high}`, cells: kept.length, support: support ? `${low} to ${high}` : 'none', least: Math.min(...span), most: Math.max(...span), constant: span.every((c) => c === span[0]) ? 'yes' : 'no' };
  return { kind: 'diagonal', points, height, facts };
}

export function hexagon(math, plan) {
  const hex = math.six.cut(plan.cell);
  const read = math.six.census(hex, false);
  const codes = { fill: math.six.FILL(), void: math.six.VOID() };
  const facts = { reading: `hexagon of ${plan.cell.shape[0]}`, triangles: read.triangles, fills: read.fills, voids: read.voids, pieces: math.six.components(hex), holes: math.six.holes(hex), euler: count(read.euler) };
  return { kind: 'hex', hex, codes, facts };
}

const CUTS = { slice, diagonal, hex: hexagon };

/* STUDY */

export function study(math, value, rand) {
  const { base, code } = design(math, value, rand);
  const { number, level } = grow(value);
  const cut = cutting(value);
  const grown = math.three.create(code, number, level, base);
  const shaped = value.crop ? crop(math, grown, value.crop, value.radius) : null;
  const cell = shaped ? shaped.cell : grown;
  const read = math.three.census(cell);
  const facts = {
    code,
    base,
    number,
    level,
    side: cell.shape[0],
    name: word(named(math, DIM, base), code),
    title: title(math, DIM, base, code),
    fills: read.fills,
    voids: read.voids,
    surface: count(read.surface),
    faces: read.faces,
    euler: read.euler,
    dimension: math.counts.dimension(code, number, DIM, base),
    crop: shaped ? { in: shaped.in, cut: shaped.cut, out: shaped.out } : null,
  };
  const plan = { code, base, number, level, cell };
  return { cell, facts, cut: CUTS[cut.view]?.(math, plan, cut) ?? null };
}
