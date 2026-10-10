import { pick, rng } from '../../lib/scene.js';
import { num as fix, path, sheet } from '../../lib/svg.js';
import { named, word } from '../designs/engine.js';

export const DIM = 2;
export const BASE = 3;
export const SHAPES = [['', 'Random'], ['twindragon', 'Twindragon'], ['terdragon', 'Terdragon'], ['flowsnake', 'Flowsnake'], ['koch', 'Koch curve'], ['gasket', 'Gasket'], ['tile', 'Tile']];
export const KINDS = SHAPES.slice(1).map(([kind]) => kind);
export const FILLS = { twindragon: 2, terdragon: 3, flowsnake: 7, koch: 4, gasket: 3 };
export const UNITS = { twindragon: 4, terdragon: 6, flowsnake: 6, koch: 6, gasket: 6, tile: 1 };
export const BUDGET = 131072;
export const MOST = 20;
export const FEW = 5;
export const STEP = 1000;
export const FADE = 500;
export const HOLD = 4000;
export const PAD = 16;
export const GAP = 0.12;
export const SMALL = 6;
export const SKEW = 1.25;
export const SIZE = 2048;
export const UNIT = 10;
const LETTERS = { Gaussian: 'i', Eisenstein: 'w' };
const SIDES = { Gaussian: 4, Eisenstein: 6 };
const CELLS = BASE * BASE;
const TOTAL = 2 ** CELLS;
const TAU = Math.PI * 2;

/* CODE */

export function bits(code) {
  let n = 0;
  for (let c = code; c; c >>>= 1) n += c & 1;
  return n;
}

export function roll(rand) {
  let code;
  do code = 1 + Math.floor(rand() * (TOTAL - 1));
  while (bits(code) < FEW || bits(code) >= CELLS);
  return String(code);
}

export function resolve(code, rand) {
  const text = String(code ?? '').trim();
  const n = Number(text);
  if (text !== '' && Number.isFinite(n)) {
    const whole = ((Math.floor(n) % TOTAL) + TOTAL) % TOTAL;
    if (whole > 0) return String(whole);
  }
  return roll(rand);
}

/* ROOM */

export function deepest(fill) {
  if (fill <= 1) return MOST;
  let level = 1;
  while (level < MOST && fill ** (level + 1) <= BUDGET) level++;
  return level;
}

export function cap(value) {
  const shape = value.shape;
  if (shape === 'tile') return deepest(bits(Number(choose(value, rng(value.seed ?? 0)).code)));
  if (FILLS[shape]) return deepest(FILLS[shape]);
  return MOST;
}

/* CHOICE */

export function choose(value, rand) {
  const picked = pick(rand, KINDS);
  const spin = rand();
  const kind = value.shape || picked;
  const code = kind === 'tile' ? resolve(value.shape ? value.code : '', rand) : '';
  const units = UNITS[kind];
  const turn = (Math.floor(spin * units) * TAU) / units;
  return { kind, code, turn };
}

export function steps(handle, onChange) {
  const { kind, code, level } = handle.facts;
  return {
    turn: (by) => onChange({ shape: KINDS[(KINDS.indexOf(kind) + by + KINDS.length) % KINDS.length] }),
    deeper: (by) => onChange({ shape: kind, level: level + by, ...(kind === 'tile' && { code }) }),
  };
}

export function spell(letter, a, c) {
  const size = Math.abs(c) === 1 ? '' : String(Math.abs(c));
  if (c === 0) return String(a);
  if (a === 0) return `${c < 0 ? '-' : ''}${size}${letter}`;
  return `${a} ${c < 0 ? '-' : '+'} ${size}${letter}`;
}

export const label = (kind) => SHAPES.find(([one]) => one === kind)?.[1] ?? kind;

/* LATTICE */

export function lattice(norm, sides, level) {
  const spacing = norm ** (-level / 2);
  return { spacing, radius: spacing / (sides === 4 ? Math.SQRT2 : Math.sqrt(3)) };
}

