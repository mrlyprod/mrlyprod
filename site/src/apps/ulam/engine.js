import { palette as hues } from '../../lib/theme/palette.gen.js';
import { blend, rgb } from '../../lib/color.js';
import { pick } from '../../lib/rand.js';
import { grid, num as fmt, runs, sheet } from '../../lib/app/svg.js';

export const LATTICES = [['square', 'Square'], ['hex', 'Hex']];
export const MARKS = [['prime', 'Primes'], ['twin', 'Twin primes'], ['squarefree', 'Squarefree'], ['mobius', 'Mobius sign']];
export const LEADS = [[0, 'Off'], [1, '1'], [2, '2'], [3, '3'], [4, '4']];
export const SIDES = [21, 401];
export const REACH = 1000000;
export const LEAD = 4;
const ROLL = 100;
const SWAY = 6;
const FAVOUR = 0.75;
const RATE = 12.5;
export const LEAST = 3000;
export const MOST = 10000;
export const EDGE = 16;
const SHOWN = 8;
const UNIT = 10;
export const GROUND = 0.12;
const JOINED = { square: 4, hex: 3 };
const BRIGHT = 382;
export const LIT = 1;
export const DIM = 2;
export const HIT = 3;
export const MISS = 4;
export const SOIL = 5;
const ORDER = [SOIL, DIM, LIT, HIT, MISS];
const ROOT3 = Math.sqrt(3);

export const title = (word) => word[0].toUpperCase() + word.slice(1);

export const HEX = Array.from({ length: 6 }, (_, i) => [Math.cos(Math.PI / 6 + (i * Math.PI) / 3), Math.sin(Math.PI / 6 + (i * Math.PI) / 3)]);

/* COLOUR */

export const ink = (paper) => (rgb(paper).reduce((sum, v) => sum + v, 0) > BRIGHT ? '#000000' : '#ffffff');

export function palette(accent, paper) {
  return [null, accent, hues.pink, hues.orange, hues.blue, blend(paper, ink(paper), GROUND)];
}

/* LATTICE */

export function lay(num, lattice, px, w, h) {
  const cx = w / 2;
  const cy = h / 2;
  if (lattice === 'hex') {
    const s = px / ROOT3;
    return {
      lattice,
      px,
      s,
      cx,
      cy,
      at: (q, r) => [cx + s * ROOT3 * (q + r / 2), cy + 1.5 * s * r],
      cell: (x, y) => {
        const [q, r] = num.gauss.Ring.nearest('Eisenstein', (x - cx) / px, (cy - y) / px);
        return [Number(q) + 0, -Number(r) + 0];
      },
    };
  }
  return { lattice, px, s: px, cx, cy, at: (x, y) => [cx + x * px, cy - y * px], cell: (x, y) => [Math.floor((x - cx) / px + 0.5), Math.floor((cy - y) / px + 0.5)] };
}

export function fitted(lattice, rings, w, h, pad = 0) {
  const side = 2 * rings + 1;
  const wide = Math.max(1, w - 2 * pad);
  const high = Math.max(1, h - 2 * pad);
  if (lattice === 'hex') return ROOT3 * Math.min(wide / (ROOT3 * side), high / (3 * rings + 2));
  return Math.min(wide, high) / side;
}

/* WIND */

const wound = new Map();

export function wind(num, name, top) {
  let held = wound.get(name);
  if (!held || held.xs.length <= top) {
    const xs = new Int32Array(top + 1);
    const ys = new Int32Array(top + 1);
    let from = 1;
    if (held) {
      xs.set(held.xs);
      ys.set(held.ys);
      from = held.xs.length;
    }
    const L = num.spiral.Lattice;
    for (let n = from; n <= top; n++) {
      const [x, y] = L.xy(name, n);
      xs[n] = Number(x);
      ys[n] = Number(y);
    }
    held = { xs, ys };
    wound.set(name, held);
  }
  return { xs: held.xs.subarray(0, top + 1), ys: held.ys.subarray(0, top + 1) };
}

