import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Post = { n: number; by: string; source: string; headline: string; text: string; at: number };

export type News = {
  people: string[] | null;
  n: number;
  posts: Post[];
  sources: Record<string, string>;
  following: Record<string, string[]>;
};

type Org = { n: number; name: string; dissolved: number | null; members: Record<string, string> };

export const HEADLINE = 120;

export const TEXT = 2000;

export const KEEP = 200;

export const FEED = 30;

export const FOLLOW = 50;

const ID = 40;

/* SOURCES */

export const orgSource = (n: number) => `org:${n}`;

function orgOf(ctx: Ctx, n: number): Org {
  if (ctx.read("people") === null) return ctx.refuse("this world has no people, so no orgs; post as yourself");
  const one = ctx.read(`people/orgs/${n}`) as Org | null;
  if (!one) return ctx.refuse(`no org ${n} in people`);
  if (one.dissolved !== null) return ctx.refuse(`${one.name} was dissolved`);
  if (!(ctx.who.id in one.members)) return ctx.refuse(`only a member of ${one.name} posts as it`);
  return one;
}

const mine = (state: News, id: string) => {
  const following = state.following[id] ?? [];
  return {
    following: following.map((source) => ({ source, name: state.sources[source] ?? source })),
    feed: state.posts
      .filter((post) => post.source === id || following.includes(post.source))
      .slice(-FEED)
      .reverse(),
  };
};

/* SCHEMAS */

const posting: Schema = {
  type: "object",
  properties: {
    headline: { type: "string", minLength: 1, maxLength: HEADLINE },
    text: { type: "string", maxLength: TEXT },
    org: { type: "integer", minimum: 1 },
  },
  required: ["headline"],
  additionalProperties: false,
};

const naming = (state: News): Schema => ({
  type: "object",
  properties: {
    source: state.people
      ? { enum: [...state.people, ...Object.keys(state.sources)] }
      : { type: "string", minLength: 1, maxLength: ID },
  },
  required: ["source"],
  additionalProperties: false,
});

/* APP */

export default app<News>({
  id: "news",
  title: "News",
  line: "Posts with headlines by people or their orgs; follow a source and read your feed.",
  boot: ({ actors }) => ({ people: actors, n: 0, posts: [], sources: {}, following: {} }),
  verbs: {
    post: {
      line: `Post a headline of 1 to ${HEADLINE} characters with a text of up to ${TEXT}, as yourself or as an org of people you belong to.`,
      args: posting,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const headline = String(args.headline).trim();
        if (!headline) return ctx.refuse("the headline is only spaces; say what happened");
        const org = args.org === undefined ? null : orgOf(ctx, Number(args.org));
        const source = org ? orgSource(org.n) : me;
        const n = state.n + 1;
        const post: Post = { n, by: me, source, headline, text: String(args.text ?? ""), at: ctx.at };
        return {
          ...state,
          n,
          posts: [...state.posts, post].slice(-KEEP),
          sources: org ? { ...state.sources, [source]: org.name } : state.sources,
        };
      },
    },
    follow: {
      line: `Follow a person or an org that has posted; you follow at most ${FOLLOW}.`,
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const source = String(args.source);
        const following = state.following[me] ?? [];
        if (source === me) return ctx.refuse("your own posts are in your feed already");
        if (following.includes(source)) return ctx.refuse(`you follow ${source} already`);
        if (following.length >= FOLLOW) return ctx.refuse(`you follow ${FOLLOW} sources; unfollow one first`);
        return { ...state, following: { ...state.following, [me]: [...following, source] } };
      },
    },
    unfollow: {
      line: "Stop following a source.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const source = String(args.source);
        const following = state.following[me] ?? [];
        if (!following.includes(source)) return ctx.refuse(`you do not follow ${source}`);
        return { ...state, following: { ...state.following, [me]: following.filter((one) => one !== source) } };
      },
    },
  },
  read: sliced(mine),
  bot,
});
