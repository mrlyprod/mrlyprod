import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix } from "@mrlyprod/ui/kit/rand.ts";
import type { Sprite } from "../shared/games/sprites.ts";
import { shuffled, sprites } from "../shared/games/sprites.ts";

export type Face = "hidden" | "open" | "flag";

export type Cell = { mine: boolean; near: number; face: Face };

export type Act = { verb: "reveal" | "flag"; args: { at: number } };

export type Over = "won" | "lost" | null;

export type State = {
  cells: Cell[];
  laid: boolean;
  over: Over;
  boom: number;
  flags: number;
  faces: Sprite[];
  seed: number;
};

export const COLS = 9;
export const ROWS = 9;
export const MINES = 10;

const WIDE = 4294967296;

function around(at: number): number[] {
  const col = at % COLS;
  const row = Math.floor(at / COLS);
  const out: number[] = [];
  for (let y = Math.max(0, row - 1); y <= Math.min(ROWS - 1, row + 1); y++)
    for (let x = Math.max(0, col - 1); x <= Math.min(COLS - 1, col + 1); x++)
      if (x !== col || y !== row) out.push(y * COLS + x);
  return out;
}

function lay(state: State, safe: number): State {
  const spots = state.cells.flatMap((_, at) => (at === safe ? [] : [at]));
  const [mixed, seed] = shuffled(state.seed, spots);
  const mined = new Set(mixed.slice(0, MINES));
  const cells = state.cells.map((cell, at) => ({
    ...cell,
    mine: mined.has(at),
    near: around(at).filter((one) => mined.has(one)).length,
  }));
  return { ...state, cells, laid: true, seed };
}

function open(state: State, from: number): State {
  const cells = state.cells.slice();
  const stack = [from];
  while (stack.length) {
    const at = stack.pop()!;
    if (cells[at].face !== "hidden" || cells[at].mine) continue;
    cells[at] = { ...cells[at], face: "open" };
    if (!cells[at].near) stack.push(...around(at));
  }
  if (cells.some((cell) => !cell.mine && cell.face !== "open")) return { ...state, cells };
  const flagged = cells.map((cell): Cell => (cell.mine ? { ...cell, face: "flag" } : cell));
  return { ...state, cells: flagged, flags: MINES, over: "won" };
}

function reveal(state: State, at: number): State {
  if (state.over || state.cells[at]?.face !== "hidden") return state;
  const ready = state.laid ? state : lay(state, at);
  if (!ready.cells[at].mine) return open(ready, at);
  const cells = ready.cells.map((cell): Cell => (cell.mine ? { ...cell, face: "open" } : cell));
  return { ...ready, cells, over: "lost", boom: at };
}

function flag(state: State, at: number): State {
  const cell = state.cells[at];
  if (state.over || !cell || cell.face === "open") return state;
  const on = cell.face === "hidden";
  const cells = state.cells.slice();
  cells[at] = { ...cell, face: on ? "flag" : "hidden" };
  return { ...state, cells, flags: state.flags + (on ? 1 : -1) };
}

export function boot({ seed }: { seed: number }): State {
  const [faces, next] = sprites(mix(seed), 2);
  const cells = Array.from({ length: COLS * ROWS }, (): Cell => ({ mine: false, near: 0, face: "hidden" }));
  return { cells, laid: false, over: null, boom: -1, flags: 0, faces, seed: next };
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  return state.cells.flatMap((cell, at): Act[] => {
    if (cell.face === "open") return [];
    const toggle: Act = { verb: "flag", args: { at } };
    return cell.face === "flag" ? [toggle] : [{ verb: "reveal", args: { at } }, toggle];
  });
}

export function step(state: State, act: Act): State {
  if (act.verb === "reveal") return reveal(state, act.args.at);
  if (act.verb === "flag") return flag(state, act.args.at);
  return state;
}

export const deal = (_value: unknown, { rand }: { rand: Rng }): State => boot({ seed: Math.floor(rand() * WIDE) });
