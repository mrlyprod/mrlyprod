import type { Ctx, Json, Knob, Knobs, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, below, sliced, stream, walk } from "@mrlyprod/os/contract/index.ts";
import { among, notice } from "../shared/people.ts";

export type Revision = { n: number; by: string | null; at: number; text: string };

export type Talk = { n: number; by: string; at: number; text: string };

export type Article = { slug: string; title: string; text: string; n: number; revisions: Revision[]; talk: Talk[] };

export type Wiki = { articles: Record<string, Article>; watching: Record<string, string[]> };

const KNOBS = {
  title: {
    line: "Longest article title, in characters.",
    schema: { type: "integer", minimum: 20, maximum: 400 },
    reality: 80,
  },
  text: {
    line: "Longest article, in characters.",
    schema: { type: "integer", minimum: 200, maximum: 100000 },
    reality: 4000,
  },
  talk: {
    line: "Longest post on a talk page, in characters.",
    schema: { type: "integer", minimum: 20, maximum: 5000 },
    reality: 500,
  },
  talks: {
    line: "Posts a talk page keeps.",
    schema: { type: "integer", minimum: 5, maximum: 1000 },
    reality: 50,
  },
  revisions: {
    line: "Revisions an article keeps.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 10,
  },
  watch: {
    line: "Articles one person may watch.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 30,
  },
  stubs: {
    line: "Articles a world starts with.",
    schema: { type: "integer", minimum: 0, maximum: 12 },
    reality: 5,
  },
} satisfies Knobs;

export const SLUG = 40;

const TOP = 20;

const OPENING = ["write"];

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

const articleAt = (state: Wiki, slug: string) =>
  Object.hasOwn(state.articles, slug) ? state.articles[slug] : undefined;

const articleOf = (state: Wiki, slug: string, refuse: Ctx["refuse"]) =>
  articleAt(state, slug) ?? refuse(`no article ${slug}; try ${among(Object.keys(state.articles))}, or write it`);

const watchesOf = (state: Wiki, id: string) => (Object.hasOwn(state.watching, id) ? state.watching[id]! : []);

function seeded(seed: number, stubs: number): Record<string, Article> {
  const { draw } = stream(seed);
  const pool = [...STUBBED];
  const out: Record<string, Article> = {};
  for (let i = 0; i < stubs && pool.length; i++) {
    const [title, text] = pool.splice(below(draw, pool.length), 1)[0]!;
    const one = article(title, text, null, 0);
    out[one.slug] = one;
  }
  return out;
}

const watchersOf = (state: Wiki, slug: string) =>
  Object.keys(state.watching).filter((id) => state.watching[id]!.includes(slug));

const lastOf = (one: Article) => one.revisions.at(-1)!;

const row = (state: Wiki, one: Article) => ({
  slug: one.slug,
  title: one.title,
  n: one.n,
  by: lastOf(one).by,
  at: lastOf(one).at,
  talks: one.talk.length,
  watchers: watchersOf(state, one.slug).length,
});

const opened = (state: Wiki, one: Article) => ({
  ...row(state, one),
  text: one.text,
  talk: one.talk,
  revisions: Object.fromEntries(
    one.revisions.map((rev) => [rev.n, { n: rev.n, by: rev.by, at: rev.at, size: rev.text.length }]),
  ),
});

const board = (state: Wiki) => {
  const all = Object.values(state.articles);
  return {
    count: all.length,
    recent: [...all]
      .sort((a, b) => lastOf(b).at - lastOf(a).at || a.slug.localeCompare(b.slug))
      .slice(0, TOP)
      .map((one) => row(state, one)),
  };
};

const mine = (state: Wiki, id: string) => ({
  ...board(state),
  watching: watchesOf(state, id).map((slug) => ({
    slug,
    title: articleAt(state, slug)?.title ?? slug,
    n: articleAt(state, slug)?.n ?? 0,
  })),
  edited: Object.values(state.articles)
    .filter((one) => one.revisions.some((rev) => rev.by === id))
    .map((one) => one.slug),
});

const whole = (state: Wiki, who: Who | null): Json => (who ? mine(state, who.id) : board(state));

const view = sliced(mine, whole);

const writing = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    title: { type: "string", minLength: 1, maxLength: knob("title") },
    text: { type: "string", minLength: 1, maxLength: knob("text") },
  },
  required: ["title", "text"],
  additionalProperties: false,
});

const naming = (more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { slug: { type: "string", minLength: 1, maxLength: SLUG }, ...more },
  required: ["slug", ...Object.keys(more)],
  additionalProperties: false,
});

function tellWatchers(state: Wiki, slug: string, ctx: Ctx, text: string) {
  const me = ctx.who.id;
  for (const id of watchersOf(state, slug)) if (id !== me) ctx.wish(notice(id, text));
}

function revised(state: Wiki, one: Article, text: string, ctx: Ctx): Wiki {
  const n = one.n + 1;
  const revision: Revision = { n, by: ctx.who.id, at: ctx.at, text };
  return put(state, { ...one, text, n, revisions: [...one.revisions, revision].slice(-ctx.knob("revisions")) });
}

const watchedBy = (state: Wiki, id: string, slug: string, knob: Knob) => {
  const list = watchesOf(state, id);
  return list.length < knob("watch") ? { ...state.watching, [id]: [...list, slug] } : state.watching;
};

