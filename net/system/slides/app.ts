import type { Block } from "../../sdk/blocks.ts";
import { BLOCKS, checked, TREE } from "../../sdk/blocks.ts";
import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Slide = { title: string; blocks: Block[] };

export type Deck = { owner: string; title: string; editors: string[]; slides: Slide[]; at: number };

export type Slides = { people: string[] | null; next: number; decks: Record<string, Deck> };

export const TITLE = 100;

export const DECKS = 50;

export const SLIDES = 50;

const ID = 40;

/* DECKS */

const held = (state: Slides, id: string, ctx: Ctx, own: boolean): Deck => {
  const deck = state.decks[id]!;
  const me = ctx.who.id;
  if (deck.owner === me) return deck;
  if (!own && deck.editors.includes(me)) return deck;
  return ctx.refuse(
    own ? `${id} is ${deck.owner}'s; only its owner may` : `${id} is ${deck.owner}'s; ask to be an editor`,
  );
};

const slideOf = (deck: Deck, id: string, n: number, refuse: Ctx["refuse"]): Slide =>
  deck.slides[n] ?? refuse(`${id} has ${deck.slides.length} slides, numbered from 0; no slide ${n}`);

const put = (state: Slides, id: string, deck: Deck): Slides => ({ ...state, decks: { ...state.decks, [id]: deck } });

const mine = (state: Slides, me: string) => ({
  decks: Object.fromEntries(
    Object.entries(state.decks).filter(([, deck]) => deck.owner === me || deck.editors.includes(me)),
  ),
});

/* SCHEMAS */

const shape = (properties: Record<string, Schema>): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties),
  additionalProperties: false,
});

const title: Schema = { type: "string", minLength: 1, maxLength: TITLE };

const deck = (state: Slides): Schema => ({ enum: Object.keys(state.decks) });

const slide: Schema = { type: "integer", minimum: 0, maximum: SLIDES - 1 };

const person = (state: Slides): Schema =>
  state.people ? { enum: state.people } : { type: "string", minLength: 1, maxLength: ID };

/* APP */

export default app<Slides>({
  id: "slides",
  title: "Slides",
  line: "Decks of slides, each a title over a block tree, shared with editors.",
  boot: ({ actors }) => ({ people: actors, next: 1, decks: {} }),
  verbs: {
    create: {
      line: `Start a deck of yours, titled 1 to ${TITLE} characters; you hold at most ${DECKS}.`,
      args: shape({ title }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const name = String(args.title).trim();
        if (!name) return ctx.refuse("a title needs more than spaces");
        if (Object.values(state.decks).filter((one) => one.owner === me).length >= DECKS)
          return ctx.refuse(`you hold ${DECKS} decks already; delete one first`);
        const id = `deck-${state.next}`;
        return put({ ...state, next: state.next + 1 }, id, {
          owner: me,
          title: name,
          editors: [],
          slides: [],
          at: ctx.at,
        });
      },
    },
    add: {
      line: `Add a slide at the end of a deck, titled 1 to ${TITLE} characters; a deck holds at most ${SLIDES}.`,
      args: (state) => shape({ deck: deck(state), title }),
      run: (state, args, ctx) => {
        const id = String(args.deck);
        const was = held(state, id, ctx, false);
        if (was.slides.length >= SLIDES) return ctx.refuse(`${id} holds ${SLIDES} slides already; drop one first`);
        return put(state, id, {
          ...was,
          slides: [...was.slides, { title: String(args.title), blocks: [] }],
          at: ctx.at,
        });
      },
    },
    write: {
      line: `Replace one slide's block tree, the slide numbered from 0: at most ${BLOCKS} blocks.`,
      args: (state) => shape({ deck: deck(state), slide, blocks: TREE }),
      run: (state, args, ctx) => {
        const id = String(args.deck);
        const was = held(state, id, ctx, false);
        const n = Number(args.slide);
        const one = slideOf(was, id, n, ctx.refuse);
        const blocks = checked(args.blocks as Block[], ctx.refuse);
        const slides = was.slides.map((each, i) => (i === n ? { ...one, blocks } : each));
        return put(state, id, { ...was, slides, at: ctx.at });
      },
    },
    drop: {
      line: "Drop one slide from a deck, numbered from 0.",
      args: (state) => shape({ deck: deck(state), slide }),
      run: (state, args, ctx) => {
        const id = String(args.deck);
        const was = held(state, id, ctx, false);
        const n = Number(args.slide);
        slideOf(was, id, n, ctx.refuse);
        return put(state, id, { ...was, slides: was.slides.filter((_, i) => i !== n), at: ctx.at });
      },
    },
    share: {
      line: "Let someone edit a deck of yours.",
      args: (state) => shape({ deck: deck(state), with: person(state) }),
      run: (state, args, ctx) => {
        const id = String(args.deck);
        const was = held(state, id, ctx, true);
        const who = String(args.with);
        if (who === ctx.who.id) return ctx.refuse("you own it already; share with someone else");
        if (was.editors.includes(who)) return ctx.refuse(`${who} edits ${id} already`);
        return put(state, id, { ...was, editors: [...was.editors, who] });
      },
    },
    delete: {
      line: "Delete a deck of yours.",
      args: (state) => shape({ deck: deck(state) }),
      run: (state, args, ctx) => {
        const id = String(args.deck);
        held(state, id, ctx, true);
        const { [id]: _, ...rest } = state.decks;
        return { ...state, decks: rest };
      },
    },
  },
  moves: (state) =>
    Object.keys(state.decks).length ? ["create", "add", "write", "drop", "share", "delete"] : ["create"],
  read: sliced(mine),
  bot,
});
