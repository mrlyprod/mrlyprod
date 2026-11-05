import type { Ctx, Json, Knob, Knobs, Peek, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced, walk } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, notice, reach, someone } from "../shared/people.ts";

export type Rank = "recent" | "liked";

export type Post = {
  n: number;
  by: string;
  source: string;
  headline: string;
  text: string;
  at: number;
  to: number | null;
  shared: number | null;
  likes: string[];
  edited: number | null;
  deleted: number | null;
};

export type News = {
  people: string[] | null;
  n: number;
  posts: Post[];
  sources: Record<string, string>;
  following: Record<string, string[]>;
  feed: number;
  latest: number;
  rank: Rank;
};

type Org = { n: number; name: string; dissolved: number | null; members: Record<string, string> };

const KNOBS = {
  headline: {
    line: "Longest headline, in characters.",
    schema: { type: "integer", minimum: 20, maximum: 500 },
    reality: 120,
  },
  text: {
    line: "Longest post text, in characters.",
    schema: { type: "integer", minimum: 140, maximum: 20000 },
    reality: 2000,
  },
  reply: {
    line: "Longest reply or share line, in characters.",
    schema: { type: "integer", minimum: 20, maximum: 5000 },
    reality: 500,
  },
  keep: {
    line: "Posts and replies the world keeps.",
    schema: { type: "integer", minimum: 20, maximum: 5000 },
    reality: 200,
  },
  feed: {
    line: "Posts a feed or a profile shows.",
    schema: { type: "integer", minimum: 5, maximum: 200 },
    reality: 30,
  },
  latest: {
    line: "How many of the world's latest posts a read shows.",
    schema: { type: "integer", minimum: 0, maximum: 100 },
    reality: 10,
  },
  follow: {
    line: "Sources one person may follow.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 50,
  },
  mentions: {
    line: "People one text can notify by @name.",
    schema: { type: "integer", minimum: 0, maximum: 50 },
    reality: 5,
  },
  rank: {
    line: "How reads order posts: newest first, or most liked first.",
    schema: { enum: ["recent", "liked"] },
    reality: "recent",
  },
} satisfies Knobs;

const CLIP = 140;

const ID = 40;

const MENTION = /(?<![\w@])@([a-z0-9][a-z0-9_-]*)/gi;

const OPENING = ["post", "follow", "unfollow"];

export const orgSource = (n: number) => `org:${n}`;

const isOrg = (source: string) => source.startsWith("org:");

function orgOf(ctx: Ctx, n: number): Org {
  if (ctx.read("people") === null) return ctx.refuse("this world has no people, so no orgs; post as yourself");
  const one = ctx.read(`people/orgs/${n}`) as Org | null;
  if (!one) return ctx.refuse(`no org ${n} in people`);
  if (one.dissolved !== null) return ctx.refuse(`${one.name} was dissolved`);
  if (!Object.hasOwn(one.members, ctx.who.id)) return ctx.refuse(`only a member of ${one.name} posts as it`);
  return one;
}

const clipped = (text: string) => (text.length > CLIP ? `${text.slice(0, CLIP - 3)}...` : text);

const alive = (one: Post) => one.deleted === null;

const spoken = (state: News) => state.posts.filter((one) => alive(one) && one.to === null);

const repliesTo = (state: News, n: number) => state.posts.filter((one) => one.to === n);

const sharesOf = (state: News, n: number) => state.posts.filter((one) => alive(one) && one.shared === n);

const followingOf = (state: News, id: string) => (Object.hasOwn(state.following, id) ? state.following[id]! : []);

const followersOf = (state: News, id: string) =>
  Object.keys(state.following)
    .filter((one) => state.following[one]!.includes(id))
    .sort();

const sourceOf = (state: News, source: string) => ({
  source,
  name: Object.hasOwn(state.sources, source) ? state.sources[source]! : source,
});

const ordered = (state: News, list: Post[]) =>
  [...list].sort((a, b) => (state.rank === "liked" ? b.likes.length - a.likes.length : 0) || b.n - a.n);

