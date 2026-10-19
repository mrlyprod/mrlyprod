import { dark, light } from '../theme/theme.js';
import { hex, rgb } from '../color.js';
import { rng } from '../rand.js';
import { ink } from '#view';
import { canon } from '../designs/engine.js';

export const RAMPS = [['fire', 'Fire'], ['heat', 'Heat'], ['diverge', 'Diverge'], ['tint', 'Tint'], ['ink', 'Ink'], ['bands', 'Bands']];
export const COMBINES = [['sum', 'Sum'], ['and', 'And'], ['xor', 'Xor'], ['hive', 'Hive']];
export const CROPS = [['', 'Off'], ['ball', 'Ball'], ['box', 'Box'], ['diamond', 'Diamond'], ['triangle', 'Triangle'], ['octagon', 'Octagon']];
export const SIZES = [128, 256, 384, 512];
export const TOP = 41;
export const BEAT = 350;
export const DEEPEST = 5;
export const GRAIN = 32;
const DIM = 2;
const MERGE = { sum: 'Sum', and: 'And', xor: 'Xor', hive: 'Sum' };

/* SCALES */

export function odd(limit) {
  const top = Math.max(1, Math.floor(limit)) | 1;
  return Array.from({ length: (top + 1) / 2 }, (_, i) => 2 * i + 1);
}

export function climb(from, beats) {
  const steps = (TOP + 1) / 2;
  const at = Math.min(steps - 1, (Math.max(1, Math.floor(from) | 1) - 1) / 2);
  return 1 + 2 * ((at + beats) % steps);
}

/* COLOUR */

const bright = ([r, g, b]) => (r * 299 + g * 587 + b * 114) / 255000 > 0.5;

const contrast = (c) => (bright(c) ? [0, 0, 0] : [255, 255, 255]);

const along = (stops, t) => {
  const k = t * (stops.length - 1);
  const i = Math.min(stops.length - 2, Math.floor(k));
  const f = k - i;
  return stops[i].map((v, c) => v + (stops[i + 1][c] - v) * f);
};

const inks = new Map();

export function palette({ paper, accent }) {
  const key = `${paper} ${accent}`;
  if (!inks.has(key)) {
    const ground = rgb(paper);
    inks.set(key, ink({ ...(bright(ground) ? light : dark), ground, accent: rgb(accent) }));
  }
  return inks.get(key);
}

export const css = (c) => `rgba(${c[0]}, ${c[1]}, ${c[2]}, ${(c[3] ?? 255) / 255})`;

export function heat(ink, r) {
  const t = Math.min(1, Math.sqrt(Math.max(0, r)));
  return t < 0.5 ? ink.mix(ink.fg, ink.blue, t * 2) : ink.mix(ink.blue, ink.mix(ink.blue, ink.ground, 0.6), t * 2 - 1);
}

export function stops(ramp, look) {
  const paper = rgb(look.paper);
  const accent = rgb(look.accent);
  if (ramp === 'tint' || ramp === 'bands') return [paper, accent];
  if (ramp === 'ink') return [paper, accent, contrast(paper)];
  const made = palette(look).Ramp;
  return (ramp === 'heat' ? made.heat() : ramp === 'diverge' ? made.diverge() : made.fire()).stops.map((c) => c.slice(0, 3));
}

export function bins(ramp, levels, look) {
  const n = Math.max(2, Math.floor(levels));
  const list = stops(ramp, look);
  if (ramp === 'bands') return Array.from({ length: n }, (_, i) => hex(list[i % 2]));
  return Array.from({ length: n }, (_, i) => hex(along(list, i / (n - 1))));
}

export function colorizer(ramp, levels, look) {
  return { Bins: { background: hex(rgb(look.paper)), ramp: bins(ramp, levels, look) } };
}

export function paint(math, { field: made, data }, colors, levels, invert, symmetric = false) {
  const image = math.moire.render(made, colors, Math.max(2, Math.floor(levels)), symmetric, Number(invert) === 1, 1);
  const out = image.colors;
  for (let i = 0; i < data.length; i++) if (Number.isNaN(data[i])) out[i * 4 + 3] = 0;
  return { shape: image.shape, colors: out };
}

