import type { Mrly } from "@mrlyprod/ui/kit/designs/engine.ts";
import { title, total } from "@mrlyprod/ui/kit/designs/engine.ts";
import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix, rng, xorshift } from "@mrlyprod/ui/kit/rand.ts";

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
  side: number;
  boards: string[];
  homes: number[];
  goal: number;
  tick: number;
  at: number[][];
  sums: number[];
  best: number[];
  since: number[];
  over: Over | null;
  seed: number;
};

export type Config = Pick<State, "side" | "boards" | "homes" | "goal" | "seed"> & { walkers: number };

export type Table = { seed?: number; units: Mrly };

export type Act = { verb: "tick" } | { verb: "aim"; args: { goal: number } };

export type Trace = { trail: Float32Array[]; stamp: Int32Array[]; log: [number, number][] };

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
const ON = 49;
export const TICK: Act = { verb: "tick" };
export const aiming = (goal: number): Act => ({ verb: "aim", args: { goal } });
const ceilings = new Map<string, number>();
const masses = new Map<string, number[]>();

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

export function grow(mrly: Mrly, value: Size, code: string) {
  const cell = mrly.two.create(code, value.number, value.level, 0, value.base);
  const side = cell.shape[0];
  const filled = Uint8Array.from(cell.types, (type) => (type ? 1 : 0));
  const at = home(filled, side);
  return {
    code,
    side,
    filled,
    home: at,
    lit: Number(mrly.sequences.counts.fill(code, value.number, DIM, value.level, value.base)),
    ...room(filled, side, at),
  };
}

export function fills(mrly: Mrly, value: Pick<Size, "base" | "number">, code: string) {
  return Number(mrly.sequences.counts.fill(code, value.number, DIM, 1, value.base));
}

export function ceiling(mrly: Mrly, value: Size, code: string) {
  const key = `${value.base}:${value.number}:${value.level}:${code}`;
  if (!ceilings.has(key)) ceilings.set(key, grow(mrly, value, code).rms);
  return ceilings.get(key)!;
}

export function kin(mrly: Mrly, value: Pick<Size, "base" | "number">, code: string, count: number) {
  const key = `${value.base}:${value.number}`;
  if (!masses.has(key))
    masses.set(
      key,
      Array.from({ length: count }, (_, c) => fills(mrly, value, String(c))),
    );
  const list = masses.get(key)!;
  return list.flatMap((mass, c) =>
    c > 0 && c < count - 1 && mass === list[Number(code)] && String(c) !== code ? [String(c)] : [],
  );
}

export function board(mrly: Mrly, value: Size, code: string) {
  const { base, number } = value;
  const grown = grow(mrly, value, code);
  return {
    ...grown,
    name: mrly.math.name.of(code, DIM, base) ?? "",
    title: title(mrly, DIM, base, code),
    fills: fills(mrly, value, code),
    of: number * number,
    dimension: mrly.sequences.counts.dimension(code, number, DIM, base),
  };
}

