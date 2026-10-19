import { num as fix, lines, path, sheet } from '../../lib/app/svg.js';
import { classes, facts } from '../../lib/designs/engine.js';

export const DIM = 2;
export const BOX = 3;
export const BUDGET = 131072;
export const MOST = 20;
export const PAD = 16;
export const GAP = 0.12;
export const SMALL = 6;
export const SKEW = 1.25;
export const SIZE = 2048;
export const UNIT = 10;
export const THIN = 0.25;
export const KEEP = 4;
export const GLANCE = 3000;
export const RINGS = [['gaussian', 'Z[i], square'], ['eisenstein', 'Z[w], hexagonal']];
export const BASES = { gaussian: ['2:0', '1:1', '2:1', '3:0'], eisenstein: ['2:0', '2:1', '3:0', '3:1'] };
export const LETTERS = { gaussian: 'i', eisenstein: 'w' };
export const DRAWN = [[0, 'Cells'], [1, 'Line']];
export const KOCH = { ring: 'eisenstein', base: '3:0', d: '0:0_1:0_2:1_2:0', t: '0_1_5_0', level: 5, line: 1 };
export const PRESETS = [['koch', 'Koch curve', 5, 1], ['terdragon', 'Terdragon', 8, 1], ['twindragon', 'Twindragon', 14, 0], ['gasket', 'Gasket', 7, 0], ['flowsnake', 'Flowsnake', 5, 0], ['carpet', 'Carpet', 5, 0]];
const CARPET = 495;
const TAU = Math.PI * 2;

/* WORDS */

export const pair = (text) => String(text).split(':').map(Number);

export const write = (held) => ({ d: held.map(({ z: [a, c] }) => `${a}:${c}`).join('_'), t: held.map(({ u }) => u).join('_') });

export const count = (d) => String(d ?? '').split('_').filter((one) => one.trim()).length;

export const twisted = (value) => String(value.t ?? '').split('_').some((one) => Number(one) > 0);

export function spell(letter, a, c) {
  const size = Math.abs(c) === 1 ? '' : String(Math.abs(c));
  if (c === 0) return String(a);
  if (a === 0) return `${c < 0 ? '-' : ''}${size}${letter}`;
  return `${a} ${c < 0 ? '-' : '+'} ${size}${letter}`;
}

export const bases = (ring) => (BASES[ring] ?? BASES.gaussian).map((one) => [one, spell(LETTERS[ring] ?? 'i', ...pair(one))]);

export function parse(value, units) {
  const digits = String(value.d ?? '').split('_').filter((one) => one.trim()).map(pair);
  const turns = String(value.t ?? '').split('_').map(Number);
  return digits.map((z, i) => ({ z, u: Number.isInteger(turns[i]) && turns[i] >= 0 ? turns[i] % units : 0 })).filter(({ z }) => z.length === 2 && z.every(Number.isInteger));
}

/* ROOM */

export function deepest(fill) {
  if (fill <= 1) return MOST;
  let level = 1;
  while (level < MOST && fill ** (level + 1) <= BUDGET) level++;
  return level;
}

export const cap = (value) => deepest(count(value.d));

export function glance(fill, level = MOST) {
  let at = 1;
  while (at < Math.min(level, MOST) && fill ** (at + 1) <= GLANCE) at++;
  return at;
}

/* DIAL */

const plain = (z) => z.map(Number);

export function dial(num, math, value) {
  const ring = num.gauss.Ring.named(value.ring) ?? 'Gaussian';
  const base = new math.radix.Base(ring, pair(value.base));
  const residues = base.residues().map(plain);
  const units = num.gauss.Ring.associates(ring, 1, 0).map(plain);
  const held = [];
  let error = '';
  for (const one of parse(value, units.length)) {
    const cls = base.class(one.z);
    if (held.some((was) => was.cls === cls)) error = `the digits ${held.find((was) => was.cls === cls).z.join(':')} and ${one.z.join(':')} are congruent modulo the base, so the second is dropped.`;
    else held.push({ ...one, cls });
  }
  if (!held.length) {
    error = 'a design needs a digit, so every residue stands in.';
    residues.forEach((z, cls) => held.push({ z, u: 0, cls }));
  }
  return { ring, letter: LETTERS[ring.toLowerCase()], base, residues, units, held, error };
}

export function toggle(dialed, cls) {
  const { held, residues } = dialed;
  if (!held.some((one) => one.cls === cls)) return write([...held, { z: residues[cls], u: 0 }]);
  return held.length > 1 ? write(held.filter((one) => one.cls !== cls)) : null;
}