export function landed(num, name, rings, share) {
  const reach = Math.max(0, Math.min(1, share)) * rings;
  const k = Math.floor(reach);
  const count = (r) => num.spiral.Lattice.count(name, 2 * r + 1);
  const from = count(k);
  return k >= rings ? from : Math.floor(from + (reach - k) * (count(k + 1) - from));
}

export function span(rings) {
  return Math.min(MOST, Math.max(LEAST, (rings / RATE) * 1000));
}

export function share(t, length, still) {
  return still ? 1 : Math.max(0, Math.min(1, t / length));
}

/* LIBRARY */

export const SPIRALS = [
  { id: 'primes', name: 'Primes', note: 'the first spiral', set: { lattice: 'square', mark: 'prime', a: 0 } },
  { id: 'twins', name: 'Twin primes', note: 'two apart', set: { lattice: 'square', mark: 'twin', a: 0 } },
  { id: 'free', name: 'Squarefree', note: 'no square divides', set: { lattice: 'square', mark: 'squarefree', a: 0 } },
  { id: 'mobius', name: 'Mobius sign', note: 'minus one dim', set: { lattice: 'square', mark: 'mobius', a: 0 } },
  { id: 'hex', name: 'Hexagons', note: 'on hexagons', set: { lattice: 'hex', mark: 'prime', a: 0 } },
];

export const POLYS = [
  { id: 'euler', name: 'Euler diagonal', note: 'prime for its first 21 values', set: { lattice: 'square', mark: 'prime', a: 4, b: -2, c: 41 } },
  { id: 'axis', name: 'Euler axis', note: 'prime for its first 40 values', set: { lattice: 'square', mark: 'prime', a: 1, b: 1, c: 41 } },
  { id: 'legendre', name: 'Legendre', note: 'prime for its first 29 values', set: { lattice: 'square', mark: 'prime', a: 2, b: 0, c: 29 } },
  { id: 'hexaxis', name: 'Hexagon axis', note: 'prime for its first 40 values', set: { lattice: 'hex', mark: 'prime', a: 1, b: 1, c: 41 } },
  { id: 'hexdiagonal', name: 'Hexagon diagonal', note: 'prime for its first 22 values', set: { lattice: 'hex', mark: 'prime', a: 3, b: 3, c: 23 } },
];

export const holds = (value, set) => Object.entries(set).every(([key, v]) => value[key] === v);

/* LINE */

export function shuffle(num, rand) {
  const lattice = pick(rand, LATTICES)[0];
  const side = SIDES[0] + 2 * Math.floor(rand() * ((SIDES[1] - SIDES[0]) / 2 + 1));
  const a = rand() < FAVOUR ? JOINED[lattice] : 1 + Math.floor(rand() * LEAD);
  const b = (a % 2) + 2 * (Math.floor(rand() * (2 * SWAY + 1)) - SWAY);
  return { lattice, side, a, b, c: num.prime.prime_from(3 + Math.floor(rand() * ROLL)) };
}

export function step(num, c, by, max) {
  if (by > 0) {
    const next = num.prime.prime_from(Math.max(0, c + 1));
    return next <= max ? next : c;
  }
  const below = num.prime.primes(Math.max(0, c - 1));
  return below.length ? below[below.length - 1] : c;
}

export function formula(a, b, c) {
  const term = (k, word) => (k === 0 ? '' : ` ${k < 0 ? '-' : '+'} ${Math.abs(k)}${word}`);
  return `${a}k²${term(b, 'k')}${term(c, '')}`;
}

export function factors(list) {
  return list.map(([p, e]) => (e > 1 ? `${p}^${e}` : String(p))).join(' · ');
}

/* STUDY */

export function spiral(num, lattice, side) {
  const name = title(lattice);
  const top = num.spiral.Lattice.count(name, side);
  return { lattice, name, side, rings: (side - 1) / 2, top, primes: num.prime.prime_count(top), ...wind(num, name, top) };
}

