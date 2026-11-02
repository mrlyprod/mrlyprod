import type { Rng } from "../../net/sdk/rand.ts";
import type { Act, State } from "./engine.ts";
import { moves, solve, tilt } from "./engine.ts";

/* POLICIES */

function smart(state: State, rand: Rng): Act | null {
  const answer = solve(state);
  if (!answer?.length) return random(state, rand);
  const p = answer[0];
  return tilt(p % state.size, Math.floor(p / state.size));
}

function random(state: State, rand: Rng): Act | null {
  const all = moves(state);
  return all.length ? all[Math.floor(rand() * all.length)] : null;
}

export function pick(state: State, mode: string, rand: Rng): Act | null {
  if (state.over) return null;
  return mode === "random" ? random(state, rand) : smart(state, rand);
}
