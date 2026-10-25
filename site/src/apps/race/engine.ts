import type { Mrly } from "../../lib/designs/engine.ts";
import { title, total } from "../../lib/designs/engine.ts";
import type { Rng } from "../../lib/rand.ts";
import { rng } from "../../lib/rand.ts";

/* TYPES */

export type Size = { base: number; number: number; level: number };

export type Field = Size & { finish: number };

export type Race = Field & { a: unknown; b: unknown; walkers: number; speed: number; heat: number };

export type Typed = [string | null, string | null];

export type Pair = [string, string];

export type Room = { cells: number; rms: number };

export type Side = ReturnType<typeof board>;

export type Plan = ReturnType<typeof plan>;

export type How = "goal" | "stall" | "limit";

export type Over = { tick: number; winner: number; how: How };

export type State = {
  tick: number;
  at: Int32Array[];
  trail: Float32Array[];
  stamp: Int32Array[];
  reach: number[];
  best: number[];
  since: number[];
  over: Over | null;
  log: [number, number][];
};

export type Series = ReturnType<typeof series>;

export type Laid = ReturnType<typeof lay>;

export const DIM = 2;
export const NUMBERS = [3, 5, 7];
export const BUDGET = 131072;
export const STILL = 1500000;
export const SHARE = 55;
export const REACH = 28;
export const HOT = 12;
export const HOLD = 3000;
export const STALL = 1000;
export const LIMIT = 60000;
export const ROLLS = 48;
export const TURN = 180;
export const GAIN = 0.25;
const MOVES = [
  [1, 0],
  [-1, 0],
  [0, 1],
  [0, -1],
];
const WIDE = 4294967296;
const ceilings = new Map<string, number>();
const masses = new Map<string, number[]>();

/* ROOM */

export function cap(number: number) {
  let level = 1;
  while (number ** (DIM * (level + 1)) <= BUDGET) level++;
  return level;
}

export function top(side: number) {
  return Math.max(1, Math.min(REACH, Math.round((((side - 1) / 2) * SHARE) / 100)));
}

export const keeping = (fade: number, speed: number) => (fade ? (1 - fade / 100) ** (1 / speed) : 1);

export const budget = (walkers: number) => Math.max(1, Math.floor(STILL / (2 * walkers)));

/* CODES */

export function reads(text: unknown, count: number) {
  const s = String(text ?? "").trim();
  if (!s) return null;
  const n = Number(s);
  if (!Number.isFinite(n)) return null;
  const whole = Math.floor(n);
  return String(((whole % count) + count) % count);
}

export function roll(rand: Rng, count: number) {
  return String(1 + Math.floor(rand() * (count - 2)));
}

export function roots(rand: Rng) {
  const code = rng(Math.floor(rand() * WIDE));
  const walk = Math.floor(rand() * WIDE);
  return { code, walk };
}

/* BOARD */

