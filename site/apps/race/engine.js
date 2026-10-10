import { rgb, rng } from '../../lib/scene.js';
import { named, title, total, word } from '../designs/engine.js';

export const DIM = 2;
export const NUMBERS = [3, 5, 7];
export const BUDGET = 131072;
export const STILL = 1500000;
export const SHARE = 55;
export const LEAST = 10;
export const NOTCH = 5;
export const REACH = 28;
export const HOT = 12;
export const HOLD = 3000;
export const STALL = 1000;
export const LIMIT = 60000;
export const ROLLS = 48;
export const TURN = 180;
export const GAIN = 0.25;
const MOVES = [[1, 0], [-1, 0], [0, 1], [0, -1]];
const WIDE = 4294967296;
const ceilings = new Map();
const masses = new Map();

/* ROOM */

export function cap(number) {
  let level = 1;
  while (number ** (DIM * (level + 1)) <= BUDGET) level++;
  return level;
}

export function goal(side, finish) {
  return Math.max(1, Math.round((((side - 1) / 2) * finish) / 100));
}

export function most(side) {
  let share = SHARE;
  while (share > LEAST && goal(side, share) > REACH) share -= NOTCH;
  return share;
}

/* CODES */

export function reads(text, count) {
  const s = String(text ?? '').trim();
  if (!s) return null;
  const n = Number(s);
  if (!Number.isFinite(n)) return null;
  const whole = Math.floor(n);
  return String(((whole % count) + count) % count);
}

export function roll(rand, count) {
  return String(1 + Math.floor(rand() * (count - 2)));
}

export function roots(rand) {
  const code = rng(Math.floor(rand() * WIDE));
  const walk = Math.floor(rand() * WIDE);
  return { code, walk };
}

/* BOARD */

export function home(filled, side) {
  const centre = (side - 1) / 2;
  let best = -1;
  let least = Infinity;
  for (let i = 0; i < filled.length; i++) {
    if (!filled[i]) continue;
    const r = Math.floor(i / side) - centre;
    const c = (i % side) - centre;
    const d = r * r + c * c;
    if (d < least) {
      least = d;
      best = i;
    }
  }
  return best;
}

export function room(filled, side, from) {
  if (from < 0) return { cells: 0, rms: 0 };
  const seen = new Uint8Array(filled.length);
  const queue = new Int32Array(filled.length);
  const hr = Math.floor(from / side);
  const hc = from % side;
  let head = 0;
  let tail = 0;
  let sum = 0;
  queue[tail++] = from;
  seen[from] = 1;
  while (head < tail) {
    const i = queue[head++];
    const r = Math.floor(i / side);
    const c = i % side;
    sum += (r - hr) ** 2 + (c - hc) ** 2;
    for (const [dr, dc] of MOVES) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr < 0 || nc < 0 || nr >= side || nc >= side) continue;
      const j = nr * side + nc;
      if (seen[j] || !filled[j]) continue;
      seen[j] = 1;
      queue[tail++] = j;
    }
  }
  return { cells: tail, rms: Math.sqrt(sum / tail) };
}

export function grow(math, value, code) {
  const cell = math.two.create(code, value.number, value.level, 0, value.base);
  const side = cell.shape[0];
  const filled = Uint8Array.from(cell.types, (type) => (type ? 1 : 0));
  const at = home(filled, side);
  return { code, side, filled, home: at, lit: Number(math.counts.fill(code, value.number, DIM, value.level, value.base)), ...room(filled, side, at) };
}

export function fills(math, value, code) {
  return Number(math.counts.fill(code, value.number, DIM, 1, value.base));
}

export function ceiling(math, value, code) {
  const key = `${value.base}:${value.number}:${value.level}:${code}`;
  if (!ceilings.has(key)) ceilings.set(key, grow(math, value, code).rms);
  return ceilings.get(key);
}