export default app<Wiki>({
  id: "wiki",
  title: "Wiki",
  line: "One encyclopedia written inside the world: articles, their revisions and a talk page each. Read wiki for the recent ones, wiki/articles for all, wiki/articles/water for one, wiki/articles/water/revisions/2 for an old text.",
  boot: ({ seed, knob }) => ({ articles: seeded(seed, knob("stubs")), watching: {} }),
  knobs: KNOBS,
  verbs: {
    write: {
      line: (knob) =>
        `Write a new article: a title of 1 to ${knob("title")} characters, a text of up to ${knob("text")}; its slug follows from the title, and you watch it.`,
      args: writing,
      run: (state, args, ctx) => {
        const title = String(args.title).trim();
        const text = String(args.text);
        const slug = slugOf(title);
        if (!slug) return ctx.refuse("the title needs a letter or a digit");
        if (!text.trim()) return ctx.refuse("the text is only spaces; write something");
        if (articleAt(state, slug)) return ctx.refuse(`there is an article ${slug}; edit it`);
        ctx.say({ note: `Wrote ${title}: read wiki/articles/${slug}`, id: slug });
        return put(
          { ...state, watching: watchedBy(state, ctx.who.id, slug, ctx.knob) },
          article(title, text, ctx.who.id, ctx.at),
        );
      },
    },
    edit: {
      line: (knob) =>
        `Replace an article's text; it keeps its last ${knob("revisions")} revisions, and those watching hear of it.`,
      args: (knob) => naming({ text: { type: "string", minLength: 1, maxLength: knob("text") } }),
      run: (state, args, ctx) => {
        const one = articleOf(state, String(args.slug), ctx.refuse);
        const text = String(args.text);
        if (!text.trim()) return ctx.refuse("the text is only spaces; write something");
        if (text === one.text) return ctx.refuse(`${one.slug} already reads so; change something`);
        tellWatchers(state, one.slug, ctx, `${ctx.who.id} edited ${one.title}, now revision ${one.n + 1}`);
        ctx.say({ note: `Edited ${one.slug}, now revision ${one.n + 1}`, id: one.slug, revision: one.n + 1 });
        return revised(state, one, text, ctx);
      },
    },
    revert: {
      line: "Put an article back to an earlier revision, as a new revision; those watching hear of it.",
      args: naming({ to: { type: "integer", minimum: 1 } }),
      run: (state, args, ctx) => {
        const one = articleOf(state, String(args.slug), ctx.refuse);
        const to = Number(args.to);
        const old = one.revisions.find((rev) => rev.n === to);
        if (!old) {
          const kept = one.revisions.map((rev) => rev.n).join(", ");
          return ctx.refuse(`${one.slug} keeps no revision ${to}; it keeps ${kept}`);
        }
        if (old.text === one.text) return ctx.refuse(`${one.slug} already reads as revision ${to}`);
        tellWatchers(state, one.slug, ctx, `${ctx.who.id} reverted ${one.title} to revision ${to}`);
        ctx.say({
          note: `Reverted ${one.slug} to revision ${to}, now revision ${one.n + 1}`,
          id: one.slug,
          revision: one.n + 1,
        });
        return revised(state, one, old.text, ctx);
      },
    },
    talk: {
      line: (knob) =>
        `Post on an article's talk page: 1 to ${knob("talk")} characters; it keeps its last ${knob("talks")} posts, and those watching hear of it.`,
      args: (knob) => naming({ text: { type: "string", minLength: 1, maxLength: knob("talk") } }),
      run: (state, args, ctx) => {
        const one = articleOf(state, String(args.slug), ctx.refuse);
        const text = String(args.text);
        if (!text.trim()) return ctx.refuse("the post is only spaces; say something");
        const n = (one.talk.at(-1)?.n ?? 0) + 1;
        tellWatchers(state, one.slug, ctx, `${ctx.who.id} on the talk page of ${one.title}: ${text}`);
        ctx.say({ note: `Posted ${n} on the talk page of ${one.slug}`, id: one.slug, talk: n });
        const talk = [...one.talk, { n, by: ctx.who.id, at: ctx.at, text }].slice(-ctx.knob("talks"));
        return put(state, { ...one, talk });
      },
    },
    watch: {
      line: (knob) =>
        `Watch an article, to hear of its edits and talk; you watch at most ${knob("watch")}, and what you watch is yours to know.`,
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const slug = articleOf(state, String(args.slug), ctx.refuse) && String(args.slug);
        const watching = watchesOf(state, me);
        const most = ctx.knob("watch");
        if (watching.includes(slug)) return ctx.refuse(`you watch ${slug} already`);
        if (watching.length >= most) return ctx.refuse(`you watch ${most} articles; unwatch one first`);
        ctx.say({ note: `Watching ${slug}`, id: slug });
        return { ...state, watching: { ...state.watching, [me]: [...watching, slug] } };
      },
    },
    unwatch: {
      line: "Stop watching an article.",
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const slug = String(args.slug);
        const watching = watchesOf(state, me);
        if (!watching.includes(slug)) return ctx.refuse(`you do not watch ${slug}`);
        ctx.say({ note: `Stopped watching ${slug}`, id: slug });
        return { ...state, watching: { ...state.watching, [me]: watching.filter((one) => one !== slug) } };
      },
    },
  },
  moves: (state) =>
    Object.keys(state.articles).length ? [...OPENING, "edit", "revert", "talk", "watch", "unwatch"] : OPENING,
  read: (state, path, who, at) => {
    const [head, slug, part, n, ...rest] = path;
    if (head !== "articles") return view(state, path, who, at);
    if (!slug) return Object.fromEntries(Object.values(state.articles).map((one) => [one.slug, row(state, one)]));
    const one = articleAt(state, slug);
    if (!one) return undefined;
    if (part === "revisions" && n)
      return walk(
        one.revisions.find((rev) => rev.n === Number(n)),
        rest,
      );
    return walk(opened(state, one), path.slice(2));
  },
});
