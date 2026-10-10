import { tidy } from '../../lib/knobs.js';
import { pick } from '../../lib/scene.js';
import { num as fix, runs, sheet } from '../../lib/svg.js';
import { named, resolve, title, total, word } from '../designs/engine.js';
import { DESIGN } from '../designs/scene.js';

export const DIM = 2;
export const VIEWS = [['plane', 'Plane'], ['word', 'Word'], ['lift', 'Lift'], ['filter', 'Filter']];
export const DESIGNED = ['lift', 'filter'];
export const KINDS = ['Parity', 'And', 'Xor', 'Sum'];
export const DISTINCT = ['Parity', 'And', 'Sum'];
export const FORMULA = { Parity: 't(i) xor t(j)', And: 't(i and j)', Xor: 't(i xor j)', Sum: 't(i + j)' };
export const LIFTS = [['', 'Random'], ...KINDS.map((kind) => [kind, `${kind}, ${FORMULA[kind]}`])];
export const FOLDS = [['sign', 'Plus-minus'], ['design', 'Design']];
export const NUMBERS = [2, 3, 5];
export const CELLS = [[0, 'Fit'], [1, '1 px'], [2, '2 px'], [4, '4 px'], [8, '8 px'], [16, '16 px']];
export const SIDE = 512;
export const WORD = 10;
export const BEAT = 1200;
export const SLIDE = 700;
export const END = 4000;
export const IN = 400;
export const OUT = 600;
export const PAD = 16;
export const FIT = 0.92;
export const GAP = 0.015;
export const UNIT = 10;
export const BUDGET = 65536;
export const ROW = 4;
const TRIES = 32;
const FALLBACK = '9';

export function cap(number) {
  let level = 1;
  while (number ** (level + 1) <= SIDE) level++;
  return level;
}

const based = (value) => tidy(DESIGN, value).base;

export function sides(value) {
  const base = based(value);
  return [[0, String(base)], ...NUMBERS.filter((n) => n > base).map((n) => [n, String(n)])];
}

export const sideOf = (value) => value.number || based(value);

export function reach(value) {
  if (value.view === 'word') return WORD;
  if (value.view === 'plane') return cap(2);
  return cap(sideOf(value)) - (value.view === 'filter' ? 1 : 0);
}

/* CHOICE */

export function flat(types, side) {
  let rows = true;
  let cols = true;
  for (let r = 0; r < side && (rows || cols); r++) {
    for (let c = 0; c < side; c++) {
      if (types[r * side + c] !== types[c]) rows = false;
      if (types[r * side + c] !== types[r * side]) cols = false;
    }
  }
  return rows || cols;
}

export function roll(math, number, base, rand) {
  const count = total(math, DIM, base);
  for (let i = 0; i < TRIES; i++) {
    const code = 1 + Math.floor(rand() * (count - 2));
    if (!flat(math.two.create(code, number, 1, 0, base).types, number)) return String(code);
  }
  return FALLBACK;
}

export function choose(math, value, rand) {
  const rolled = pick(rand, DISTINCT);
  const kind = KINDS.includes(value.lift) ? value.lift : rolled;
  if (!DESIGNED.includes(value.view)) return { kind, code: '' };
  const typed = String(value.code ?? '').trim();
  const code = typed ? String(resolve(typed, total(math, DIM, value.base), rand)) : roll(math, sideOf(value), value.base, rand);
  return { kind, code };
}

/* STUDY */

let kept = null;

const keyed = (value, chosen) => ({ plane: chosen.kind, word: 0, lift: [chosen.code, value.number, value.base], filter: [chosen.code, value.number, value.base, value.fold] })[value.view];

export function study(units, given, rand) {
  const value = { ...given, number: sideOf(given) };
  const chosen = choose(units.math, value, rand);
  const key = JSON.stringify([value.view, value.level, keyed(value, chosen)]);
  if (kept && kept.num === units.num && kept.math === units.math && kept.key === key) return kept.plan;
  const plan = SURVEYS[value.view](units, value, chosen);
  kept = { num: units.num, math: units.math, key, plan };
  return plan;
}

const flip = (bits) => Uint8Array.from(bits, (bit) => (bit ? 0 : 1));

const count = (grid, bit) => {
  let n = 0;
  for (let i = 0; i < grid.length; i++) if (grid[i] === bit) n++;
  return n;
};