export function home(filled: ArrayLike<number>, side: number) {
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

export function room(filled: ArrayLike<number>, side: number, from: number): Room {
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

export function grow(math: Mrly, value: Size, code: string) {
  const cell = math.two.create(code, value.number, value.level, 0, value.base);
  const side = cell.shape[0];
  const filled = Uint8Array.from(cell.types, (type) => (type ? 1 : 0));
  const at = home(filled, side);
  return {
    code,
    side,
    filled,
    home: at,
    lit: Number(math.counts.fill(code, value.number, DIM, value.level, value.base)),
    ...room(filled, side, at),
  };
}

export function fills(math: Mrly, value: Pick<Size, "base" | "number">, code: string) {
  return Number(math.counts.fill(code, value.number, DIM, 1, value.base));
}

export function ceiling(math: Mrly, value: Size, code: string) {
  const key = `${value.base}:${value.number}:${value.level}:${code}`;
  if (!ceilings.has(key)) ceilings.set(key, grow(math, value, code).rms);
  return ceilings.get(key)!;
}

export function kin(math: Mrly, value: Pick<Size, "base" | "number">, code: string, count: number) {
  const key = `${value.base}:${value.number}`;
  if (!masses.has(key))
    masses.set(
      key,
      Array.from({ length: count }, (_, c) => fills(math, value, String(c))),
    );
  const list = masses.get(key)!;
  return list.flatMap((mass, c) =>
    c > 0 && c < count - 1 && mass === list[Number(code)] && String(c) !== code ? [String(c)] : [],
  );
}

export function board(math: Mrly, value: Size, code: string) {
  const { base, number } = value;
  const grown = grow(math, value, code);
  return {
    ...grown,
    name: math.name.of(code, DIM, base) ?? "",
    title: title(math, DIM, base, code),
    fills: fills(math, value, code),
    of: number * number,
    dimension: math.counts.dimension(code, number, DIM, base),
  };
}

/* PAIR */

export function pair(math: Mrly, value: Field, rand: Rng, typed: Typed = [null, null]): Pair {
  const count = total(math, DIM, value.base);
  const far = value.finish;
  const able = (code: string) => ceiling(math, value, code) >= far;
  const any = (not?: string) => {
    let code: string | null = null;
    for (let i = 0; i < ROLLS; i++) {
      code = roll(rand, count);
      if (code !== not && able(code)) return code;
    }
    return code!;
  };
  const mate = (code: string) => {
    const list = kin(math, value, code, count);
    for (let i = 0; i < Math.min(list.length, ROLLS); i++) {
      const j = i + Math.floor(rand() * (list.length - i));
      [list[i], list[j]] = [list[j], list[i]];
      if (able(list[i])) return list[i];
    }
    return null;
  };
  const both = (): Pair | null => {
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

export function plan(math: Mrly, value: Race, rand: Rng) {
  const count = total(math, DIM, value.base);
  const codes = pair(math, value, rand, [reads(value.a, count), reads(value.b, count)]);
  const sides = codes.map((code) => board(math, value, code));
  return {
    sides,
    side: sides[0].side,
    goal: value.finish,
    walkers: value.walkers,
    speed: value.speed,
    still: budget(value.walkers),
    keep: keeping(value.heat, value.speed),
  };
}

export function heat(value: number, stamp: number, tick: number, keep: number) {
  return keep === 1 || tick === stamp || value === 0 ? value : value * keep ** (tick - stamp);
}

export function spread(at: Int32Array, home: number, side: number) {
  if (!at.length) return 0;
  const hr = Math.floor(home / side);
  const hc = home % side;
  let total = 0;
  for (const i of at) total += (Math.floor(i / side) - hr) ** 2 + ((i % side) - hc) ** 2;
  return Math.sqrt(total / at.length);
}

export function open(plan: Plan): State {
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

const verdict = ([a, b]: number[]) => (a > b ? 0 : b > a ? 1 : -1);

function reached(state: State, plan: Pick<Plan, "goal">): Over | null {
  const hit = state.reach.map((r) => r >= plan.goal);
  return hit[0] || hit[1]
    ? { tick: state.tick, winner: hit[0] && hit[1] ? verdict(state.reach) : hit[0] ? 0 : 1, how: "goal" }
    : null;
}

export function step(state: State, plan: Plan, rand: Rng) {
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
      const j = nr * side + nc;
      if (nr < 0 || nc < 0 || nr >= side || nc >= side || !filled[j]) continue;
      at[w] = j;
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
  state.over = reached(state, plan);
  if (state.over) return state;
  if (tick - Math.max(state.since[0], state.since[1]) >= STALL)
    state.over = { tick, winner: verdict(state.reach), how: "stall" };
  else if (tick >= LIMIT) state.over = { tick, winner: verdict(state.reach), how: "limit" };
  return state;
}

export function aim(state: State, plan: Pick<Plan, "goal">) {
  if (!state.over) state.over = reached(state, plan);
  else if (state.over.how === "goal" && !reached(state, plan)) state.over = null;
  return state;
}

export function bake(state: State, keep: number) {
  state.trail.forEach((trail, s) => {
    const stamp = state.stamp[s];
    for (let i = 0; i < trail.length; i++) {
      trail[i] = heat(trail[i], stamp[i], state.tick, keep);
      stamp[i] = state.tick;
    }
  });
  return state;
}

/* SERIES */

export function series(plan: Plan, walk: number, wait = HOLD) {
  const hold = Number.isFinite(wait) ? Math.ceil((wait * plan.speed) / 1000) : Infinity;
  let k = 0;
  let start = 0;
  let version = 0;
  let state = open(plan);
  let rand = rng((walk + k) >>> 0);
  const next = () => {
    start += state.over!.tick + hold;
    k++;
    state = open(plan);
    rand = rng((walk + k) >>> 0);
    version++;
  };
  const to = (at: number) => {
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
    }
  };
  const finish = (limit = Infinity) => {
    if (state.over) return;
    while (!state.over && state.tick < limit) {
      step(state, plan, rand);
      version++;
    }
  };
  return {
    to,
    finish,
    get state() {
      return state;
    },
    get k() {
      return k;
    },
    get start() {
      return start;
    },
    get version() {
      return version;
    },
  };
}

/* LAYOUT */

export function lay(w: number, h: number, n: number, gap: number, pad: number) {
  const across = Math.min((w - 2 * pad - gap) / (2 * n), (h - 2 * pad) / n);
  const down = Math.min((w - 2 * pad) / n, (h - 2 * pad - gap) / (2 * n));
  const flat = across >= down;
  const raw = Math.max(flat ? across : down, 1e-3);
  const cell = raw >= 1 ? Math.floor(raw) : raw;
  const edge = cell * n;
  const x0 = flat ? (w - 2 * edge - gap) / 2 : (w - edge) / 2;
  const y0 = flat ? (h - edge) / 2 : (h - 2 * edge - gap) / 2;
  const boards = [[x0, y0], flat ? [x0 + edge + gap, y0] : [x0, y0 + edge + gap]].map(([x, y]) => ({
    x: Math.round(x),
    y: Math.round(y),
  }));
  return { cell, edge, flat, boards };
}
