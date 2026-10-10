import { named, word } from '../designs/engine.js';

export const BOARD = 128;
export const LINE = 255;
export const ROWS = 1024;
export const MASK = 27;
export const CODES = 16;
export const KEEP = 16;

const SHAPES = {
  glider: [[1, 0], [2, 1], [0, 2], [1, 2], [2, 2]],
  pentomino: [[1, 0], [2, 0], [0, 1], [1, 1], [1, 2]],
  blinker: [[0, 0], [1, 0], [2, 0]],
  block: [[0, 0], [1, 0], [0, 1], [1, 1]],
};

export const STARTS = [['noise', 'Noise'], ['design', 'Design'], ['one', 'One'], ['glider', 'Glider'], ['pentomino', 'R-pentomino'], ['blinker', 'Blinker'], ['block', 'Block']];

/* COUNTS */

export function parse(text, limit = 9999) {
  const clean = String(text ?? '').trim();
  const out = new Set();
  if (/^\d+$/.test(clean)) for (const ch of clean) out.add(+ch);
  else {
    for (const part of clean.match(/\d+(?:-\d+)?/g) ?? []) {
      const [a, b = a] = part.split('-').map(Number);
      for (let n = Math.min(a, b); n <= Math.min(limit, Math.max(a, b)); n++) out.add(n);
    }
  }
  return [...out].filter((n) => n <= limit).sort((a, b) => a - b);
}

export function spell(counts) {
  if (!counts.length) return '-';
  if (counts.every((n) => n < 10)) return counts.join('');
  const runs = [];
  for (const n of counts) {
    const last = runs[runs.length - 1];
    if (last && last[1] === n - 1) last[1] = n;
    else runs.push([n, n]);
  }
  return runs.map(([a, b]) => (a === b ? `${a}` : `${a}-${b}`)).join(',');
}

export function sources(life) {
  return life.Source.all().filter((seq) => !seq.is_random()).map((seq) => seq.name());
}

export function drawn(life, text, limit) {
  try {
    const seq = life.Source.parse(String(text ?? '').trim().toLowerCase());
    const name = seq.name();
    const counts = life.Counts.drawn(seq, false, false);
    const list = Array.from(counts.values(limit));
    counts.free();
    return { counts: list, name };
  } catch {
    return null;
  }
}

function side(life, text, limit) {
  const made = drawn(life, text, limit);
  if (made) return made;
  const counts = parse(text, limit);
  return { counts, name: spell(counts) };
}

export function ruleOf(life, value, cells) {
  const birth = side(life, value.born, cells);
  const survive = side(life, value.stay, cells);
  return { birth: birth.counts, survive: survive.counts, name: `B${birth.name}/S${survive.name}` };
}

/* MASK */

export function codeOf(text) {
  const n = Number.parseInt(String(text), 10);
  return Number.isInteger(n) ? Math.min(CODES - 1, Math.max(0, n)) : 7;
}

export function levelOf(number, level, cap) {
  let l = Math.max(1, Math.floor(Number(level) || 1));
  while (l > 1 && number ** l > cap) l--;
  return l;
}

export function fitLevel(number, cap) {
  let l = 1;
  while (number ** (l + 1) <= cap) l++;
  return l;
}

export function maskOf(life, value) {
  const code = codeOf(value.code);
  const number = Number(value.number) || 3;
  const level = levelOf(number, value.level, MASK);
  const mask = life.design_mask(2, code, number, level);
  let cells = 0;
  for (const bit of mask.data) cells += bit;
  return { code, number, level, mask, cells };
}

export function maskName(math, code) {
  return word(named(math, 2, 2), code);
}

/* SEEDS */

function place(types, width, design, side, top, left) {
  for (let r = 0; r < side; r++) for (let c = 0; c < side; c++) types[(top + r) * width + left + c] = design[r * side + c];
}

export function sow(value, rand, math) {
  const flat = value.mode !== 'wolfram';
  const width = flat ? BOARD : LINE;
  const height = flat ? BOARD : 1;
  const types = new Uint8Array(width * height);
  const from = value.from;
  const shape = SHAPES[from];
  if (from === 'one') types[(height >> 1) * width + (width >> 1)] = 1;
  else if (shape) {
    const left = (width >> 1) - 1;
    const top = (height >> 1) - 1;
    const mid = Math.max(...shape.map(([, y]) => y)) >> 1;
    for (const [x, y] of shape) {
      if (flat) types[(top + y) * width + left + x] = 1;
      else if (y === mid) types[left + x] = 1;
    }
  } else if (from === 'design') {
    const number = Number(value.number) || 3;
    const side = number ** fitLevel(number, width);
    const design = math.two.create(codeOf(value.code), number, fitLevel(number, width), 0, 2).types;
    const left = (width - side) >> 1;
    if (flat) place(types, width, design, side, left, left);
    else types.set(design.subarray((side >> 1) * side, (side >> 1) * side + side), left);
  } else {
    const density = Number(value.density);
    for (let i = 0; i < types.length; i++) types[i] = rand() < density ? 1 : 0;
  }
  return types;
}

/* FATE */

export function population(types) {
  let n = 0;
  for (let i = 0; i < types.length; i++) n += types[i];
  return n;
}

export function hash(types) {
  let h = 2166136261;
  for (let i = 0; i < types.length; i++) h = Math.imul(h ^ types[i], 16777619);
  return h >>> 0;
}

export function fate(seen, h, pop) {
  const at = seen.lastIndexOf(h);
  const verdict = pop === 0 ? 'dead' : at < 0 ? '' : at === seen.length - 1 ? 'still' : `loop ${seen.length - at}`;
  seen.push(h);
  if (seen.length > KEEP) seen.shift();
  return verdict;
}
