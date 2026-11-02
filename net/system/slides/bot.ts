import type { Block } from "../../sdk/blocks.ts";
import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Slides } from "./app.ts";

/* WORDS */

export const TITLES = ["Quarter plan", "Soup night", "The garden", "Why lentils"];

const SLIDE_TITLES = ["Agenda", "Where we are", "What is next", "Questions", "Thanks"];

const TREES: Block[][] = [
  [
    { kind: "li", text: "ship" },
    { kind: "li", text: "measure" },
    { kind: "li", text: "repeat" },
  ],
  [
    { kind: "h2", text: "One number" },
    { kind: "p", text: "48 hours a week" },
  ],
  [{ kind: "quote", text: "Water the tomatoes on Tuesdays." }],
];

/* BOT */

const bot: Bot<Slides> = ({ state, me, actors, draw }): Step => {
  const own = Object.keys(state.decks).filter((id) => state.decks[id]!.owner === me);
  const shared = Object.keys(state.decks).filter((id) => state.decks[id]!.editors.includes(me));
  const mine = [...own, ...shared];
  if (!own.length || below(draw, 10) === 0) return { verb: "create", args: { title: pick(draw, TITLES) } };
  const id = pick(draw, mine);
  const deck = state.decks[id]!;
  const others = actors.filter((one) => one !== me && !deck.editors.includes(one));
  if (own.includes(id) && others.length && below(draw, 6) === 0)
    return { verb: "share", args: { deck: id, with: pick(draw, others) } };
  if (own.length > 5 && below(draw, 4) === 0) return { verb: "delete", args: { deck: pick(draw, own) } };
  if (!deck.slides.length || (deck.slides.length < 6 && below(draw, 3) === 0))
    return { verb: "add", args: { deck: id, title: pick(draw, SLIDE_TITLES) } };
  const slide = below(draw, deck.slides.length);
  if (deck.slides.length > 4 && below(draw, 4) === 0) return { verb: "drop", args: { deck: id, slide } };
  return { verb: "write", args: { deck: id, slide, blocks: pick(draw, TREES) } };
};

export default bot;
