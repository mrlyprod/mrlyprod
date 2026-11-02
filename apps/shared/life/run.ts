import * as life from "@mrlyprod/life";
import { bits, population, types } from "./engine.ts";
import type { Mode, Recipe, Stop } from "./recipe.ts";

/* TYPES */

export type State = {
  mode: Mode;
  x: number;
  cap: number;
  side: number;
  reach: number;
  budget: number;
  birth: number[];
  survive: number[];
  span: number[];
  mask: string;
  rows: number;
  cols: number;
  cells: string;
  origin: number[];
  background: number;
  gen: number;
  pop: number;
  seen: Record<string, number[]>;
  stop: Stop | null;
};

export type Config = Recipe;

export type Act = { verb: "tick" };

export const TICK: Act = { verb: "tick" };

/* KEYS */

function times(ah: number, al: number, bh: number, bl: number): [number, number] {
  const a1 = al >>> 16;
  const a0 = al & 0xffff;
  const b1 = bl >>> 16;
  const b0 = bl & 0xffff;
  const p01 = a0 * b1;
  const p10 = a1 * b0;
  const mid = ((a0 * b0) >>> 16) + (p01 & 0xffff) + (p10 & 0xffff);
  const carry = a1 * b1 + (p01 >>> 16) + (p10 >>> 16) + (mid >>> 16);
  return [(carry + Math.imul(al, bh) + Math.imul(ah, bl)) >>> 0, Math.imul(al, bl) >>> 0];
}

const BYTES = Array.from({ length: 256 }, (_, i) => i.toString(16).padStart(2, "0"));

const word = (n: number) => BYTES[n >>> 24] + BYTES[(n >>> 16) & 255] + BYTES[(n >>> 8) & 255] + BYTES[n & 255];

const hex = (hi: number, lo: number) => word(hi) + word(lo);

function hashed(rows: number, cols: number, cells: Uint8Array, background: number): string {
  let [ah, al] = [0xcbf29ce4, 0x84222325];
  let [bh, bl] = [0x9e3779b9, 0x7f4a7c15];
  const quads =
    cells.byteOffset % 4 || cells.length < 64
      ? null
      : new Uint32Array(cells.buffer, cells.byteOffset, cells.length >> 2);
  const nibble = (q: number) => Math.imul(quads![q] & 0x01010101, 0x01020408) >>> 24;
  for (let at = 0; at < cells.length; at += 64) {
    let lo = 0;
    let hi = 0;
    if (quads && at + 64 <= cells.length) {
      const q = at >> 2;
      for (let i = 0; i < 8; i++) lo |= nibble(q + i) << (4 * i);
      for (let i = 0; i < 8; i++) hi |= nibble(q + 8 + i) << (4 * i);
    } else {
      const end = Math.min(64, cells.length - at);
      for (let i = 0; i < 32 && i < end; i++) lo |= (cells[at + i] & 1) << i;
      for (let i = 32; i < end; i++) hi |= (cells[at + i] & 1) << (i - 32);
    }
    [ah, al] = times((ah ^ hi) >>> 0, (al ^ lo) >>> 0, 0x100, 0x1b3);
    const rh = ((bh << 29) | (bl >>> 3)) >>> 0;
    const rl = ((bl << 29) | (bh >>> 3)) >>> 0;
    [bh, bl] = times((rh ^ hi) >>> 0, (rl ^ lo) >>> 0, 0xbf58476d, 0x1ce4e5b9);
  }
  [ah, al] = times(ah, (al ^ cols) >>> 0, 0x100, 0x1b3);
  [bh, bl] = times(bh, (bl ^ rows) >>> 0, 0x94d049bb, 0x133111eb);
  return hex(ah, al) + hex(bh, (bl ^ background) >>> 0);
}

/* FRAMES */

const ZERO = 48;

const encoder = new TextEncoder();

const decoder = new TextDecoder();

type Frame = { rows: number; cols: number; cells: Uint8Array; origin: number[] };