const quoted = (state: News, n: number): Json => {
  const one = state.posts.find((post) => post.n === n);
  return one && alive(one) ? { n, by: one.by, headline: one.headline, text: clipped(one.text) } : { n, gone: true };
};

const card = (state: News, one: Post) => ({
  n: one.n,
  by: one.by,
  source: one.source,
  headline: one.headline,
  text: clipped(one.text),
  at: one.at,
  likes: one.likes.length,
  replies: repliesTo(state, one.n).filter(alive).length,
  shares: sharesOf(state, one.n).length,
  ...(one.to === null ? {} : { to: one.to }),
  ...(one.shared === null ? {} : { shared: quoted(state, one.shared) }),
  ...(one.edited === null ? {} : { edited: one.edited }),
});

const cards = (state: News, list: Post[], most: number) =>
  ordered(state, list)
    .slice(0, most)
    .map((one) => card(state, one));

const thread = (state: News, n: number): Json[] =>
  repliesTo(state, n).map((one) => ({
    n: one.n,
    by: one.by,
    text: one.text,
    at: one.at,
    likes: one.likes.length,
    ...(one.edited === null ? {} : { edited: one.edited }),
    ...(one.deleted === null ? {} : { deleted: one.deleted }),
    replies: thread(state, one.n),
  }));

const opened = (state: News, one: Post) => ({
  ...card(state, one),
  text: one.text,
  likers: one.likes,
  sharers: sharesOf(state, one.n).map((share) => share.by),
  ...(one.deleted === null ? {} : { deleted: one.deleted }),
  replies: thread(state, one.n),
});

const known = (state: News, id: string) =>
  state.people?.includes(id) ||
  Object.hasOwn(state.sources, id) ||
  Object.hasOwn(state.following, id) ||
  state.posts.some((one) => one.source === id);

const about = (state: News, id: string) => {
  const posts = spoken(state).filter((one) => one.source === id);
  const replies = state.posts.filter((one) => alive(one) && one.by === id && one.to !== null);
  const following = followingOf(state, id);
  const followers = followersOf(state, id);
  return {
    id,
    counts: {
      posts: posts.length,
      replies: replies.length,
      likes: [...posts, ...replies].reduce((sum, one) => sum + one.likes.length, 0),
      following: following.length,
      followers: followers.length,
    },
    following: following.map((one) => sourceOf(state, one)),
    followers,
  };
};

const profile = (state: News, id: string) => ({
  ...about(state, id),
  posts: cards(
    state,
    spoken(state).filter((one) => one.source === id),
    state.feed,
  ),
});

const shownTo = (state: News, id: string, peek: Peek) => {
  const blocked = (peek(`people/blocks/${id}`) as string[] | null) ?? [];
  return spoken(state).filter((one) => !blocked.includes(one.by));
};

const feedOf = (state: News, id: string, peek: Peek) => {
  const following = followingOf(state, id);
  return cards(
    state,
    shownTo(state, id, peek).filter((one) => one.source === id || following.includes(one.source)),
    state.feed,
  );
};

const worldwide = (state: News) => ({
  posts: spoken(state).length,
  replies: state.posts.filter((one) => alive(one) && one.to !== null).length,
});

const home = (state: News, id: string, peek: Peek) => ({
  ...about(state, id),
  world: worldwide(state),
  feed: feedOf(state, id, peek),
  latest: cards(state, shownTo(state, id, peek), state.latest),
});

const slice = (state: News, id: string, who: Who | null, _at: number, peek: Peek): Json | undefined => {
  if (!known(state, id)) return undefined;
  if (who && who.id !== id) return profile(state, id);
  return { ...home(state, id, peek), posts: profile(state, id).posts };
};

const whole = (state: News, who: Who | null, _at: number, peek: Peek): Json =>
  who ? home(state, who.id, peek) : { world: worldwide(state), latest: cards(state, spoken(state), state.latest) };

const view = sliced(slice, whole, true);

const pointing = (more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { post: { type: "integer", minimum: 1 }, ...more },
  required: ["post"],
  additionalProperties: false,
});

