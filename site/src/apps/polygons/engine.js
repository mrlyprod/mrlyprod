import { rng } from '../../lib/rand.js';

const R3 = Math.sqrt(3);

/* LATTICES */

const ring = (r, from) => Array.from({ length: 6 }, (_, k) => [r * Math.cos(((from + 60 * k) * Math.PI) / 180), r * Math.sin(((from + 60 * k) * Math.PI) / 180)]);

const out = (deg, r, s) => {
  const a = (deg * Math.PI) / 180;
  return [r * Math.cos(a) - s * Math.sin(a), r * Math.sin(a) + s * Math.cos(a)];
};

const HEX = ring(1 / R3, 30);

const E = 1 / (1 + R3);

const AP = (E * R3) / 2;

const MIXED = ring(E, 30);

const block = (deg) => [out(deg, AP, -E / 2), out(deg, AP + E, -E / 2), out(deg, AP + E, E / 2), out(deg, AP, E / 2)];

export const LATTICES = {
  square: { a: [1, 0], b: [0, 1], tiles: [{ pts: [[0, 0], [1, 0], [1, 1], [0, 1]], face: 0 }] },
  triangle: {
    a: [1, 0],
    b: [0.5, R3 / 2],
    tiles: [
      { pts: [[0, 0], [1, 0], [0.5, R3 / 2]], face: 0 },
      { pts: [[1, 0], [1.5, R3 / 2], [0.5, R3 / 2]], face: 0 },
    ],
  },
  hex: { a: [1, 0], b: [0.5, R3 / 2], tiles: [{ pts: HEX, face: 0 }] },
  cube: {
    a: [1, 0],
    b: [0.5, R3 / 2],
    whole: true,
    tiles: [
      { pts: [[0, 0], HEX[3], HEX[4], HEX[5]], face: 0 },
      { pts: [[0, 0], HEX[1], HEX[2], HEX[3]], face: 1 },
      { pts: [[0, 0], HEX[5], HEX[0], HEX[1]], face: 2 },
    ],
  },
  mix: {
    a: [1, 0],
    b: [0.5, R3 / 2],
    tiles: [
      { pts: MIXED, face: 0 },
      { pts: block(0), face: 1 },
      { pts: block(60), face: 1 },
      { pts: block(120), face: 1 },
      { pts: [MIXED[0], out(0, AP + E, E / 2), out(60, AP + E, -E / 2)], face: 2 },
      { pts: [MIXED[1], out(60, AP + E, E / 2), out(120, AP + E, -E / 2)], face: 2 },
    ],
  },
};

export const SHAPES = Object.keys(LATTICES);

const mean = (pts) => [pts.reduce((sum, p) => sum + p[0], 0) / pts.length, pts.reduce((sum, p) => sum + p[1], 0) / pts.length];

export function tiles(shape, w, h, size) {
  const { a, b, tiles: protos, whole } = LATTICES[shape];
  const crisp = shape === 'square';
  const s = crisp ? Math.max(2, Math.round(size)) : size;
  const ox = crisp ? Math.round(w / 2) : w / 2;
  const oy = crisp ? Math.round(h / 2) : h / 2;
  const cells = [];
  const j0 = Math.floor(-oy / (b[1] * s)) - 2;
  const j1 = Math.ceil((h - oy) / (b[1] * s)) + 2;
  for (let j = j0; j <= j1; j++) {
    const rx = ox + j * b[0] * s;
    const ry = oy + j * b[1] * s;
    const i0 = Math.floor(-rx / (a[0] * s)) - 2;
    const i1 = Math.ceil((w - rx) / (a[0] * s)) + 2;
    for (let i = i0; i <= i1; i++) {
      const x = rx + i * a[0] * s;
      protos.forEach(({ pts, face }, n) => {
        const at = pts.map(([px, py]) => [x + px * s, ry + py * s]);
        const [cx, cy] = mean(at);
        if (cx < -s || cx > w + s || cy < -s || cy > h + s) return;
        cells.push({ pts: at, cx, cy, gx: whole ? x : cx, gy: whole ? ry : cy, i, j, n: whole ? 0 : n, face });
      });
    }
  }
  return { cells, crisp, size: s };
}

/* NOISE */

export function hash(...ns) {
  let h = 2166136261;
  for (const n of ns) {
    h = Math.imul(h ^ (n | 0), 16777619);
    h ^= h >>> 13;
    h = Math.imul(h, 0x5bd1e995);
    h ^= h >>> 15;
  }
  return (h >>> 0) / 4294967296;
}

