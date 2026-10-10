export const ROOTS = ['strip', '-1,2,2,3', '-2,3,6,7', '-3,4,12,13'];
export const CAPS = [8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192];
export const ORDER_CAP = 64;
export const LOOKS = [['fill', 'Filled'], ['rings', 'Rings'], ['labels', 'Labelled']];
export const OCTAVE = 1000;
export const HOLD = 4000;
export const PAD = 24;
export const BAND = 0.22;
export const DUST = 0.4;
export const LEAST = 9;
export const PALE = 0.55;
export const ACROSS = 3;
const EDGE_BIAS = 1e-6;
export const STRIDE = 4;

export const named = (name) => (name === 'strip' ? 'Strip' : `(${name.split(',').join(', ')})`);

export const stack = (cap) => Math.min(ORDER_CAP, Math.floor(Math.sqrt(cap / 2)));

export function choose(root, rand) {
  return root || ROOTS[Math.floor(rand() * ROOTS.length)];
}

export function step(cap, by) {
  const i = CAPS.indexOf(cap);
  return CAPS[Math.min(CAPS.length - 1, Math.max(0, (i < 0 ? CAPS.indexOf(2048) : i) + by))];
}

/* STUDY */

let slot = null;

export function study(num, name, cap) {
  if (slot?.name === name && slot.cap === cap) return slot;
  const a = num.apollonian;
  const packing = a.grow(name, cap);
  const quad = packing.root.map((c) => [c.k, c.x, c.y]);
  const inner = packing.root.filter((c) => c.k > 0);
  const hull = packing.root.find((c) => c.k < 0);
  const rows = [...inner, ...packing.circles];
  const total = rows.length;
  const data = new Float64Array(total * STRIDE);
  const ints = new Int32Array(total * 3);
  rows.forEach((c, i) => {
    data[i * STRIDE] = c.x / c.k;
    data[i * STRIDE + 1] = c.y / c.k;
    data[i * STRIDE + 2] = 1 / c.k;
    data[i * STRIDE + 3] = c.k;
    ints[i * 3] = c.k;
    ints[i * 3 + 1] = c.x;
    ints[i * 3 + 2] = c.y;
  });
  const k0 = packing.circles[0]?.k ?? inner.at(-1)?.k ?? cap;
  const octaves = Math.max(1, Math.ceil(Math.log2(cap / k0)) + 1);
  const bands = Uint8Array.from({ length: total }, (_, i) => Math.min(octaves - 1, Math.max(0, Math.floor(Math.log2(data[i * STRIDE + 3] / k0)))));
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
  const facts = {
    name,
    title: `${name === 'strip' ? 'strip ' : ''}(${quad.map(([k]) => k).join(', ')})`,
    cap,
    circles: packing.circles.length,
    ford: packing.strip ? touches.length : null,
    broken: packing.broken,
  };
  slot = {
    name,
    strip: packing.strip,
    cap,
    packing: packing.strip ? packing : null,
    frame: Array.from(a.frame(packing)),
    quad,
    k0,
    octaves,
    total,
    roots: inner.length,
    data,
    ints,
    bands,
    fords,
    hull: hull ? { cx: hull.x / hull.k || 0, cy: hull.y / hull.k || 0, r: -1 / hull.k } : null,
    at,
    span: Math.max(0, Math.log2(cap / k0)) * OCTAVE,
    facts,
    depths: new Map(),
  };
  return slot;
}

const FLAT = { bars: [], nodes: null, bright: null, want: null };

export function depth(num, plan, order) {
  if (!plan.strip || order <= 0) return FLAT;
  if (plan.depths.has(order)) return plan.depths.get(order);
  const lit = count(plan.data, plan.roots, plan.total, 2 * order * order) - plan.roots;
  const shadow = num.apollonian.shadow({ ...plan.packing, circles: plan.packing.circles.slice(0, lit) }, order);
  const root = plan.data[3];
  const bars = num.lattice.farey(order).map((n) => ({ x: n.num / n.den, h: n.brightness / order, k: plan.at.get(n.num / n.den)?.k ?? root })).sort((p, q) => p.k - q.k);
  const read = { bars, nodes: shadow.nodes, bright: Number(shadow.bright), want: Number(shadow.want) };
  plan.depths.set(order, read);
  return read;
}

/* TIME */

export function clock(t, span, hold, still, once = false) {
  if (still) return { loop: 0, share: 1 };
  const whole = span + hold;
  const loop = once ? 0 : Math.floor(t / whole);
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

export function layout(frame, strip, w, h, pad, band) {
  const [x0, y0, x1, y1] = frame;
  const room = Math.max(1, h - 2 * pad - band);
  const wide = Math.max(1, w - 2 * pad);
  const fit = Math.min(wide / (x1 - x0 || 1), room / (y1 - y0 || 1));
  const k = strip ? Math.min(fit, w / ACROSS / (x1 - x0 || 1)) : fit;
  const ox = w / 2 - ((x0 + x1) / 2) * k;
  const top = (h - (y1 - y0) * k - band) / 2;
  const oy = top + y1 * k;
  const periods = strip ? [Math.ceil(-ox / k - 1 + EDGE_BIAS), Math.floor((w - ox) / k - EDGE_BIAS)] : [0, 0];
  return { k, ox, oy, periods, line: oy - y0 * k };
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

export function thick(band, bands) {
  return 1.4 - 0.9 * (bands > 1 ? band / (bands - 1) : 0);
}

export function said(read) {
  const triple = `(${read.k}, ${read.x}, ${read.y})`;
  if (read.ford) return read.den ? `${triple} Ford at ${read.num}/${read.den}` : `${triple} Ford`;
  return read.line ? `${triple} on the line` : triple;
}