export function reach(sides, angle, level, turn) {
  if (sides !== 4) return 1;
  const phi = turn - level * angle;
  return Math.abs(Math.cos(phi)) + Math.abs(Math.sin(phi));
}

export function corners(sides, angle, level, turn, radius) {
  const start = (sides === 4 ? Math.PI / 4 : Math.PI / 6) - level * angle + turn;
  const out = new Float64Array(2 * sides);
  for (let j = 0; j < sides; j++) {
    const phi = start + (j * TAU) / sides;
    out[2 * j] = radius * Math.cos(phi);
    out[2 * j + 1] = radius * Math.sin(phi);
  }
  return out;
}

export function half(sides, angle, level, turn, radius) {
  const ring = corners(sides, angle, level, turn, radius);
  let hx = 0;
  let hy = 0;
  for (let j = 0; j < sides; j++) {
    hx = Math.max(hx, Math.abs(ring[2 * j]));
    hy = Math.max(hy, Math.abs(ring[2 * j + 1]));
  }
  return [hx, hy];
}

export function span(pts) {
  let x0 = Infinity;
  let y0 = Infinity;
  let x1 = -Infinity;
  let y1 = -Infinity;
  for (let i = 0; i < pts.length; i += 2) {
    x0 = Math.min(x0, pts[i]);
    x1 = Math.max(x1, pts[i]);
    y0 = Math.min(y0, pts[i + 1]);
    y1 = Math.max(y1, pts[i + 1]);
  }
  return [x0, y0, x1, y1];
}

/* STUDY */

export function study(num, math, value, rand) {
  const { kind, code, turn } = choose(value, rand);
  const design = kind === 'tile' ? num.radix.tile(BASE, code) : num.radix[kind]();
  const ring = design.ring();
  const letter = LETTERS[ring];
  const sides = SIDES[ring];
  const root = design.base();
  const [ba, bc] = root.value().map(Number);
  const [bx, by] = num.gauss.Ring.place(ring, ba, bc);
  const angle = Math.atan2(by, bx);
  const norm = Number(root.norm());
  const size = design.size();
  const top = Math.max(1, Math.min(value.shape ? value.level : MOST, deepest(size)));
  const sign = kind === 'tile' ? 1 : -1;
  const cos = Math.cos(turn);
  const sin = Math.sin(turn);
  const levels = [];
  const stage = [Infinity, Infinity, -Infinity, -Infinity];
  for (let level = 1; level <= top; level++) {
    const raw = design.plane(level);
    const pts = new Float32Array(raw.length * 2);
    for (let i = 0; i < raw.length; i++) {
      const [x, y] = raw[i];
      pts[2 * i] = x * cos - y * sin;
      pts[2 * i + 1] = (x * sin + y * cos) * sign;
    }
    levels.push(pts);
    const [a0, b0, a1, b1] = span(pts);
    const [hx, hy] = half(sides, angle, level, turn, lattice(norm, sides, level).radius);
    stage[0] = Math.min(stage[0], a0 - hx);
    stage[1] = Math.min(stage[1], b0 - hy);
    stage[2] = Math.max(stage[2], a1 + hx);
    stage[3] = Math.max(stage[3], b1 + hy);
  }
  const { radius } = lattice(norm, sides, top);
  const [x0, y0, x1, y1] = span(levels[top - 1]);
  const counts = new Map();
  const distinct = (level) => {
    if (!counts.has(level)) counts.set(level, design.distinct(level));
    return counts.get(level);
  };
  const fill = (level) => Number(design.fill(level));
  const name = kind === 'tile' ? word(named(math, DIM, BASE), code) : label(kind);
  const facts = { kind, name, code: kind === 'tile' ? code : design.code(), ring: `Z[${letter}]`, base: spell(letter, ba, bc), norm, digits: size, canonical: design.canonical(), dimension: design.dimension(), level: top, fill: fill(top), distinct: distinct(top) };
  return { kind, design, sides, norm, angle, turn, sign, top, levels, frame: [x0 - radius, y0 - radius, x1 + radius, y1 + radius], stage, fill, distinct, facts };
}