/* PALETTES */

export const PALETTES = {
  sea: ['teal', 'cyan', 'blue', 'accent'],
  sun: ['yellow', 'orange', 'red', 'accent'],
  leaf: ['accent', 'yellow', 'green', 'mint'],
  clay: ['brown', 'orange', 'pink', 'accent'],
  rainbow: ['red', 'orange', 'yellow', 'green', 'teal', 'blue', 'indigo', 'purple'],
  dusk: ['purple', 'pink', 'red'],
  meadow: ['green', 'mint', 'teal'],
  ice: ['cyan', 'blue', 'indigo'],
  gold: ['orange', 'yellow'],
  lime: ['yellow', 'green', 'mint'],
  night: ['indigo', 'purple', 'pink'],
  tint: ['tint1', 'tint2', 'tint3', 'accent'],
  soft: ['tint1', 'tint2'],
  deep: ['tint2', 'tint3', 'accent'],
  mono: ['ink1', 'ink2', 'ink3', 'ink4'],
};

const CYCLE = Object.keys(PALETTES).slice(0, 11);

const STRIDES = [1, 3, 5, 7, 9];

export function art(seed, k, palette) {
  const roll = rng(Math.floor(hash(seed, k, 0x9e37) * 4294967296));
  const start = Math.floor(hash(seed, 1) * CYCLE.length);
  const stride = STRIDES[Math.floor(hash(seed, 2) * STRIDES.length)];
  const hues = PALETTES[palette] ?? PALETTES[CYCLE[(start + k * stride) % CYCLE.length]];
  return { turn: roll() * 2 * Math.PI, hues, seed: Math.floor(roll() * 4294967296) };
}

export function spread(turn, w, h) {
  const ux = Math.cos(turn);
  const uy = Math.sin(turn);
  const ends = [[0, 0], [w, 0], [0, h], [w, h]].map(([x, y]) => (x - w / 2) * ux + (y - h / 2) * uy);
  const low = Math.min(...ends);
  const span = Math.max(Math.max(...ends) - low, 1e-9);
  return (x, y) => Math.min(1, Math.max(0, ((x - w / 2) * ux + (y - h / 2) * uy - low) / span));
}

export function paint(cell, look, along, { noise, spark }) {
  const { seed, hues } = look;
  const pick = (salt) => hash(seed, cell.i, cell.j, cell.n, salt);
  if (pick(11) < spark) return 'accent';
  const lump = hash(seed, Math.floor(cell.i / 2), Math.floor(cell.j / 2));
  const jitter = 0.65 * lump + 0.35 * pick(7) - 0.5;
  const k = Math.min(Math.max(along(cell.gx, cell.gy) * hues.length + jitter * noise, 0), hues.length - 1e-9);
  return hues[Math.floor(k)];
}

/* WAVES */

export const FIRST = 1000;

export const CROSS = 2000;

export const BAND = 500;

export const WAVES = ['diagonal', 'sweep', 'ripple', 'rain'];

export function wave(kind, seed, n, w, h, start) {
  if (kind === 'diagonal') return { start, along: spread(Math.PI / 4, w, h) };
  if (kind === 'sweep') return { start, along: spread(hash(seed, n, 3) * 2 * Math.PI, w, h) };
  if (kind === 'rain') return { start, rain: hash(seed, n, 4) * 4294967296 };
  return ripple(hash(seed, n, 5) * w, hash(seed, n, 6) * h, w, h, start);
}

export function ripple(x, y, w, h, start) {
  const far = Math.max(...[[0, 0], [w, 0], [0, h], [w, h]].map(([cx, cy]) => Math.hypot(cx - x, cy - y)), 1e-9);
  return { start, along: (cx, cy) => Math.min(1, Math.hypot(cx - x, cy - y) / far) };
}

export const phase = (one, cell) => (one.rain === undefined ? one.along(cell.gx, cell.gy) : hash(one.rain, cell.i, cell.j, cell.n));

export const peak = (one, cell) => one.start + phase(one, cell) * CROSS + BAND / 2;

export function glow(one, cell, t) {
  const x = (t - one.start - phase(one, cell) * CROSS) / BAND;
  return x <= 0 || x >= 1 ? 0 : Math.sin(Math.PI * x) ** 2;
}

export const live = (one, t) => t >= one.start && t <= one.start + CROSS + BAND;
