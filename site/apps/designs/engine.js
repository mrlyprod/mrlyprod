export const SIDE = 3;
export const LIMIT = 512;
const BUDGET = 60000;
const ROTATION = 0;

const worlds = new Map();
const groups = new Map();
const words = new Map();
const reps = new Map();

/* SPACE */

export function cap(dim) {
  let level = 1;
  while (SIDE ** (dim * (level + 1)) <= BUDGET) level++;
  return level;
}

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

export function resolve(code, count, rand) {
  const text = String(code ?? '').trim();
  const n = Number(text);
  if (text === '' || !Number.isFinite(n)) return 1 + Math.floor(rand() * (count - 1));
  const whole = Math.floor(n);
  return ((whole % count) + count) % count;
}

export function roll(math, dim, base, rand = Math.random) {
  return String(1 + Math.floor(rand() * (total(math, dim, base) - 1)));
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
  const text = bang.to_mrly();
  bang.free();
  return text;
}

/* NAMES */

const corners = (math, cell, dim, base) => math.bang.factory.residue_corners(dim, base).filter((_, i) => cell.types[i]);

const seeds = {
  2: (math, word, number) => math.two.named(word, number, 1, ROTATION),
  3: (math, word, number) => math.three.named(word, number, 1),
};

export function named(math, dim, base) {
  const key = `${dim}:${base}`;
  if (!words.has(key)) {
    const list = [...math.bang.sources('Classics', dim).map((one) => one.design), ...math.bang.catalog.antis(dim)];
    words.set(key, list.map((word) => ({ code: math.bang.corners_to_code(corners(math, seeds[dim](math, word, base), dim, base), dim, base), name: word.toLowerCase() })));
  }
  return words.get(key);
}

export function word(list, code) {
  return list.find((one) => one.code === String(code))?.name ?? '';
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
  if (filter === 'named') return named(math, dim, base).map((one) => one.code);
  if (filter === 'classes') return classes(math, dim, base)?.map((one) => one.code) ?? null;
  return codes(math, dim, base);
}

export function next(list, code, by) {
  if (!list?.length) return String(code);
  const at = list.indexOf(String(code));
  if (at < 0) return list[0];
  return list[(at + by + list.length) % list.length];
}
