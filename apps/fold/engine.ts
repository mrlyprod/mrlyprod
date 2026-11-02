import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Over = "done" | "stuck" | null;

export type State = {
  size: number;
  target: string;
  holes: string;
  folds: number[];
  moves: number;
  par: number;
  over: Over;
  seed: number;
};

export type Config = Pick<State, "size" | "seed"> & { folds: number };

export type Table = { seed?: number; round?: number };

export type Act =
  | { verb: "fold"; args: { axis: number; line: number; from: number } }
  | { verb: "punch"; args: { x: number; y: number } };

export type Line = { at: Int32Array; lo: number; hi: number };

export type Sheet = { xs: Line; ys: Line };

export const X = 0;
export const Y = 1;
export const LOW = -1;
export const HIGH = 1;
export const AXES = [X, Y];
export const SIDES = [LOW, HIGH];
export const SMALL = 4;
export const LARGE = 9;
export const DEEP = 6;

export const fold = (axis: number, line: number, from: number): Act => ({ verb: "fold", args: { axis, line, from } });
export const punch = (x: number, y: number): Act => ({ verb: "punch", args: { x, y } });

const PRESENT = 1;
const BAD = 2;
const NEED = 4;
const NARROW = 2;
const TRIES = 64;
const SHARE = 0.35;
const HALF = 3;

/* CODES */

export const code = (axis: number, line: number, from: number) => line * 4 + axis * 2 + (from === HIGH ? 1 : 0);

export function decode(one: number): { axis: number; line: number; from: number } {
  return { axis: (one >> 1) & 1, line: one >> 2, from: one & 1 ? HIGH : LOW };
}

/* LINES */

export const flat = (size: number): Line => ({ at: Int32Array.from({ length: size }, (_, i) => i), lo: 0, hi: size });

export function fits(line: Line, k: number, from: number): boolean {
  if (!Number.isInteger(k) || k <= line.lo || k >= line.hi || !SIDES.includes(from)) return false;
  const flap = from === LOW ? k - line.lo : line.hi - k;
  return flap <= line.hi - line.lo - flap;
}

export const flip = (k: number, p: number) => 2 * k - 1 - p;

export const flapped = (k: number, from: number, p: number) => (from === LOW ? p < k : p >= k);

export function folded(line: Line, k: number, from: number): Line {
  const at = line.at.map((p) => (flapped(k, from, p) ? flip(k, p) : p));
  return from === LOW ? { at, lo: k, hi: line.hi } : { at, lo: line.lo, hi: k };
}

export function sheet(state: Pick<State, "size" | "folds">, count = state.folds.length): Sheet {
  const out = [flat(state.size), flat(state.size)];
  for (const one of state.folds.slice(0, count)) {
    const { axis, line, from } = decode(one);
    out[axis] = folded(out[axis], line, from);
  }
  return { xs: out[X], ys: out[Y] };
}

/* STACKS */

export function flags(state: Pick<State, "size" | "target" | "holes">, { xs, ys }: Sheet): Uint8Array {
  const { size, target, holes } = state;
  const out = new Uint8Array(size * size);
  for (let y = 0; y < size; y++)
    for (let x = 0; x < size; x++) {
      const cell = y * size + x;
      const p = ys.at[y] * size + xs.at[x];
      out[p] |= PRESENT | (target[cell] === "1" ? (holes[cell] === "1" ? 0 : NEED) : BAD);
    }
  return out;
}

export const needed = (one: number) => (one & (NEED | BAD)) === NEED;

export const broken = (one: number) => (one & (NEED | BAD)) === (NEED | BAD);

export const valid = (one: number) => (one & (PRESENT | BAD)) === PRESENT;

export function stack(state: Pick<State, "size">, { xs, ys }: Sheet, x: number, y: number): number[] {
  const out: number[] = [];
  for (let b = 0; b < state.size; b++)
    for (let a = 0; a < state.size; a++) if (xs.at[a] === x && ys.at[b] === y) out.push(b * state.size + a);
  return out;
}

/* PLAN */

type Node = { line: Line; next: [number, number][] };

function graph(start: Line, axis: number): Node[] {
  const out: Node[] = [{ line: start, next: [] }];
  const seen = new Map([[start.at.join(), 0]]);
  for (let i = 0; i < out.length; i++) {
    const { line } = out[i];
    for (let k = line.lo + 1; k < line.hi; k++)
      for (const from of SIDES) {
        if (!fits(line, k, from)) continue;
        const to = folded(line, k, from);
        const name = to.at.join();
        let j = seen.get(name);
        if (j === undefined) {
          j = out.length;
          seen.set(name, j);
          out.push({ line: to, next: [] });
        }
        out[i].next.push([code(axis, k, from), j]);
      }
  }
  return out;
}

