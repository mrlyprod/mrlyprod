import { pick } from '../../lib/rand.js';

export const ROOTS = ['strip', '-1,2,2,3', '-2,3,6,7', '-3,4,12,13'];
export const CAPS = [8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192];
export const ROLLED = CAPS.slice(-5);
export const ORDER_CAP = 64;
export const INKS = [['size', 'By curvature'], ['shade', 'Octave shades'], ['ford', 'Ford circles apart'], ['one', 'One ink']];
export const LOOKS = [['wash', 'Wash'], ['solid', 'Solid'], ['rings', 'Rings']];
export const SIX = ['blue', 'teal', 'green', 'yellow', 'orange', 'pink'];
export const OCTAVE = 1000;
export const HOLD = 4000;
export const PAD = 24;
export const LEAST = 9;
export const PALE = 0.55;
export const STRIDE = 4;
const KEEP = 6;

export const GLANCE = 128;

export const quadruple = (root) => (root === 'strip' ? '(0, 0, 2, 2)' : `(${root.split(',').join(', ')})`);

export const named = (name) => (name === 'strip' ? 'Strip' : `Bounded (${name.split(',').join(', ')})`);

export const stack = (cap) => Math.min(ORDER_CAP, Math.floor(Math.sqrt(cap / 2)));

export function roll(rand) {
  const root = pick(rand, ROOTS);
  const cap = pick(rand, ROLLED);
  return { root, cap, ink: pick(rand, INKS)[0], look: pick(rand, LOOKS)[0] };
}

export function step(cap, by) {
  const i = CAPS.indexOf(cap);
  return CAPS[Math.min(CAPS.length - 1, Math.max(0, (i < 0 ? CAPS.indexOf(2048) : i) + by))];
}

/* STUDY */

const PLANS = new Map();

export function study(num, name, cap) {
  const id = `${name}|${cap}`;
  const kept = PLANS.get(id);
  if (kept) {
    PLANS.delete(id);
    PLANS.set(id, kept);
    return kept;
  }
  const plan = grown(num, name, cap);
  PLANS.set(id, plan);
  if (PLANS.size > KEEP) PLANS.delete(PLANS.keys().next().value);
  return plan;
}

function grown(num, name, cap) {
  const a = num.apollonian;
  const packing = a.grow(name, cap);
  const quad = packing.root.map((c) => [c.k, c.x, c.y]);
  const inner = packing.root.filter((c) => c.k > 0);
  const hull = packing.root.find((c) => c.k < 0);
  const rows = [...inner, ...packing.circles];
  const total = rows.length;
  const data = a.flat(name, cap);
  const ints = Int32Array.from(rows.flatMap((c) => [c.k, c.x, c.y]));
  const k0 = packing.circles[0]?.k ?? inner.at(-1)?.k ?? cap;
  const octaves = Math.max(1, Math.ceil(Math.log2(cap / k0)) + 1);
  const bands = Uint8Array.from({ length: total }, (_, i) => Math.min(octaves - 1, Math.max(0, Math.floor(Math.log2(data[i * STRIDE + 3] / k0)))));
  const hues = Uint8Array.from({ length: total }, (_, i) => Math.floor(Math.log2(data[i * STRIDE + 3])) % SIX.length);
  const touches = packing.strip ? a.touches(packing) : [];
  const at = new Map(touches.map((t) => [t.num / t.den, t]));
  const fords = new Uint8Array(total);
  if (packing.strip) {
    rows.forEach((c, i) => {
      if (i >= inner.length && at.get(data[i * STRIDE])?.k !== c.k) return;
      const circle = a.Circle.from(c);
      fords[i] = a.on_line(circle) && a.is_ford(circle) ? 1 : 0;
      circle.free();
    });
  }
  const circles = packing.circles.length;
  const facts = {
    name,
    title: `${name === 'strip' ? 'strip ' : ''}(${quad.map(([k]) => k).join(', ')})`,
    cap,
    circles,
    drawn: circles + packing.root.filter((c) => c.k !== 0).length,
    quads: packing.quads,
    broken: packing.broken,
    strayed: packing.strayed,
    line: packing.strip ? touches.length : null,
    ford: packing.strip ? fords.subarray(inner.length).reduce((n, v) => n + v, 0) : null,
    shadow: packing.strip ? shadow(num, packing, data, inner.length, stack(cap)) : null,
  };
  return {
    name,
    strip: packing.strip,
    cap,
    frame: Array.from(a.frame(packing)),
    quad,
    k0,
    octaves,
    total,
    roots: inner.length,
    data,
    ints,
    bands,
    hues,
    fords,
    hull: hull ? { cx: hull.x / hull.k || 0, cy: hull.y / hull.k || 0, r: -1 / hull.k } : null,
    at,
    span: Math.max(0, Math.log2(cap / k0)) * OCTAVE,
    facts,
  };
}

export function shadow(num, packing, data, roots, order) {
  const lit = count(data, roots, roots + packing.circles.length, 2 * order * order) - roots;
  const read = num.apollonian.shadow({ ...packing, circles: packing.circles.slice(0, lit) }, order);
  return { order, reach: read.reach, covered: read.covered, nodes: read.nodes, touched: read.touched, missed: read.missed, offford: read.offford, bright: Number(read.bright), want: Number(read.want) };
}

/* TIME */

export function clock(t, span, hold, still) {
  if (still) return { loop: 0, share: 1 };
  const whole = span + hold;
  const loop = Math.floor(t / whole);
  const local = t - loop * whole;
  return { loop, share: span > 0 ? Math.min(1, local / span) : 1 };
}

export function reach(k0, cap, share) {
  return cap > k0 ? k0 * (cap / k0) ** share : cap;
}

export function count(data, from, to, K) {
  let lo = from;
  let hi = to;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (data[mid * STRIDE + 3] <= K) lo = mid + 1;
    else hi = mid;
  }
  return lo;
}

/* LAYOUT */

export function layout(frame, w, h, pad) {
  const [x0, y0, x1, y1] = frame;
  const room = Math.max(1, h - 2 * pad);
  const wide = Math.max(1, w - 2 * pad);
  const k = Math.min(wide / (x1 - x0 || 1), room / (y1 - y0 || 1));
  return { k, ox: w / 2 - ((x0 + x1) / 2) * k, oy: h / 2 + ((y0 + y1) / 2) * k };
}

export function hit(data, shown, x, y) {
  for (let i = 0; i < shown; i++) {
    const dx = x - data[i * STRIDE];
    const dy = y - data[i * STRIDE + 1];
    const r = data[i * STRIDE + 2];
    if (dx * dx + dy * dy <= r * r) return i;
  }
  return -1;
}

/* LOOK */

export function glyph(radius, digits) {
  return Math.min(0.9 * radius, (2.4 * radius) / digits);
}

export function shade(band, bands) {
  return bands > 1 ? PALE + (1 - PALE) * (band / (bands - 1)) : 1;
}

export function nib(px) {
  return Math.min(1.3, Math.max(0.4, px / 12));
}

export function said(read) {
  const triple = `(${read.k}, ${read.x}, ${read.y})`;
  if (read.ford) return read.den ? `${triple} Ford at ${read.num}/${read.den}` : `${triple} Ford`;
  return read.line ? `${triple} on the line` : triple;
}
