import { num as fix, path, sheet } from '../../lib/svg.js';

export const LOOKS = [['line', 'Line'], ['dots', 'Dots'], ['both', 'Both']];
export const FACES = [['walk', 'Walk'], ['stairs', 'Stairs']];
export const TOP = 10000;
export const SPANS = [10, 300];
export const ROLL = 2000;
export const HOLD = 2500;
export const FADE = 600;
export const FLASH = 700;
export const PAD = 24;
export const GHOST = 0.22;
export const AXIS = 0.12;
export const BANDS = 12;
export const DOT = 0.05;
export const KEEP = 8000;
export const LEAST = 30;
export const MOST = 200;
export const DASH = [4, 5];
export const LINE = 1.25;
export const TRAIL = 2.2;
export const BEAD = 1.3;
export const PIP = 2;
export const HEAD = 3.5;
export const ZEROS = 100;
export const REACH = [10, 1000];
export const BEAT = 300;
export const FILL = 0.1;
export const EDGE = 0.5;
export const GUESS = 0.45;
export const CURVE = 1.5;
export const STAIR = 2.5;
const BASE = 24;
const RISE = 16;
const STEPS = 36;
const POLISH = 200;
const NEAR = 1e-4;
const PASSES = 20;
const SAMPLES = 600;
const START = 1.5;
const ROOM = 1.04;

const lines = new WeakMap();

const lineOf = (num) => {
  let line = lines.get(num);
  if (!line) {
    line = new num.zeta.Line();
    lines.set(num, line);
  }
  return line;
};

function memo(build) {
  let last = null;
  return (num, ...key) => {
    if (last && last.num === num && last.key.every((one, i) => one === key[i])) return last.plan;
    const plan = build(num, ...key);
    last = { num, key, plan };
    return plan;
  };
}

/* CHOICE */

export function choose(value, rand) {
  return value.from > 0 ? value.from : Math.floor(rand() * ROLL);
}

export function density(top) {
  return Math.min(MOST, Math.max(LEAST, Math.round(BASE + RISE * Math.log(1 + top / 20))));
}

export function longest(from, span) {
  return Math.max(1, Math.floor(KEEP / density((from || ROLL) + span)));
}

/* WALK */

function bisect(f, a, b, steps) {
  let fa = f(a);
  for (let k = 0; k < steps; k++) {
    const m = (a + b) / 2;
    const fm = f(m);
    if (Math.sign(fm) === Math.sign(fa)) {
      a = m;
      fa = fm;
    } else b = m;
  }
  return (a + b) / 2;
}

export function refine(line, a, b) {
  const t = bisect((u) => line.z(u), a, b, STEPS);
  if (t >= POLISH) return t;
  const lo = t - NEAR;
  const hi = t + NEAR;
  if (Math.sign(line.exact(lo)) === Math.sign(line.exact(hi))) return t;
  return bisect((u) => line.exact(u), lo, hi, PASSES);
}

function walk(num, from, span) {
  const line = lineOf(num);
  const to = from + span;
  const per = density(to);
  const n = Math.round(span * per) + 1;
  const xs = new Float64Array(n);
  const ys = new Float64Array(n);
  const found = [];
  let x0 = -1;
  let y0 = -1;
  let x1 = 1;
  let y1 = 1;
  let reach = 0;
  let last = 0;
  let before = from;
  for (let i = 0; i < n; i++) {
    const t = from + i / per;
    const [p, z] = line.point(t);
    const x = p.re;
    const y = p.im;
    p.free();
    xs[i] = x;
    ys[i] = y;
    x0 = Math.min(x0, x);
    y0 = Math.min(y0, y);
    x1 = Math.max(x1, x);
    y1 = Math.max(y1, y);
    reach = Math.max(reach, Math.hypot(x, y));
    if (i && Math.sign(z) !== Math.sign(last)) found.push(refine(line, before, t));
    last = z;
    before = t;
  }
  const zeros = Float64Array.from(found);
  const first = line.count(from) + 1;
  const facts = { face: 'walk', from, to, zeros: zeros.length, first, reach, samples: n };
  return { from, to, span, per, n, xs, ys, zeros, frame: [x0, y0, x1, y1], reach, first, facts };
}

