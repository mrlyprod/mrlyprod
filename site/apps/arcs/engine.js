import { named, resolve, total, word } from '../designs/engine.js';
import { num as fix, runs, sheet } from '../../lib/svg.js';

export const DIM = 2;
export const BUDGET = 65536;
export const LOOKS = [['all', 'All'], ['loops', 'Loops'], ['strands', 'Strands']];
export const UNIT = 10;
export const PAD = 16;
export const SWAY = 0.5;
export const ORBIT = [7000, 5300];
export const BEAT = 400;
export const PACE = 40;
export const LEAST = 3000;
export const MOST = 40000;
export const HOLD = 4000;
export const FILL = 0.08;
export const FADE = 0.55;
const HALF = Math.PI / 2;

/* ROOM */

export function cap(base) {
  const b = Number(base) === 2 ? 2 : 3;
  let level = 1;
  while (b ** (2 * (level + 1)) <= BUDGET) level++;
  return level;
}

/* DESIGN */

export function looping(math, base, level, rand = Math.random) {
  const pool = Array.from({ length: total(math, DIM, base) - 1 }, (_, i) => i + 1);
  for (let i = 0; i < pool.length; i++) {
    const j = i + Math.floor(rand() * (pool.length - i));
    [pool[i], pool[j]] = [pool[j], pool[i]];
    if (math.arcs.draw(pool[i], base, level).loops) return String(pool[i]);
  }
  return String(pool[0]);
}

export function choose(math, value, rand) {
  const text = String(value.code ?? '').trim();
  if (text !== '' && Number.isFinite(Number(text))) return String(resolve(text, total(math, DIM, value.base), rand));
  return looping(math, value.base, value.level, rand);
}

/* ARCS */

export function corner(filled, which, x, y) {
  if (which) return filled ? [x + 1, y + 1] : [x, y + 1];
  return filled ? [x, y] : [x + 1, y];
}

export function arc(plan, a) {
  const { side, cells } = plan;
  const i = a >> 1;
  const x = i % side;
  const y = (i - x) / side;
  const [cx, cy] = corner(cells[i] & 1, a & 1, x, y);
  const right = cx !== x;
  const down = cy !== y;
  return { x, y, cx, cy, av: down ? -HALF : HALF, ah: right ? Math.PI : 0, sweep: right !== down ? HALF : -HALF };
}

export function curves(drawn) {
  const n = drawn.side;
  const cells = Uint8Array.from(drawn.cells);
  const count = 2 * n * n;
  const nodes = 2 * n * (n + 1);
  const ends = new Int32Array(count * 2);
  const at = new Int32Array(nodes * 2).fill(-1);
  const filled = new Uint8Array(n * n);
  for (let i = 0; i < n * n; i++) {
    const x = i % n;
    const y = (i - x) / n;
    filled[i] = cells[i] & 1;
    for (let which = 0; which < 2; which++) {
      const [cx, cy] = corner(filled[i], which, x, y);
      const a = 2 * i + which;
      const v = n * (n + 1) + y * (n + 1) + cx;
      const h = cy * n + x;
      ends[2 * a] = v;
      ends[2 * a + 1] = h;
      at[2 * v + (at[2 * v] < 0 ? 0 : 1)] = a;
      at[2 * h + (at[2 * h] < 0 ? 0 : 1)] = a;
    }
  }
  const seen = new Uint8Array(count);
  const order = new Int32Array(count);
  const dir = new Int8Array(count);
  const bounds = [0];
  const closed = [];
  let k = 0;
  const bit = (a) => (cells[a >> 1] >> (1 + (a & 1))) & 1;
  const walk = (start, from) => {
    let a = start;
    let node = from;
    for (;;) {
      seen[a] = 1;
      order[k] = a;
      const entered = ends[2 * a] === node;
      dir[k] = entered ? 1 : -1;
      k++;
      node = entered ? ends[2 * a + 1] : ends[2 * a];
      const next = at[2 * node] === a ? at[2 * node + 1] : at[2 * node];
      if (next < 0) return 0;
      if (next === start) return 1;
      a = next;
    }
  };
  const take = (a, node) => {
    if (walk(a, node) !== bit(a)) throw new Error(`arcs: the walk and the crate disagree on the curve through arc ${a}`);
    closed.push(bit(a));
    bounds.push(k);
  };
  const edge = (node) => {
    if (node < n * (n + 1)) return node < n || node >= n * n;
    const x = (node - n * (n + 1)) % (n + 1);
    return x === 0 || x === n;
  };
  for (let node = 0; node < nodes; node++) {
    if (!edge(node)) continue;
    const a = at[2 * node];
    if (a >= 0 && !seen[a]) take(a, node);
  }
  for (let a = 0; a < count; a++) if (!seen[a]) take(a, ends[2 * a]);
  const loops = closed.reduce((sum, one) => sum + one, 0);
  if (loops !== drawn.loops || closed.length - loops !== drawn.strands) throw new Error(`arcs: the walk finds ${loops} loops and ${closed.length - loops} strands, the crate counts ${drawn.loops} and ${drawn.strands}`);
  return { side: n, cells, filled, order, dir, bounds: Int32Array.from(bounds), closed: Uint8Array.from(closed), loops: drawn.loops, strands: drawn.strands };
}

