import { pick } from '../../lib/scene.js';
import { marks, num as fix, runs, sheet } from '../../lib/svg.js';

export const RINGS = [['square', 'Square, a + bi'], ['hex', 'Hex, a + bw']];
export const LOOKS = [['fate', 'Fate'], ['norm', 'Norm'], ['plain', 'Plain']];
export const NAMES = { square: 'gaussian', hex: 'eisenstein' };
export const LETTERS = { square: 'i', hex: 'w' };
export const FATES = ['composite', 'split', 'inert', 'ramified', 'unit', 'zero'];
export const REACH = { square: 200, hex: 231 };
export const SPAN = [20, 120];
export const STEP = 10;
export const BANDS = 8;
export const HOLD = 4000;
export const PAD = 16;
export const UNIT = 10;
export const FAINT = 0.12;
export const HALF = 0.5;
export const FADE = 0.6;
export const INSET = 0.1;
const CLASSES = { Composite: 0, Split: 1, Inert: 2, Ramified: 3, Unit: 4, Zero: 5 };
const ABSENT = 255;

export const cap = (ring) => REACH[ring];

/* STUDY */

export function choose(value, rand) {
  if (!value.roll) return { word: value.ring, limit: value.limit, look: value.look };
  const [low, high] = SPAN;
  return { word: pick(rand, Object.keys(NAMES)), limit: low + Math.floor(rand() * (high - low + 1)), look: pick(rand, LOOKS)[0] };
}

let kept = null;

export function study(num, word, r) {
  if (kept && kept.num === num && kept.word === word && kept.r === r) return kept.plan;
  const plan = survey(num, word, r);
  kept = { num, word, r, plan };
  return plan;
}

function survey(num, word, r) {
  const g = num.gauss;
  const ring = g.Ring.named(NAMES[word]);
  const side = 2 * r + 1;
  const top = Number(g.Ring.top(ring, r));
  const units = g.Ring.units(ring);
  const window = new g.Window(ring, r);
  const census = window.census();
  const count = census.points;
  const a = new Int32Array(count);
  const b = new Int32Array(count);
  const norm = new Uint32Array(count);
  const fate = new Uint8Array(count);
  const grid = new Uint8Array(side * side).fill(ABSENT);
  const shells = g.shells(ring, top);
  const classes = g.classes(ring, top);
  let n = 0;
  let k = 0;
  const put = (x, y, m, f) => {
    if (x < -r || x > r || y < -r || y > r) return;
    if (word === 'hex' && !window.holds(x, y)) return;
    a[n] = x;
    b[n] = y;
    norm[n] = m;
    fate[n] = f;
    grid[(y + r) * side + (x + r)] = f;
    n++;
  };
  put(0, 0, 0, CLASSES[window.class(0, 0)]);
  for (let m = 1; m <= top; m++) {
    const some = shells[m] / units;
    for (let j = 0; j < some; j++, k++) {
      const [x, y] = classes[k];
      const f = CLASSES[window.class(x, y)];
      for (const [u, v] of g.Ring.associates(ring, x, y)) put(Number(u), Number(v), m, f);
    }
  }
  if (n !== count || k !== classes.length) throw new Error(`gaussian: the shells lay ${n} points, the window holds ${count}`);
  window.free();
  const ex = g.Ring.place(ring, 1, 0);
  const ey = g.Ring.place(ring, 0, 1);
  let x0 = Infinity;
  let y0 = Infinity;
  let x1 = -Infinity;
  let y1 = -Infinity;
  const held = new Uint32Array(top + 1);
  for (let p = 0; p < count; p++) {
    const x = a[p] * ex[0] + b[p] * ey[0];
    const y = a[p] * ex[1] + b[p] * ey[1];
    x0 = Math.min(x0, x);
    x1 = Math.max(x1, x);
    y0 = Math.min(y0, y);
    y1 = Math.max(y1, y);
    held[norm[p]]++;
  }
  let whole = top;
  for (let m = 1; m <= top; m++) {
    if (held[m] !== shells[m]) {
      whole = m - 1;
      break;
    }
  }
  const [busy, most] = g.peak(ring, whole);
  const facts = { ...census, word, ring: NAMES[word], reach: r, top, symmetry: g.Ring.symmetry(ring), busy, most };
  return { word, ring, r, side, top, count, a, b, norm, fate, grid, ex, ey, frame: [x0 - 0.5, y0 - 0.5, x1 + 0.5, y1 + 0.5], facts };
}

/* POINT */

export function hit(num, plan, x, y) {
  const { r, side, grid, ring } = plan;
  if (x < -r || x > r || y < -r || y > r) return null;
  const fate = grid[(y + r) * side + (x + r)];
  if (fate === ABSENT) return null;
  const R = num.gauss.Ring;
  const norm = Number(R.norm(ring, x, y));
  const factors = num.factor.factorize_wide(norm).map(([p, e]) => [Number(p), e]);
  const associates = R.associates(ring, x, y).map(([u, v]) => [Number(u), Number(v)]);
  const conjugate = R.conjugate(ring, x, y).map(Number);
  return { a: x, b: y, norm, fate, factors, associates, conjugate };
}

export function name(word, a, b) {
  const letter = LETTERS[word];
  const size = Math.abs(b) === 1 ? '' : String(Math.abs(b));
  if (b === 0) return String(a);
  if (a === 0) return `${b < 0 ? '-' : ''}${size}${letter}`;
  return `${a} ${b < 0 ? '-' : '+'} ${size}${letter}`;
}

