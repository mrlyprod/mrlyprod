import { mix, xorshift } from "../../net/sdk/rand.ts";

/* TYPES */

export type Over = "done" | null;

export type State = {
  size: number;
  board: string;
  lamp: number;
  way: number;
  stars: number;
  taps: number;
  par: number;
  over: Over;
  seed: number;
};

export type Config = Pick<State, "size" | "stars" | "seed"> & { mirrors: number };

export type Table = { seed?: number; round?: number };

export type Act = { verb: "tilt"; args: { x: number; y: number } };

export type Hop = { p: number; way: number };

export const EMPTY = ".";
export const STAR = "*";
export const RISE = "/";
export const FALL = "\\";
export const STEPS: [number, number][] = [
  [0, -1],
  [1, 0],
  [0, 1],
  [-1, 0],
];
export const SMALL = 6;
export const LARGE = 14;
export const FEW = 4;
export const MANY = 40;
export const SKY = 8;

export const tilt = (x: number, y: number): Act => ({ verb: "tilt", args: { x, y } });

const BOUNCE: Record<string, number[]> = {
  [RISE]: [1, 0, 3, 2],
  [FALL]: [3, 2, 1, 0],
};
const TRIES = 96;
const BUDGET = 60000;
const DEEPEST = 12;
const APART = 2;

/* CELLS */

export const mirror = (one: string) => one === RISE || one === FALL;

export const flip = (one: string) => (one === RISE ? FALL : RISE);

export function next(size: number, p: number, way: number): number {
  const x = (p % size) + STEPS[way][0];
  const y = Math.floor(p / size) + STEPS[way][1];
  return x < 0 || y < 0 || x >= size || y >= size ? -1 : y * size + x;
}

export const bounce = (one: string, way: number) => (mirror(one) ? BOUNCE[one][way] : way);

export function trace(state: Pick<State, "size" | "board" | "lamp" | "way">): Hop[] {
  const { size, board } = state;
  const out: Hop[] = [];
  let p = state.lamp;
  let way = state.way;
  for (let i = 0; p >= 0 && i <= 4 * board.length; i++) {
    way = bounce(board[p], way);
    out.push({ p, way });
    p = next(size, p, way);
  }
  return out;
}

export function lit(state: Pick<State, "size" | "board" | "lamp" | "way">): Set<number> {
  return new Set(
    trace(state)
      .map((hop) => hop.p)
      .filter((p) => state.board[p] === STAR),
  );
}

export const count = (board: string, set: string) => [...board].filter((one) => set.includes(one)).length;

/* SOLVE */

export function solve(state: Pick<State, "size" | "board" | "lamp" | "way">): number[] | null {
  const { size, board, lamp, way } = state;
  const n = board.length;
  const total = count(board, STAR);
  const chosen = new Int8Array(n).fill(-1);
  const seen = new Int32Array(n);
  const order: number[] = [];
  let shone = 0;
  let spent = 0;
  let found: number[] | null = null;
  const walk = (start: number, from: number, cap: number, flips: number): void => {
    if (found || spent > BUDGET) return;
    spent++;
    const marked: number[] = [];
    let p = start;
    let w = from;
    let steps = 0;
    while (p >= 0 && steps++ <= 4 * n) {
      const one = board[p];
      if (mirror(one) && chosen[p] < 0) {
        const kept = one === RISE ? 0 : 1;
        for (const pick of [kept, 1 - kept]) {
          const cost = flips + (pick === kept ? 0 : 1);
          if (cost > cap) continue;
          chosen[p] = pick;
          order.push(p);
          const turned = BOUNCE[pick === 0 ? RISE : FALL][w];
          walk(next(size, p, turned), turned, cap, cost);
          order.pop();
          chosen[p] = -1;
          if (found) break;
        }
        for (const q of marked) if (--seen[q] === 0) shone--;
        return;
      }
      if (mirror(one)) w = BOUNCE[chosen[p] === 0 ? RISE : FALL][w];
      if (one === STAR) {
        if (seen[p]++ === 0) shone++;
        marked.push(p);
      }
      p = next(size, p, w);
    }
    if (shone === total) found = order.filter((q) => (chosen[q] === 0 ? RISE : FALL) !== board[q]);
    for (const q of marked) if (--seen[q] === 0) shone--;
  };
  for (let cap = 0; cap <= DEEPEST && !found && spent <= BUDGET; cap++) walk(lamp, way, cap, 0);
  return found;
}

