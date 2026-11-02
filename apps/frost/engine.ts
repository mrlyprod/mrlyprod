import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Over = "done" | null;

export type State = {
  size: number;
  board: string;
  turn: number;
  stones: number;
  flowers: number;
  over: Over;
  seed: number;
};

export type Config = Pick<State, "size" | "flowers" | "seed"> & { frost: number };

export type Table = { seed?: number; round?: number };

export type Act = { verb: "drop"; args: { x: number; y: number } } | { verb: "wait" };

export const EMPTY = ".";
export const STONE = "#";
export const FLOWER = "*";
export const SEED = "o";
export const WAYS = "nesw";
export const LOST = "NESW";
export const STEPS: [number, number][] = [
  [0, -1],
  [1, 0],
  [0, 1],
  [-1, 0],
];
export const SMALL = 9;
export const LARGE = 25;
export const MOST = 4;
export const BLOOMS = 12;
export const NEVER = 1 << 30;

export const drop = (x: number, y: number): Act => ({ verb: "drop", args: { x, y } });
export const WAIT: Act = { verb: "wait" };

const EDGE = 1;
const APART = 4;
const FIRST = 3;
const TRIES = 64;

/* CELLS */

export const frosty = (one: string) => one === SEED || WAYS.includes(one) || LOST.includes(one);

export const open = (one: string) => one === EMPTY || one === FLOWER;

export function parent(size: number, p: number, one: string): number {
  const way = WAYS.indexOf(one.toLowerCase());
  if (way < 0 || one === SEED) return -1;
  const [dx, dy] = STEPS[way];
  return p + dy * size + dx;
}

export function around(size: number, p: number): number[] {
  const x = p % size;
  const y = Math.floor(p / size);
  const out: number[] = [];
  for (const [dx, dy] of STEPS) {
    const a = x + dx;
    const b = y + dy;
    if (a >= 0 && a < size && b >= 0 && b < size) out.push(b * size + a);
  }
  return out;
}

export function depths(state: Pick<State, "size" | "board">): Int32Array {
  const { size, board } = state;
  const out = new Int32Array(board.length).fill(-1);
  const deep = (p: number): number => {
    if (out[p] >= 0) return out[p];
    const up = parent(size, p, board[p]);
    out[p] = up < 0 ? 0 : deep(up) + 1;
    return out[p];
  };
  for (let p = 0; p < board.length; p++) if (frosty(board[p])) deep(p);
  return out;
}

export const touches = (size: number, board: string, p: number) =>
  around(size, p).filter((q) => frosty(board[q])).length;

export const reachable = (size: number, board: string, p: number) =>
  board[p] === EMPTY && touches(size, board, p) === 1;

export function front(state: Pick<State, "size" | "board">): number[] {
  const out: number[] = [];
  for (let p = 0; p < state.board.length; p++)
    if (open(state.board[p]) && touches(state.size, state.board, p) === 1) out.push(p);
  return out;
}

export const count = (board: string, set: string) => [...board].filter((one) => set.includes(one)).length;

export const saved = (state: Pick<State, "board">) => count(state.board, FLOWER);

export const lost = (state: Pick<State, "board">) => count(state.board, LOST);

/* GROWTH */

export function grow(size: number, board: string): string {
  const out = board.split("");
  for (let p = 0; p < board.length; p++) {
    if (!open(board[p])) continue;
    const near = around(size, p).filter((q) => frosty(board[q]));
    if (near.length !== 1) continue;
    const q = near[0];
    const dx = (q % size) - (p % size);
    const dy = Math.floor(q / size) - Math.floor(p / size);
    const way = STEPS.findIndex(([a, b]) => a === dx && b === dy);
    out[p] = board[p] === FLOWER ? LOST[way] : WAYS[way];
  }
  return out.join("");
}

