import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix } from "@mrlyprod/ui/kit/rand.ts";
import type { Sprite } from "../shared/games/sprites.ts";
import { below, named, sprites } from "../shared/games/sprites.ts";

export type Act = { verb: "answer"; args: { at: number } };

export type State = {
  sprite: Sprite;
  options: string[];
  answer: number;
  chose: number;
  score: number;
  over: boolean;
  seed: number;
};

const OPTIONS = 3;
const WIDE = 4294967296;

function round(seed: number): Pick<State, "sprite" | "options" | "answer" | "seed"> {
  const [trio, a] = sprites(seed, OPTIONS);
  const [answer, next] = below(a, OPTIONS);
  return { sprite: trio[answer], options: trio.map(named), answer, seed: next };
}

export function boot({ seed }: { seed: number }): State {
  return { ...round(mix(seed)), chose: -1, score: 0, over: false };
}

export function moves(state: State): Act[] {
  return state.over ? [] : state.options.map((_, at) => ({ verb: "answer", args: { at } }));
}

export function step(state: State, act: Act): State {
  const { at } = act.args;
  if (state.over || !state.options[at]) return state;
  if (at !== state.answer) return { ...state, chose: at, over: true };
  return { ...state, ...round(state.seed), score: state.score + 1 };
}

export const deal = (_value: unknown, { rand }: { rand: Rng }): State => boot({ seed: Math.floor(rand() * WIDE) });
