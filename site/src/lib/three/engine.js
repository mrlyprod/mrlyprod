import { hex } from "../color.js";
import { canon } from "../designs/engine.js";

export const DIM = 3;
export const CARPET = "23";
export const NUMBERS = [2, 3, 4, 5];
export const SHAPES = ["ball", "box", "diamond", "octahedron", "tetrahedron", "pyramid"];
export const DEN = 120;
export const REACH = 108;
export const SIDE = 128;
export const CROPPED = 81;
export const FACES = 400000;
export const CUBES = 250000;
export const TALLY = 600000;
export const COUNTED = 600000;
export const LAYERS = 21;
export const COMBINES = [
  ["sum", "Sum"],
  ["xor", "Xor"],
  ["and", "And"],
];
export const SIZES = [48, 64, 80];
export const RAMPS = [
  ["fire", "Fire"],
  ["heat", "Heat"],
  ["diverge", "Diverge"],
];
export const PLANES = [
  ["x", "X plane", [1, 0, 0]],
  ["y", "Y plane", [0, 1, 0]],
  ["z", "Z plane", [0, 0, 1]],
  ["d", "Diagonal", [1, 1, 1]],
];
export const WIDE = { x: 256, y: 256, z: 256, d: 384 };
const MERGE = { sum: "Sum", xor: "Xor", and: "And" };

const n = (v) => Number(v);

/* MEMO */

export function memo(limit = 8) {
  const kept = new Map();
  return (key, make) => {
    if (kept.has(key)) {
      const one = kept.get(key);
      kept.delete(key);
      kept.set(key, one);
      return one;
    }
    const one = make();
    kept.set(key, one);
    while (kept.size > limit) {
      const [first, old] = kept.entries().next().value;
      kept.delete(first);
      old?.free?.();
    }
    return one;
  };
}

const cells = memo(6);
const derived = memo(16);
const fields = memo(3);
const meshes = memo(2);
const ids = new WeakMap();
let next = 0;

function keyOf(one) {
  if (!ids.has(one)) ids.set(one, ++next);
  return ids.get(one);
}

/* DESIGN */

export const cap = (math, number, crop) => math.bang.level_cap(number, 1, crop ? CROPPED : SIDE);

export const counted = (math, number) => math.bang.level_cap(number, DIM, COUNTED);

export const side = (math, number, level) => n(math.counts.grid(number, 1, level));

export function top(math, number, count, centre) {
  const edge = side(math, number, Math.min(count, counted(math, number)));
  return centre === "centre" ? Math.floor((edge - 1) / 2) : edge - 1;
}

export function design(math, value) {
  const base = Number(value.base) === 3 ? 3 : 2;
  const text = String(value.code ?? "").trim();
  return { base, code: canon(math, DIM, base, text) };
}

export function named(math, code, base) {
  return math.name.of(code, DIM, base) ?? "";
}

export function tag(name, code, base) {
  return `${base === 3 ? "b3 " : ""}${name || "code"} ${code}`;
}

export function grown(math, code, base, number, level) {
  return cells(`${code} ${base} ${number} ${level}`, () => math.three.create(code, number, level, base));
}

export const tensor = (cell) => ({ shape: cell.shape, data: cell.types });

export const flat = (kept) => ({ shape: kept.shape, types: kept.data });

export function faces(math, cell) {
  return meshes(keyOf(cell), () => math.three.quads(cell));
}

export function tally(math, cell) {
  return derived(`tally ${keyOf(cell)}`, () => (cell.types.length <= TALLY ? math.three.census(cell) : null));
}

export function load(math, cell, cubes) {
  return derived(`load ${keyOf(cell)} ${cubes}`, () => (cubes ? math.three.fills(cell) : n(math.three.surface(cell))));
}

/* CROP */

export function region(math, name, radius, anti, den = DEN) {
  const frac = new math.shape.Frac(radius, den);
  const core = math.shape.named(name, DIM, frac);
  frac.free();
  return anti ? { Anti: core } : core;
}

export function walls(core) {
  const list = core?.Polytope?.walls;
  if (!list) return null;
  return list.map(({ normal, offset }) => {
    const size = Math.hypot(...normal);
    const reach = n(offset.num) / n(offset.den);
    const sum = normal.reduce((a, b) => a + b, 0);
    return { normal: normal.map((one) => -one / size), constant: (2 * reach - sum) / size };
  });
}

export function cropped(math, cell, value) {
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

export function circle(math, made, value) {
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

export function reading(count, r) {
  const at = Math.max(1, Math.min(r, count.top));
  return { at, seen: count.seen[at], inside: count.inside[at], cut: count.cut[at] };
}

/* FIELD */

export const odd = (limit) => Array.from({ length: (Math.max(1, Math.floor(limit)) + 1) >> 1 }, (_, i) => 2 * i + 1);

export function field(math, made, value) {
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

export function most(math, value) {
  return field(math, design(math, value), value).max;
}

export function colorizer(ink, ramp, levels) {
  const line = (ink.Ramp[ramp] ?? ink.Ramp.fire)();
  const count = Math.max(2, Math.floor(levels));
  return {
    Bins: { background: hex(line.at(0)), ramp: Array.from({ length: count }, (_, i) => hex(line.at(i / (count - 1)))) },
  };
}

export function plane(math, made, key, offset, paint) {
  return derived(
    `plane ${keyOf(made)} ${key} ${offset} ${paint.levels} ${paint.invert} ${JSON.stringify(paint.colorizer)}`,
    () => {
      const [, , normal] = PLANES.find(([one]) => one === key);
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

export function shell(math, made, value) {
  return derived(
    `shell ${keyOf(made)} ${value.threshold} ${value.crop} ${value.radius} ${value.anti} ${value.policy}`,
    () => {
      const solid = flat(made.volume.solid(value.threshold));
      const cell = value.crop ? cropped(math, solid, value).cell : solid;
      return { cell, count: made.volume.count(value.threshold), faces: n(math.three.surface(cell)) };
    },
  );
}