/* DESIGN */

export function design(math, value) {
  const base = Number(value.base) === 2 ? 2 : 3;
  const code = canon(math, DIM, base, value.code);
  const level = Math.min(DEEPEST, Math.max(1, Math.round(Number(value.level) || 1)));
  return { base, code, level, name: math.name.of(code, DIM, base) ?? '' };
}

const tables = new WeakMap();

export function presets(math) {
  if (!tables.has(math)) {
    tables.set(
      math,
      math.moire.all(1).map((one) => {
        const { name, spec, level, lattice, combine } = one.toJSON();
        one.free();
        return { name, base: spec.base, code: String(spec.code), level, combine: lattice === 'Hex' ? 'hive' : combine.toLowerCase() };
      }),
    );
  }
  return tables.get(math);
}

export function presetOf(list, value) {
  return list.find((one) => one.base === Number(value.base) && one.code === String(value.code) && one.level === Number(value.level) && one.combine === value.combine)?.name ?? '';
}

export function drawn(math, seed) {
  const rand = rng(seed);
  const list = presets(math);
  const { name, ...picked } = list[Math.floor(rand() * list.length)];
  return { seed, ...picked, limit: 1 + 2 * Math.floor(rand() * ((TOP + 1) / 2)) };
}

/* LAW */

function memo(size) {
  const held = new Map();
  return (key, make) => {
    if (held.has(key)) return held.get(key);
    const made = make();
    held.set(key, made);
    if (held.size > size) held.delete(held.keys().next().value);
    return made;
  };
}

const witnesses = memo(128);

const matrices = memo(8);

const witness = (math, scale) => witnesses(scale, () => math.moire.pairs.witness(scale));

export function matrix(math, limit) {
  return matrices(limit, () => {
    const scales = odd(limit).filter((n) => n >= 3);
    return { scales, grid: scales.map((a) => scales.map((b) => math.moire.pairs.correlation(a, b))) };
  });
}

export function law(math, limit, scale, mate) {
  const { scales } = matrix(math, limit);
  const row = scales.filter((n) => n <= scale).at(-1) ?? scales[0] ?? 0;
  const seen = row ? witness(math, row) : null;
  const pair = seen?.scales.includes(mate) ? mate : (seen?.scales.at(-1) ?? 0);
  const r = seen && pair ? seen.row[seen.scales.indexOf(pair)] : null;
  return { row, mate: pair, r, witness: seen, k: scales.length };
}

/* STUDY */

export function study(math, value) {
  const made = design(math, value);
  const numbers = odd(value.limit);
  return { ...made, numbers, spec: math.moire.Spec.new(made.code, made.base, DIM), merge: MERGE[value.combine] ?? 'Sum', lattice: value.combine === 'hive' ? 'Hex' : 'Square' };
}

export function swatch(math, value, size, look) {
  const made = field(math, study(math, value), size);
  const out = paint(math, made, colorizer(value.ramp, value.levels, look), value.levels, value.invert);
  made.field.free();
  return out;
}

export function crop(math, name, radius, anti, size) {
  if (!name) return null;
  const frac = new math.shape.Frac(radius, GRAIN);
  const shape = math.shape.named(name, DIM, frac);
  frac.free();
  return Uint8Array.from(math.shape.regions(shape, [size, size]).data, (region) => ((Number(anti) === 1 ? region === 0 : region > 0) ? 1 : 0));
}

export function field(math, plan, size, kept = null) {
  const made = math.moire.stack(plan.spec, plan.numbers, plan.merge, plan.level, plan.lattice, size, []);
  const data = made.data;
  if (!kept) return { field: made, data, size };
  made.free();
  for (let i = 0; i < data.length; i++) if (!kept[i]) data[i] = NaN;
  return { field: math.moire.Field.from_data(data, size), data, size };
}