function cropped(cols: number, cells: Uint8Array, r0: number, r1: number, origin: number[]): Frame {
  if (r1 < 0) return { rows: 0, cols: 0, cells: new Uint8Array(0), origin };
  let c0 = cols;
  let c1 = -1;
  for (let r = r0; r <= r1; r++) {
    const o = r * cols;
    for (let c = 0; c < c0; c++)
      if (cells[o + c] !== ZERO) {
        c0 = c;
        break;
      }
    for (let c = cols - 1; c > c1; c--)
      if (cells[o + c] !== ZERO) {
        c1 = c;
        break;
      }
  }
  const h = r1 - r0 + 1;
  const w = c1 - c0 + 1;
  const out = new Uint8Array(h * w);
  for (let r = 0; r < h; r++) {
    const k = (r0 + r) * cols + c0;
    if (w >= 64) out.set(cells.subarray(k, k + w), r * w);
    else for (let c = 0; c < w; c++) out[r * w + c] = cells[k + c];
  }
  return { rows: h, cols: w, cells: out, origin: [origin[0] + r0, origin[1] + c0] };
}

/* WALLS */

function walls(state: State) {
  const lo = -Math.floor((state.cap - state.side) / 2);
  return { lo, hi: lo + state.cap - 1, reach: state.reach };
}

function hit(state: State, { rows, cols, origin }: Frame): boolean {
  if (!rows) return false;
  const { lo, hi, reach } = walls(state);
  const [r0, c0] = origin;
  return r0 - lo < reach || c0 - lo < reach || hi - (r0 + rows - 1) < reach || hi - (c0 + cols - 1) < reach;
}

function wallBy(state: State, { rows, cols, origin }: Frame, dr: number, dc: number): number {
  const { lo, hi, reach } = walls(state);
  const [r0, c0] = origin;
  let k = Number.POSITIVE_INFINITY;
  const room = (space: number, speed: number) => {
    if (speed > 0) k = Math.min(k, Math.floor(space / speed) + 1);
  };
  room(hi - reach - (r0 + rows - 1), dr);
  room(r0 - lo - reach, -dr);
  room(hi - reach - (c0 + cols - 1), dc);
  room(c0 - lo - reach, -dc);
  return k;
}

/* LAW */

function law(state: State): [Uint8Array, number] {
  const n = state.budget;
  const member = (counts: number[]) => {
    const on = new Uint8Array(n + 1);
    for (const k of counts) if (k <= n) on[k] = 1;
    return on;
  };
  const b = member(state.birth);
  const s = member(state.survive);
  const bg = state.background;
  const next = bg ? s[n] : b[0];
  const rule = new Uint8Array(2 * (n + 1));
  for (let k = 0; k <= n; k++) {
    rule[k] = ZERO + ((bg ? s[n - k] : b[k]) ^ next);
    rule[n + 1 + k] = ZERO + ((bg ? b[n - k] : s[k]) ^ next);
  }
  return [rule, next];
}

/* PLAN */

const SLIDE = 4;

type Run = [number, number, number];

type Line = { taps: number[]; runs: Run[]; low: number; high: number };

type Group = { across: Line; down: Line };

type Factor = { n: number; groups: Group[] };

type Term = { sign: number; chain: Factor[] };

type Kernel = { n: number; terms: Term[]; centre: number; most: number };

function line(taps: number[]): Line {
  const runs: Run[] = [];
  for (let i = 0; i < taps.length; ) {
    const step = taps[i + 1] - taps[i];
    let j = i + 1;
    while (j + 1 < taps.length && taps[j + 1] - taps[j] === step) j++;
    if (j - i + 1 >= SLIDE) {
      runs.push([taps[i], step, j - i + 1]);
      i = j + 1;
    } else runs.push([taps[i++], 1, 1]);
  }
  const low = Math.min(...runs.map(([a, step, count]) => (count > 1 ? a - step : a)));
  return { taps, runs, low, high: taps[taps.length - 1] };
}

const cost = ({ runs }: Line) => runs.reduce((sum, [, , count]) => sum + (count > 1 ? 3 : 1), 0);

const alone = ({ taps }: Line) => taps.length === 1 && !taps[0];

const priced = ({ across, down }: Group) =>
  down.taps.length === 1 ? cost(across) : alone(across) ? cost(down) : cost(across) + cost(down) + 1;

const sum = (groups: Group[]) => groups.reduce((total, group) => total + priced(group), 0);