/* TIME */

export function phase(t, top, still) {
  if (still) return { level: top, k: 1, from: 0 };
  const loop = top * STEP + HOLD;
  const at = ((t % loop) + loop) % loop;
  const level = Math.min(top, Math.floor(at / STEP) + 1);
  const k = Math.min(1, (at - (level - 1) * STEP) / FADE);
  const from = level > 1 ? level - 1 : t >= loop ? top : 0;
  return { level, k, from };
}

export const ease = (k) => k * k * (3 - 2 * k);

/* LAYOUT */

export function lay(frame, w, h, pad, pinned = 0) {
  const [x0, y0, x1, y1] = frame;
  const fw = x1 - x0 || 1;
  const fh = y1 - y0 || 1;
  const quarter = (fw > SKEW * fh && h > SKEW * w) || (fh > SKEW * fw && w > SKEW * h);
  const [bw, bh] = quarter ? [fh, fw] : [fw, fh];
  const k = pinned > 0 ? pinned : Math.min((w - 2 * pad) / bw, (h - 2 * pad) / bh);
  return { k, cx: (x0 + x1) / 2, cy: (y0 + y1) / 2, ox: w / 2, oy: h / 2, quarter };
}

/* FILES */

export function bands(words) {
  const n = words.length;
  const xs = new Float64Array(n);
  const ys = new Float64Array(n);
  for (let i = 0; i < n; i++) {
    xs[i] = Number(words[i][0]);
    ys[i] = Number(words[i][1]);
  }
  const order = Uint32Array.from({ length: n }, (_, i) => i).sort((p, q) => ys[p] - ys[q] || xs[p] - xs[q]);
  const out = [];
  let i = 0;
  while (i < n) {
    const first = order[i];
    let j = i + 1;
    while (j < n && ys[order[j]] === ys[first] && xs[order[j]] === xs[order[j - 1]] + 1) j++;
    out.push([ys[first], xs[first], j - i]);
    i = j;
  }
  return out;
}

export function picture(plan, accent) {
  const { kind, design, top, levels, frame, sides, norm, angle, turn, sign } = plan;
  if (kind === 'tile') {
    const side = BASE ** top;
    const px = Math.min(side * UNIT, SIZE);
    const body = bands(design.words(top)).map(([row, col, len]) => `<rect x="${col}" y="${row}" width="${len}" height="1"/>`);
    return sheet(px, px, `<g fill="${accent}" shape-rendering="crispEdges">${body.join('')}</g>`, `0 0 ${side} ${side}`);
  }
  const [x0, y0, x1, y1] = frame;
  const k = SIZE / Math.max(x1 - x0, y1 - y0);
  const { spacing, radius } = lattice(norm, sides, top);
  let d;
  if (2 * radius * k < SMALL) {
    const s = Math.max(spacing * reach(sides, angle, top, turn) * k, 1);
    d = `M${fix(-s / 2)} ${fix(-s / 2)}h${fix(s)}v${fix(s)}h${fix(-s)}Z`;
  } else {
    const ring = corners(sides, angle, top, turn, radius * k * (1 - GAP));
    for (let j = 1; j < ring.length; j += 2) ring[j] *= sign;
    d = path(ring, true);
  }
  const pts = levels[top - 1];
  const seen = new Set();
  const uses = [];
  for (let i = 0; i < pts.length; i += 2) {
    const key = `${fix((pts[i] - x0) * k)} ${fix((pts[i + 1] - y0) * k)}`;
    if (seen.has(key)) continue;
    seen.add(key);
    const [x, y] = key.split(' ');
    uses.push(`<use href="#c" x="${x}" y="${y}"/>`);
  }
  return sheet((x1 - x0) * k, (y1 - y0) * k, `<defs><path id="c" d="${d}"/></defs><g fill="${accent}">${uses.join('')}</g>`);
}
