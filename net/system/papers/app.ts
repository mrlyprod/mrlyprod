import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Section = { heading: string; text: string };

export type Reference = { title: string; by: string };

export type Status = "draft" | "published";

export type Paper = {
  owner: string;
  title: string;
  status: Status;
  sections: Section[];
  references: Reference[];
  at: number;
};

export type Papers = { next: number; papers: Record<string, Paper> };

export const TITLE = 200;

export const HEADING = 100;

export const TEXT = 2000;

export const SECTIONS = 20;

export const REFERENCES = 50;

export const PAPERS = 50;

/* PAPERS */

const held = (state: Papers, id: string, ctx: Ctx, writing: boolean): Paper => {
  const paper = state.papers[id]!;
  if (paper.owner !== ctx.who.id) return ctx.refuse(`${id} is ${paper.owner}'s; only its author may`);
  if (writing && paper.status === "published") return ctx.refuse(`${id} is published and frozen`);
  return paper;
};

const put = (state: Papers, id: string, paper: Paper): Papers => ({
  ...state,
  papers: { ...state.papers, [id]: paper },
});

const mine = (state: Papers, me: string) => ({
  papers: Object.fromEntries(Object.entries(state.papers).filter(([, paper]) => paper.owner === me)),
});

/* SCHEMAS */

const shape = (properties: Record<string, Schema>): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties),
  additionalProperties: false,
});

const paper = (state: Papers): Schema => ({ enum: Object.keys(state.papers) });

const sections: Schema = {
  type: "array",
  maxItems: SECTIONS,
  items: shape({
    heading: { type: "string", minLength: 1, maxLength: HEADING },
    text: { type: "string", maxLength: TEXT },
  }),
};

/* APP */

export default app<Papers>({
  id: "papers",
  title: "Papers",
  line: "Papers with sections and references, a draft until its author publishes it, then frozen.",
  boot: () => ({ next: 1, papers: {} }),
  verbs: {
    create: {
      line: `Start a draft of yours, titled 1 to ${TITLE} characters; you hold at most ${PAPERS}.`,
      args: shape({ title: { type: "string", minLength: 1, maxLength: TITLE } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const name = String(args.title).trim();
        if (!name) return ctx.refuse("a title needs more than spaces");
        if (Object.values(state.papers).filter((one) => one.owner === me).length >= PAPERS)
          return ctx.refuse(`you hold ${PAPERS} papers already`);
        const id = `paper-${state.next}`;
        return put({ ...state, next: state.next + 1 }, id, {
          owner: me,
          title: name,
          status: "draft",
          sections: [],
          references: [],
          at: ctx.at,
        });
      },
    },
    write: {
      line: `Replace a draft's sections: at most ${SECTIONS}, each a heading and a text of ${TEXT} characters.`,
      args: (state) => shape({ paper: paper(state), sections }),
      run: (state, args, ctx) => {
        const id = String(args.paper);
        const was = held(state, id, ctx, true);
        return put(state, id, { ...was, sections: args.sections as Section[], at: ctx.at });
      },
    },
    cite: {
      line: `Add a reference to a draft: a title and who wrote it; a paper cites at most ${REFERENCES}.`,
      args: (state) =>
        shape({
          paper: paper(state),
          title: { type: "string", minLength: 1, maxLength: TITLE },
          by: { type: "string", minLength: 1, maxLength: HEADING },
        }),
      run: (state, args, ctx) => {
        const id = String(args.paper);
        const was = held(state, id, ctx, true);
        if (was.references.length >= REFERENCES) return ctx.refuse(`${id} cites ${REFERENCES} works already`);
        const one = { title: String(args.title), by: String(args.by) };
        if (was.references.some((ref) => ref.title === one.title && ref.by === one.by))
          return ctx.refuse(`${id} cites ${one.title} already`);
        return put(state, id, { ...was, references: [...was.references, one], at: ctx.at });
      },
    },
    publish: {
      line: "Publish a draft of yours with at least one section; a published paper is frozen.",
      args: (state) => shape({ paper: paper(state) }),
      run: (state, args, ctx) => {
        const id = String(args.paper);
        const was = held(state, id, ctx, true);
        if (!was.sections.length) return ctx.refuse(`${id} has no section yet; write one first`);
        return put(state, id, { ...was, status: "published", at: ctx.at });
      },
    },
  },
  moves: (state) => (Object.keys(state.papers).length ? ["create", "write", "cite", "publish"] : ["create"]),
  read: sliced(mine),
  bot,
});
