import { pick, rgb, veil } from '../../lib/scene.js';
import { grid, marks, num as fmt, path, runs, sheet } from '../../lib/svg.js';

export const LATTICES = [['square', 'Square'], ['hex', 'Hex']];
export const MARKS = [['prime', 'Primes'], ['twin', 'Twin primes'], ['squarefree', 'Squarefree'], ['mobius', 'Mobius']];
export const RINGS = 440;
export const ROLL = 100;
export const LEAD = 4;
export const SWAY = 6;
export const RATE = 12.5;
export const LEAST = 3000;
export const MOST = 20000;
export const HOLD = 5000;
export const FIT = 0.94;
export const LABEL = 18;
const NEAR = { square: 1, hex: 2 };
const JOINED = { square: 4, hex: 3 };
export const FAVOUR = 0.75;
export const UNIT = 10;
export const FAINT = 0.35;
export const GROUND = 0.12;
export const HALF = 0.5;
export const RAIL = 0.6;
export const WIDTH = 0.3;
export const OUTLINE = 0.15;
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

export function mix(a, b, t) {
  const p = rgb(a);
  const q = rgb(b);
  return `rgb(${p.map((v, i) => Math.round(v + (q[i] - v) * t)).join(', ')})`;
}

export function palette(accent, paper) {
  const half = mix(accent, paper, HALF);
  return [null, { color: accent, hollow: false }, { color: veil(accent, FAINT), hollow: false }, { color: half, hollow: false }, { color: half, hollow: true }, { color: veil(accent, GROUND), hollow: false }];
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

export function fitted(lattice, rings, w, h) {
  const side = 2 * rings + 1;
  if (lattice === 'hex') return ROOT3 * FIT * Math.min(w / (ROOT3 * side), h / (3 * rings + 2));
  return (FIT * Math.min(w, h)) / side;
}

export function reach(num, geo, w, h) {
  const name = title(geo.lattice);
  let most = 0;
  for (const [x, y] of [[0, 0], [w, 0], [0, h], [w, h]]) {
    const [q, r] = geo.cell(x, y);
    most = Math.max(most, Number(num.spiral.Lattice.ring_of(name, q, r)));
  }
  return Math.min(RINGS, most + 1);
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
  if (still) return 1;
  const loop = length + HOLD;
  const phase = ((t % loop) + loop) % loop;
  return Math.min(1, phase / length);
}

/* LINE */

export function choose(num, value, rand) {
  if (!value.roll) return { lattice: value.lattice, a: value.a, b: value.b, c: value.c };
  const lattice = pick(rand, LATTICES)[0];
  const a = rand() < FAVOUR ? JOINED[lattice] : 1 + Math.floor(rand() * LEAD);
  const b = (a % 2) + 2 * (Math.floor(rand() * (2 * SWAY + 1)) - SWAY);
  return { lattice, a, b, c: num.prime.prime_from(3 + Math.floor(rand() * ROLL)) };
}

export function step(num, c, by, max) {
  if (by > 0) {
    const next = num.prime.prime_from(c + 1);
    return next <= max ? next : c;
  }
  const below = num.prime.primes(Math.max(0, c - 1));
  return below.length ? below[below.length - 1] : c;
}

export function segments(num, lattice, cells) {
  const name = title(lattice);
  const ring = num.spiral.Lattice.ring_of;
  const out = [];
  let run = [0];
  for (let i = 1; i < cells.length; i++) {
    if (Number(ring(name, cells[i][0] - cells[i - 1][0], cells[i][1] - cells[i - 1][1])) <= NEAR[lattice]) run.push(i);
    else {
      if (run.length > 1) out.push(run);
      run = [i];
    }
  }
  if (run.length > 1) out.push(run);
  return out;
}

export function formula(a, b, c) {
  const term = (k, word) => (k === 0 ? '' : ` ${k < 0 ? '-' : '+'} ${Math.abs(k)}${word}`);
  return `${a}k²${term(b, 'k')}${term(c, '')}`;
}

export function factors(list) {
  return list.map(([p, e]) => (e > 1 ? `${p}^${e}` : String(p))).join(' · ');
}

/* STUDY */

export function study(num, value, w, h, dpr) {
  const { lattice, a, b, c } = value;
  const name = title(lattice);
  const L = num.spiral.Lattice;
  let rings = value.rings;
  let geo;
  if (value.fit) geo = lay(num, lattice, fitted(lattice, rings, w, h), w, h);
  else {
    geo = lay(num, lattice, value.cell * dpr, w, h);
    rings = reach(num, geo, w, h);
  }
  const side = 2 * rings + 1;
  const top = L.count(name, side);
  const marked = num.spiral.marks(title(value.mark), top);
  const primes = num.prime.prime_count(top);
  const line = a > 0 ? num.spiral.diagonal(name, side, a, b, c) : null;
  const next = line && line.streak < line.values.length ? line.values[line.streak] : null;
  const tone = new Uint8Array(top + 1);
  let lit = 0;
  for (let n = 1; n <= top; n++) {
    if (marked[n] === 1) {
      tone[n] = LIT;
      lit++;
    } else if (marked[n] === -1) tone[n] = DIM;
    else if (value.faint) tone[n] = SOIL;
  }
  if (line) line.values.forEach((n, i) => (tone[n] = line.hit[i] ? HIT : MISS));
  const facts = {
    lattice,
    mark: value.mark,
    rings,
    side,
    top,
    primes,
    lit,
    density: primes / top,
    a,
    b,
    c,
    rolled: Boolean(value.roll),
    line: line && { count: line.values.length, hits: line.hits, share: line.share, streak: line.streak, next: next === null ? null : { n: next, factors: num.prime.pile(next).factors } },
  };
  return { lattice, name, rings, side, top, geo, tone, line, runs: line ? segments(num, lattice, line.cells) : [], ...wind(num, name, top), facts };
}

/* SVG */

export function picture(plan, inside, tones) {
  const { lattice, tone, xs, ys, top, line, runs: rail } = plan;
  const hex = lattice === 'hex';
  const s = UNIT / ROOT3;
  const raw = hex ? (q, r) => [ROOT3 * s * (q + r / 2), 1.5 * s * r] : (x, y) => [x * UNIT, -y * UNIT];
  const shown = [];
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (let n = 1; n <= top; n++) {
    if (!inside(n)) continue;
    shown.push(n);
    const [x, y] = raw(xs[n], ys[n]);
    x0 = Math.min(x0, x);
    x1 = Math.max(x1, x);
    y0 = Math.min(y0, y);
    y1 = Math.max(y1, y);
  }
  if (!shown.length) return sheet(UNIT, UNIT, '');
  const half = hex ? s : UNIT / 2;
  const width = x1 - x0 + 2 * half;
  const height = y1 - y0 + 2 * half;
  const at = (x, y) => {
    const [px, py] = raw(x, y);
    return [px - x0 + half, py - y0 + half];
  };
  const edge = UNIT * OUTLINE;
  const cols = Math.round(width / UNIT);
  const rows = Math.round(height / UNIT);
  const index = (n) => {
    const [x, y] = at(xs[n], ys[n]);
    return Math.round((y - UNIT / 2) / UNIT) * cols + Math.round((x - UNIT / 2) / UNIT);
  };
  const corners = (r) => HEX.map(([cx, cy]) => `${fmt(cx * r)} ${fmt(cy * r)}`).join('L');
  let body = hex ? `<defs><path id="h" d="M${corners(s)}Z"/><path id="o" d="M${corners(s - edge / 2)}Z"/></defs>` : '';
  for (const t of ORDER) {
    const ids = shown.filter((n) => tone[n] === t);
    if (!ids.length) continue;
    const { color, hollow } = tones[t];
    const paint = hollow ? `fill="none" stroke="${color}" stroke-width="${fmt(edge)}"` : `fill="${color}"`;
    const inset = hollow ? edge / 2 : 0;
    const cells = hex
      ? ids.map((n) => {
          const [x, y] = at(xs[n], ys[n]);
          return `<use href="#${hollow ? 'o' : 'h'}" x="${fmt(x)}" y="${fmt(y)}"/>`;
        })
      : (hollow ? marks(ids.map(index), cols).map(([col, row]) => [row, col, 1]) : runs(grid(rows, cols, ids.map(index)))).map(([row, col, len]) => `<rect x="${fmt(col * UNIT + inset)}" y="${fmt(row * UNIT + inset)}" width="${fmt(len * UNIT - 2 * inset)}" height="${fmt(UNIT - 2 * inset)}"/>`);
    body += `<g ${paint}${hex ? '' : ' shape-rendering="crispEdges"'}>${cells.join('')}</g>`;
  }
  if (line) {
    const seen = new Set(shown);
    for (const run of rail) {
      const kept = run.filter((i) => seen.has(line.values[i]));
      if (kept.length < 2) continue;
      body += `<path d="${path(kept.flatMap((i) => at(line.cells[i][0], line.cells[i][1])))}" fill="none" stroke="${tones[LIT].color}" stroke-opacity="${RAIL}" stroke-width="${fmt(UNIT * WIDTH)}" stroke-linecap="round" stroke-linejoin="round"/>`;
    }
  }
  return sheet(width, height, body);
}

export function table(line) {
  return ['k,n,prime', ...line.values.map((n, k) => `${k},${n},${line.hit[k] ? 1 : 0}`)].join('\n');
}
