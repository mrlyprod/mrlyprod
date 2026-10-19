import { rng } from '../../lib/rand.js';
import { num as fix, path, sheet } from '../../lib/app/svg.js';

export const VIEWS = [['walk', 'Walk'], ['stairs', 'Stairs']];
export const LOOKS = [['line', 'Line'], ['dots', 'Dots'], ['both', 'Both']];
export const REACH = 200;
export const PER = 50;
export const STEP = 0.05;
export const ZEROS = 100;
export const LISTED = 8;
export const UPTO = [10, 1000];
export const FIRST = 12;
export const STATES = [
  { id: 'smooth', name: 'No zeros', note: 'the smooth guess', set: { view: 'stairs', x: 100, zeros: 0 } },
  { id: 'few', name: '3 zeros', note: 'the first folds', set: { view: 'stairs', x: 100, zeros: 3 } },
  { id: 'ten', name: '10 zeros', note: 'the start', set: { view: 'stairs', x: 100, zeros: 10 } },
  { id: 'all', name: '100 zeros', note: 'the steps appear', set: { view: 'stairs', x: 100, zeros: 100 } },
];
export const ROLLS = { t: [10, 150], zeros: [1, 60], x: [50, 500] };
export const HOLD = 2500;
export const FADE = 600;
export const FLASH = 700;
export const BEAT = 300;
export const FOLDING = 4000;
export const PAD = 24;
export const GHOST = 0.45;
export const BANDS = 12;
export const DOT = 0.05;
export const DASH = [4, 5];
export const LINE = 1.2;
export const TRAIL = 2.2;
export const BEAD = 1.3;
export const PIP = 2;
export const HEAD = 3.5;
export const RING = 5;
export const FILL = 0.1;
export const CURVE = 1.5;
export const STAIR = 2;
const LEAST = 2;
const SAMPLES = 600;
const START = 1.5;
const ROOM = 1.04;
const KEPT = 8;
const NEAR = 1e-6;

const lines = new WeakMap();

const walks = new WeakMap();

const folds = new WeakMap();

const lineOf = (num) => {
  if (!lines.has(num)) lines.set(num, new num.zeta.Line());
  return lines.get(num);
};

/* WALK */

export function study(num) {
  if (walks.has(num)) return walks.get(num);
  const line = lineOf(num);
  const n = REACH * PER + 1;
  const xs = new Float64Array(n);
  const ys = new Float64Array(n);
  const peak = new Float64Array(n);
  let far = LEAST;
  for (let i = 0; i < n; i++) {
    const [p] = line.point(i / PER);
    xs[i] = p.re;
    ys[i] = p.im;
    far = Math.max(far, Math.abs(p.re), Math.abs(p.im));
    peak[i] = far;
  }
  const plan = { n, xs, ys, peak, zeros: line.zeros(ZEROS) };
  walks.set(num, plan);
  return plan;
}

export function index(t) {
  return Math.max(0, Math.min(REACH * PER, Math.round(t * PER)));
}