export function plan(state: State): Act[] {
  if (state.over) return [];
  const { size } = state;
  const now = sheet(state);
  const gx = graph(now.xs, X);
  const gy = graph(now.ys, Y);
  const ny = gy.length;
  const cost = new Int32Array(gx.length * ny).fill(-1);
  const pick = new Int32Array(gx.length * ny);
  const kept: Uint8Array[] = [];
  const owed = new Int32Array(gx.length * ny).fill(-1);
  const look = (ix: number, iy: number) => {
    const id = ix * ny + iy;
    if (!kept[id]) {
      kept[id] = flags(state, { xs: gx[ix].line, ys: gy[iy].line });
      owed[id] = kept[id].reduce((n, one) => n + (needed(one) ? 1 : 0), 0);
    }
    return kept[id];
  };
  const lost = (here: Uint8Array, there: Uint8Array, one: number) => {
    const { axis, line, from } = decode(one);
    let n = 0;
    for (let p = 0; p < here.length; p++) {
      if (!needed(here[p])) continue;
      let x = p % size;
      let y = Math.floor(p / size);
      if (axis === X && flapped(line, from, x)) x = flip(line, x);
      if (axis === Y && flapped(line, from, y)) y = flip(line, y);
      if (!valid(there[y * size + x])) n++;
    }
    return n;
  };
  const solve = (ix: number, iy: number): number => {
    const id = ix * ny + iy;
    if (cost[id] >= 0) return cost[id];
    const here = look(ix, iy);
    let best = owed[id];
    let choice = -1;
    const xn = gx[ix].next.length;
    const edges = xn + gy[iy].next.length;
    for (let e = 0; e < edges && best > 1; e++) {
      const [one, jx, jy] =
        e < xn ? [gx[ix].next[e][0], gx[ix].next[e][1], iy] : [gy[iy].next[e - xn][0], ix, gy[iy].next[e - xn][1]];
      const there = look(jx, jy);
      const paid = 1 + lost(here, there, one);
      if (paid + Math.min(1, owed[jx * ny + jy]) > best) continue;
      const value = paid + solve(jx, jy);
      if (value <= best) {
        best = value;
        choice = e;
      }
    }
    cost[id] = best;
    pick[id] = choice;
    return best;
  };
  if (look(0, 0).some(broken)) return [];
  solve(0, 0);
  const out: Act[] = [];
  let ix = 0;
  let iy = 0;
  for (;;) {
    const here = look(ix, iy);
    const choice = pick[ix * ny + iy];
    if (choice < 0) {
      for (let p = 0; p < here.length; p++) if (needed(here[p])) out.push(punch(p % size, Math.floor(p / size)));
      return out;
    }
    const xn = gx[ix].next.length;
    const [one, jx, jy] =
      choice < xn
        ? [gx[ix].next[choice][0], gx[ix].next[choice][1], iy]
        : [gy[iy].next[choice - xn][0], ix, gy[iy].next[choice - xn][1]];
    const there = look(jx, jy);
    const { axis, line, from } = decode(one);
    for (let p = 0; p < here.length; p++) {
      if (!needed(here[p])) continue;
      let x = p % size;
      let y = Math.floor(p / size);
      const at = [x, y];
      if (axis === X && flapped(line, from, x)) x = flip(line, x);
      if (axis === Y && flapped(line, from, y)) y = flip(line, y);
      if (!valid(there[y * size + x])) out.push(punch(at[0], at[1]));
    }
    out.push(fold(axis, line, from));
    ix = jx;
    iy = jy;
  }
}

export const par = (state: State) => plan(state).length;

/* RULES */

function striped(holes: string, size: number): boolean {
  for (let i = 0; i < size; i++) {
    let row = true;
    let column = true;
    for (let j = 0; j < size; j++) {
      row &&= holes[i * size + j] === "1";
      column &&= holes[j * size + i] === "1";
    }
    if (row || column) return true;
  }
  return false;
}

