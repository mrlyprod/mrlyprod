import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Iden } from "./app.ts";

/* WORDS */

export const HANDLES = ["ada", "bay-leaf", "cook42", "dune", "eve", "fig", "gardener", "hours"];

/* BOT */

const bot: Bot<Iden> = ({ state, me, draw }): Step => {
  const taken = new Set(Object.values(state.cards).map((card) => card.handle));
  const mine = state.cards[me]?.handle;
  const free = HANDLES.filter((handle) => !taken.has(handle));
  if (!free.length || (mine !== me && below(draw, 4) !== 0)) return null;
  return { verb: "handle", args: { handle: pick(draw, free) } };
};

export default bot;
