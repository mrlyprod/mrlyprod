import type { Ctx, Json, Knob, Knobs, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced, walk } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, notice, someone } from "../shared/people.ts";

export type Section = { heading: string; text: string };

export type Reference = { title: string; by: string };

export type Comment = { by: string; text: string; at: number };

export type Status = "draft" | "published";

export type Paper = {
  owner: string;
  title: string;
  status: Status;
  sections: Section[];
  references: Reference[];
  readers: string[];
  comments: Comment[];
  at: number;
};

export type Papers = { people: string[] | null; listed: number; next: number; papers: Record<string, Paper> };

const ID = 40;

const POINTER: Schema = { type: "string", pattern: "^paper-[0-9]+$", maxLength: ID };

const KNOBS = {
  title: { line: "Characters one title holds.", schema: { type: "integer", minimum: 10, maximum: 1000 }, reality: 200 },
  heading: {
    line: "Characters one heading or author holds.",
    schema: { type: "integer", minimum: 5, maximum: 500 },
    reality: 100,
  },
  text: {
    line: "Characters one section holds.",
    schema: { type: "integer", minimum: 50, maximum: 50000 },
    reality: 2000,
  },
  sections: { line: "Sections one paper holds.", schema: { type: "integer", minimum: 1, maximum: 200 }, reality: 20 },
  references: { line: "Works one paper cites.", schema: { type: "integer", minimum: 0, maximum: 1000 }, reality: 50 },
  own: { line: "Papers one person may hold.", schema: { type: "integer", minimum: 1, maximum: 1000 }, reality: 50 },
  sharers: {
    line: "People one draft may be shared with.",
    schema: { type: "integer", minimum: 1, maximum: 256 },
    reality: 20,
  },
  comment: {
    line: "Characters one comment holds.",
    schema: { type: "integer", minimum: 10, maximum: 5000 },
    reality: 500,
  },
  comments: { line: "Comments one paper keeps.", schema: { type: "integer", minimum: 1, maximum: 1000 }, reality: 100 },
  listed: {
    line: "Newest published papers one overview lists.",
    schema: { type: "integer", minimum: 1, maximum: 500 },
    reality: 20,
  },
} satisfies Knobs;

const can = (paper: Paper, me: string | null) =>
  paper.status === "published" || paper.owner === me || (!!me && paper.readers.includes(me));

const opened = (state: Papers, id: string, ctx: Ctx): Paper => {
  const paper = state.papers[id];
  return paper && can(paper, ctx.who.id)
    ? paper
    : ctx.refuse(`you can see no paper ${id}; read papers for those you can, or ask its author to share it`);
};

const held = (state: Papers, id: string, ctx: Ctx, writing: boolean): Paper => {
  const paper = opened(state, id, ctx);
  if (paper.owner !== ctx.who.id) return ctx.refuse(`${id} is ${paper.owner}'s; only its author may`);
  if (writing && paper.status === "published") return ctx.refuse(`${id} is published and frozen`);
  return paper;
};

const put = (state: Papers, id: string, paper: Paper | null): Papers => {
  const papers = { ...state.papers };
  if (paper) papers[id] = paper;
  else delete papers[id];
  return { ...state, papers };
};

const card = (paper: Paper) => ({
  title: paper.title,
  owner: paper.owner,
  status: paper.status,
  sections: paper.sections.length,
  references: paper.references.length,
  comments: paper.comments.length,
  at: paper.at,
});

const close = (paper: Paper, me: string | null) => !!me && (paper.owner === me || paper.readers.includes(me));

const keyed = <T>(list: [string, Paper][], shown: (paper: Paper) => T) =>
  Object.fromEntries(list.map(([id, paper]) => [id, shown(paper)]));

const split = (state: Papers, me: string | null) => {
  const all = Object.entries(state.papers);
  return {
    near: all.filter(([, paper]) => close(paper, me)),
    far: all
      .filter(([, paper]) => paper.status === "published" && !close(paper, me))
      .sort(([, a], [, b]) => b.at - a.at),
  };
};

const overview = (state: Papers, me: string | null): Json => {
  const { near, far } = split(state, me);
  return { papers: keyed(near, card), published: keyed(far.slice(0, state.listed), card) };
};

const open = ({ readers: _, ...paper }: Paper) => paper;

