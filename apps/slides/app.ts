import type { Ctx, Knob, Knobs, Schema } from "@mrlyprod/os/contract/index.ts";
import { app } from "@mrlyprod/os/contract/index.ts";
import type { Base, Shared } from "../docs/share.ts";
import { SHARING, shape, sharing } from "../docs/share.ts";
import type { Block, Limits } from "../shared/blocks.ts";
import { checked, TREE_KNOBS, tree } from "../shared/blocks.ts";

export type Slide = { title: string; blocks: Block[] };

export type Deck = Shared & { slides: Slide[] };

export type Slides = Base & { decks: Record<string, Deck> };

const KNOBS = {
  ...SHARING,
  ...TREE_KNOBS,
  slides: { line: "Slides one deck holds.", schema: { type: "integer", minimum: 1, maximum: 500 }, reality: 50 },
} satisfies Knobs;

const limits = (knob: Knob): Limits => ({
  blocks: knob("blocks"),
  text: knob("text"),
  depth: knob("depth"),
});

const slideOf = (deck: Deck, id: string, n: number, refuse: Ctx["refuse"]): Slide =>
  deck.slides[n] ?? refuse(`${id} has ${deck.slides.length} slides, numbered from 0; no slide ${n}`);

const put = (state: Slides, id: string, deck: Deck | null): Slides => {
  const decks = { ...state.decks };
  if (deck) decks[id] = deck;
  else delete decks[id];
  return { ...state, decks };
};

const slide = (knob: Knob): Schema => ({ type: "integer", minimum: 0, maximum: knob("slides") - 1 });

const { pointer, writable, changed, verbs, read } = sharing<Slides, Deck>({
  id: "slides",
  noun: "deck",
  arg: "deck",
  prefix: "deck",
  many: "decks",
  items: (state) => state.decks,
  put,
  blank: () => ({ slides: [] }),
  brief: (one) => ({ slides: one.slides.length }),
  full: (_state, _id, one) => ({ slides: one.slides }),
});

export default app<Slides>({
  id: "slides",
  title: "Slides",
  line: "Decks of slides, each a title over a block tree: private until shared with editors or readers, or made public. Read slides/decks/<id> for one in full.",
  boot: ({ actors, knob }) => ({ people: actors, next: 1, listed: knob("listed"), decks: {} }),
  verbs: {
    ...verbs,
    add: {
      line: (knob) =>
        `Add a slide at the end of a deck, titled 1 to ${knob("title")} characters; a deck holds at most ${knob("slides")}.`,
      args: (knob) => shape({ deck: pointer, title: { type: "string", minLength: 1, maxLength: knob("title") } }),
      run: (state, args, ctx) => {
        const id = String(args.deck);
        const was = writable(state, id, ctx);
        const most = ctx.knob("slides");
        if (was.slides.length >= most) return ctx.refuse(`${id} holds ${most} slides already; drop one first`);
        const slides = [...was.slides, { title: String(args.title), blocks: [] }];
        ctx.say({ note: `Added slide ${slides.length - 1} to ${id}`, id, slide: slides.length - 1 });
        return put(state, id, { ...changed(id, was, ctx, "added a slide to"), slides });
      },
    },
    write: {
      line: (knob) => `Replace one slide's block tree, the slide numbered from 0: at most ${knob("blocks")} blocks.`,
      args: (knob) => shape({ deck: pointer, slide: slide(knob), blocks: tree(limits(knob)) }),
      run: (state, args, ctx) => {
        const id = String(args.deck);
        const was = writable(state, id, ctx);
        const n = Number(args.slide);
        const one = slideOf(was, id, n, ctx.refuse);
        const blocks = checked(args.blocks as Block[], ctx.knob("blocks"), ctx.refuse);
        const slides = was.slides.map((each, i) => (i === n ? { ...one, blocks } : each));
        ctx.say({ note: `Wrote slide ${n} of ${id}`, id, slide: n });
        return put(state, id, { ...changed(id, was, ctx, "changed"), slides });
      },
    },
    drop: {
      line: "Drop one slide from a deck, numbered from 0.",
      args: (knob) => shape({ deck: pointer, slide: slide(knob) }),
      run: (state, args, ctx) => {
        const id = String(args.deck);
        const was = writable(state, id, ctx);
        const n = Number(args.slide);
        slideOf(was, id, n, ctx.refuse);
        const slides = was.slides.filter((_, i) => i !== n);
        ctx.say({ note: `Dropped slide ${n} of ${id}; ${slides.length} left, numbered from 0`, id, slide: n });
        return put(state, id, { ...changed(id, was, ctx, "dropped a slide from"), slides });
      },
    },
  },
  moves: (state) =>
    Object.keys(state.decks).length
      ? ["create", "rename", "add", "write", "drop", "share", "unshare", "publish", "delete"]
      : ["create"],
  read,
  knobs: KNOBS,
});