export const turn = (dialed, cls, u) => write(dialed.held.map((one) => (one.cls === cls ? { ...one, u } : one)));

export const every = (dialed) => write(dialed.residues.map((z) => ({ z, u: 0 })));

export const flat = (dialed) => write(dialed.held.map((one) => ({ ...one, u: 0 })));

export function rebase(num, math, ring, base) {
  const key = (BASES[ring] ?? []).includes(base) ? base : BASES[ring][0];
  return { ring, base: key, ...every(dial(num, math, { ring, base: key, d: '', t: '' })) };
}

export function coded(dialed, math, text) {
  const n = Number(String(text).trim());
  if (!Number.isInteger(n) || n < 1 || n >= 2 ** dialed.residues.length) return null;
  const one = math.radix.Radix.from_code(dialed.base, n);
  const patch = write(one.digits().map((z) => ({ z: plain(z), u: 0 })));
  one.free();
  return patch;
}

export function rolled(dialed, rand) {
  const { residues, units } = dialed;
  const least = Math.min(2, residues.length);
  let picked;
  do picked = residues.filter(() => rand() < 0.5);
  while (picked.length < least);
  return write(picked.map((z) => ({ z, u: Math.floor(rand() * units.length) })));
}

export function spelled(num, radix) {
  const ring = radix.ring();
  const units = num.gauss.Ring.associates(ring, 1, 0).map((u) => plain(u).join(':'));
  const [a, c] = plain(radix.base().value());
  return { ring: ring.toLowerCase(), base: `${a}:${c}`, d: write(radix.digits().map((z) => ({ z: plain(z), u: 0 }))).d, t: radix.twists().map((u) => units.indexOf(plain(u).join(':'))).join('_') };
}

export function preset(num, math, name) {
  const row = PRESETS.find(([one]) => one === name);
  if (!row) return null;
  const radix = name === 'carpet' ? math.radix.tile(BOX, CARPET) : math.radix[name]();
  const patch = { ...spelled(num, radix), level: row[2], line: row[3] };
  radix.free();
  return patch;
}

export const wears = (value, patch) => Boolean(patch) && ['ring', 'base', 'd', 't'].every((key) => String(value[key]) === String(patch[key]));

/* BOX */

export function boxOf(dialed) {
  let code = 0;
  for (const { z: [x, y], u } of dialed.held) {
    if (u !== 0 || x < 0 || y < 0 || x >= BOX || y >= BOX) return '';
    code += 2 ** (x + BOX * y);
  }
  return String(code);
}

export function boxes(math) {
  const named = math.name.words(DIM, BOX).map(([code, word]) => [String(code), word]);
  const seen = new Set(named.map(([code]) => String(facts(math, DIM, BOX, code).rep)));
  const rest = (classes(math, DIM, BOX) ?? []).map(({ code }) => [String(code), '']).filter(([code]) => !seen.has(code));
  return { named, rest };
}

export function boxed(num, math, code) {
  const radix = math.radix.tile(BOX, code);
  const patch = spelled(num, radix);
  radix.free();
  return patch;
}

/* LATTICE */

export function lattice(norm, sides, level) {
  const spacing = norm ** (-level / 2);
  return { spacing, radius: spacing / (sides === 4 ? Math.SQRT2 : Math.sqrt(3)) };
}

export function reach(sides, angle, level) {
  if (sides !== 4) return 1;
  const phi = -level * angle;
  return Math.abs(Math.cos(phi)) + Math.abs(Math.sin(phi));
}

export function corners(sides, angle, level, radius) {
  const start = (sides === 4 ? Math.PI / 4 : Math.PI / 6) - level * angle;
  const out = new Float64Array(2 * sides);
  for (let j = 0; j < sides; j++) {
    const phi = start + (j * TAU) / sides;
    out[2 * j] = radius * Math.cos(phi);
    out[2 * j + 1] = radius * Math.sin(phi);
  }
  return out;
}