function factor(mask: Uint8Array, n: number): Factor {
  const half = (n - 1) / 2;
  const grouped = (cross: boolean) => {
    const groups = new Map<string, { across: number[]; down: number[] }>();
    for (let i = 0; i < n; i++) {
      const line: number[] = [];
      for (let j = 0; j < n; j++) if (mask[cross ? j * n + i : i * n + j]) line.push(j - half);
      if (!line.length) continue;
      const key = line.join();
      const group = groups.get(key) ?? (cross ? { across: [], down: line } : { across: line, down: [] });
      (cross ? group.across : group.down).push(i - half);
      groups.set(key, group);
    }
    return [...groups.values()].map(({ across, down }) => ({ across: line(across), down: line(down) }));
  };
  const rows = grouped(false);
  const cols = grouped(true);
  return { n, groups: sum(cols) < sum(rows) ? cols : rows };
}

function split(mask: Uint8Array, n: number): Factor[] {
  for (let p = 3; p < n; p += 2) {
    if (n % p) continue;
    const q = n / p;
    const outer = new Uint8Array(p * p);
    let inner: Uint8Array | null = null;
    let kron = true;
    for (let at = 0; at < p * p && kron; at++) {
      const block = new Uint8Array(q * q);
      const top = Math.floor(at / p) * q;
      const left = (at % p) * q;
      for (let r = 0; r < q; r++) block.set(mask.subarray((top + r) * n + left, (top + r) * n + left + q), r * q);
      if (!block.includes(1)) continue;
      outer[at] = 1;
      if (!inner) inner = block;
      else kron = block.every((bit, i) => bit === inner![i]);
    }
    if (kron && inner) return [...split(outer, p), ...split(inner, q)];
  }
  return [factor(mask, n)];
}

const price = (terms: Term[]) =>
  terms.reduce((total, { chain }) => total + 1 + chain.reduce((part, { groups }) => part + sum(groups), 0), 0);

const weight = ({ groups }: Factor) =>
  groups.reduce((total, { across, down }) => total + across.taps.length * down.taps.length, 0);

function plan(mask: Uint8Array, n: number): Kernel {
  const centre = (n * n - 1) / 2;
  const popped = (grid: Uint8Array, bit: number) => grid.map((v, i) => (i === centre ? bit : v));
  const full = split(new Uint8Array(n * n).fill(1), n);
  const rest = mask.map((v) => 1 - v);
  const options: [Term[], number][] = [[[{ sign: 1, chain: [factor(mask, n)] }], 0]];
  for (const bit of [0, 1]) {
    options.push([[{ sign: 1, chain: split(popped(mask, bit), n) }], bit]);
    options.push([
      [
        { sign: 1, chain: full },
        { sign: -1, chain: split(popped(rest, bit), n) },
      ],
      1 - bit,
    ]);
  }
  const [terms, middle] = options.reduce((best, option) => (price(option[0]) < price(best[0]) ? option : best));
  const most = Math.max(...terms.map(({ chain }) => chain.reduce((total, f) => total * weight(f), 1)));
  return { n, terms, centre: middle, most };
}

const plans = new Map<string, Kernel>();

function kernel(mask: string, n: number): Kernel {
  let made = plans.get(mask);
  if (made?.n === n) return made;
  if (plans.size >= 64) plans.clear();
  made = plan(types(mask), n);
  plans.set(mask, made);
  return made;
}

/* COUNTS */

type Board = { h: number; w: number; wrap: boolean; lanes: number; bits: number; hb: number };

function board(h: number, w: number, wrap: boolean, most: number): Board {
  let lanes = most < 16 ? 8 : most < 256 ? 4 : most < 65536 ? 2 : 1;
  while (wrap && h % lanes) lanes /= 2;
  return { h, w, wrap, lanes, bits: 32 / lanes, hb: Math.ceil(h / lanes) };
}

function packed(cells: Uint8Array, rows: number, cols: number, pad: number, { w, lanes, bits, hb }: Board): Int32Array {
  const out = new Int32Array(hb * w);
  const ks = new Int32Array(lanes);
  for (let rb = 0; rb < hb; rb++) {
    const lo = Math.max(0, Math.ceil((pad - rb) / hb));
    const hi = Math.min(lanes - 1, Math.floor((rows - 1 + pad - rb) / hb));
    if (lo > hi) continue;
    for (let lane = lo; lane <= hi; lane++) ks[lane] = (lane * hb + rb - pad) * cols;
    const o = rb * w + pad;
    for (let c = 0; c < cols; c++) {
      let v = 0;
      for (let lane = lo; lane <= hi; lane++) v |= (cells[ks[lane] + c] & 1) << (lane * bits);
      out[o + c] = v;
    }
  }
  return out;
}