export function snap(t) {
  return Math.max(0, Math.min(REACH, +(Math.round(t / STEP) * STEP).toPrecision(12)));
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

export function hop(zeros, t, dir) {
  if (dir > 0) {
    const next = zeros.find((zero) => zero > t + NEAR);
    return next === undefined ? REACH : Math.min(REACH, next);
  }
  const last = zeros.findLast((zero) => zero < t - NEAR);
  return last ?? 0;
}

export function reading(num, plan, t) {
  const line = lineOf(num);
  const [p, z] = line.point(t);
  const count = passed(plan.zeros, t);
  const list = Array.from(plan.zeros.subarray(0, LISTED), (zero) => [zero, zero <= t]);
  return { t, re: p.re, im: p.im, z, theta: line.theta(t), count, next: plan.zeros[count] ?? null, list };
}

export function shelves(zeros, value) {
  const walked = Array.from(zeros.subarray(0, FIRST), (zero, i) => {
    const set = { view: 'walk', t: snap(zero) };
    return { id: `zero${i + 1}`, label: `Zero ${i + 1}`, sub: zero.toFixed(4), set, on: value.view === 'walk' && value.t === set.t };
  });
  const climbed = STATES.map(({ id, name, note, set }) => ({ id, label: name, sub: note, set, on: value.view === 'stairs' && value.x === set.x && value.zeros === set.zeros }));
  return [['Zeros', walked], ['Staircase', climbed]];
}

export function roll(seed) {
  const rand = rng(seed);
  const between = ([lo, hi]) => lo + Math.floor(rand() * (hi - lo + 1));
  return { t: between(ROLLS.t), zeros: between(ROLLS.zeros), x: between(ROLLS.x) };
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

export function phase(t, length, still) {
  if (still) return { at: 1, veil: 1 };
  const loop = length + HOLD;
  const p = ((t % loop) + loop) % loop;
  return { at: Math.min(1, p / length), veil: Math.min(1, (loop - p) / FADE) };
}

export function plane(rect, reach, pad) {
  return { k: Math.max(0, (Math.min(rect.w, rect.h) / 2 - pad) / reach), ox: rect.x + rect.w / 2, oy: rect.y + rect.h / 2 };
}

/* STAIRS */

function climb(num, x) {
  const gammas = study(num).zeros;
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
  const kept = (build) => {
    const seen = new Map();
    return (k) => {
      if (!seen.has(k)) seen.set(k, build(k));
      return seen.get(k);
    };
  };
  const psi = stair[x - 1];
  const curve = kept((k) => num.zeta.psi_curve(gammas.subarray(0, k), grid));
  const facts = kept((k) => ({ x, zeros: k, psi, formula: num.zeta.psi_formula(x, gammas.subarray(0, k)), gap: num.zeta.psi_gap(x, gammas.subarray(0, k)) }));
  const top = kept((k) => Math.max(psi, ...curve(0), ...curve(k)) * ROOM);
  return { x, gammas, stair, grid, ladder, curve, facts, top };
}

export function fold(num, x) {
  if (!folds.has(num)) folds.set(num, new Map());
  const kept = folds.get(num);
  let plan = kept.get(x);
  if (plan) kept.delete(x);
  else plan = climb(num, x);
  kept.set(x, plan);
  if (kept.size > KEPT) kept.delete(kept.keys().next().value);
  return plan;
}

export function ladder(rect, x, top, pad) {
  const left = rect.x + pad;
  const right = Math.max(left + 1, rect.x + rect.w - pad);
  const roof = rect.y + pad;
  const floor = Math.max(roof + 1, rect.y + rect.h - pad);
  return { left, right, roof, floor, kx: (right - left) / (x - 1), ky: (floor - roof) / top };
}

export function along(box, xs, ys) {
  const pts = [];
  for (let i = 0; i < xs.length; i++) pts.push(box.left + (xs[i] - 1) * box.kx, box.floor - ys[i] * box.ky);
  return pts;
}

/* FILES */

const ink = (color, alpha) => `stroke="${color}" stroke-opacity="${fix(alpha)}"`;

function walked(plan, rect, head, kept, look, inks) {
  const i = index(head);
  const box = plane(rect, plan.peak[i], PAD);
  const X = (j) => box.ox + plan.xs[j] * box.k;
  const Y = (j) => box.oy - plan.ys[j] * box.k;
  const every = Math.max(1, Math.round(PER * DOT));
  const lined = look !== 'dots';
  const dotted = look !== 'line';
  const stroke = (from, to, alpha, width, cap) => {
    if (to <= from) return '';
    const pts = [];
    for (let j = from; j <= to; j++) pts.push(X(j), Y(j));
    return `<path d="${path(pts)}" fill="none" ${ink(inks.accent, alpha)} stroke-width="${fix(width)}" stroke-linecap="${cap}" stroke-linejoin="round"/>`;
  };
  const dots = (from, to, alpha, r) => {
    const out = [];
    for (let j = Math.ceil(from / every) * every; j <= to; j += every) out.push(`<circle cx="${fix(X(j))}" cy="${fix(Y(j))}" r="${fix(r)}"/>`);
    return out.length ? `<g fill="${inks.accent}" fill-opacity="${fix(alpha)}">${out.join('')}</g>` : '';
  };
  const body = [lined ? stroke(0, i, GHOST, LINE, 'round') : dots(0, i, GHOST, BEAD)];
  for (const { from, to, first, alpha } of bands(i, kept)) {
    if (lined) body.push(stroke(from, to, alpha, TRAIL, 'butt'));
    if (dotted) body.push(dots(first, to, alpha, PIP));
  }
  body.push(`<circle cx="${fix(box.ox)}" cy="${fix(box.oy)}" r="${RING}" fill="none" stroke="${inks.zero}" stroke-width="2"/>`);
  body.push(`<circle cx="${fix(X(i))}" cy="${fix(Y(i))}" r="${HEAD}" fill="${inks.head}"/>`);
  return body.join('');
}

function stepped(steps, rect, k, top, inks) {
  const box = ladder(rect, steps.x, top, PAD);
  const edge = along(box, steps.ladder.xs, steps.ladder.ys);
  const floor = [box.left + (steps.x - 1) * box.kx, box.floor, box.left, box.floor];
  const id = 'zeta-stairs';
  return [
    `<clipPath id="${id}"><rect x="${fix(box.left)}" y="${fix(box.roof)}" width="${fix(box.right - box.left)}" height="${fix(box.floor - box.roof)}"/></clipPath>`,
    `<g clip-path="url(#${id})">`,
    `<path d="${path([...edge, ...floor], true)}" fill="${inks.zero}" fill-opacity="${FILL}"/>`,
    `<path d="${path(along(box, steps.grid, steps.curve(0)))}" fill="none" stroke="${inks.guess}" stroke-width="1" stroke-dasharray="${DASH.join(' ')}"/>`,
    `<path d="${path(along(box, steps.grid, steps.curve(k)))}" fill="none" stroke="${inks.accent}" stroke-width="${CURVE}" stroke-linejoin="round"/>`,
    `<path d="${path(edge)}" fill="none" stroke="${inks.zero}" stroke-width="${STAIR}" stroke-linejoin="round"/>`,
    '</g>',
  ].join('');
}

export function picture({ plan, steps, view, head, k, zeros, kept, look, w, h }, inks) {
  const rect = { x: 0, y: 0, w, h };
  return sheet(w, h, view === 'stairs' ? stepped(steps, rect, k, steps.top(zeros), inks) : walked(plan, rect, head, kept, look, inks));
}

export function table(zeros) {
  const rows = ['zero,t'];
  zeros.forEach((t, i) => rows.push(`${i + 1},${t.toFixed(6)}`));
  return `${rows.join('\n')}\n`;
}
