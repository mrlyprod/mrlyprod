import type { Dating } from "@mrlyprod/apps/dating/app.ts";
import { deckOf, left, matchesOf } from "@mrlyprod/apps/dating/app.ts";
import { below, pick, walk } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

export const LINES = [
  "Bakes too much bread.",
  "Will walk anywhere.",
  "Reads the last page first.",
  "Knows every bird by song.",
  "Fixes old radios.",
  "Loses at chess, cheerfully.",
  "Grows tomatoes on a balcony.",
  "Always early, never sorry.",
];

export const SEEKS = [
  "A long walk",
  "Someone to cook with",
  "A chess rival",
  "A friend first",
  "Someone who writes back",
  "A travel partner",
];

const profile = (state: Dating, draw: () => number): Step => {
  const most = state.dials.profile;
  const line = pick(draw, LINES).slice(0, most);
  return { verb: "join", args: below(draw, 2) ? { line, seeking: pick(draw, SEEKS).slice(0, most) } : { line } };
};

const bot: Bot<Dating> = ({ state, me, at, draw, world }): Step => {
  if (!state.dials.open) return null;
  const peek = (path: string) => walk(world.apps, path.split("/")) ?? null;
  if (!state.profiles[me]) return below(draw, 2) ? profile(state, draw) : null;
  const held = matchesOf(state, me, peek);
  if (held.length && !below(draw, 6)) return { verb: "unmatch", args: { person: pick(draw, held).id } };
  const deck = deckOf(state, me, peek);
  if (deck.length && left(state, me, at) && below(draw, 4))
    return { verb: "swipe", args: { person: pick(draw, deck), like: below(draw, 3) > 0 } };
  if (!below(draw, 24)) return { verb: "leave", args: {} };
  return below(draw, 12) ? null : profile(state, draw);
};

export default bot;
