import type { Rng } from "../../net/sdk/rand.ts";
import type { Act, State } from "./engine.ts";
import { moves, step } from "./engine.ts";

/* POLICIES */

const margin = (state: State, who: number) => state.points[who - 1] - state.points[2 - who];

const WIDE = 8;

function reply(state: State): number {
  let high = -Infinity;
  for (const act of moves(state)) high = Math.max(high, margin(step(state, act), state.turn));
  return high === -Infinity ? margin(state, state.turn) : high;
}

function greedy(state: State, rand: Rng): Act | null {
  let best: Act[] = [];
  let high = -Infinity;
  for (const act of moves(state)) {
    const score = margin(step(state, act), state.turn);
    if (score > high) {
      high = score;
      best = [act];
    } else if (score === high) best.push(act);
  }
  return best.length ? best[Math.floor(rand() * best.length)] : null;
}

function smart(state: State, rand: Rng): Act | null {
  const ranked = moves(state)
    .map((act) => ({ act, next: step(state, act), tie: rand() }))
    .map((one) => ({ ...one, now: margin(one.next, state.turn) }))
    .sort((a, b) => b.now - a.now || a.tie - b.tie)
    .slice(0, WIDE);
  let best: Act | null = null;
  let high = -Infinity;
  for (const { act, next } of ranked) {
    const score = -reply(next);
    if (score > high) {
      high = score;
      best = act;
    }
  }
  return best;
}

function random(state: State, rand: Rng): Act | null {
  const all = moves(state);
  return all.length ? all[Math.floor(rand() * all.length)] : null;
}

export function pick(state: State, mode: string, rand: Rng): Act | null {
  if (state.over) return null;
  return mode === "random" ? random(state, rand) : mode === "greedy" ? greedy(state, rand) : smart(state, rand);
}
