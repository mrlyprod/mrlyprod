import type { Block } from "../../sdk/blocks.ts";
import { at, BLOCKS, checked, edited, PATH, TEXT, TREE } from "../../sdk/blocks.ts";
import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Doc = { owner: string; title: string; editors: string[]; blocks: Block[]; at: number };

export type Docs = { people: string[] | null; next: number; docs: Record<string, Doc> };

export const TITLE = 100;

export const DOCS = 50;

const ID = 40;

/* DOCS */

const held = (state: Docs, id: string, ctx: Ctx, own: boolean): Doc => {
  const doc = state.docs[id]!;
  const me = ctx.who.id;
  if (doc.owner === me) return doc;
  if (!own && doc.editors.includes(me)) return doc;
  return ctx.refuse(
    own ? `${id} is ${doc.owner}'s; only its owner may` : `${id} is ${doc.owner}'s; ask to be an editor`,
  );
};

const put = (state: Docs, id: string, doc: Doc): Docs => ({ ...state, docs: { ...state.docs, [id]: doc } });

const mine = (state: Docs, me: string) => ({
  docs: Object.fromEntries(
    Object.entries(state.docs).filter(([, doc]) => doc.owner === me || doc.editors.includes(me)),
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

const doc = (state: Docs): Schema => ({ enum: Object.keys(state.docs) });

const person = (state: Docs): Schema =>
  state.people ? { enum: state.people } : { type: "string", minLength: 1, maxLength: ID };

/* APP */

export default app<Docs>({
  id: "docs",
  title: "Docs",
  line: "Documents as block trees: headings, paragraphs, lists, quotes and code, shared with editors.",
  boot: ({ actors }) => ({ people: actors, next: 1, docs: {} }),
  verbs: {
    create: {
      line: `Start a document of yours, titled 1 to ${TITLE} characters; you hold at most ${DOCS}.`,
      args: shape({ title }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const name = String(args.title).trim();
        if (!name) return ctx.refuse("a title needs more than spaces");
        if (Object.values(state.docs).filter((one) => one.owner === me).length >= DOCS)
          return ctx.refuse(`you hold ${DOCS} documents already; delete one first`);
        const id = `doc-${state.next}`;
        return put({ ...state, next: state.next + 1 }, id, {
          owner: me,
          title: name,
          editors: [],
          blocks: [],
          at: ctx.at,
        });
      },
    },
    write: {
      line: `Replace a document's whole block tree: at most ${BLOCKS} blocks, each text ${TEXT} characters.`,
      args: (state) => shape({ doc: doc(state), blocks: TREE }),
      run: (state, args, ctx) => {
        const id = String(args.doc);
        const was = held(state, id, ctx, false);
        return put(state, id, { ...was, blocks: checked(args.blocks as Block[], ctx.refuse), at: ctx.at });
      },
    },
    edit: {
      line: "Change one block's text, named by its path of indexes.",
      args: (state) => shape({ doc: doc(state), at: PATH, text: { type: "string", maxLength: TEXT } }),
      run: (state, args, ctx) => {
        const id = String(args.doc);
        const was = held(state, id, ctx, false);
        const path = args.at as number[];
        if (!at(was.blocks, path)) return ctx.refuse(`${id} has no block at ${path.join("/")}`);
        return put(state, id, { ...was, blocks: edited(was.blocks, path, String(args.text)), at: ctx.at });
      },
    },
    share: {
      line: "Let someone edit a document of yours.",
      args: (state) => shape({ doc: doc(state), with: person(state) }),
      run: (state, args, ctx) => {
        const id = String(args.doc);
        const was = held(state, id, ctx, true);
        const who = String(args.with);
        if (who === ctx.who.id) return ctx.refuse("you own it already; share with someone else");
        if (was.editors.includes(who)) return ctx.refuse(`${who} edits ${id} already`);
        return put(state, id, { ...was, editors: [...was.editors, who] });
      },
    },
    delete: {
      line: "Delete a document of yours.",
      args: (state) => shape({ doc: doc(state) }),
      run: (state, args, ctx) => {
        const id = String(args.doc);
        held(state, id, ctx, true);
        const { [id]: _, ...rest } = state.docs;
        return { ...state, docs: rest };
      },
    },
  },
  moves: (state) => (Object.keys(state.docs).length ? ["create", "write", "edit", "share", "delete"] : ["create"]),
  read: sliced(mine),
  bot,
});