const everything = (state: Papers, me: string | null) => {
  const { near, far } = split(state, me);
  return { papers: keyed(near, (paper) => paper), published: keyed(far, open) };
};

const deep = sliced<Papers>(
  (state, id) => everything(state, id),
  (state, who) => everything(state, who?.id ?? null),
);

const paperAt = (state: Papers, path: string[], me: string | null): Json | undefined => {
  const paper = Object.hasOwn(state.papers, path[1]!) ? state.papers[path[1]!] : undefined;
  if (!paper || !can(paper, me)) return undefined;
  return walk(close(paper, me) ? paper : open(paper), path.slice(2));
};

const read = (state: Papers, path: string[], who: Who | null, at = 0): Json | undefined => {
  const me = who?.id ?? null;
  if (!path.length) return overview(state, me);
  return path[0] === "papers" && path[1] !== undefined ? paperAt(state, path, me) : deep(state, path, who, at);
};

const shape = (properties: Record<string, Schema>): Schema => ({
  type: "object",
  properties,
  required: Object.keys(properties),
  additionalProperties: false,
});

const person: Schema = { type: "string", minLength: 1, maxLength: ID };

const sections = (knob: Knob): Schema => ({
  type: "array",
  maxItems: knob("sections"),
  items: shape({
    heading: { type: "string", minLength: 1, maxLength: knob("heading") },
    text: { type: "string", maxLength: knob("text") },
  }),
});

const titled = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("title") });

