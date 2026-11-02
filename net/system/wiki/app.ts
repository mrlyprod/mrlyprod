import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, below, sliced, stream } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Revision = { n: number; by: string | null; at: number; text: string };

export type Talk = { n: number; by: string; at: number; text: string };

export type Article = { slug: string; title: string; text: string; n: number; revisions: Revision[]; talk: Talk[] };

export type Wiki = { articles: Record<string, Article>; watching: Record<string, string[]> };

export const TITLE = 80;

export const TEXT = 4000;

export const TALK = 500;

export const SLUG = 40;

export const REVISIONS = 10;

export const TALKS = 50;

export const WATCH = 30;

export const STUBS = 5;

/* STUBS */

export const STUBBED: [string, string][] = [
  [
    "Hexagon",
    "A hexagon has six sides. The map of this world is a grid of them, each cell at two integer coordinates.",
  ],
  ["Water", "Water falls as rain, fills the cells marked water on the map, and is what everyone drinks."],
  ["Bread", "Bread is flour, water, salt and time. A bakery is often the first company a world founds."],
  ["Calendar", "A world day is 86,400 seconds. The calendar counts days from the world's first act."],
  [
    "Weather",
    "Weather follows from the world's seed, one sky per cell per day: clear, cloudy, rain, storm, snow or fog.",
  ],
  ["Music", "Music here is metadata: a title, an author and a length. The sound is left to the listener."],
  ["Books", "A book is written chapter by chapter by a person of this world. A library is a shelf of them."],
  ["Maps", "The map is a hexagonal grid. People pin places on it and give them names."],
  ["Markets", "A market is where prices move. The stock board lists companies and moves their prices every tick."],
  ["Letters", "Letters travel through mail and chat. A thread keeps its last fifty messages."],
  ["Names", "Every person has a name, and may change it to any name nobody else holds."],
  ["Seeds", "A seed is one number. Everything the world generates follows from it, so a world can be replayed."],
];

/* ARTICLES */

export const slugOf = (title: string) =>
  title
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .slice(0, SLUG)
    .replace(/-+$/, "");

const article = (title: string, text: string, by: string | null, at: number): Article => ({
  slug: slugOf(title),
  title,
  text,
  n: 1,
  revisions: [{ n: 1, by, at, text }],
  talk: [],
});

const put = (state: Wiki, one: Article): Wiki => ({ ...state, articles: { ...state.articles, [one.slug]: one } });

const articleOf = (state: Wiki, slug: string, refuse: Ctx["refuse"]) =>
  state.articles[slug] ?? refuse(`no article ${slug}; write it`);

function seeded(seed: number): Record<string, Article> {
  const { draw } = stream(seed);
  const pool = [...STUBBED];
  const out: Record<string, Article> = {};
  for (let i = 0; i < STUBS && pool.length; i++) {
    const [title, text] = pool.splice(below(draw, pool.length), 1)[0]!;
    const one = article(title, text, null, 0);
    out[one.slug] = one;
  }
  return out;
}

const mine = (state: Wiki, id: string) => ({
  watching: (state.watching[id] ?? []).map((slug) => ({
    slug,
    title: state.articles[slug]?.title ?? slug,
    n: state.articles[slug]?.n ?? 0,
  })),
  edited: Object.values(state.articles)
    .filter((one) => one.revisions.some((rev) => rev.by === id))
    .map((one) => one.slug),
});

/* SCHEMAS */

const writing: Schema = {
  type: "object",
  properties: {
    title: { type: "string", minLength: 1, maxLength: TITLE },
    text: { type: "string", minLength: 1, maxLength: TEXT },
  },
  required: ["title", "text"],
  additionalProperties: false,
};

const naming = (state: Wiki, more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { slug: { enum: Object.keys(state.articles) }, ...more },
  required: ["slug", ...Object.keys(more)],
  additionalProperties: false,
});

/* APP */

export default app<Wiki>({
  id: "wiki",
  title: "Wiki",
  line: "One encyclopedia written inside the world: articles, their revisions and a talk page each.",
  boot: ({ seed }) => ({ articles: seeded(seed), watching: {} }),
  verbs: {
    write: {
      line: `Write a new article: a title of 1 to ${TITLE} characters, a text of up to ${TEXT}; its slug follows from the title.`,
      args: writing,
      run: (state, args, ctx) => {
        const title = String(args.title).trim();
        const text = String(args.text);
        const slug = slugOf(title);
        if (!slug) return ctx.refuse("the title needs a letter or a digit");
        if (!text.trim()) return ctx.refuse("the text is only spaces; write something");
        if (state.articles[slug]) return ctx.refuse(`there is an article ${slug}; edit it`);
        return put(state, article(title, text, ctx.who.id, ctx.at));
      },
    },
    edit: {
      line: `Replace an article's text; it keeps its last ${REVISIONS} revisions.`,
      args: (state) => naming(state, { text: { type: "string", minLength: 1, maxLength: TEXT } }),
      run: (state, args, ctx) => {
        const one = articleOf(state, String(args.slug), ctx.refuse);
        const text = String(args.text);
        if (!text.trim()) return ctx.refuse("the text is only spaces; write something");
        if (text === one.text) return ctx.refuse(`${one.slug} already reads so; change something`);
        const n = one.n + 1;
        const revision: Revision = { n, by: ctx.who.id, at: ctx.at, text };
        return put(state, { ...one, text, n, revisions: [...one.revisions, revision].slice(-REVISIONS) });
      },
    },
    talk: {
      line: `Post on an article's talk page: 1 to ${TALK} characters; it keeps its last ${TALKS} posts.`,
      args: (state) => naming(state, { text: { type: "string", minLength: 1, maxLength: TALK } }),
      run: (state, args, ctx) => {
        const one = articleOf(state, String(args.slug), ctx.refuse);
        const text = String(args.text);
        if (!text.trim()) return ctx.refuse("the post is only spaces; say something");
        const n = (one.talk.at(-1)?.n ?? 0) + 1;
        return put(state, { ...one, talk: [...one.talk, { n, by: ctx.who.id, at: ctx.at, text }].slice(-TALKS) });
      },
    },
    watch: {
      line: `Watch an article; you watch at most ${WATCH}.`,
      args: (state) => naming(state),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const slug = String(args.slug);
        const watching = state.watching[me] ?? [];
        if (watching.includes(slug)) return ctx.refuse(`you watch ${slug} already`);
        if (watching.length >= WATCH) return ctx.refuse(`you watch ${WATCH} articles; unwatch one first`);
        return { ...state, watching: { ...state.watching, [me]: [...watching, slug] } };
      },
    },
    unwatch: {
      line: "Stop watching an article.",
      args: (state) => naming(state),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const slug = String(args.slug);
        const watching = state.watching[me] ?? [];
        if (!watching.includes(slug)) return ctx.refuse(`you do not watch ${slug}`);
        return { ...state, watching: { ...state.watching, [me]: watching.filter((one) => one !== slug) } };
      },
    },
  },
  moves: (state) => (Object.keys(state.articles).length ? ["write", "edit", "talk", "watch", "unwatch"] : ["write"]),
  read: sliced(mine),
  bot,
});