export function pair(mrly: Mrly, value: Field, rand: Rng, typed: Typed = [null, null]): Pair {
  const count = total(mrly, DIM, value.base);
  const far = value.finish;
  const able = (code: string) => ceiling(mrly, value, code) >= far;
  const any = (not?: string) => {
    let code: string | null = null;
    for (let i = 0; i < ROLLS; i++) {
      code = roll(rand, count);
      if (code !== not && able(code)) return code;
    }
    return code!;
  };
  const mate = (code: string) => {
    const list = kin(mrly, value, code, count);
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

export function plan(mrly: Mrly, value: Race, rand: Rng) {
  const count = total(mrly, DIM, value.base);
  const codes = pair(mrly, value, rand, [reads(value.a, count), reads(value.b, count)]);
  const sides = codes.map((code) => board(mrly, value, code));
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

export const bits = (filled: ArrayLike<number>) => Array.from(filled, (one) => (one ? "1" : "0")).join("");

export function configOf(plan: Plan, seed: number): Config {
  return {
    side: plan.side,
    boards: plan.sides.map(({ filled }) => bits(filled)),
    homes: plan.sides.map(({ home }) => home),
    walkers: plan.walkers,
    goal: plan.goal,
    seed,
  };
}

export function heat(value: number, stamp: number, tick: number, keep: number) {
  return keep === 1 || tick === stamp || value === 0 ? value : value * keep ** (tick - stamp);
}

export function spread(at: readonly number[], home: number, side: number) {
  const hr = Math.floor(home / side);
  const hc = home % side;
  let total = 0;
  for (const i of at) total += (Math.floor(i / side) - hr) ** 2 + ((i % side) - hc) ** 2;
  return total;
}

const far = (sum: number, count: number) => (count ? Math.sqrt(sum / count) : 0);

export const reach = ({ sums, at }: State): [number, number] => [
  far(sums[0], at[0].length),
  far(sums[1], at[1].length),
];

const verdict = ([a, b]: number[]) => (a > b ? 0 : b > a ? 1 : -1);

function reached(state: State): Over | null {
  const now = reach(state);
  const hit = now.map((r) => r >= state.goal);
  return hit[0] || hit[1]
    ? { tick: state.tick, winner: hit[0] && hit[1] ? verdict(now) : hit[0] ? 0 : 1, how: "goal" }
    : null;
}

export function boot({ side, boards, homes, walkers, goal, seed }: Config): State {
  return {
    side,
    boards,
    homes,
    goal,
    tick: 0,
    at: homes.map((home) => Array.from({ length: home < 0 ? 0 : walkers }, () => home)),
    sums: [0, 0],
    best: [0, 0],
    since: [0, 0],
    over: null,
    seed: mix(seed),
  };
}

function tick(state: State): State {
  if (state.over) return state;
  const tick = state.tick + 1;
  const { side, boards, homes } = state;
  let seed = state.seed;
  const sums = state.sums.slice();
  const best = state.best.slice();
  const since = state.since.slice();
  const at = state.at.map((from, s) => {
    const board = boards[s];
    const to = from.slice();
    for (let w = 0; w < to.length; w++) {
      seed = xorshift(seed);
      const i = to[w];
      const [dr, dc] = MOVES[seed >>> 30];
      const nr = Math.floor(i / side) + dr;
      const nc = (i % side) + dc;
      const j = nr * side + nc;
      if (nr < 0 || nc < 0 || nr >= side || nc >= side || board.charCodeAt(j) !== ON) continue;
      to[w] = j;
    }
    sums[s] = spread(to, homes[s], side);
    if (far(sums[s], to.length) > far(best[s], to.length) + GAIN) {
      best[s] = sums[s];
      since[s] = tick;
    }
    return to;
  });
  const next: State = { ...state, tick, at, sums, best, since, seed };
  const goal = reached(next);
  if (goal) return { ...next, over: goal };
  if (tick - Math.max(since[0], since[1]) >= STALL)
    return { ...next, over: { tick, winner: verdict(reach(next)), how: "stall" } };
  if (tick >= LIMIT) return { ...next, over: { tick, winner: verdict(reach(next)), how: "limit" } };
  return next;
}

function aim(state: State, goal: number): State {
  const next = { ...state, goal };
  if (!state.over) return { ...next, over: reached(next) };
  if (state.over.how === "goal" && !reached(next)) return { ...next, over: null };
  return next;
}

export function step(state: State, act: Act): State {
  if (act.verb === "tick") return tick(state);
  if (act.verb === "aim") return aim(state, act.args.goal);
  return state;
}

export const moves = (state: State): Act[] => (state.over ? [] : [TICK]);

export function read(state: State, path = ""): unknown {
  let at: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (at === null || typeof at !== "object") return undefined;
    at = (at as Record<string, unknown>)[key];
  }
  return at;
}

export function planned(mrly: Mrly, value: Race, seed: number | undefined) {
  const { code, walk } = roots(rng(seed));
  return { plan: plan(mrly, value, code), walk };
}

export const opening = (plan: Plan, walk: number, k = 0): State =>
  boot({ ...configOf(plan, walk), goal: plan.goal, seed: (walk + k) >>> 0 });

export function deal(value: Race, { seed, units }: Table): State {
  const { plan, walk } = planned(units, value, seed);
  return opening(plan, walk);
}

export function series(plan: Plan, walk: number, wait = HOLD) {
  const hold = Number.isFinite(wait) ? Math.ceil((wait * plan.speed) / 1000) : Infinity;
  const cells = plan.side * plan.side;
  const open = () => opening(plan, walk, k);
  const blank = (): Trace => ({
    trail: plan.sides.map(() => new Float32Array(cells)),
    stamp: plan.sides.map(() => new Int32Array(cells)),
    log: [[0, 0]],
  });
  let k = 0;
  let start = 0;
  let version = 0;
  let state = open();
  let trace = blank();
  const advance = () => {
    const was = state;
    state = step(state, TICK);
    const { tick } = state;
    state.at.forEach((at, s) => {
      const trail = trace.trail[s];
      const stamp = trace.stamp[s];
      const from = was.at[s];
      for (let w = 0; w < at.length; w++) {
        const j = at[w];
        if (j === from[w]) continue;
        trail[j] = heat(trail[j], stamp[j], tick, plan.keep) + 1;
        stamp[j] = tick;
      }
    });
    trace.log.push(reach(state));
    version++;
  };
  const next = () => {
    start += state.over!.tick + hold;
    k++;
    state = open();
    trace = blank();
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
      advance();
    }
  };
  const finish = (limit = Infinity) => {
    while (!state.over && state.tick < limit) advance();
  };
  const aim = (goal: number) => {
    state = step(state, aiming(goal));
  };
  return {
    to,
    finish,
    aim,
    get state() {
      return state;
    },
    get trace() {
      return trace;
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

export function bake(trace: Trace, tick: number, keep: number) {
  trace.trail.forEach((trail, s) => {
    const stamp = trace.stamp[s];
    for (let i = 0; i < trail.length; i++) {
      trail[i] = heat(trail[i], stamp[i], tick, keep);
      stamp[i] = tick;
    }
  });
  return trace;
}

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