function lifted(
  src: Int32Array,
  { w, wrap, lanes, bits, hb }: Board,
  rb: number,
  d: number,
  strip: Int32Array,
): number {
  let m = Math.floor((rb + d) / hb);
  const k = (rb + d - m * hb) * w;
  if (wrap) m = ((m % lanes) + lanes) % lanes;
  else if (m >= lanes || m <= -lanes) return -1;
  if (!m) return k;
  if (m > 0) {
    const s = m * bits;
    if (wrap) {
      const u = 32 - s;
      for (let c = 0; c < w; c++) strip[c] = (src[k + c] >>> s) | (src[k + c] << u);
    } else for (let c = 0; c < w; c++) strip[c] = src[k + c] >>> s;
  } else {
    const s = -m * bits;
    for (let c = 0; c < w; c++) strip[c] = src[k + c] << s;
  }
  return -2;
}

const WIDE = 48;

function added(out: Int32Array, o: number, from: Int32Array, k: number, n: number) {
  if (n < WIDE) {
    for (let c = 0; c < n; c++) out[o + c] = (out[o + c] + from[k + c]) | 0;
    return;
  }
  const into = out.subarray(o, o + n);
  const by = from.subarray(k, k + n);
  for (let c = 0; c < n; c++) into[c] = (into[c] + by[c]) | 0;
}

function rolled(
  win: Int32Array,
  o: number,
  p: number,
  top: Int32Array,
  kh: number,
  bottom: Int32Array,
  kl: number,
  n: number,
) {
  if (n < WIDE) {
    for (let c = 0; c < n; c++) win[o + c] = (win[p + c] + top[kh + c] - bottom[kl + c]) | 0;
    return;
  }
  const into = win.subarray(o, o + n);
  const last = win.subarray(p, p + n);
  const plus = top.subarray(kh, kh + n);
  const minus = bottom.subarray(kl, kl + n);
  for (let c = 0; c < n; c++) into[c] = (last[c] + plus[c] - minus[c]) | 0;
}

function copied(into: Int32Array, o: number, from: Int32Array, k: number, n: number) {
  if (n < WIDE) for (let c = 0; c < n; c++) into[o + c] = from[k + c];
  else into.set(from.subarray(k, k + n), o);
}

function circled(strip: Int32Array, from: Int32Array, k0: number, start: number, w: number) {
  let k = start;
  for (let i = 0; i < strip.length; i++) {
    strip[i] = from[k0 + k];
    if (++k === w) k = 0;
  }
}

function slid(
  out: Int32Array,
  o: number,
  strip: Int32Array,
  win: Int32Array,
  at: number,
  e: number,
  count: number,
  w: number,
) {
  const first = Math.min(e, w);
  for (let c = 0; c < first; c++) {
    let total = 0;
    for (let t = 0; t < count; t++) total = (total + strip[at + c + t * e]) | 0;
    win[c] = total;
    out[o + c] = (out[o + c] + total) | 0;
  }
  const hi = at + (count - 1) * e;
  const lo = at - e;
  for (let c = first; c < w; c++) {
    const total = (win[c - e] + strip[hi + c] - strip[lo + c]) | 0;
    win[c] = total;
    out[o + c] = (out[o + c] + total) | 0;
  }
}

function across(out: Int32Array, src: Int32Array, b: Board, { runs, low, high }: Line, by: number, d: number) {
  const { w, wrap, hb } = b;
  const pad = Math.max(-low, high, 0) * by;
  const strip = new Int32Array(w + 2 * pad);
  const lane = new Int32Array(w);
  const win = new Int32Array(w);
  const start = ((-pad % w) + w) % w;
  const plain = !wrap && runs.every(([, , count]) => count === 1);
  for (let rb = 0; rb < hb; rb++) {
    const at = lifted(src, b, rb, d, lane);
    if (at === -1) continue;
    const from = at === -2 ? lane : src;
    const k0 = at === -2 ? 0 : at;
    const o = rb * w;
    if (plain) {
      for (const [a] of runs) {
        const dc = a * by;
        const lo = Math.max(0, -dc);
        added(out, o + lo, from, k0 + lo + dc, Math.min(w, w - dc) - lo);
      }
      continue;
    }
    if (wrap) circled(strip, from, k0, start, w);
    else copied(strip, pad, from, k0, w);
    for (const [a, step, count] of runs) {
      if (count === 1) added(out, o, strip, pad + a * by, w);
      else slid(out, o, strip, win, pad + a * by, step * by, count, w);
    }
  }
}

