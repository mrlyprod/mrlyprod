import type { Rng } from "@mrlyprod/ui/kit/rand.ts";
import { mix } from "@mrlyprod/ui/kit/rand.ts";
import type { Sprite } from "../shared/games/sprites.ts";
import { shuffled, sprites } from "../shared/games/sprites.ts";

export type Card = Sprite & { gone: boolean };

export type Act = { verb: "flip"; args: { at: number } };

export type State = {
  cards: Card[];
  flash: boolean;
  open: number;
  pair: number[];
  pairs: number;
  over: boolean;
  seed: number;
};

export const SIDE = 3;

const PAIRS = 4;
const WIDE = 4294967296;

const same = (a: Sprite, b: Sprite) => a.design === b.design && a.number === b.number && a.hue === b.hue;

function round(seed: number): Pick<State, "cards" | "flash" | "open" | "seed"> {
  const [faces, a] = sprites(seed, PAIRS + 1);
  const dealt = faces.flatMap((face, i) => (i < PAIRS ? [face, face] : [face]));
  const [order, next] = shuffled(a, dealt);
  return { cards: order.map((face) => ({ ...face, gone: false })), flash: true, open: -1, seed: next };
}

export function boot({ seed }: { seed: number }): State {
  return { ...round(mix(seed)), pair: [], pairs: 0, over: false };
}

export function moves(state: State): Act[] {
  if (state.over) return [];
  return state.cards.flatMap((card, at) => (card.gone || at === state.open ? [] : [{ verb: "flip", args: { at } }]));
}

export function step(state: State, act: Act): State {
  const { at } = act.args;
  const card = state.cards[at];
  if (state.over || !card) return state;
  if (state.flash) return { ...state, flash: false };
  if (card.gone || at === state.open) return state;
  if (state.open < 0) return { ...state, open: at };
  const pair = [state.open, at];
  if (!same(state.cards[state.open], card)) return { ...state, open: -1, pair, over: true };
  const cards = state.cards.map((one, i) => (pair.includes(i) ? { ...one, gone: true } : one));
  const pairs = state.pairs + 1;
  if (cards.filter((one) => !one.gone).length > 1) return { ...state, cards, open: -1, pair, pairs };
  return { ...state, ...round(state.seed), pair, pairs };
}

export const deal = (_value: unknown, { rand }: { rand: Rng }): State => boot({ seed: Math.floor(rand() * WIDE) });
