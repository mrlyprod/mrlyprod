import type { Doc, Docs } from "@mrlyprod/apps/docs/app.ts";
import type { Block } from "@mrlyprod/apps/shared/blocks.ts";
import type { Obj } from "@mrlyprod/os/contract/index.ts";
import { below, pick } from "@mrlyprod/os/contract/index.ts";
import type { Bot, Step } from "./types.ts";

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

const owning = (doc: Doc, id: string, draw: () => number, others: string[]): Step => {
  const fresh = others.filter((one) => !doc.editors.includes(one) && !doc.readers.includes(one));
  const sharers = [...doc.editors, ...doc.readers];
  const turn = below(draw, 12);
  if (turn === 0 && fresh.length)
    return { verb: "share", args: { doc: id, with: pick(draw, fresh), role: pick(draw, ["editor", "reader"]) } };
  if (turn === 1 && sharers.length > 1) return { verb: "unshare", args: { doc: id, with: pick(draw, sharers) } };
  if (turn === 2 && doc.blocks.length) return { verb: "publish", args: { doc: id, on: !doc.open } };
  return null;
};

const changing = (doc: Doc, id: string, draw: () => number): Step => {
  const size = doc.blocks.length;
  const turn = below(draw, 10);
  const text = pick(draw, LINES);
  if (doc.history.length && turn === 0)
    return { verb: "restore", args: { doc: id, revision: pick(draw, doc.history).n } };
  if (turn === 1) return { verb: "rename", args: { doc: id, title: pick(draw, TITLES) } };
  if (size > 1 && turn === 2) return { verb: "cut", args: { doc: id, at: [below(draw, size)] } };
  if (size && turn < 5) {
    const near = pick(draw, doc.blocks).id;
    const place: Obj = below(draw, 4) === 0 ? { after: 0 } : below(draw, 4) === 0 ? { under: near } : { after: near };
    return { verb: "insert", args: { doc: id, ...place, text } };
  }
  if (size && turn < 7) {
    const i = below(draw, size);
    const kids = doc.blocks[i]!.kids ?? [];
    const at = kids.length && below(draw, 2) === 0 ? [i, below(draw, kids.length)] : [i];
    return { verb: "edit", args: { doc: id, at, text } };
  }
  if (size && turn < 9) return { verb: "append", args: { doc: id, text, kind: pick(draw, ["p", "li", "quote"]) } };
  return { verb: "write", args: { doc: id, blocks: pick(draw, TREES) } };
};

const bot: Bot<Docs> = ({ state, me, actors, draw }): Step => {
  const ids = Object.keys(state.docs);
  const own = ids.filter((id) => state.docs[id]!.owner === me);
  const editing = ids.filter((id) => state.docs[id]!.editors.includes(me));
  const reading = ids.filter((id) => state.docs[id]!.readers.includes(me));
  const owners = Object.values(state.docs).map((doc) => doc.owner);
  const others = [...new Set([...actors, ...owners])].filter((one) => one !== me);
  if (!own.length || below(draw, 8) === 0) return { verb: "create", args: { title: pick(draw, TITLES) } };
  if (own.length > 6 && below(draw, 4) === 0) return { verb: "delete", args: { doc: pick(draw, own) } };
  if (reading.length && below(draw, 12) === 0) return { verb: "unshare", args: { doc: pick(draw, reading), with: me } };
  const id = pick(draw, [...own, ...editing]);
  const doc = state.docs[id]!;
  return (own.includes(id) ? owning(doc, id, draw, others) : null) ?? changing(doc, id, draw);
};

export default bot;
