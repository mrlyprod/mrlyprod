import type { Block } from "@mrlyprod/apps/shared/blocks.ts";
import type { Deck, Slides } from "@mrlyprod/apps/slides/app.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

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

const owning = (deck: Deck, id: string, draw: () => number, others: string[]): Step => {
  const fresh = others.filter((one) => !deck.editors.includes(one) && !deck.readers.includes(one));
  const sharers = [...deck.editors, ...deck.readers];
  const turn = below(draw, 12);
  if (turn === 0 && fresh.length)
    return { verb: "share", args: { deck: id, with: pick(draw, fresh), role: pick(draw, ["editor", "reader"]) } };
  if (turn === 1 && sharers.length > 1) return { verb: "unshare", args: { deck: id, with: pick(draw, sharers) } };
  if (turn === 2 && deck.slides.length) return { verb: "publish", args: { deck: id, on: !deck.open } };
  if (turn === 3) return { verb: "rename", args: { deck: id, title: pick(draw, TITLES) } };
  return null;
};

const bot: Bot<Slides> = ({ state, me, actors, draw }): Step => {
  const ids = Object.keys(state.decks);
  const own = ids.filter((id) => state.decks[id]!.owner === me);
  const editing = ids.filter((id) => state.decks[id]!.editors.includes(me));
  const owners = Object.values(state.decks).map((one) => one.owner);
  const others = [...new Set([...actors, ...owners])].filter((one) => one !== me);
  if (!own.length || below(draw, 10) === 0) return { verb: "create", args: { title: pick(draw, TITLES) } };
  if (own.length > 5 && below(draw, 4) === 0) return { verb: "delete", args: { deck: pick(draw, own) } };
  const id = pick(draw, [...own, ...editing]);
  const deck = state.decks[id]!;
  const act = own.includes(id) ? owning(deck, id, draw, others) : null;
  if (act) return act;
  if (!deck.slides.length || (deck.slides.length < 6 && below(draw, 3) === 0))
    return { verb: "add", args: { deck: id, title: pick(draw, SLIDE_TITLES) } };
  const slide = below(draw, deck.slides.length);
  if (deck.slides.length > 4 && below(draw, 4) === 0) return { verb: "drop", args: { deck: id, slide } };
  return { verb: "write", args: { deck: id, slide, blocks: pick(draw, TREES) } };
};

export default bot;
