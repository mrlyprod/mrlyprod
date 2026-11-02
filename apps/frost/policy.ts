import type { Rng } from "../../net/sdk/rand.ts";
import type { Act, State } from "./engine.ts";
import { drop, EMPTY, FLOWER, front, future, LOST, moves, NEVER, step, WAIT } from "./engine.ts";

/* POLICIES */

export function fates(state: State): number[] {
  const when = future(state.size, state.board);
  const out: number[] = [];
  for (let p = 0; p < state.board.length; p++)
    if (state.board[p] === FLOWER) out.push(when[p]);
    else if (LOST.includes(state.board[p])) out.push(0);
  return out.sort((a, b) => a - b);
}

function better(a: number[], b: number[]): boolean {
  const kept = (list: number[]) => list.filter((one) => one >= NEVER).length;
  if (kept(a) !== kept(b)) return kept(a) > kept(b);
  for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return a[i] > b[i];
  return false;
}

const tries = (state: State): Act[] => [
  WAIT,
  ...front(state)
    .filter((p) => state.board[p] === EMPTY)
    .map((p) => drop(p % state.size, Math.floor(p / state.size))),
];

function smart(state: State): Act | null {
  let top: number[] | null = null;
  let choice: Act = WAIT;
  for (const act of tries(state)) {
    const then = fates(step(state, act));
    if (!top || better(then, top)) {
      top = then;
      choice = act;
    }
  }
  return choice;
}

function random(state: State, rand: Rng): Act | null {
  const all = moves(state);
  return all.length ? all[Math.floor(rand() * all.length)] : null;
}

export function pick(state: State, mode: string, rand: Rng): Act | null {
  if (state.over) return null;
  return mode === "random" ? random(state, rand) : smart(state);
}
