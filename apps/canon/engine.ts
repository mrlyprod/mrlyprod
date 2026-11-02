import { mix, xorshift } from "../../net/sdk/rand.ts";
import type { Mrly } from "../shared/designs/engine.ts";

/* TYPES */

export type Over = "one" | "two" | "draw" | null;

export type State = {
  number: number;
  level: number;
  motif: string;
  board: string;
  turn: number;
  points: number[];
  moves: number;
  last: number;
  over: Over;
  seed: number;
};

export type Config = Pick<State, "number" | "level" | "motif" | "seed">;

export type Table = { seed?: number; round?: number; units: { math: Mrly } };

export type Act = { verb: "claim"; args: { cell: number } };

export type Tally = { owners: Int8Array; points: number[] };

export const ONE = 1;
export const TWO = 2;
export const claim = (cell: number): Act => ({ verb: "claim", args: { cell } });
export const other = (who: number) => (who === ONE ? TWO : ONE);

export const DIM = 2;
export const BASE = 3;
export const HOME = "473";
export const NUMBERS = [3, 5, 7];
export const ROOM = 81;

const FREE = "0";

/* DOOR */

export function cap(number: number): number {
  let level = 1;
  while (number ** (level + 1) <= ROOM) level++;
  return level;
}

export function coded(math: Mrly, code: unknown): string {
  const text = String(code ?? "").trim();
  const count = Number(math.bang.factory.total_codes(DIM, BASE));
  if (text === "" || !Number.isFinite(Number(text))) return HOME;
  return String(((Math.floor(Number(text)) % count) + count) % count);
}

export function grown(math: Mrly, number: number, level: number, code: string): string {
  const cell = math.two.create(code, number, level, 0, BASE);
  return Array.from(cell.types, (type) => (type ? "1" : "0")).join("");
}

/* TREE */

export const fills = (motif: string) => [...motif].flatMap((one, i) => (one === "1" ? [i] : []));

export function starts(f: number, level: number): number[] {
  const out = [0];
  for (let d = 0; d <= level; d++) out.push(out[d] + f ** d);
  return out;
}

export function digits(cell: number, f: number, level: number): number[] {
  const out = new Array<number>(level);
  for (let d = level - 1; d >= 0; d--) {
    out[d] = cell % f;
    cell = Math.floor(cell / f);
  }
  return out;
}

export const join = (path: readonly number[], f: number) => path.reduce((at, one) => at * f + one, 0);

export function echoes(state: Pick<State, "motif" | "level">, cell: number): number[] {
  const f = fills(state.motif).length;
  const path = digits(cell, f, state.level);
  const out: number[] = [];
  for (let r = 0; r < state.level; r++) {
    const one = join([...path.slice(r), ...path.slice(0, r)], f);
    if (!out.includes(one)) out.push(one);
  }
  return out;
}

export function spot(state: Pick<State, "number" | "motif" | "level">, cell: number): [number, number] {
  const { number: n, level } = state;
  const slots = fills(state.motif);
  let x = 0;
  let y = 0;
  for (const one of digits(cell, slots.length, level)) {
    x = x * n + (slots[one] % n);
    y = y * n + Math.floor(slots[one] / n);
  }
  return [x, y];
}

/* SCORE */

export function tally(state: Pick<State, "motif" | "level" | "board">): Tally {
  const f = fills(state.motif).length;
  const { level, board } = state;
  const at = starts(f, level);
  const owners = new Int8Array(at[level + 1]);
  const points = [0, 0];
  for (let i = 0; i < board.length; i++) {
    const who = board.charCodeAt(i) - 48;
    owners[at[level] + i] = who;
    if (who) points[who - 1]++;
  }
  for (let d = level - 1; d >= 0; d--) {
    const worth = f ** (level - d);
    for (let p = 0; p < f ** d; p++) {
      let lead = 0;
      for (let k = 0; k < f; k++) {
        const who = owners[at[d + 1] + p * f + k];
        lead += who === ONE ? 1 : who === TWO ? -1 : 0;
      }
      const who = lead > 0 ? ONE : lead < 0 ? TWO : d ? owners[at[d + 1] + p * f + (p % f)] : 0;
      owners[at[d] + p] = who;
      if (who) points[who - 1] += worth;
    }
  }
  return { owners, points };
}

/* RULES */

export function boot({ number, level, motif, seed }: Config): State {
  if (motif.length !== number * number) throw new Error(`canon: the motif is not ${number} by ${number}`);
  const f = fills(motif).length;
  const next = xorshift(mix(seed));
  const board = FREE.repeat(f ** level);
  return {
    number,
    level,
    motif,
    board,
    turn: next & 1 ? TWO : ONE,
    points: [0, 0],
    moves: 0,
    last: -1,
    over: board.length ? null : "draw",
    seed: next,
  };
}

export function step(state: State, act: Act): State {
  if (state.over || act.verb !== "claim") return state;
  const { cell } = act.args;
  if (!Number.isInteger(cell) || cell < 0 || cell >= state.board.length || state.board[cell] !== FREE) return state;
  const marks = state.board.split("");
  for (const one of echoes(state, cell)) marks[one] = String(state.turn);
  const board = marks.join("");
  const { points } = tally({ ...state, board });
  const full = !board.includes(FREE);
  const over: Over = !full ? null : points[0] > points[1] ? "one" : points[1] > points[0] ? "two" : "draw";
  return { ...state, board, points, turn: other(state.turn), moves: state.moves + 1, last: cell, over };
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  const out: Act[] = [];
  for (let cell = 0; cell < state.board.length; cell++)
    if (state.board[cell] === FREE && Math.min(...echoes(state, cell)) === cell) out.push(claim(cell));
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
  { code, number, level }: Pick<Config, "number" | "level"> & { code: unknown },
  { seed, round = 0, units }: Table,
): State {
  const motif = grown(units.math, number, 1, coded(units.math, code));
  return boot({ number, level, motif, seed: dealt(seed, round) });
}