export function future(size: number, board: string, stone = -1): Int32Array {
  const n = board.length;
  const when = new Int32Array(n).fill(NEVER);
  const ice = new Uint8Array(n);
  const near = new Uint8Array(n);
  const free = (p: number) => p !== stone && !ice[p] && open(board[p]);
  for (let p = 0; p < n; p++) if (frosty(board[p])) ice[p] = 1;
  let ring: number[] = [];
  for (let p = 0; p < n; p++) {
    if (!free(p)) continue;
    for (const q of around(size, p)) if (ice[q]) near[p]++;
    if (near[p] === 1) ring.push(p);
  }
  for (let t = 1; ring.length; t++) {
    for (const p of ring) {
      ice[p] = 1;
      when[p] = t;
    }
    const touched: number[] = [];
    for (const p of ring)
      for (const q of around(size, p))
        if (free(q)) {
          near[q]++;
          touched.push(q);
        }
    ring = [...new Set(touched)].filter((q) => near[q] === 1);
  }
  return when;
}

export const doomed = (state: Pick<State, "size" | "board">, when = future(state.size, state.board)) =>
  [...state.board].some((one, p) => one === FLOWER && when[p] < NEVER);

/* RULES */

function settle(state: State): State {
  const when = future(state.size, state.board);
  if (doomed(state, when)) return state;
  let board = state.board;
  let turn = state.turn;
  while (board.includes(EMPTY) || board.includes(FLOWER)) {
    const grown = grow(state.size, board);
    if (grown === board) break;
    board = grown;
    turn++;
  }
  return { ...state, board, turn, over: "done" };
}

export function boot({ size, frost, flowers, seed }: Config): State {
  if (!Number.isInteger(size) || size < SMALL || size > LARGE)
    throw new Error(`frost: the field is not ${SMALL} to ${LARGE} wide`);
  if (!Number.isInteger(frost) || frost < 1 || frost > MOST) throw new Error(`frost: not 1 to ${MOST} seeds`);
  if (!Number.isInteger(flowers) || flowers < 1 || flowers > BLOOMS)
    throw new Error(`frost: not 1 to ${BLOOMS} flowers`);
  let next = mix(seed);
  const roll = (n: number) => {
    next = xorshift(next);
    return next % n;
  };
  const cells = size * size;
  let board = EMPTY.repeat(cells);
  for (let t = 0; t < TRIES; t++) {
    const marks = EMPTY.repeat(cells).split("");
    const seeds: number[] = [];
    for (let i = 0; i < frost * TRIES && seeds.length < frost; i++) {
      const x = EDGE + roll(size - 2 * EDGE);
      const y = EDGE + roll(size - 2 * EDGE);
      if (seeds.every((p) => Math.abs((p % size) - x) + Math.abs(Math.floor(p / size) - y) >= APART))
        seeds.push(y * size + x);
    }
    for (const p of seeds) marks[p] = SEED;
    const when = future(size, marks.join(""));
    const spots: number[] = [];
    for (let p = 0; p < cells; p++) if (when[p] >= FIRST && when[p] < NEVER) spots.push(p);
    const chosen: number[] = [];
    while (chosen.length < flowers && spots.length) {
      const p = spots.splice(roll(spots.length), 1)[0];
      if (chosen.every((q) => !around(size, q).includes(p))) chosen.push(p);
    }
    if (chosen.length < flowers) continue;
    for (const p of chosen) marks[p] = FLOWER;
    board = marks.join("");
    break;
  }
  return settle({ size, board, turn: 0, stones: 0, flowers: count(board, FLOWER), over: null, seed: next });
}

export function step(state: State, act: Act): State {
  if (state.over) return state;
  let board = state.board;
  let stones = state.stones;
  if (act.verb === "drop") {
    const { x, y } = act.args;
    const { size } = state;
    if (!Number.isInteger(x) || !Number.isInteger(y) || x < 0 || y < 0 || x >= size || y >= size) return state;
    const p = y * size + x;
    if (!reachable(size, board, p)) return state;
    board = `${board.slice(0, p)}${STONE}${board.slice(p + 1)}`;
    stones++;
  } else if (act.verb !== "wait") return state;
  return settle({ ...state, board: grow(state.size, board), stones, turn: state.turn + 1 });
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  const out: Act[] = [WAIT];
  for (const p of front(state))
    if (state.board[p] === EMPTY) out.push(drop(p % state.size, Math.floor(p / state.size)));
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

export function deal(
  { size, frost, flowers }: Pick<Config, "size" | "frost" | "flowers">,
  { seed, round = 0 }: Table,
): State {
  return boot({ size, frost, flowers, seed: dealt(seed, round) });
}