export function half(sides, angle, level, radius) {
  const ring = corners(sides, angle, level, radius);
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

export function study(num, math, value, most = MOST) {
  const { ring, letter, base, held, units, error } = dial(num, math, value);
  const bare = new math.radix.Radix(base, held.map(({ z }) => z), held.map(() => [1, 0]));
  const radix = bare.with_twists(Uint32Array.from(held.map(({ u }) => u)));
  bare.free();
  const [ba, bc] = plain(base.value());
  const [bx, by] = num.gauss.Ring.place(ring, ba, bc);
  const angle = Math.atan2(by, bx);
  const norm = Number(base.norm());
  const sides = units.length;
  const size = radix.size();
  const top = Math.min(deepest(size), most);
  const down = sides === 4 && bc === 0;
  const sign = down ? 1 : -1;
  const levels = [];
  const stage = [Infinity, Infinity, -Infinity, -Infinity];
  const bounds = [Infinity, Infinity, -Infinity, -Infinity];
  for (let level = 1; level <= top; level++) {
    const raw = radix.plane(level);
    const pts = new Float32Array(raw.length * 2);
    for (let i = 0; i < raw.length; i++) {
      pts[2 * i] = raw[i][0];
      pts[2 * i + 1] = raw[i][1] * sign;
    }
    levels.push(pts);
    const [a0, b0, a1, b1] = span(pts);
    bounds[0] = Math.min(bounds[0], a0);
    bounds[1] = Math.min(bounds[1], b0);
    bounds[2] = Math.max(bounds[2], a1);
    bounds[3] = Math.max(bounds[3], b1);
    const [hx, hy] = half(sides, angle, level, lattice(norm, sides, level).radius);
    stage[0] = Math.min(stage[0], a0 - hx);
    stage[1] = Math.min(stage[1], b0 - hy);
    stage[2] = Math.max(stage[2], a1 + hx);
    stage[3] = Math.max(stage[3], b1 + hy);
  }
  const counts = new Map();
  const distinct = (level) => {
    if (!counts.has(level)) counts.set(level, radix.distinct(level));
    return counts.get(level);
  };
  const fill = (level) => Number(radix.fill(level));
  const still = { ring: `Z[${letter}]`, base: spell(letter, ba, bc), norm, digits: size, units: sides, code: radix.code(), canonical: radix.canonical(), dimension: radix.dimension(), cap: top, error };
  const facts = (level) => ({ ...still, level, fill: fill(level), distinct: distinct(level), glued: distinct(level) < fill(level) });
  return { radix, down, sides, norm, angle, sign, cap: top, levels, stage, bounds, facts };
}

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

export function picture(plan, accent, level, line) {
  const { radix, down, levels, sides, norm, angle, sign } = plan;
  const pts = levels[level - 1];
  const { spacing, radius } = lattice(norm, sides, level);
  const [p0, q0, p1, q1] = span(pts);
  const [x0, y0, x1, y1] = line ? [p0, q0, p1, q1] : [p0 - radius, q0 - radius, p1 + radius, q1 + radius];
  const k = SIZE / Math.max(x1 - x0, y1 - y0, spacing);
  const w = Math.max((x1 - x0) * k, 1);
  const h = Math.max((y1 - y0) * k, 1);
  if (line) {
    const points = Array.from(pts, (v, i) => (i % 2 ? (v - y0) * k : (v - x0) * k));
    return lines({ width: w, height: h, strokes: [{ points, color: accent, width: Math.max(1, spacing * k * THIN) }] });
  }
  if (down) {
    const runs = bands(radix.words(level));
    const [lx, ly, hx, hy] = runs.reduce(([a, b, c, d], [row, col, len]) => [Math.min(a, col), Math.min(b, row), Math.max(c, col + len), Math.max(d, row + 1)], [Infinity, Infinity, -Infinity, -Infinity]);
    const side = Math.max(hx - lx, hy - ly);
    const px = Math.min(side * UNIT, SIZE);
    const body = runs.map(([row, col, len]) => `<rect x="${col}" y="${row}" width="${len}" height="1"/>`);
    return sheet((px * (hx - lx)) / side, (px * (hy - ly)) / side, `<g fill="${accent}" shape-rendering="crispEdges">${body.join('')}</g>`, `${lx} ${ly} ${hx - lx} ${hy - ly}`);
  }
  let d;
  if (2 * radius * k < SMALL) {
    const s = Math.max(spacing * reach(sides, angle, level) * k, 1);
    d = `M${fix(-s / 2)} ${fix(-s / 2)}h${fix(s)}v${fix(s)}h${fix(-s)}Z`;
  } else {
    const ring = corners(sides, angle, level, radius * k * (1 - GAP));
    for (let j = 1; j < ring.length; j += 2) ring[j] *= sign;
    d = path(ring, true);
  }
  const seen = new Set();
  const uses = [];
  for (let i = 0; i < pts.length; i += 2) {
    const key = `${fix((pts[i] - x0) * k)} ${fix((pts[i + 1] - y0) * k)}`;
    if (seen.has(key)) continue;
    seen.add(key);
    const [x, y] = key.split(' ');
    uses.push(`<use href="#c" x="${x}" y="${y}"/>`);
  }
  return sheet(w, h, `<defs><path id="c" d="${d}"/></defs><g fill="${accent}">${uses.join('')}</g>`);
}