/* RULES */

function judge(state: State): State {
  return lit(state).size === state.stars ? { ...state, over: "done" } : state;
}

export function boot({ size, mirrors, stars, seed }: Config): State {
  if (!Number.isInteger(size) || size < SMALL || size > LARGE)
    throw new Error(`beam: the room is not ${SMALL} to ${LARGE} wide`);
  if (!Number.isInteger(mirrors) || mirrors < FEW || mirrors > MANY)
    throw new Error(`beam: not ${FEW} to ${MANY} mirrors`);
  if (!Number.isInteger(stars) || stars < 1 || stars > SKY) throw new Error(`beam: not 1 to ${SKY} stars`);
  let rolled = mix(seed);
  const roll = (n: number) => {
    rolled = xorshift(rolled);
    return rolled % n;
  };
  const cells = size * size;
  const room = Math.min(mirrors, Math.floor(cells / 3));
  let best: State | null = null;
  for (let t = 0; t < TRIES; t++) {
    const marks = EMPTY.repeat(cells).split("");
    for (let placed = 0; placed < room; ) {
      const p = roll(cells);
      if (marks[p] !== EMPTY) continue;
      marks[p] = roll(2) ? RISE : FALL;
      placed++;
    }
    const side = roll(4);
    const along = roll(size);
    const way = (side + 2) % 4;
    const lamp = [along, size - 1 + along * size, (size - 1) * size + along, along * size][side];
    const path = trace({ size, board: marks.join(""), lamp, way });
    const hit = new Set(path.filter((hop) => mirror(marks[hop.p])).map((hop) => hop.p));
    if (hit.size < 2) continue;
    const spots: number[] = [];
    path.forEach((hop, i) => {
      if (marks[hop.p] === EMPTY && i > 0 && !spots.includes(hop.p)) spots.push(hop.p);
    });
    const chosen: number[] = [];
    while (chosen.length < stars && spots.length) {
      const p = spots.splice(roll(spots.length), 1)[0];
      const x = p % size;
      const y = Math.floor(p / size);
      if (chosen.every((q) => Math.abs((q % size) - x) + Math.abs(Math.floor(q / size) - y) >= APART)) chosen.push(p);
    }
    if (chosen.length < stars) continue;
    for (const p of chosen) marks[p] = STAR;
    for (const p of hit) if (roll(2)) marks[p] = flip(marks[p]);
    for (let p = 0; p < cells; p++) if (mirror(marks[p]) && !hit.has(p) && roll(2)) marks[p] = flip(marks[p]);
    const state: State = {
      size,
      board: marks.join(""),
      lamp,
      way,
      stars,
      taps: 0,
      par: 0,
      over: null,
      seed: rolled,
    };
    if (judge(state).over) continue;
    const answer = solve(state);
    if (!answer) continue;
    const dealt = { ...state, par: answer.length };
    if (!best || dealt.par > best.par) best = dealt;
    if (dealt.par >= Math.min(DEEPEST, 2 + Math.floor(room / 4))) break;
  }
  if (!best) throw new Error("beam: no room dealt");
  return best;
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "tilt") return state;
  const { x, y } = act.args;
  const { size } = state;
  if (!Number.isInteger(x) || !Number.isInteger(y) || x < 0 || y < 0 || x >= size || y >= size) return state;
  const p = y * size + x;
  if (!mirror(state.board[p])) return state;
  const board = `${state.board.slice(0, p)}${flip(state.board[p])}${state.board.slice(p + 1)}`;
  return judge({ ...state, board, taps: state.taps + 1 });
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  const out: Act[] = [];
  for (let p = 0; p < state.board.length; p++)
    if (mirror(state.board[p])) out.push(tilt(p % state.size, Math.floor(p / state.size)));
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
  let n = mix(seed ?? 0);
  for (let i = 0; i < round; i++) n = xorshift(n);
  return n;
}

export function deal(
  { size, mirrors, stars }: Pick<Config, "size" | "mirrors" | "stars">,
  { seed, round = 0 }: Table,
): State {
  return boot({ size, mirrors, stars, seed: dealt(seed, round) });
}