export function toned(num, value, top) {
  const marked = num.spiral.marks(title(value.mark), top);
  const tone = new Uint8Array(top + 1);
  let lit = 0;
  for (let n = 1; n <= top; n++) {
    if (marked[n] === 1) {
      tone[n] = LIT;
      lit++;
    } else if (marked[n] === -1) tone[n] = DIM;
    else if (value.faint) tone[n] = SOIL;
  }
  return { tone, lit };
}

export function quadratic(num, value) {
  return { line: value.a > 0 ? num.spiral.diagonal(title(value.lattice), value.side, value.a, value.b, value.c) : null };
}

export function study(num, value, w, h, pad = 0) {
  const plan = spiral(num, value.lattice, value.side);
  return { ...plan, geo: lay(num, value.lattice, fitted(value.lattice, plan.rings, w, h, pad), w, h), ...toned(num, value, plan.top), ...quadratic(num, value) };
}

export function read(num, plan, value) {
  const { line } = plan;
  const next = line && line.streak < line.values.length ? line.values[line.streak] : null;
  return {
    lattice: plan.lattice,
    mark: value.mark,
    rings: plan.rings,
    side: plan.side,
    top: plan.top,
    primes: plan.primes,
    lit: plan.lit,
    density: plan.primes / plan.top,
    a: value.a,
    b: value.b,
    c: value.c,
    line: line && { count: line.values.length, hits: line.hits, share: line.share, streak: line.streak, values: line.values.slice(0, SHOWN), more: line.values.length > SHOWN, next: next === null ? null : { n: next, factors: num.prime.pile(next).factors } },
  };
}

/* SVG */

export function picture(plan, tones) {
  const { lattice, xs, ys, top, line } = plan;
  const tone = plan.tone.slice();
  if (line) line.values.forEach((n, i) => (tone[n] = line.hit[i] ? HIT : MISS));
  const hex = lattice === 'hex';
  const s = UNIT / ROOT3;
  const raw = hex ? (q, r) => [ROOT3 * s * (q + r / 2), 1.5 * s * r] : (x, y) => [x * UNIT, -y * UNIT];
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (let n = 1; n <= top; n++) {
    const [x, y] = raw(xs[n], ys[n]);
    x0 = Math.min(x0, x);
    x1 = Math.max(x1, x);
    y0 = Math.min(y0, y);
    y1 = Math.max(y1, y);
  }
  const half = hex ? s : UNIT / 2;
  const width = x1 - x0 + 2 * half;
  const height = y1 - y0 + 2 * half;
  const at = (x, y) => {
    const [px, py] = raw(x, y);
    return [px - x0 + half, py - y0 + half];
  };
  const cols = Math.round(width / UNIT);
  const rows = Math.round(height / UNIT);
  const index = (n) => {
    const [x, y] = at(xs[n], ys[n]);
    return Math.round((y - UNIT / 2) / UNIT) * cols + Math.round((x - UNIT / 2) / UNIT);
  };
  let body = hex ? `<defs><path id="h" d="M${HEX.map(([cx, cy]) => `${fmt(cx * s)} ${fmt(cy * s)}`).join('L')}Z"/></defs>` : '';
  for (const t of ORDER) {
    const ids = [];
    for (let n = 1; n <= top; n++) if (tone[n] === t) ids.push(n);
    if (!ids.length) continue;
    const cells = hex
      ? ids.map((n) => {
          const [x, y] = at(xs[n], ys[n]);
          return `<use href="#h" x="${fmt(x)}" y="${fmt(y)}"/>`;
        })
      : runs(grid(rows, cols, ids.map(index))).map(([row, col, len]) => `<rect x="${fmt(col * UNIT)}" y="${fmt(row * UNIT)}" width="${fmt(len * UNIT)}" height="${UNIT}"/>`);
    body += `<g fill="${tones[t]}"${hex ? '' : ' shape-rendering="crispEdges"'}>${cells.join('')}</g>`;
  }
  return sheet(width, height, body);
}

export function table(line) {
  return ['k,n,prime', ...line.values.map((n, k) => `${k},${n},${line.hit[k] ? 1 : 0}`)].join('\n');
}