export default app<Papers>({
  id: "papers",
  title: "Papers",
  line: "Papers with sections and references: a private draft, shared with whom its author picks, until published, then frozen and open to all, with comments the author hears of. Read papers/papers/<id> for any you may see, in full.",
  boot: ({ actors, knob }) => ({ people: actors, listed: knob("listed"), next: 1, papers: {} }),
  verbs: {
    create: {
      line: (knob) =>
        `Start a draft of yours, titled 1 to ${knob("title")} characters; you hold at most ${knob("own")}.`,
      args: (knob) => shape({ title: titled(knob) }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const name = String(args.title).trim();
        const most = ctx.knob("own");
        if (!name) return ctx.refuse("a title needs more than spaces");
        if (Object.values(state.papers).filter((one) => one.owner === me).length >= most)
          return ctx.refuse(`you hold ${most} papers already; delete a draft first`);
        const id = `paper-${state.next}`;
        ctx.say({ note: `Started ${id}: ${name}`, id });
        return put({ ...state, next: state.next + 1 }, id, {
          owner: me,
          title: name,
          status: "draft",
          sections: [],
          references: [],
          readers: [],
          comments: [],
          at: ctx.at,
        });
      },
    },
    write: {
      line: (knob) =>
        `Replace a draft's sections: at most ${knob("sections")}, each a heading and a text of ${knob("text")} characters.`,
      args: (knob) => shape({ paper: POINTER, sections: sections(knob) }),
      run: (state, args, ctx) => {
        const id = String(args.paper);
        const was = held(state, id, ctx, true);
        ctx.say({ note: `Wrote ${(args.sections as Section[]).length} sections of ${id}`, id });
        return put(state, id, { ...was, sections: args.sections as Section[], at: ctx.at });
      },
    },
    cite: {
      line: (knob) =>
        `Add a reference to a draft: a title and who wrote it; a paper cites at most ${knob("references")}.`,
      args: (knob) =>
        shape({
          paper: POINTER,
          title: titled(knob),
          by: { type: "string", minLength: 1, maxLength: knob("heading") },
        }),
      run: (state, args, ctx) => {
        const id = String(args.paper);
        const was = held(state, id, ctx, true);
        const most = ctx.knob("references");
        if (was.references.length >= most) return ctx.refuse(`${id} cites ${most} works already`);
        const one = { title: String(args.title), by: String(args.by) };
        if (was.references.some((ref) => ref.title === one.title && ref.by === one.by))
          return ctx.refuse(`${id} cites ${one.title} already`);
        ctx.say({ note: `${id} now cites ${one.title}`, id });
        return put(state, id, { ...was, references: [...was.references, one], at: ctx.at });
      },
    },
    share: {
      line: (knob) =>
        `Let someone read and comment on a draft of yours, and tell them; at most ${knob("sharers")} people.`,
      args: shape({ paper: POINTER, with: person }),
      run: (state, args, ctx) => {
        const id = String(args.paper);
        const me = ctx.who.id;
        const was = held(state, id, ctx, true);
        const who = someone(state.people, String(args.with), ctx);
        const most = ctx.knob("sharers");
        if (who === me) return ctx.refuse("you wrote it; share it with someone else");
        if (blockedBy(ctx, who, me)) return ctx.refuse(`${who} is not taking this from you`);
        if (was.readers.includes(who)) return ctx.refuse(`${who} can read ${id} already`);
        if (was.readers.length >= most) return ctx.refuse(`${id} is shared with ${most} people; unshare someone first`);
        ctx.wish(notice(who, `${me} shared draft ${id} with you: ${was.title}`));
        ctx.say({ note: `Shared ${id} with ${who}`, id, with: who });
        return put(state, id, { ...was, readers: [...was.readers, who] });
      },
    },
    unshare: {
      line: "Stop sharing a draft of yours with someone, or leave one shared with you by naming yourself.",
      args: shape({ paper: POINTER, with: person }),
      run: (state, args, ctx) => {
        const id = String(args.paper);
        const me = ctx.who.id;
        const who = String(args.with);
        const was = opened(state, id, ctx);
        if (was.owner !== me && who !== me)
          return ctx.refuse(`${id} is ${was.owner}'s; you may only leave it yourself`);
        if (!was.readers.includes(who)) return ctx.refuse(`${id} is not shared with ${who}`);
        const told = who === me ? was.owner : who;
        ctx.wish(notice(told, `${me} ${who === me ? "left" : "unshared"} ${id}: ${was.title}`));
        ctx.say({ note: who === me ? `Left ${id}` : `Stopped sharing ${id} with ${who}`, id });
        return put(state, id, { ...was, readers: was.readers.filter((one) => one !== who) });
      },
    },
    publish: {
      line: "Publish a draft of yours with at least one section; a published paper is frozen and open to all, and whoever it was shared with hears.",
      args: () => shape({ paper: POINTER }),
      run: (state, args, ctx) => {
        const id = String(args.paper);
        const me = ctx.who.id;
        const was = held(state, id, ctx, true);
        if (!was.sections.length) return ctx.refuse(`${id} has no section yet; write one first`);
        for (const who of was.readers) ctx.wish(notice(who, `${me} published ${id}: ${was.title}`));
        ctx.say({ note: `Published ${id}; it is frozen and open to all`, id });
        return put(state, id, { ...was, status: "published", at: ctx.at });
      },
    },
    comment: {
      line: (knob) =>
        `Comment on a published paper or a draft shared with you, 1 to ${knob("comment")} characters; everyone who can read it sees it, and its author hears. A paper keeps its last ${knob("comments")}.`,
      args: (knob) => shape({ paper: POINTER, text: { type: "string", minLength: 1, maxLength: knob("comment") } }),
      run: (state, args, ctx) => {
        const id = String(args.paper);
        const me = ctx.who.id;
        const was = opened(state, id, ctx);
        const text = String(args.text).trim();
        if (!text) return ctx.refuse("the comment is only spaces; say something");
        if (was.owner !== me && blockedBy(ctx, was.owner, me))
          return ctx.refuse(`${was.owner} is not taking this from you`);
        if (was.owner !== me) ctx.wish(notice(was.owner, `${me} commented on ${id}: ${text}`));
        const comments = [...was.comments, { by: me, text, at: ctx.at }].slice(-ctx.knob("comments"));
        ctx.say({ note: `Commented on ${id}`, id });
        return put(state, id, { ...was, comments });
      },
    },
    delete: {
      line: "Delete a draft of yours; a published paper stays.",
      args: () => shape({ paper: POINTER }),
      run: (state, args, ctx) => {
        const id = String(args.paper);
        const me = ctx.who.id;
        const was = held(state, id, ctx, true);
        for (const who of was.readers) ctx.wish(notice(who, `${me} deleted draft ${id}: ${was.title}`));
        ctx.say({ note: `Deleted ${id}`, id });
        return put(state, id, null);
      },
    },
  },
  moves: (state) =>
    Object.keys(state.papers).length
      ? ["create", "write", "cite", "share", "unshare", "publish", "comment", "delete"]
      : ["create"],
  read,
  knobs: KNOBS,
});
