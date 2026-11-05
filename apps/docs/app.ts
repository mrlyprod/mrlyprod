import type { Ctx, Knob, Knobs, Obj, Schema } from "@mrlyprod/os/contract/index.ts";
import { app } from "@mrlyprod/os/contract/index.ts";
import type { Block, Limits } from "../shared/blocks.ts";
import { checked, count, cut, edited, inserted, KIND, TREE_KNOBS, tree } from "../shared/blocks.ts";
import type { Base, Shared } from "./share.ts";
import { member, roleOf, SHARING, shape, sharing } from "./share.ts";

export type Held = Block & { id: number; kids?: Held[] };

export type Revision = { n: number; by: string; at: number; blocks: Held[] };

export type Doc = Shared & { blocks: Held[]; serial: number; revision: number; history: Revision[] };

export type Docs = Base & { docs: Record<string, Doc> };

const KNOBS = {
  ...SHARING,
  ...TREE_KNOBS,
  revisions: {
    line: "Past revisions one document keeps.",
    schema: { type: "integer", minimum: 0, maximum: 200 },
    reality: 20,
  },
} satisfies Knobs;

const limits = (knob: Knob): Limits => ({
  blocks: knob("blocks"),
  text: knob("text"),
  depth: knob("depth"),
});

const put = (state: Docs, id: string, doc: Doc | null): Docs => {
  const docs = { ...state.docs };
  if (doc) docs[id] = doc;
  else delete docs[id];
  return { ...state, docs };
};

const past = (id: string, one: Revision) => ({
  n: one.n,
  by: one.by,
  at: one.at,
  blocks: count(one.blocks),
  path: `docs/revisions/${id}/${one.n}`,
});

const { pointer, writable, changed, verbs, read } = sharing<Docs, Doc>({
  id: "docs",
  noun: "document",
  arg: "doc",
  prefix: "doc",
  many: "docs",
  items: (state) => state.docs,
  put,
  blank: () => ({ blocks: [], serial: 1, revision: 0, history: [] }),
  brief: (one) => ({ blocks: count(one.blocks), revision: one.revision }),
  full: (_state, id, one) => ({
    blocks: one.blocks,
    revision: one.revision,
    history: one.history.map((each) => past(id, each)),
  }),
  more: (state, me) => ({
    revisions: Object.fromEntries(
      Object.entries(state.docs)
        .filter(([, one]) => member(roleOf(one, me)))
        .map(([id, one]) => [id, Object.fromEntries(one.history.map((each) => [each.n, each.blocks]))]),
    ),
  }),
});

const numbered = (blocks: Block[], from: number): [Held[], number] => {
  let serial = from;
  const go = (list: Block[]): Held[] =>
    list.map((one) => {
      const id = serial++;
      return (one.kids ? { ...one, id, kids: go(one.kids) } : { ...one, id }) as Held;
    });
  return [go(blocks), serial];
};

const located = (blocks: Held[], id: number, trail: number[] = []): number[] | null => {
  for (const [i, one] of blocks.entries()) {
    if (one.id === id) return [...trail, i];
    const deeper = located(one.kids ?? [], id, [...trail, i]);
    if (deeper) return deeper;
  }
  return null;
};

const spot = (blocks: Held[], args: Obj): number[] | null => {
  if (args.after === 0) return [0];
  const found = located(blocks, Number(args.after ?? args.under));
  if (!found) return null;
  return args.under === undefined ? [...found.slice(0, -1), found.at(-1)! + 1] : [...found, 0];
};

const reading = (schema: Schema): Schema => {
  const item = schema.items as Schema;
  const kids = item.properties?.kids;
  return {
    ...schema,
    items: {
      ...item,
      properties: { ...item.properties, id: { type: "integer" }, ...(kids ? { kids: reading(kids) } : {}) },
    },
  };
};

const revised = (id: string, was: Doc, blocks: Held[], ctx: Ctx, what: string, serial = was.serial): Doc => {
  const keep = ctx.knob("revisions");
  const old: Revision = { n: was.revision, by: was.by, at: was.at, blocks: was.blocks };
  return {
    ...changed(id, was, ctx, what),
    blocks,
    serial,
    revision: was.revision + 1,
    history: keep ? [...was.history, old].slice(-keep) : [],
  };
};

const fresh = (args: Obj, ctx: Ctx): Block => {
  const text = String(args.text);
  if (!text.trim()) return ctx.refuse("the text is only spaces; say something");
  return { kind: (args.kind ?? "p") as Block["kind"], text };
};

const BLOCK: Schema = { type: "integer", minimum: 1 };

const said = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("text") });