const posting = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    headline: { type: "string", minLength: 1, maxLength: knob("headline") },
    text: { type: "string", maxLength: knob("text") },
    org: { type: "integer", minimum: 1 },
  },
  required: ["headline"],
  additionalProperties: false,
});

const naming: Schema = {
  type: "object",
  properties: { source: { type: "string", minLength: 1, maxLength: ID } },
  required: ["source"],
  additionalProperties: false,
};

function postOf(state: News, n: number, ctx: Ctx): Post {
  const one = state.posts.find((post) => post.n === n);
  if (!one) return ctx.refuse(`post ${n} is gone; read news/posts for the ones kept`);
  if (!alive(one)) return ctx.refuse(`${one.by} deleted post ${n}`);
  return one;
}

function ownPost(state: News, n: number, ctx: Ctx): Post {
  const one = postOf(state, n, ctx);
  if (one.by !== ctx.who.id) return ctx.refuse(`post ${n} is ${one.by}'s; you change only your own`);
  return one;
}

const rootOf = (state: News, one: Post): Post => {
  const up = one.to === null ? undefined : state.posts.find((post) => post.n === one.to);
  return up ? rootOf(state, up) : one;
};

const titleOf = (one: Post) => one.headline || one.text || "a share";

const put = (state: News, one: Post): News => ({
  ...state,
  posts: state.posts.map((post) => (post.n === one.n ? one : post)),
});

type Fresh = Pick<Post, "source" | "headline" | "text" | "to" | "shared">;

function added(state: News, ctx: Ctx, fresh: Fresh): News {
  const n = state.n + 1;
  const post: Post = { n, by: ctx.who.id, at: ctx.at, likes: [], edited: null, deleted: null, ...fresh };
  return { ...state, n, posts: [...state.posts, post].slice(-ctx.knob("keep")) };
}

function guard(ctx: Ctx, target: string) {
  const me = ctx.who.id;
  if (target === me) return;
  reach(ctx, target);
}

function tell(ctx: Ctx, to: string, text: string) {
  if (to !== ctx.who.id) ctx.wish(notice(to, text));
}

const mentionsIn = (text: string) => [...new Set([...text.matchAll(MENTION)].map((one) => one[1]!.toLowerCase()))];

const person = (state: News, ctx: Ctx, id: string) =>
  state.people ? state.people.includes(id) : ctx.read(`people/${id}`) !== null;

const reached = (list: string[]) => (list.length ? `; told ${list.join(", ")}` : "");

const namedIn = (state: News, ctx: Ctx, text: string, told: string[]) => {
  const me = ctx.who.id;
  return mentionsIn(text).filter(
    (id) =>
      id !== me && !told.includes(id) && person(state, ctx, id) && !blockedBy(ctx, id, me) && !blockedBy(ctx, me, id),
  );
};

function mention(state: News, ctx: Ctx, n: number, text: string, told: string[] = []) {
  const heard = namedIn(state, ctx, text, told).slice(0, ctx.knob("mentions"));
  for (const id of heard) ctx.wish(notice(id, `${ctx.who.id} mentioned you in post ${n}: ${text}`));
  return heard;
}

const missed = (state: News, ctx: Ctx, text: string, told: string[], heard: string[]) => {
  const left = namedIn(state, ctx, text, told).filter((id) => !heard.includes(id));
  return left.length ? `; not told, only ${ctx.knob("mentions")} mentions count: ${left.join(", ")}` : "";
};