function down(out: Int32Array, src: Int32Array, b: Board, { runs }: Line, by: number) {
  const { w, hb } = b;
  const upper = new Int32Array(w);
  const lower = new Int32Array(w);
  const zero = new Int32Array(w);
  const pick = (at: number, line: Int32Array) => (at === -2 ? line : at === -1 ? zero : src);
  const spot = (at: number) => Math.max(at, 0);
  const direct = runs.filter(([, , count]) => count === 1);
  for (let rb = 0; rb < hb && direct.length; rb++) {
    for (const [t] of direct) {
      const at = lifted(src, b, rb, t * by, upper);
      if (at !== -1) added(out, rb * w, pick(at, upper), spot(at), w);
    }
  }
  for (const [a, step, count] of runs) {
    if (count === 1) continue;
    const e = step * by;
    const win = new Int32Array(hb * w);
    for (let rb = 0; rb < hb; rb++) {
      const o = rb * w;
      if (rb < e) {
        for (let t = 0; t < count; t++) {
          const at = lifted(src, b, rb, (a + t * step) * by, upper);
          if (at !== -1) added(win, o, pick(at, upper), spot(at), w);
        }
      } else {
        const hi = lifted(src, b, rb, (a + (count - 1) * step) * by, upper);
        const lo = lifted(src, b, rb, (a - step) * by, lower);
        rolled(win, o, o - e * w, pick(hi, upper), spot(hi), pick(lo, lower), spot(lo), w);
      }
      added(out, o, win, o, w);
    }
  }
}

function convolved(src: Int32Array, b: Board, { groups }: Factor, by: number): Int32Array {
  const out = new Int32Array(src.length);
  for (const { across: a, down: d } of groups) {
    if (d.taps.length === 1) across(out, src, b, a, by, d.taps[0] * by);
    else if (alone(a)) down(out, src, b, d, by);
    else {
      const tmp = new Int32Array(src.length);
      across(tmp, src, b, a, by, 0);
      down(out, tmp, b, d, by);
    }
  }
  return out;
}

function counted(base: Int32Array, b: Board, terms: Term[]): Int32Array {
  let counts: Int32Array | null = null;
  for (const { sign, chain } of terms) {
    let acc = base;
    let by = 1;
    for (let i = chain.length - 1; i >= 0; i--) {
      acc = convolved(acc, b, chain[i], by);
      by *= chain[i].n;
    }
    if (!counts) counts = sign === 1 ? acc : acc.map((v) => -v);
    else for (let i = 0; i < counts.length; i++) counts[i] = (counts[i] + sign * acc[i]) | 0;
  }
  return counts ?? new Int32Array(base.length);
}

type Next = { cells: Uint8Array; pop: number; r0: number; r1: number; background: number };

function applied(counts: Int32Array, base: Int32Array, { w, lanes, bits, hb }: Board, rule: Uint8Array, live: number) {
  const out = new Uint8Array(lanes * hb * w);
  const mask = lanes === 1 ? -1 : (1 << bits) - 1;
  const stride = hb * w;
  let total = 0;
  let r0 = -1;
  let r1 = -1;
  for (let rb = 0; rb < hb; rb++) {
    const o = rb * w;
    let any = 0;
    for (let c = o; c < o + w; c++) {
      let n = counts[c];
      let x = base[c];
      for (let at = c; at < out.length; at += stride) {
        const v = rule[(n & mask) + (x & 1) * live];
        out[at] = v;
        total += v;
        any |= v;
        n >>= bits;
        x >>= bits;
      }
    }
    if (!(any & 1)) continue;
    for (let lane = 0; lane < lanes; lane++) {
      const r = lane * hb + rb;
      const k = r * w;
      let alive = false;
      for (let c = 0; c < w && !alive; c++) alive = out[k + c] !== ZERO;
      if (!alive) continue;
      if (r0 < 0 || r < r0) r0 = r;
      if (r > r1) r1 = r;
    }
  }
  return { cells: out, pop: total - out.length * ZERO, r0, r1 };
}