/* STUDY */

let kept = null;

export function shuffle(list, rand) {
  for (let i = list.length - 1; i > 0; i--) {
    const j = Math.floor(rand() * (i + 1));
    const t = list[i];
    list[i] = list[j];
    list[j] = t;
  }
  return list;
}

export function study(math, value, rand) {
  const { base, level } = value;
  const code = choose(math, value, rand);
  const key = `${base}:${code}:${level}`;
  if (!kept || kept.math !== math || kept.key !== key) {
    const plan = curves(math.arcs.draw(code, base, level));
    const law = math.arcs.law(code, base, level);
    kept = { math, key, plan, law: law ? { formula: law.formula, loops: Number(law.loops) } : null };
  }
  const { plan, law } = kept;
  const loops = [];
  for (let k = 0; k < plan.closed.length; k++) if (plan.closed[k]) loops.push(k);
  const lit = Int32Array.from(shuffle(loops, rand));
  const clock = schedule(Array.from(lit, (k) => plan.bounds[k + 1] - plan.bounds[k]));
  let filled = 0;
  for (const on of plan.filled) filled += on;
  const facts = { code, name: word(named(math, DIM, base), code), base, level, side: plan.side, cells: plan.side * plan.side, filled, loops: plan.loops, strands: plan.strands, law };
  return { ...plan, code, lit, clock, facts };
}

/* TIME */

export function schedule(lengths) {
  const n = lengths.length;
  const dur = Float64Array.from(lengths, (len) => BEAT + (len / PACE) * 1000);
  const sum = dur.reduce((a, b) => a + b, 0);
  const span = n ? Math.min(MOST, Math.max(LEAST, sum)) : 0;
  const start = new Float64Array(n);
  const end = new Float64Array(n);
  let before = 0;
  for (let k = 0; k < n; k++) {
    const rest = sum - dur[k];
    dur[k] = Math.min(dur[k], span);
    start[k] = rest > 0 ? Math.max(0, (before * (span - dur[k])) / rest) : 0;
    end[k] = Math.min(span, start[k] + dur[k]);
    before += dur[k];
  }
  const by = (times) => Int32Array.from(times.keys()).sort((a, b) => times[a] - times[b] || a - b);
  return { span, dur, start, end, byStart: by(start), byEnd: by(end) };
}

export function phase(t, span, still) {
  if (still) return Infinity;
  const loop = span + HOLD;
  return ((t % loop) + loop) % loop;
}

export function drift(t, box) {
  return [SWAY * Math.max(0, box.x) * Math.sin(t / ORBIT[0]), SWAY * Math.max(0, box.y) * Math.sin(t / ORBIT[1])];
}

/* LAYOUT */

export function fit(side, w, h, pad) {
  const px = Math.max(1, Math.min(w, h) - 2 * pad) / side;
  return { px, x: (w - px * side) / 2, y: (h - px * side) / 2 };
}

/* SVG */

export const bend = (r, x, y, clockwise) => `A${fix(r)} ${fix(r)} 0 0 ${clockwise ? 1 : 0} ${fix(x)} ${fix(y)}`;

export function trail(plan, k, unit) {
  const { order, dir, bounds } = plan;
  const r = unit / 2;
  let d = '';
  for (let j = bounds[k]; j < bounds[k + 1]; j++) {
    const g = arc(plan, order[j]);
    const v = [g.cx * unit, (g.y + 0.5) * unit];
    const h = [(g.x + 0.5) * unit, g.cy * unit];
    const [from, to] = dir[j] > 0 ? [v, h] : [h, v];
    if (j === bounds[k]) d += `M${fix(from[0])} ${fix(from[1])}`;
    d += bend(r, to[0], to[1], dir[j] > 0 ? g.sweep > 0 : g.sweep < 0);
  }
  return plan.closed[k] ? `${d}Z` : d;
}

export function picture(plan, value, shades) {
  const { side, closed, filled } = plan;
  const w = side * UNIT;
  const body = [];
  if (value.cells) {
    const boxes = runs({ shape: [side, side], types: filled }).map(([row, col, len]) => `<rect x="${fix(col * UNIT)}" y="${fix(row * UNIT)}" width="${fix(len * UNIT)}" height="${fix(UNIT)}"/>`);
    if (boxes.length) body.push(`<g fill="${shades.fill}">${boxes.join('')}</g>`);
  }
  const strands = [];
  const loops = [];
  for (let k = 0; k < closed.length; k++) (closed[k] ? loops : strands).push(trail(plan, k, UNIT));
  const stroke = (d, color) => `<path d="${d}" fill="none" stroke="${color}" stroke-width="${fix(value.width * UNIT)}" stroke-linecap="round" stroke-linejoin="round"/>`;
  if (value.look !== 'loops' && strands.length) body.push(stroke(strands.join(''), shades.dim));
  if (value.look !== 'strands' && loops.length) body.push(stroke(loops.join(''), shades.accent));
  return sheet(w, w, body.join(''));
}