export function boot({ size, folds, seed }: Config): State {
  if (!Number.isInteger(size) || size < SMALL || size > LARGE)
    throw new Error(`fold: the sheet is not ${SMALL} to ${LARGE} wide`);
  let next = mix(seed);
  const roll = (n: number) => {
    next = xorshift(next);
    return next % n;
  };
  const cells = size * size;
  let holes = "0".repeat(cells);
  for (let t = 0; t < TRIES; t++) {
    const made = new Uint8Array(cells);
    const chain: number[] = [];
    for (let stage = 0; stage <= folds; stage++) {
      const now = sheet({ size, folds: chain });
      const spots: [number, number][] = [];
      for (let y = now.ys.lo; y < now.ys.hi; y++) for (let x = now.xs.lo; x < now.xs.hi; x++) spots.push([x, y]);
      const last = stage === folds;
      const many = last ? Math.max(1, Math.round(spots.length * SHARE)) : roll(2);
      for (let i = 0; i < many; i++) {
        const [x, y] = spots.splice(roll(spots.length), 1)[0];
        for (const cell of stack({ size }, now, x, y)) made[cell] = 1;
      }
      if (last) break;
      const options: number[] = [];
      const wide = Math.max(now.xs.hi - now.xs.lo, now.ys.hi - now.ys.lo);
      for (const axis of AXES) {
        const line = axis === X ? now.xs : now.ys;
        const width = line.hi - line.lo;
        if (width < wide - 1 || width <= NARROW) continue;
        for (let k = line.lo + 1; k < line.hi; k++)
          for (const from of SIDES) {
            const flap = from === LOW ? k - line.lo : line.hi - k;
            if (fits(line, k, from) && 3 * flap >= width)
              for (let w = 2 * flap >= width - 1 ? HALF : 1; w > 0; w--) options.push(code(axis, k, from));
          }
      }
      chain.push(options[roll(options.length)]);
    }
    holes = Array.from(made, (one) => String(one)).join("");
    if (holes.includes("1") && !striped(holes, size)) break;
  }
  const dealt: State = {
    size,
    target: holes,
    holes: "0".repeat(cells),
    folds: [],
    moves: 0,
    par: 0,
    over: null,
    seed: next,
  };
  if (!holes.includes("1")) return { ...dealt, over: "done" };
  return { ...dealt, par: par(dealt) };
}

function judge(state: State): State {
  if (state.holes === state.target) return { ...state, over: "done" };
  return flags(state, sheet(state)).some(broken) ? { ...state, over: "stuck" } : state;
}

export function step(state: State, act: Act): State {
  if (state.over) return state;
  const now = sheet(state);
  if (act.verb === "fold") {
    const { axis, line, from } = act.args;
    if (!AXES.includes(axis) || !fits(axis === X ? now.xs : now.ys, line, from)) return state;
    return judge({ ...state, folds: [...state.folds, code(axis, line, from)], moves: state.moves + 1 });
  }
  if (act.verb !== "punch") return state;
  const { x, y } = act.args;
  if (!Number.isInteger(x) || !Number.isInteger(y)) return state;
  if (x < now.xs.lo || x >= now.xs.hi || y < now.ys.lo || y >= now.ys.hi) return state;
  const cells = stack(state, now, x, y);
  if (!cells.every((cell) => state.target[cell] === "1") || cells.every((cell) => state.holes[cell] === "1"))
    return state;
  const marks = state.holes.split("");
  for (const cell of cells) marks[cell] = "1";
  return judge({ ...state, holes: marks.join(""), moves: state.moves + 1 });
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  const now = sheet(state);
  const out: Act[] = [];
  for (const axis of AXES) {
    const line = axis === X ? now.xs : now.ys;
    for (let k = line.lo + 1; k < line.hi; k++)
      for (const from of SIDES) if (fits(line, k, from)) out.push(fold(axis, k, from));
  }
  const marks = flags(state, now);
  for (let p = 0; p < marks.length; p++)
    if (needed(marks[p])) out.push(punch(p % state.size, Math.floor(p / state.size)));
  return out;
}

export function read(state: State, path = ""): unknown {
  let at: unknown = state;
  for (const key of path.split("/").filter(Boolean)) {
    if (at === null || typeof at !== "object") return undefined;
    at = (at as Record<string, unknown>)[key];
  }
  return at;
}

/* DEAL */

export function dealt(seed: number | undefined, round: number): number {
  let next = mix(seed ?? 0);
  for (let i = 0; i < round; i++) next = xorshift(next);
  return next;
}

export function deal({ size, folds }: Pick<Config, "size" | "folds">, { seed, round = 0 }: Table): State {
  return boot({ size, folds, seed: dealt(seed, round) });
}