export const study = memo(walk);

/* TIME */

export function phase(t, length, still) {
  if (still) return { at: 1, veil: 1, play: 0 };
  const loop = length + HOLD;
  const p = ((t % loop) + loop) % loop;
  return { at: Math.min(1, p / length), veil: Math.min(1, (loop - p) / FADE), play: p };
}

export function index(share, n) {
  return Math.round(share * (n - 1));
}

export function passed(zeros, t) {
  let lo = 0;
  let hi = zeros.length;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (zeros[mid] <= t) lo = mid + 1;
    else hi = mid;
  }
  return lo;
}

export function flares(zeros, from, speed, play) {
  const out = [];
  for (let z = zeros.length - 1; z >= 0; z--) {
    const age = play - ((zeros[z] - from) / speed) * 1000;
    if (age < 0) continue;
    if (age > FLASH) break;
    out.push(age / FLASH);
  }
  return out;
}

export function bands(head, kept) {
  const out = [];
  const width = kept / BANDS;
  let owned = 0;
  for (let b = BANDS - 1; b >= 0; b--) {
    const from = Math.max(0, Math.round(head - (b + 1) * width));
    const to = Math.max(0, Math.round(head - b * width));
    out.push({ from, to, first: b === BANDS - 1 ? from : owned, alpha: GHOST + (1 - GHOST) * (1 - b / BANDS) });
    owned = to + 1;
  }
  return out;
}

/* FIT */

export function fit(frame, w, h, pad) {
  const [x0, y0, x1, y1] = frame;
  const k = Math.max(0, Math.min((w - 2 * pad) / (x1 - x0 || 1), (h - 2 * pad) / (y1 - y0 || 1)));
  return { k, ox: w / 2 - ((x0 + x1) / 2) * k, oy: h / 2 + ((y0 + y1) / 2) * k };
}

/* STAIRS */

function climb(num, x, zeros) {
  const gammas = zeros ? lineOf(num).zeros(zeros) : new Float64Array(0);
  const stair = num.zeta.psi_stair(x);
  const grid = Float64Array.from({ length: SAMPLES }, (_, i) => START + ((x - START) * i) / (SAMPLES - 1));
  const ladder = { xs: new Float64Array(2 * x - 1), ys: new Float64Array(2 * x - 1) };
  for (let n = 1, j = 0; n <= x; n++) {
    ladder.xs[j] = n;
    ladder.ys[j++] = stair[n - 1];
    if (n < x) {
      ladder.xs[j] = n + 1;
      ladder.ys[j++] = stair[n - 1];
    }
  }
  const curves = new Map();
  const curve = (k) => {
    if (!curves.has(k)) {
      const some = gammas.subarray(0, k);
      curves.set(k, Float64Array.from(grid, (u) => num.zeta.psi_formula(u, some)));
    }
    return curves.get(k);
  };
  const psi = stair[x - 1];
  const formula = num.zeta.psi_formula(x, gammas);
  const top = Math.max(psi, formula, ...curve(zeros)) * ROOM;
  const facts = { face: 'stairs', x, zeros, first: 1, last: zeros ? gammas[zeros - 1] : null, psi, formula, gap: Math.abs(formula - psi) };
  return { x, zeros, gammas, first: 1, stair, grid, ladder, curve, top, facts };
}

export const fold = memo(climb);

export function layout(plan, w, h, pad) {
  return { left: pad, right: w - pad, roof: pad, base: h - pad, kx: (w - 2 * pad) / (plan.x - 1), ky: (h - 2 * pad) / plan.top };
}

/* FILES */