const tally = (runs) => {
  const read = { runs: runs.length, longest: 0, singles: 0, doubles: 0 };
  for (const run of runs) {
    read.longest = Math.max(read.longest, run);
    if (run === 1) read.singles++;
    if (run === 2) read.doubles++;
  }
  return read;
};

export const print = (tile, side) => Array.from({ length: side }, (_, r) => Array.from(tile.slice(r * side, (r + 1) * side), (bit) => (bit ? '-' : '+')).join('')).join('/');

const filled = (types) => Uint8Array.from(types, (bit) => (bit ? 1 : 0));

const corner = (tile, number) => {
  const at = Array.from(tile.types).findIndex(Boolean);
  return at < 0 ? [0, 0] : [Math.floor(at / number), at % number];
};

const SURVEYS = {
  word: ({ num }, { view, level }) => {
    const m = num.morse;
    const rows = Array.from({ length: level }, (_, k) => m.stage(k + 1));
    const last = rows[level - 1];
    const boundary = m.boundary(last);
    const facts = { view, level, letters: last.length, plus: count(last, 0), minus: count(last, 1), ...tally(m.runs(last)), doubling: m.faults(boundary, m.doubling(boundary.length)) === 0, rule: m.faults(m.digits(last.length), last) === 0 };
    facts.cube = facts.longest <= 2;
    return { view, level, number: 2, side: last.length, anchor: [0, 0], panels: [], rows: rows.map(flip), facts };
  },
  plane: ({ num }, { view, level }, { kind }) => {
    const m = num.morse;
    const side = 2 ** level;
    const grid = m.lift(kind, side);
    const read = m.fold(grid, side, 2);
    const twin = kind !== KINDS[0] && m.faults(grid, m.lift(KINDS[0], side)) === 0 ? KINDS[0] : '';
    const facts = { view, level, kind, formula: m.Lift.formula(kind), side, plus: count(grid, 0), minus: count(grid, 1), folds: read.folds, faults: read.faults, first: read.first ?? null, tile: print(read.tile, 2), twin };
    return { view, level, number: 2, side, anchor: [0, 0], panels: [{ types: flip(grid) }], rows: [], facts };
  },
  lift: ({ num, math }, { view, level, number, base }, { code }) => {
    const m = num.morse;
    const tile = math.two.create(code, number, 1, 0, base);
    const design = math.two.create(code, number, level, 0, base);
    const side = design.shape[0];
    const sign = flip(tile.types);
    const power = m.power(sign, number, level);
    const types = filled(design.types);
    const agree = m.faults(types, power);
    const facts = { view, level, code, name: word(named(math, DIM, base), code), title: title(math, DIM, base, code), number, base, side, cells: side * side, filled: count(types, 1), plus: count(power, 0), agree, differ: side * side - agree, tile: print(sign, number), exact: number === 2 && m.faults(power, m.lift(KINDS[0], side)) === 0 };
    return { view, level, number, side, anchor: corner(tile, number), panels: [{ types }, { types: flip(power) }], rows: [], facts };
  },
  filter: ({ num, math }, { view, level, number, base, fold }, { code }) => {
    const m = num.morse;
    const signed = fold === 'sign';
    const tile = math.two.create(code, number, 1, 0, base);
    const sign = flip(tile.types);
    const grown = (k) => (signed ? m.power(sign, number, k) : filled(math.two.create(code, number, k, 0, base).types));
    const low = grown(level);
    const wide = m.upsample(low, number ** level, number);
    const high = grown(level + 1);
    const diff = m.difference(wide, high);
    const side = number ** (level + 1);
    const cells = side * side;
    const differ = count(diff, 1);
    const closed = signed ? m.faults(diff, m.repeat(sign, number, side)) === 0 : differ === count(low, 1) * (number * number - count(filled(tile.types), 1));
    const morse = number === 2 ? m.faults(diff, m.lift(KINDS[0], side)) : null;
    const facts = { view, level, fold, code, name: word(named(math, DIM, base), code), title: title(math, DIM, base, code), number, base, side, cells, differ, closed, morse, half: morse !== null && morse * 2 === cells, tile: print(sign, number) };
    const shown = (grid) => ({ types: signed ? flip(grid) : grid });
    return { view, level: level + 1, number, side, anchor: corner(tile, number), panels: [wide, high, diff].map(shown), rows: [], facts };
  },
};