export const product = (factors) => factors.map(([p, e]) => (e > 1 ? `${p}^${e}` : String(p))).join(' · ');

export function verdict(word, point) {
  const { fate, norm, factors } = point;
  const [ca, cb] = point.conjugate;
  if (fate === 5) return 'the origin';
  if (fate === 4) return 'norm 1, a unit';
  if (fate === 3) return `norm ${norm}, ${norm} ramifies`;
  if (fate === 2) return `norm ${norm} = ${product(factors)}, ${factors[0][0]} stays prime`;
  if (fate === 1) return `norm ${norm} = (${name(word, point.a, point.b)})(${name(word, ca, cb)}), split`;
  return `norm ${norm} = ${product(factors)}, composite`;
}

/* TIME */

export function share(t, span, still) {
  if (still || span <= 0) return 1;
  const loop = span + HOLD;
  const phase = ((t % loop) + loop) % loop;
  return Math.min(1, phase / span);
}

export function reached(part, count) {
  return Math.round(part * part * (count - 1));
}

/* LAYOUT */

export function lay(frame, w, h, pad, cell = 0) {
  const [x0, y0, x1, y1] = frame;
  const k = cell > 0 ? cell : Math.min((w - 2 * pad) / (x1 - x0 || 1), (h - 2 * pad) / (y1 - y0 || 1));
  return { k, ox: w / 2 - ((x0 + x1) / 2) * k, oy: h / 2 + ((y0 + y1) / 2) * k };
}

/* STYLE */

export const SKIP = 0;
export const FAINTLY = 1;
export const HOLLOW = 2;
export const HOLLOW_HALF = 3;
export const STRONG = 4;
export const HALF_TONE = 5;
export const BAND = 6;

export function styleOf(look, faint, fate, norm, top) {
  if (fate === 5) return SKIP;
  if (fate === 0) return faint ? FAINTLY : SKIP;
  if (fate === 3) return HOLLOW;
  if (fate === 4) return look === 'plain' ? HOLLOW : HOLLOW_HALF;
  if (look === 'norm') return BAND + Math.min(BANDS - 1, Math.floor(Math.sqrt(norm / top) * BANDS));
  if (look === 'fate' && fate === 2) return HALF_TONE;
  return STRONG;
}

export function palette(mix, veil, accent, paper) {
  const half = mix(accent, paper, HALF);
  const tones = [null, { color: veil(accent, FAINT), hollow: false }, { color: accent, hollow: true }, { color: half, hollow: true }, { color: accent, hollow: false }, { color: half, hollow: false }];
  for (let k = 0; k < BANDS; k++) tones.push({ color: mix(accent, paper, (FADE * k) / (BANDS - 1)), hollow: false });
  return tones;
}

/* FILES */

export function picture(plan, style, tones) {
  const { word, r, side, count, a, b, ex, ey, frame } = plan;
  const [x0, y0, x1, y1] = frame;
  const w = (x1 - x0) * UNIT;
  const h = (y1 - y0) * UNIT;
  const inset = INSET * UNIT;
  const groups = tones.map((tone) => (tone ? [] : null));
  if (word === 'square') {
    const cells = tones.map((tone) => (tone ? new Uint8Array(side * side) : null));
    for (let p = 0; p < count; p++) if (cells[style[p]]) cells[style[p]][(b[p] + r) * side + (a[p] + r)] = 1;
    cells.forEach((types, s) => {
      if (!types) return;
      const gap = s === FAINTLY ? 0 : inset;
      const spans = tones[s].hollow ? marks(Array.from(types.keys()).filter((i) => types[i]), side).map(([col, row]) => [row, col, 1]) : runs({ shape: [side, side], types });
      for (const [row, col, len] of spans) groups[s].push(`<rect x="${fix(col * UNIT + gap)}" y="${fix((side - 1 - row) * UNIT + gap)}" width="${fix(len * UNIT - 2 * gap)}" height="${fix(UNIT - 2 * gap)}"/>`);
    });
  } else {
    for (let p = 0; p < count; p++) {
      const s = style[p];
      if (!groups[s]) continue;
      const x = a[p] * ex[0] + b[p] * ey[0] - x0;
      const y = y1 - (a[p] * ex[1] + b[p] * ey[1]);
      groups[s].push(`<circle cx="${fix(x * UNIT)}" cy="${fix(y * UNIT)}" r="${fix(UNIT / 2 - (s === FAINTLY ? 0 : inset))}"/>`);
    }
  }
  const body = groups.map((marks, s) => {
    if (!marks || !marks.length) return '';
    const { color, hollow } = tones[s];
    const paint = hollow ? `fill="none" stroke="${color}" stroke-width="${fix(UNIT * INSET)}"` : `fill="${color}"`;
    return `<g ${paint}>${marks.join('')}</g>`;
  });
  return sheet(w, h, body.join(''));
}

export function table(plan) {
  const { count, a, b, norm, fate } = plan;
  const rows = ['a,b,norm,fate'];
  for (let p = 0; p < count; p++) if (fate[p] >= 1 && fate[p] <= 3) rows.push(`${a[p]},${b[p]},${norm[p]},${FATES[fate[p]]}`);
  return `${rows.join('\n')}\n`;
}