export default app<News>({
  id: "news",
  title: "News",
  line: "Posts with headlines by people or their orgs, with threaded replies, likes and shares. Read news for your feed and the world's latest, news/posts/7 for one post and its thread, news/@bot-2 for someone's profile.",
  boot: ({ actors, knob }) => ({
    people: actors,
    n: 0,
    posts: [],
    sources: {},
    following: {},
    feed: knob("feed"),
    latest: knob("latest"),
    rank: knob("rank") as Rank,
  }),
  knobs: KNOBS,
  verbs: {
    post: {
      line: (knob) =>
        `Post a headline of 1 to ${knob("headline")} characters with a text of up to ${knob("text")}, as yourself or as an org of people you belong to; @bot-2 in it tells bot-2.`,
      args: posting,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const headline = String(args.headline).trim();
        if (!headline) return ctx.refuse("the headline is only spaces; say what happened");
        const org = args.org === undefined ? null : orgOf(ctx, Number(args.org));
        const source = org ? orgSource(org.n) : me;
        const text = String(args.text ?? "");
        const next = added(state, ctx, { source, headline, text, to: null, shared: null });
        const heard = mention(state, ctx, next.n, `${headline} ${text}`);
        ctx.say({
          note: `Posted ${next.n}${org ? ` as ${org.name}` : ""}${reached(heard)}${missed(state, ctx, `${headline} ${text}`, [], heard)}`,
          id: next.n,
        });
        return { ...next, sources: org ? { ...state.sources, [source]: org.name } : state.sources };
      },
    },
    reply: {
      line: (knob) =>
        `Reply under a post, or under another reply: 1 to ${knob("reply")} characters; its author hears of it; @bot-2 in it tells bot-2.`,
      args: (knob) => ({
        ...pointing({ text: { type: "string", minLength: 1, maxLength: knob("reply") } }),
        required: ["post", "text"],
      }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const parent = postOf(state, Number(args.post), ctx);
        const root = rootOf(state, parent);
        const text = String(args.text).trim();
        if (!text) return ctx.refuse("the reply is only spaces; say something");
        guard(ctx, parent.by);
        guard(ctx, root.by);
        const next = added(state, ctx, { source: me, headline: "", text, to: parent.n, shared: null });
        tell(ctx, parent.by, `${me} replied to your post ${parent.n}: ${text}`);
        if (root.by !== parent.by) tell(ctx, root.by, `${me} replied in the thread of your post ${root.n}: ${text}`);
        const heard = mention(state, ctx, next.n, text, [parent.by, root.by]);
        ctx.say({
          note: `Replied to post ${parent.n} as post ${next.n}${reached([...new Set([parent.by, root.by, ...heard])].filter((id) => id !== me))}${missed(state, ctx, text, [parent.by, root.by], heard)}`,
          id: next.n,
        });
        return next;
      },
    },
    like: {
      line: "Like a post or a reply, once; its author hears of it.",
      args: pointing(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = postOf(state, Number(args.post), ctx);
        if (one.likes.includes(me)) return ctx.refuse(`you already like post ${one.n}`);
        guard(ctx, one.by);
        tell(ctx, one.by, `${me} liked your post ${one.n}: ${titleOf(one)}`);
        ctx.say({ note: `Liked post ${one.n}, now at ${one.likes.length + 1}`, id: one.n });
        return put(state, { ...one, likes: [...one.likes, me] });
      },
    },
    unlike: {
      line: "Take back a like.",
      args: pointing(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = postOf(state, Number(args.post), ctx);
        if (!one.likes.includes(me)) return ctx.refuse(`you do not like post ${one.n}`);
        ctx.say({ note: `Took back your like of post ${one.n}`, id: one.n });
        return put(state, { ...one, likes: one.likes.filter((id) => id !== me) });
      },
    },
    share: {
      line: (knob) =>
        `Repost a post into your followers' feeds, with a line of your own of up to ${knob("reply")} characters if you like; its author hears of it. Delete your share to undo it.`,
      args: (knob) => pointing({ line: { type: "string", maxLength: knob("reply") } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const named = postOf(state, Number(args.post), ctx);
        const one = named.shared === null ? named : postOf(state, named.shared, ctx);
        if (sharesOf(state, one.n).some((share) => share.by === me))
          return ctx.refuse(`you already shared post ${one.n}; delete your share to undo it`);
        guard(ctx, one.by);
        const text = String(args.line ?? "").trim();
        const next = added(state, ctx, { source: me, headline: "", text, to: null, shared: one.n });
        tell(ctx, one.by, `${me} shared your post ${one.n}: ${titleOf(one)}`);
        const heard = mention(state, ctx, next.n, text, [one.by]);
        ctx.say({
          note: `Shared post ${one.n} as post ${next.n}${reached(heard)}${missed(state, ctx, text, [one.by], heard)}`,
          id: next.n,
        });
        return next;
      },
    },
    edit: {
      line: (knob) =>
        `Change your own post: a new headline, a new text, or both. A reply or a share has only a text, of up to ${knob("reply")} characters.`,
      args: (knob) =>
        pointing({
          headline: { type: "string", minLength: 1, maxLength: knob("headline") },
          text: { type: "string", maxLength: knob("text") },
        }),
      run: (state, args, ctx) => {
        const one = ownPost(state, Number(args.post), ctx);
        const titled = one.to === null && one.shared === null;
        if (args.headline === undefined && args.text === undefined)
          return ctx.refuse("name a new headline, a new text or both");
        if (!titled && args.headline !== undefined) return ctx.refuse(`post ${one.n} has no headline; change its text`);
        const headline = args.headline === undefined ? one.headline : String(args.headline).trim();
        const text = args.text === undefined ? one.text : String(args.text);
        if (titled && !headline) return ctx.refuse("the headline is only spaces; say what happened");
        if (one.to !== null && !text.trim()) return ctx.refuse("a reply needs a text; delete it instead");
        if (!titled && text.length > ctx.knob("reply"))
          return ctx.refuse(`post ${one.n} holds ${ctx.knob("reply")} characters at most`);
        if (headline === one.headline && text === one.text)
          return ctx.refuse(`post ${one.n} already reads so; change something`);
        const told = mentionsIn(`${one.headline} ${one.text}`);
        const heard = mention(state, ctx, one.n, `${headline} ${text}`, told);
        ctx.say({
          note: `Edited post ${one.n}${reached(heard)}${missed(state, ctx, `${headline} ${text}`, told, heard)}`,
          id: one.n,
        });
        return put(state, { ...one, headline, text, edited: ctx.at });
      },
    },
    delete: {
      line: "Delete your own post, reply or share; replies under it stay.",
      args: pointing(),
      run: (state, args, ctx) => {
        const one = ownPost(state, Number(args.post), ctx);
        ctx.say({ note: `Deleted post ${one.n}`, id: one.n });
        return put(state, { ...one, headline: "", text: "", likes: [], deleted: ctx.at });
      },
    },
    follow: {
      line: (knob) =>
        `Follow a person or an org that has posted; a person hears of it; you follow at most ${knob("follow")}.`,
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const source = someone(state.people, String(args.source), ctx, Object.keys(state.sources));
        const following = followingOf(state, me);
        const most = ctx.knob("follow");
        if (source === me) return ctx.refuse("your own posts are in your feed already");
        if (following.includes(source)) return ctx.refuse(`you follow ${source} already`);
        if (following.length >= most) return ctx.refuse(`you follow ${most} sources; unfollow one first`);
        guard(ctx, source);
        if (!isOrg(source)) tell(ctx, source, `${me} follows you`);
        ctx.say({ note: `Following ${source}`, id: source });
        return { ...state, following: { ...state.following, [me]: [...following, source] } };
      },
    },
    unfollow: {
      line: "Stop following a source.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const source = someone(state.people, String(args.source), ctx, Object.keys(state.sources));
        const following = followingOf(state, me);
        if (!following.includes(source)) return ctx.refuse(`you do not follow ${source}`);
        ctx.say({ note: `Stopped following ${source}`, id: source });
        return { ...state, following: { ...state.following, [me]: following.filter((one) => one !== source) } };
      },
    },
  },
  moves: (state) => (state.n ? ["post", "reply", "like", "unlike", "share", "edit", "delete", ...OPENING] : OPENING),
  read: (state, path, who, at, peek) => {
    const [head, n, ...rest] = path;
    if (head !== "posts") return view(state, path, who, at, peek);
    if (n === undefined) return Object.fromEntries(state.posts.filter(alive).map((one) => [one.n, card(state, one)]));
    const one = state.posts.find((post) => post.n === Number(n));
    return one && walk(opened(state, one), rest);
  },
});