/* TIME */

export const ease = (p) => p * p * (3 - 2 * p);

export function span(levels) {
  return levels * BEAT + (levels - 1) * SLIDE + END;
}

export function phase(t, levels, still) {
  if (still) return { level: levels, p: 0, alpha: 1 };
  const loop = span(levels);
  const u = ((t % loop) + loop) % loop;
  const alpha = Math.min(1, u / IN, (loop - u) / OUT);
  if (u >= loop - END) return { level: levels, p: 0, alpha };
  const step = BEAT + SLIDE;
  const k = Math.floor(u / step);
  const r = u - k * step;
  return { level: k + 1, p: r < BEAT ? 0 : ease((r - BEAT) / SLIDE), alpha };
}

/* CAMERA */

export function block(plan, k) {
  const { number: n, level, anchor } = plan;
  const size = n ** k;
  const sum = (n ** level - size) / (n - 1);
  return [anchor[1] * sum, anchor[0] * sum, size];
}

export function camera(plan, k, p) {
  const [x0, y0, s0] = block(plan, k);
  if (!p || k >= plan.level) return [x0, y0, s0];
  const [x1, y1] = block(plan, k + 1);
  return [x1 + (x0 - x1) * (1 - p), y1 + (y0 - y1) * (1 - p), s0 * plan.number ** p];
}

/* LAYOUT */

export function panels(w, h, n) {
  const wide = w >= h;
  return Array.from({ length: n }, (_, i) => (wide ? { x: (w * i) / n, y: 0, w: w / n, h } : { x: 0, y: (h * i) / n, w, h: h / n }));
}

export function lay(panel, side, cell, pad) {
  const room = Math.min(panel.w, panel.h) - 2 * pad;
  const px = cell > 0 ? cell : Math.max(1, Math.floor((room * FIT) / side));
  const size = px * side;
  return { x: Math.round(panel.x + (panel.w - size) / 2), y: Math.round(panel.y + (panel.h - size) / 2), size, px };
}

export function tower(w, h, levels, pad) {
  const room = w - 2 * pad;
  const letters = 2 ** levels;
  const unit = Math.max(1, Math.floor(room / letters));
  const width = Math.min(unit * letters, room);
  const gap = Math.round(h * GAP);
  const tall = Math.max(1, Math.floor((h - 2 * pad - (levels - 1) * gap) / levels));
  const top = Math.round((h - (levels * tall + (levels - 1) * gap)) / 2);
  return { x: Math.round((w - width) / 2), w: width, unit, rows: Array.from({ length: levels }, (_, k) => ({ y: top + k * (tall + gap), h: tall })) };
}

/* FILES */

const cells = (types, side, dx, dy, unit) => runs({ shape: [side, side], types }).map(([row, col, len]) => `<rect x="${fix(dx + col * unit)}" y="${fix(dy + row * unit)}" width="${fix(len * unit)}" height="${fix(unit)}"/>`).join('');

export const fits = (plan) => plan.rows.reduce((sum, row) => sum + row.length, 0) + plan.panels.length * plan.side * plan.side <= BUDGET;

export function picture(plan, fill) {
  const { view, side, panels: grids, rows, level } = plan;
  const paint = (body) => `<g fill="${fill}" shape-rendering="crispEdges">${body}</g>`;
  if (view === 'word') {
    const width = UNIT * 2 ** level;
    const tall = UNIT * ROW;
    const body = rows.map((letters, k) => {
      const wide = width / letters.length;
      return runs({ shape: [1, letters.length], types: letters }).map(([, col, len]) => `<rect x="${fix(col * wide)}" y="${fix(k * (tall + UNIT))}" width="${fix(len * wide)}" height="${fix(tall)}"/>`).join('');
    });
    return sheet(width, level * tall + (level - 1) * UNIT, paint(body.join('')));
  }
  const size = side * UNIT;
  const gap = grids.length > 1 ? Math.round(size / 10) : 0;
  const body = grids.map((panel, i) => cells(panel.types, side, i * (size + gap), 0, UNIT)).join('');
  return sheet(size * grids.length + gap * (grids.length - 1), size, paint(body));
}