export function kin(math, value, code, count) {
  const key = `${value.base}:${value.number}`;
  if (!masses.has(key)) masses.set(key, Array.from({ length: count }, (_, c) => fills(math, value, String(c))));
  const list = masses.get(key);
  return list.flatMap((mass, c) => (c > 0 && c < count - 1 && mass === list[Number(code)] && String(c) !== code ? [String(c)] : []));
}

export function board(math, value, code) {
  const { base, number } = value;
  const grown = grow(math, value, code);
  return { ...grown, name: word(named(math, DIM, base), code), title: title(math, DIM, base, code), fills: fills(math, value, code), of: number * number, dimension: math.counts.dimension(code, number, DIM, base) };
}

/* PAIR */

export function pair(math, value, rand, typed = [null, null]) {
  const count = total(math, DIM, value.base);
  const far = goal(value.number ** value.level, value.finish);
  const able = (code) => ceiling(math, value, code) >= far;
  const any = (not) => {
    let code = null;
    for (let i = 0; i < ROLLS; i++) {
      code = roll(rand, count);
      if (code !== not && able(code)) return code;
    }
    return code;
  };
  const mate = (code) => {
    const list = kin(math, value, code, count);
    for (let i = 0; i < Math.min(list.length, ROLLS); i++) {
      const j = i + Math.floor(rand() * (list.length - i));
      [list[i], list[j]] = [list[j], list[i]];
      if (able(list[i])) return list[i];
    }
    return null;
  };
  const both = () => {
    for (let i = 0; i < ROLLS; i++) {
      const a = roll(rand, count);
      const b = able(a) ? mate(a) : null;
      if (b) return [a, b];
    }
    return null;
  };
  const [a, b] = typed;
  if (a !== null && b !== null) return [a, b];
  if (a !== null) return [a, mate(a) ?? any(a)];
  if (b !== null) return [mate(b) ?? any(b), b];
  return both() ?? [any(), any()];
}

/* RACE */

export function plan(math, value, rand) {
  const count = total(math, DIM, value.base);
  const codes = pair(math, value, rand, [reads(value.a, count), reads(value.b, count)]);
  const sides = codes.map((code) => board(math, value, code));
  const side = sides[0].side;
  return { sides, side, goal: goal(side, value.finish), walkers: value.walkers, speed: value.speed, still: Math.max(1, Math.floor(STILL / (2 * value.walkers))), keep: value.heat ? (1 - value.heat / 100) ** (1 / value.speed) : 1 };
}

export function heat(value, stamp, tick, keep) {
  return keep === 1 || tick === stamp || value === 0 ? value : value * keep ** (tick - stamp);
}

export function spread(at, home, side) {
  if (!at.length) return 0;
  const hr = Math.floor(home / side);
  const hc = home % side;
  let total = 0;
  for (const i of at) total += (Math.floor(i / side) - hr) ** 2 + ((i % side) - hc) ** 2;
  return Math.sqrt(total / at.length);
}

export function open(plan) {
  const cells = plan.side * plan.side;
  return {
    tick: 0,
    at: plan.sides.map(({ home }) => new Int32Array(home < 0 ? 0 : plan.walkers).fill(home)),
    trail: plan.sides.map(() => new Float32Array(cells)),
    stamp: plan.sides.map(() => new Int32Array(cells)),
    reach: [0, 0],
    best: [0, 0],
    since: [0, 0],
    over: null,
    log: [[0, 0]],
  };
}

const verdict = ([a, b]) => (a > b ? 0 : b > a ? 1 : -1);

