import type { Rng } from "../../net/sdk/rand.ts";
import type { Act, State } from "./engine.ts";
import { LEFT, moves, plan, RIGHT, turn } from "./engine.ts";

/* POLICIES */

function smart(state: State): Act | null {
  const want = plan(state);
  const block = want.findIndex((q) => q !== 0);
  return block < 0 ? null : turn(block, want[block] === 3 ? LEFT : RIGHT);
}

function silly(state: State, rand: Rng): Act | null {
  const all = moves(state);
  return all.length ? all[Math.floor(rand() * all.length)] : null;
}

export function pick(state: State, mode: string, rand: Rng): Act | null {
  if (state.over) return null;
  return mode === "silly" ? silly(state, rand) : smart(state);
}