export default app<Docs>({
  id: "docs",
  title: "Docs",
  line: "Documents as block trees: private until shared with editors or readers, or made public; every change keeps a revision. Read docs/docs/<id> for one in full.",
  boot: ({ actors, knob }) => ({ people: actors, next: 1, listed: knob("listed"), docs: {} }),
  verbs: {
    ...verbs,
    write: {
      line: (knob) =>
        `Replace a document's whole block tree, losing nothing to its history: at most ${knob("blocks")} blocks, each text ${knob("text")} characters. To add to it, use append or insert.`,
      args: (knob) => shape({ doc: pointer, blocks: reading(tree(limits(knob))) }),
      run: (state, args, ctx) => {
        const id = String(args.doc);
        const was = writable(state, id, ctx);
        const given = checked(args.blocks as Block[], ctx.knob("blocks"), ctx.refuse);
        const [blocks, serial] = numbered(given, was.serial);
        ctx.say({ note: `Rewrote ${id}, now revision ${was.revision + 1}; its blocks have new ids`, id });
        return put(state, id, revised(id, was, blocks, ctx, "rewrote", serial));
      },
    },
    edit: {
      line: "Replace one block's text, named by its id as docs/docs/<id> shows it; what was there is overwritten. Ids hold still while others edit. To add a block, use append or insert.",
      args: (knob) => shape({ doc: pointer, block: BLOCK, text: { type: "string", maxLength: knob("text") } }),
      run: (state, args, ctx) => {
        const id = String(args.doc);
        const was = writable(state, id, ctx);
        const path = located(was.blocks, Number(args.block));
        if (!path) return ctx.refuse(`${id} has no block ${args.block}; read docs/docs/${id}`);
        ctx.say({ note: `Edited block ${args.block} of ${id}`, id });
        const blocks = edited(was.blocks, path, String(args.text)) as Held[];
        return put(state, id, revised(id, was, blocks, ctx, "edited"));
      },
    },
    append: {
      line: "Add one block after the last, keeping everything already there; kind is p unless you say otherwise.",
      args: (knob) => shape({ doc: pointer, text: said(knob), kind: KIND }, ["kind"]),
      run: (state, args, ctx) => {
        const id = String(args.doc);
        const was = writable(state, id, ctx);
        const block = was.serial;
        const one = { ...fresh(args, ctx), id: block };
        const blocks = checked([...was.blocks, one], ctx.knob("blocks"), ctx.refuse) as Held[];
        ctx.say({ note: `Added block ${block} to ${id}`, id, block });
        return put(state, id, revised(id, was, blocks, ctx, "added to", block + 1));
      },
    },
    insert: {
      line: "Add one block right after the block with an id, or first under one with under; after 0 puts it first in the document. Ids hold still while others edit: read docs/docs/<id>. Kind is p unless you say otherwise.",
      args: (knob) =>
        shape(
          {
            doc: pointer,
            after: { type: "integer", minimum: 0 },
            under: { type: "integer", minimum: 1 },
            text: said(knob),
            kind: KIND,
          },
          ["after", "under", "kind"],
        ),
      run: (state, args, ctx) => {
        const id = String(args.doc);
        const was = writable(state, id, ctx);
        if ((args.after === undefined) === (args.under === undefined))
          return ctx.refuse("say one place: after a block's id, or under a block's id");
        const path = spot(was.blocks, args);
        if (!path) return ctx.refuse(`${id} has no block ${args.after ?? args.under}; read docs/docs/${id}`);
        const deepest = ctx.knob("depth");
        if (path.length > deepest) return ctx.refuse(`blocks nest at most ${deepest} levels; add it higher up`);
        const block = was.serial;
        const one = { ...fresh(args, ctx), id: block };
        const blocks = checked(inserted(was.blocks, path, one), ctx.knob("blocks"), ctx.refuse) as Held[];
        ctx.say({ note: `Added block ${block} to ${id}`, id, block });
        return put(state, id, revised(id, was, blocks, ctx, "added to", block + 1));
      },
    },
    cut: {
      line: "Remove one block and all under it, named by its id as docs/docs/<id> shows it.",
      args: () => shape({ doc: pointer, block: BLOCK }),
      run: (state, args, ctx) => {
        const id = String(args.doc);
        const was = writable(state, id, ctx);
        const path = located(was.blocks, Number(args.block));
        if (!path) return ctx.refuse(`${id} has no block ${args.block}; read docs/docs/${id}`);
        ctx.say({ note: `Cut block ${args.block} from ${id}`, id });
        return put(state, id, revised(id, was, cut(was.blocks, path) as Held[], ctx, "cut from"));
      },
    },
    restore: {
      line: (knob) =>
        `Put a document back as it was at a past revision; the present joins its history. It keeps the last ${knob("revisions")}: see docs/docs/<id>/history.`,
      args: () => shape({ doc: pointer, revision: { type: "integer", minimum: 0 } }),
      run: (state, args, ctx) => {
        const id = String(args.doc);
        const was = writable(state, id, ctx);
        const n = Number(args.revision);
        const old = was.history.find((one) => one.n === n);
        if (!old)
          return ctx.refuse(
            `${id} keeps ${was.history.map((one) => one.n).join(", ") || "no past revision"}; it is at ${was.revision}`,
          );
        ctx.say({ note: `Restored ${id} to revision ${n}, now revision ${was.revision + 1}`, id });
        return put(state, id, revised(id, was, old.blocks, ctx, `restored revision ${n} of`));
      },
    },
  },
  moves: (state) =>
    Object.keys(state.docs).length
      ? [
          "create",
          "rename",
          "write",
          "edit",
          "append",
          "insert",
          "cut",
          "restore",
          "share",
          "unshare",
          "publish",
          "delete",
        ]
      : ["create"],
  read,
  knobs: KNOBS,
});