export function step(state, plan, rand) {
  if (state.over) return state;
  const tick = state.tick + 1;
  const { side, keep } = plan;
  plan.sides.forEach(({ filled, home }, s) => {
    const at = state.at[s];
    const trail = state.trail[s];
    const stamp = state.stamp[s];
    for (let w = 0; w < at.length; w++) {
      const i = at[w];
      const r = Math.floor(i / side);
      const c = i % side;
      const [dr, dc] = MOVES[Math.floor(rand() * 4)];
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nc >= 0 && nr < side && nc < side && filled[nr * side + nc]) at[w] = nr * side + nc;
      const j = at[w];
      trail[j] = heat(trail[j], stamp[j], tick, keep) + 1;
      stamp[j] = tick;
    }
    state.reach[s] = spread(at, home, side);
    if (state.reach[s] > state.best[s] + GAIN) {
      state.best[s] = state.reach[s];
      state.since[s] = tick;
    }
  });
  state.tick = tick;
  state.log.push([state.reach[0], state.reach[1]]);
  const hit = state.reach.map((r) => r >= plan.goal);
  if (hit[0] || hit[1]) state.over = { tick, winner: hit[0] && hit[1] ? verdict(state.reach) : hit[0] ? 0 : 1, how: 'goal' };
  else if (tick - Math.max(state.since[0], state.since[1]) >= STALL) state.over = { tick, winner: verdict(state.reach), how: 'stall' };
  else if (tick >= LIMIT) state.over = { tick, winner: verdict(state.reach), how: 'limit' };
  return state;
}

/* SERIES */

export function series(plan, walk) {
  const hold = Math.ceil((HOLD * plan.speed) / 1000);
  const tally = [0, 0, 0];
  let k = 0;
  let start = 0;
  let version = 0;
  let state = open(plan);
  let rand = rng((walk + k) >>> 0);
  const count = () => tally[state.over.winner < 0 ? 2 : state.over.winner]++;
  const next = () => {
    start += state.over.tick + hold;
    k++;
    state = open(plan);
    rand = rng((walk + k) >>> 0);
    version++;
  };
  const to = (at) => {
    for (;;) {
      const local = at - start;
      if (state.over) {
        if (local - state.over.tick < hold) return;
        next();
        continue;
      }
      if (state.tick >= local) return;
      step(state, plan, rand);
      version++;
      if (state.over) count();
    }
  };
  const finish = (limit = Infinity) => {
    if (state.over) return;
    while (!state.over && state.tick < limit) {
      step(state, plan, rand);
      version++;
    }
    if (state.over) count();
  };
  return { to, finish, get state() { return state; }, get k() { return k; }, get start() { return start; }, get tally() { return tally; }, get version() { return version; } };
}

/* LAYOUT */

export function lay(w, h, n, gap, pad) {
  const across = Math.min((w - 2 * pad - gap) / (2 * n), (h - 2 * pad) / n);
  const down = Math.min((w - 2 * pad) / n, (h - 2 * pad - gap) / (2 * n));
  const flat = across >= down;
  const raw = Math.max(flat ? across : down, 1e-3);
  const cell = raw >= 1 ? Math.floor(raw) : raw;
  const edge = cell * n;
  const x0 = flat ? (w - 2 * edge - gap) / 2 : (w - edge) / 2;
  const y0 = flat ? (h - edge) / 2 : (h - 2 * edge - gap) / 2;
  const boards = [[x0, y0], flat ? [x0 + edge + gap, y0] : [x0, y0 + edge + gap]].map(([x, y]) => ({ x: Math.round(x), y: Math.round(y) }));
  return { cell, edge, flat, boards };
}

/* COLOUR */

const hex = (r, g, b) => `#${[r, g, b].map((v) => Math.round(Math.min(1, Math.max(0, v)) * 255).toString(16).padStart(2, '0')).join('')}`;

export function turn(color, degrees) {
  const [r, g, b] = rgb(color).map((v) => v / 255);
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  const l = (max + min) / 2;
  const d = max - min;
  if (!d) return hex(r, g, b);
  const s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
  let h = max === r ? (g - b) / d + (g < b ? 6 : 0) : max === g ? (b - r) / d + 2 : (r - g) / d + 4;
  h = (((h / 6 + degrees / 360) % 1) + 1) % 1;
  const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
  const p = 2 * l - q;
  const channel = (at) => {
    const t = ((at % 1) + 1) % 1;
    if (t < 1 / 6) return p + (q - p) * 6 * t;
    if (t < 1 / 2) return q;
    if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
    return p;
  };
  return hex(channel(h + 1 / 3), channel(h), channel(h - 1 / 3));
}