function evolved(state: State, cells: Uint8Array, rows: number, cols: number, pad: number, wrap: boolean): Next {
  const { terms, centre, most } = kernel(state.mask, state.span[0]);
  const b = board(rows + 2 * pad, cols + 2 * pad, wrap, most);
  const base = packed(cells, rows, cols, pad, b);
  const counts = counted(base, b, terms);
  const [rule, background] = law(state);
  return { ...applied(counts, base, b, rule, state.budget + 1 - centre), background };
}

const at = (kind: string, generation: number, period: number): Stop => ({
  kind,
  generation,
  period,
  dr: 0,
  dc: 0,
  wall_by: 0,
});

/* RULES */

export function boot(config: Config): State {
  const run = new life.recipe.Run(config);
  try {
    const frame = run.frame();
    const [rows, cols] = frame.shape;
    const origin = Array.from(run.origin(), Number);
    const mask = run.mask();
    const stop = run.stop();
    return {
      mode: config.mode,
      x: config.x,
      cap: run.cap(),
      side: run.side(),
      reach: run.reach(),
      budget: run.budget(),
      birth: Array.from(run.birth()),
      survive: Array.from(run.survive()),
      span: Array.from(mask.shape),
      mask: bits(mask.data),
      rows,
      cols,
      cells: bits(frame.types),
      origin,
      background: run.background(),
      gen: 0,
      pop: population(frame.types),
      seen: { [hashed(rows, cols, Uint8Array.from(frame.types), 0)]: [0, ...origin] },
      stop: stop ? at(stop.kind, stop.generation, stop.period) : null,
    };
  } finally {
    run.free();
  }
}

function fate(state: State, frame: Frame, pop: number, background: number, gen: number): [Stop | null, State["seen"]] {
  const island = state.mode === "Island";
  if (!pop) return [at("dead", gen, 1), state.seen];
  if (island && hit(state, frame)) return [at("wall", gen, 0), state.seen];
  const id = hashed(frame.rows, frame.cols, frame.cells, background);
  const seen = state.seen[id];
  if (!seen) return [null, { ...state.seen, [id]: [gen, ...frame.origin] }];
  const period = gen - seen[0];
  const dr = frame.origin[0] - seen[1];
  const dc = frame.origin[1] - seen[2];
  if (!island || (!dr && !dc)) return [at(period === 1 ? "still" : "loop", gen, period), state.seen];
  const by = gen + wallBy(state, frame, dr, dc) * period;
  const wall_by = by <= state.x ? by : 0;
  return [{ kind: "mover", generation: gen, period, dr, dc, wall_by }, state.seen];
}

function settled(state: State, frame: Frame, pop: number, background: number): State {
  const gen = state.gen + 1;
  const [stop, seen] = fate(state, frame, pop, background, gen);
  const { rows, cols, origin } = frame;
  return { ...state, rows, cols, cells: decoder.decode(frame.cells), origin, background, gen, pop, seen, stop };
}

function tick(state: State): State {
  if (state.stop) return state;
  if (state.gen >= state.x) return { ...state, stop: at("limit", state.gen, 0) };
  const { rows, cols, origin } = state;
  const cells = encoder.encode(state.cells);
  if (state.mode === "Island") {
    const reach = state.reach;
    const next = evolved(state, cells, rows, cols, reach, false);
    const frame = cropped(cols + 2 * reach, next.cells, next.r0, next.r1, [origin[0] - reach, origin[1] - reach]);
    return settled(state, frame, next.pop, next.background);
  }
  const next = evolved(state, cells, rows, cols, 0, true);
  return settled(state, { rows, cols, cells: next.cells, origin }, next.pop, next.background);
}

export function step(state: State, act: Act): State {
  if (act.verb === "tick") return tick(state);
  return state;
}

export const moves = (state: State): Act[] => (state.stop ? [] : [TICK]);

export function read(state: State, path = ""): unknown {
  let at: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (at === null || typeof at !== "object") return undefined;
    at = (at as Record<string, unknown>)[key];
  }
  return at;
}
