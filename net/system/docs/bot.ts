import type { Block } from "../../sdk/blocks.ts";
import type { Bot, Step } from "../../sdk/system.ts";
import { below, pick } from "../../sdk/system.ts";
import type { Docs } from "./app.ts";

/* WORDS */

export const TITLES = ["Plan", "Lentil soup", "Letter to the bank", "Notes from the walk", "Garden"];

const LINES = [
  "First draft.",
  "Simmer the lentils with an onion and a bay leaf.",
  "Water the tomatoes on Tuesdays.",
  "Ask about the fixed rate.",
  "Ship, measure, repeat.",
];

const TREES: Block[][] = [
  [
    { kind: "h1", text: "Plan" },
    { kind: "p", text: LINES[4]! },
  ],
  [
    { kind: "h1", text: "Soup" },
    { kind: "li", text: "lentils", kids: [{ kind: "li", text: "red ones" }] },
    { kind: "li", text: "onion" },
  ],
  [
    { kind: "quote", text: LINES[2]! },
    { kind: "code", text: "water --days tue" },
  ],
];

/* BOT */

const bot: Bot<Docs> = ({ state, me, actors, draw }): Step => {
  const own = Object.keys(state.docs).filter((id) => state.docs[id]!.owner === me);
  const shared = Object.keys(state.docs).filter((id) => state.docs[id]!.editors.includes(me));
  const mine = [...own, ...shared];
  if (!own.length || below(draw, 8) === 0) return { verb: "create", args: { title: pick(draw, TITLES) } };
  const id = pick(draw, mine);
  const doc = state.docs[id]!;
  const others = actors.filter((one) => one !== me && !doc.editors.includes(one));
  if (own.includes(id) && others.length && below(draw, 6) === 0)
    return { verb: "share", args: { doc: id, with: pick(draw, others) } };
  if (own.length > 6 && below(draw, 4) === 0) return { verb: "delete", args: { doc: pick(draw, own) } };
  if (doc.blocks.length && below(draw, 2) === 0) {
    const i = below(draw, doc.blocks.length);
    const kids = doc.blocks[i]!.kids ?? [];
    const path = kids.length && below(draw, 2) === 0 ? [i, below(draw, kids.length)] : [i];
    return { verb: "edit", args: { doc: id, at: path, text: pick(draw, LINES) } };
  }
  return { verb: "write", args: { doc: id, blocks: pick(draw, TREES) } };
};

export default bot;
