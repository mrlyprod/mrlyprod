import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix } from "@mrlyprod/ui/kit/rand.ts";
import type { Sprite } from "../shared/games/sprites.ts";
import { below, sprites } from "../shared/games/sprites.ts";

export type Act = { verb: "pick"; args: { at: number } };

export type State = {
  items: Sprite[];
  target: number;
  wrong: number;
  score: number;
  over: boolean;
  seed: number;
};

export const SIDE = 3;

const WIDE = 4294967296;

function round(seed: number): Pick<State, "items" | "target" | "seed"> {
  const [items, a] = sprites(seed, SIDE * SIDE);
  const [target, next] = below(a, items.length);
  return { items, target, seed: next };
}

export function boot({ seed }: { seed: number }): State {
  return { ...round(mix(seed)), wrong: -1, score: 0, over: false };
}

export function moves(state: State): Act[] {
  return state.over ? [] : state.items.map((_, at) => ({ verb: "pick", args: { at } }));
}

export function step(state: State, act: Act): State {
  const { at } = act.args;
  if (state.over || !state.items[at]) return state;
  if (at !== state.target) return { ...state, wrong: at, over: true };
  return { ...state, ...round(state.seed), score: state.score + 1 };
}

export const deal = (_value: unknown, { rand }: { rand: Rng }): State => boot({ seed: Math.floor(rand() * WIDE) });
