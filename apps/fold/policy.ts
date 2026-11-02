import type { Rng } from "../../net/sdk/rand.ts";
import type { Act, State } from "./engine.ts";
import { moves, plan, step } from "./engine.ts";

/* POLICIES */

const ROOM = 256;

const known = new Map<string, Act>();

const name = (state: State) => `${state.size}:${state.target}:${state.holes}:${state.folds.join()}`;

function smart(state: State): Act | null {
  const hit = known.get(name(state));
  if (hit) return hit;
  const acts = plan(state);
  if (known.size > ROOM) known.clear();
  let at = state;
  for (const act of acts) {
    known.set(name(at), act);
    at = step(at, act);
  }
  return acts[0] ?? null;
}

function random(state: State, rand: Rng): Act | null {
  const all = moves(state);
  return all.length ? all[Math.floor(rand() * all.length)] : null;
}

export function pick(state: State, mode: string, rand: Rng): Act | null {
  if (state.over) return null;
  return mode === "random" ? random(state, rand) : smart(state);
}