export function picture(plan, { w, h, pad }, head, kept, look, accent) {
  const box = fit(plan.frame, w, h, pad);
  const X = (i) => box.ox + plan.xs[i] * box.k;
  const Y = (i) => box.oy - plan.ys[i] * box.k;
  const every = Math.max(1, Math.round(plan.per * DOT));
  const lined = look !== 'dots';
  const dotted = look !== 'line';
  const stroke = (from, to, alpha, width, cap) => {
    if (to <= from) return '';
    const pts = [];
    for (let i = from; i <= to; i++) pts.push(X(i), Y(i));
    return `<path d="${path(pts)}" fill="none" stroke="${accent}" stroke-opacity="${fix(alpha)}" stroke-width="${fix(width)}" stroke-linecap="${cap}" stroke-linejoin="round"/>`;
  };
  const dots = (from, to, alpha, r) => {
    const out = [];
    for (let i = Math.ceil(from / every) * every; i <= to; i += every) out.push(`<circle cx="${fix(X(i))}" cy="${fix(Y(i))}" r="${fix(r)}"/>`);
    return out.length ? `<g fill="${accent}" fill-opacity="${fix(alpha)}">${out.join('')}</g>` : '';
  };
  const body = [];
  body.push(`<g stroke="${accent}" stroke-opacity="${fix(AXIS)}" stroke-width="1" fill="none"><path d="M0 ${fix(box.oy)}H${fix(w)}M${fix(box.ox)} 0V${fix(h)}"/><circle cx="${fix(box.ox)}" cy="${fix(box.oy)}" r="${fix(box.k)}" stroke-dasharray="${DASH.join(' ')}"/></g>`);
  body.push(lined ? stroke(0, head, GHOST, LINE, 'round') : dots(0, head, GHOST, BEAD));
  for (const { from, to, first, alpha } of bands(head, kept)) {
    if (lined) body.push(stroke(from, to, alpha, TRAIL, 'butt'));
    if (dotted) body.push(dots(first, to, alpha, PIP));
  }
  body.push(`<circle cx="${fix(X(head))}" cy="${fix(Y(head))}" r="${fix(HEAD)}" fill="${accent}"/>`);
  return sheet(w, h, body.join(''));
}

export function staircase(plan, { w, h, pad }, k, accent) {
  const box = layout(plan, w, h, pad);
  const X = (u) => box.left + (u - 1) * box.kx;
  const Y = (v) => box.base - v * box.ky;
  const along = (xs, ys) => {
    const pts = [];
    for (let i = 0; i < xs.length; i++) pts.push(X(xs[i]), Y(ys[i]));
    return pts;
  };
  const edge = along(plan.ladder.xs, plan.ladder.ys);
  const stroke = (pts, alpha, width, extra = '') => `<path d="${path(pts)}" fill="none" stroke="${accent}" stroke-opacity="${fix(alpha)}" stroke-width="${fix(width)}" stroke-linejoin="round"${extra}/>`;
  const floor = [X(plan.x), box.base, X(1), box.base];
  const body = [];
  body.push(`<g stroke="${accent}" stroke-opacity="${fix(AXIS)}" stroke-width="1" fill="none"><path d="M0 ${fix(box.base)}H${fix(w)}M${fix(box.left)} 0V${fix(h)}"/></g>`);
  body.push(`<clipPath id="zeta-frame"><rect x="${fix(box.left)}" y="${fix(box.roof)}" width="${fix(box.right - box.left)}" height="${fix(box.base - box.roof)}"/></clipPath>`);
  const inner = [];
  inner.push(`<path d="${path([...edge, ...floor], true)}" fill="${accent}" fill-opacity="${fix(FILL)}"/>`);
  inner.push(stroke(edge, EDGE, STAIR));
  inner.push(stroke(along(plan.grid, plan.curve(0)), GUESS, 1, ` stroke-dasharray="${DASH.join(' ')}"`));
  inner.push(stroke(along(plan.grid, plan.curve(k)), 1, CURVE));
  body.push(`<g clip-path="url(#zeta-frame)">${inner.join('')}</g>`);
  return sheet(w, h, body.join(''));
}

export function table({ zeros, first }) {
  const rows = ['zero,t'];
  zeros.forEach((t, i) => rows.push(`${first + i},${t.toFixed(6)}`));
  return `${rows.join('\n')}\n`;
}
