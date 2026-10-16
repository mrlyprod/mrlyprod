import { pick, rng } from '../../lib/scene.js';

export const SIDE = 3;
export const LIMIT = 512;
const BUDGET = 60000;
const ROTATION = 0;
const HOME = 'carpet';
const UPTO = { 2: 4, 3: 2 };

const worlds = new Map();
const groups = new Map();
const reps = new Map();
const runs = new Map();

/* SPACE */

export const cap = (math, dim) => math.bang.level_cap(SIDE, dim, BUDGET);

export function total(math, dim, base) {
  return Number(math.bang.factory.total_codes(dim, base));
}

export function distinct(math, dim, base) {
  return Number(math.bang.baseq.distinct_designs(base, dim));
}

export function codes(math, dim, base) {
  const count = total(math, dim, base);
  return count <= LIMIT ? Array.from({ length: count }, (_, code) => String(code)) : null;
}

export function roll(math, dim, base, rand = Math.random) {
  return math.bang.random_code(dim, base, new math.Rng(Math.floor(rand() * 2 ** 32)), 1);
}

export function settle(math, dim, base, code, rand) {
  const text = String(code ?? '').trim();
  if (text === '' || !Number.isFinite(Number(text))) return roll(math, dim, base, rand);
  const count = total(math, dim, base);
  return String(((Math.floor(Number(text)) % count) + count) % count);
}

export function home(math, dim, base) {
  const words = math.name.words(dim, base);
  return (words.find(([, name]) => name === HOME) ?? words[0])?.[0] ?? '0';
}

export function canon(math, dim, base, code) {
  const text = String(code ?? '').trim();
  return text === '' || !Number.isFinite(Number(text)) ? home(math, dim, base) : settle(math, dim, base, text);
}

export function drawn(math, dim, base, list, code, seed) {
  const rand = rng(seed);
  const others = (list ?? []).filter((one) => one !== String(code));
  return others.length ? pick(rand, others) : roll(math, dim, base, rand);
}

export function sequence(math, base) {
  if (!runs.has(base)) runs.set(base, { upto: UPTO[base], terms: math.bang.baseq.sequence(base, UPTO[base]).map(String) });
  return runs.get(base);
}

/* CLASSES */

function universe(math, dim) {
  if (!worlds.has(dim)) worlds.set(dim, math.bang.bang(dim));
  return worlds.get(dim);
}

function group(math, dim, base) {
  const key = `${dim}:${base}`;
  if (!groups.has(key)) groups.set(key, math.bang.baseq.group(base, dim));
  return groups.get(key);
}

function orbits(math, dim, base) {
  if (base === 2) {
    return math.bang.universe_codes(dim).map((code) => ({ code, orbit: facts(math, dim, base, code).orbit }));
  }
  try {
    return math.bang.baseq.representatives(base, dim).map(([code, orbit]) => ({ code, orbit }));
  } catch {
    return null;
  }
}

export function classes(math, dim, base) {
  const key = `${dim}:${base}`;
  if (!reps.has(key)) reps.set(key, orbits(math, dim, base));
  return reps.get(key);
}

export function facts(math, dim, base, code) {
  if (base === 2) {
    const design = universe(math, dim).design(code);
    const read = { rep: design.class_rep, orbit: design.orbit_size, anf: design.anf(), degree: design.degree() };
    design.free();
    return read;
  }
  const maps = group(math, dim, base);
  return { rep: math.bang.baseq.canonical(maps, code), orbit: math.bang.baseq.orbit(maps, code).length, anf: '', degree: -1 };
}

export function title(math, dim, base, code) {
  const bang = new math.name.Bang(code, dim, base);
  const text = bang.to_prose();
  bang.free();
  return text;
}

/* GROWTH */

export function grow(math, dim, base, code, level) {
  return dim === 2 ? math.two.create(code, SIDE, level, ROTATION, base) : math.three.create(code, SIDE, level, base);
}

export function census(math, dim, cell) {
  const read = dim === 2 ? math.two.census(cell) : math.three.census(cell);
  return Object.fromEntries(Object.entries(read).map(([key, value]) => [key, Number(value)]));
}

export function dimension(math, dim, base, code) {
  return math.counts.dimension(code, SIDE, dim, base);
}

/* LISTS */

export function gallery(math, dim, base, filter) {
  if (filter === 'named') return math.name.words(dim, base).map(([code]) => code);
  if (filter === 'classes') return classes(math, dim, base)?.map((one) => one.code) ?? null;
  return codes(math, dim, base);
}

export function next(list, code, by) {
  if (!list?.length) return String(code);
  const at = list.indexOf(String(code));
  if (at < 0) return list[0];
  return list[(at + by + list.length) % list.length];
}
