import type { Ctx, Json, Knob, Knobs, Obj, Peek, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, pick, sliced, stream, walk } from "@mrlyprod/os/contract/index.ts";
import { notice, reach } from "../shared/people.ts";

export type Status = "open" | "granted" | "declined" | "withdrawn";

export type Decide = "vote" | "ruler" | "lottery";

export type Comment = { n: number; by: string; text: string; at: number };

export type Wish = {
  n: number;
  by: string;
  text: string;
  at: number;
  backers: string[];
  status: Status;
  withdrawn: number | null;
  decided: number | null;
  decider: string | null;
  comments: Comment[];
};

export type Wishes = {
  wishes: Record<string, Wish>;
  n: number;
  people: string[] | null;
  decide: Decide;
  ruler: string | null;
};

type Ending = "granted" | "declined";

const KNOBS = {
  text: {
    line: "Longest wish, in characters.",
    schema: { type: "integer", minimum: 40, maximum: 4000 },
    reality: 280,
  },
  open: {
    line: "Open wishes one person may hold.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 5,
  },
  comment: {
    line: "Longest comment on a wish, in characters.",
    schema: { type: "integer", minimum: 20, maximum: 4000 },
    reality: 280,
  },
  comments: {
    line: "Comments one wish keeps.",
    schema: { type: "integer", minimum: 5, maximum: 1000 },
    reality: 50,
  },
  near: {
    line: "Percent of its words a new wish shares with an open wish to be called close to it.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 60,
  },
  same: {
    line: "Percent of words two wishes share, counting both, for a new wish to be refused as asking the same; 100 catches only copies.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 60,
  },
} satisfies Knobs;

const DECIDE = "vote.decide";

const VOTE = "vote";

const TOP = 20;

const OPENING = ["post"];

const CLIP = 80;

const WANTED = "the people want this; nobody has built it";

const STOP = new Set(
  "the and for with that this from into more than have should would could want need please let make build add new app can our all get some there any where everyone anyone every people someone each also just very like about over when what who how where which they them their its was were are".split(
    " ",
  ),
);

const SYNONYMS: Record<string, string> = Object.fromEntries(
  [
    "square plaza green commons",
    "meal dinner lunch supper breakfast feast potluck",
    "board noticeboard bulletin",
    "kitchen cookhouse",
    "shop store stall",
    "job work",
    "chat talk",
    "book story novel",
  ].flatMap((line) => {
    const [head, ...rest] = line.split(" ");
    return rest.map((word) => [word, head!] as [string, string]);
  }),
);

const stem = (raw: string) => {
  let word = raw;
  if (word.length > 4 && word.endsWith("ies")) word = `${word.slice(0, -3)}y`;
  else if (word.length > 5 && word.endsWith("ing")) word = word.slice(0, -3);
  else if (word.length > 4 && word.endsWith("ed")) word = word.slice(0, -2);
  else if (word.length > 4 && /(s|x|z|ch|sh)es$/.test(word)) word = word.slice(0, -2);
  else if (word.length > 3 && word.endsWith("s") && !word.endsWith("ss")) word = word.slice(0, -1);
  if (word.length > 4 && word.endsWith("e")) word = word.slice(0, -1);
  return SYNONYMS[word] ?? word;
};

type Asked = { wish: number | null; result: string | null; n: number };

type Open = Record<string, number>;

const ruleOf = (knob: Knob) => (knob<string>(DECIDE) as Decide | null) ?? "vote";

const live = (one: Wish) => one.status === "open";

const listed = (state: Wishes) => Object.values(state.wishes);

const openBy = (state: Wishes, id: string) => listed(state).filter((one) => live(one) && one.by === id);

const ballotsOf = (peek: Peek): Open => {
  const all = (peek("vote/ballots") ?? {}) as Record<string, Asked>;
  return Object.fromEntries(
    Object.values(all)
      .filter((one) => one.wish !== null && one.result === null)
      .map((one) => [String(one.wish), one.n]),
  );
};

const marks = (one: Wish, open: Open) => ({
  ...(one.status === "granted" ? { means: WANTED } : {}),
  ...(live(one) && Object.hasOwn(open, one.n) ? { ballot: open[one.n]! } : {}),
});

const row = (one: Wish, open: Open) => ({
  n: one.n,
  by: one.by,
  text: one.text,
  status: one.status,
  backers: one.backers.length,
  comments: one.comments.length,
  at: one.at,
  ...(one.decider === null ? {} : { decider: one.decider }),
  ...(one.decided === null ? {} : { decided: one.decided }),
  ...marks(one, open),
});

const shaped = (state: Wishes, open: Open) =>
  Object.fromEntries(Object.values(state.wishes).map((one) => [one.n, { ...one, ...marks(one, open) }]));

const ranked = (state: Wishes) =>
  listed(state)
    .filter(live)
    .sort((a, b) => b.backers.length - a.backers.length || a.n - b.n);

const counted = (state: Wishes, status: Status) => listed(state).filter((one) => one.status === status).length;

const board = (state: Wishes, open: Open) => ({
  decide: state.decide,
  ...(state.ruler === null ? {} : { ruler: state.ruler }),
  counts: {
    open: counted(state, "open"),
    granted: counted(state, "granted"),
    declined: counted(state, "declined"),
    withdrawn: counted(state, "withdrawn"),
  },
  ranked: ranked(state)
    .slice(0, TOP)
    .map((one) => row(one, open)),
  granted: listed(state)
    .filter((one) => one.status === "granted")
    .sort((a, b) => (b.decided ?? 0) - (a.decided ?? 0))
    .slice(0, TOP)
    .map((one) => row(one, open)),
});

const backing = (state: Wishes, id: string) =>
  listed(state)
    .filter((one) => live(one) && one.backers.includes(id))
    .map((one) => one.n);

const own = (state: Wishes, id: string, open: Open) => ({
  ...board(state, open),
  posted: openBy(state, id).map((one) => one.n),
  backing: backing(state, id),
  deciding: listed(state)
    .filter((one) => live(one) && one.decider === id)
    .map((one) => one.n),
});

const profile = (state: Wishes, id: string, open: Open) => {
  const posted = listed(state).filter((one) => one.by === id);
  return {
    id,
    posted: posted.filter((one) => one.status !== "withdrawn").map((one) => row(one, open)),
    backing: backing(state, id),
    granted: posted.filter((one) => one.status === "granted").length,
  };
};

const slice = (state: Wishes, id: string, who: Who | null, _at: number, peek: Peek): Json | undefined =>
  who && who.id !== id ? profile(state, id, ballotsOf(peek)) : own(state, id, ballotsOf(peek));

const whole = (state: Wishes, who: Who | null, _at: number, peek: Peek): Json =>
  who
    ? own(state, who.id, ballotsOf(peek))
    : { ...board(state, ballotsOf(peek)), wishes: shaped(state, ballotsOf(peek)) };

const view = sliced(slice, whole, true);

const posting = (knob: Knob): Schema => ({
  type: "object",
  properties: { text: { type: "string", minLength: 1, maxLength: knob("text") } },
  required: ["text"],
  additionalProperties: false,
});

const naming = (more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { wish: { type: "integer", minimum: 1 }, ...more },
  required: ["wish", ...Object.keys(more)],
  additionalProperties: false,
});

const wishOf = (state: Wishes, args: Obj) => state.wishes[String(args.wish)]!;

const put = (state: Wishes, one: Wish): Wishes => ({ ...state, wishes: { ...state.wishes, [one.n]: one } });

function tell(ctx: Ctx, list: readonly string[], text: string) {
  const me = ctx.who.id;
  for (const to of new Set(list)) if (to !== me) ctx.wish(notice(to, text));
}

function guard(ctx: Ctx, target: string) {
  const me = ctx.who.id;
  if (target === me) return;
  reach(ctx, target);
}

const clipped = (text: string) => (text.length > CLIP ? `${text.slice(0, CLIP - 3)}...` : text);

const wordsOf = (text: string) =>
  new Set(
    (text.toLowerCase().match(/[\p{L}\p{N}]+/gu) ?? [])
      .filter((word) => word.length > 2 && !STOP.has(word))
      .map(stem)
      .filter((word) => word.length > 2 && !STOP.has(word)),
  );

function sameOf(state: Wishes, text: string, percent: number): Wish | null {
  const mine = wordsOf(text);
  const found = listed(state)
    .filter(live)
    .map((one) => {
      const words = wordsOf(one.text);
      const shared = [...words].filter((word) => mine.has(word)).length;
      const total = words.size + mine.size;
      return { one, shared, score: total ? (shared * 200) / total : 0 };
    })
    .filter(({ shared, score }) => shared >= 2 && score >= percent)
    .sort((a, b) => b.score - a.score || a.one.n - b.one.n);
  return found[0]?.one ?? null;
}

function nearOf(state: Wishes, text: string, percent: number): Wish | null {
  const mine = wordsOf(text);
  const scored = listed(state)
    .filter(live)
    .map((one) => {
      const words = wordsOf(one.text);
      const shared = [...words].filter((word) => mine.has(word)).length;
      const least = Math.min(words.size, mine.size);
      return { one, shared, least, score: least ? (shared * 100) / least : 0 };
    })
    .filter(({ shared, least, score }) => shared >= Math.min(2, least) && least > 0 && score >= percent)
    .sort((a, b) => b.score - a.score || b.one.backers.length - a.one.backers.length || a.one.n - b.one.n);
  return scored[0]?.one ?? null;
}

const posted = (n: number, near: Wish | null): Json =>
  near
    ? {
        note: `Posted wish ${n}. Close to wish ${near.n}, ${near.backers.length} backers: "${clipped(near.text)}". If it asks the same, back it: wish.back {"wish":${near.n}}.`,
        id: n,
        near: near.n,
      }
    : { note: `Posted wish ${n}`, id: n };

function deciderOf(state: Wishes, ctx: Ctx) {
  if (state.decide === "ruler") return state.ruler;
  return state.decide === "lottery" && state.people ? pick(ctx.draw, state.people) : null;
}

function ended(state: Wishes, args: Obj, ctx: Ctx, status: Ending): Wishes {
  const one = wishOf(state, args);
  if (!live(one)) return ctx.refuse(`wish ${one.n} is ${one.status} already`);
  if (state.decide === VOTE && ctx.read(VOTE) === null)
    return ctx.refuse(`this world has no ballots, so nothing decides wish ${one.n}; back it and comment`);
  if (state.decide === VOTE && ctx.via !== VOTE)
    return ctx.refuse(`a ballot decides wish ${one.n} here: open one with vote.open and name the wish`);
  if (state.decide !== VOTE && ctx.who.id !== one.decider)
    return ctx.refuse(`${one.decider} decides wish ${one.n}; ask them`);
  const line = status === "granted" ? `Wish ${one.n} granted: ${WANTED}` : `Wish ${one.n} declined`;
  if (ctx.via === VOTE)
    for (const to of new Set([one.by, ...one.backers])) ctx.wish(notice(to, `${line}: ${one.text}`, true));
  else tell(ctx, [one.by, ...one.backers], `${line}: ${one.text}`);
  ctx.say({ note: `${line}.`, id: one.n });
  return put(state, { ...one, status, decided: ctx.at });
}

const judging = (knob: Knob, how: string, word: string) =>
  ruleOf(knob) === VOTE
    ? `A ${how} ballot marks a wish ${word}: open one with vote.open and name the wish. No one marks it by hand here.`
    : `Mark an open wish ${word}, if it falls to you: read wish for what you are deciding.`;

export default app<Wishes>({
  id: "wish",
  title: "Wish",
  line: "How you ask for a change: post a request, others back and discuss it, and it is decided. A wish is only a request: granted means the people want it and nobody has built it; nothing in the world changes. Read wish for the most backed first, wish/wishes/3 for one wish and its comments, wish/@bot-2 for someone's wishes.",
  boot: ({ seed, actors, knob }) => {
    const decide = actors?.length ? ruleOf(knob) : "vote";
    return {
      wishes: {},
      n: 0,
      people: actors,
      decide,
      ruler: decide === "ruler" ? pick(stream(seed).draw, actors!) : null,
    };
  },
  knobs: KNOBS,
  verbs: {
    post: {
      line: (knob) =>
        `Ask for a change or a new thing: 1 to ${knob("text")} characters; up to ${knob("open")} open wishes each.`,
      args: (knob) => posting(knob),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const text = String(args.text).trim();
        const most = ctx.knob("open");
        if (!text) return ctx.refuse("the wish is only spaces; say what you want");
        if (openBy(state, me).length >= most) return ctx.refuse(`you have ${most} open wishes; withdraw one first`);
        const same = listed(state).find((one) => live(one) && one.text.toLowerCase() === text.toLowerCase());
        if (same) return ctx.refuse(`wish ${same.n} already asks this; back it instead`);
        const twin = sameOf(state, text, ctx.knob("same"));
        if (twin)
          return ctx.refuse(
            `wish ${twin.n} asks nearly the same, ${twin.backers.length} backers: "${clipped(twin.text)}"; back it: wish.back {"wish":${twin.n}}, or comment on it`,
          );
        const n = state.n + 1;
        const decider = deciderOf(state, ctx);
        if (decider) tell(ctx, [decider], `${me} wished, and it is yours to decide: wish ${n}: ${text}`);
        const one: Wish = {
          n,
          by: me,
          text,
          at: ctx.at,
          backers: [],
          status: "open",
          withdrawn: null,
          decided: null,
          decider,
          comments: [],
        };
        ctx.say(posted(n, nearOf(state, text, ctx.knob("near"))));
        return { ...put(state, one), n };
      },
    },
    back: {
      line: "Back someone else's open wish, once; its author hears of it.",
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = wishOf(state, args);
        if (!live(one)) return ctx.refuse(`wish ${one.n} is ${one.status}`);
        if (one.by === me) return ctx.refuse(`you posted wish ${one.n}; others back it`);
        if (one.backers.includes(me)) return ctx.refuse(`you already back wish ${one.n}`);
        guard(ctx, one.by);
        const backers = [...one.backers, me];
        tell(ctx, [one.by], `${me} backed your wish ${one.n}, now at ${backers.length}: ${one.text}`);
        ctx.say({ note: `Backed wish ${one.n}, now at ${backers.length}`, id: one.n, backers: backers.length });
        return put(state, { ...one, backers });
      },
    },
    unback: {
      line: "Stop backing a wish.",
      args: naming(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = wishOf(state, args);
        if (!live(one)) return ctx.refuse(`wish ${one.n} is ${one.status}`);
        if (!one.backers.includes(me)) return ctx.refuse(`you do not back wish ${one.n}`);
        ctx.say({ note: `Stopped backing wish ${one.n}`, id: one.n, backers: one.backers.length - 1 });
        return put(state, { ...one, backers: one.backers.filter((id) => id !== me) });
      },
    },
    comment: {
      line: (knob) =>
        `Comment on a wish: 1 to ${knob("comment")} characters; its author and earlier commenters hear of it.`,
      args: (knob) => naming({ text: { type: "string", minLength: 1, maxLength: knob("comment") } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = wishOf(state, args);
        const text = String(args.text).trim();
        if (!text) return ctx.refuse("the comment is only spaces; say something");
        guard(ctx, one.by);
        tell(ctx, [one.by, ...one.comments.map((said) => said.by)], `${me} on wish ${one.n}: ${text}`);
        const said: Comment = { n: (one.comments.at(-1)?.n ?? 0) + 1, by: me, text, at: ctx.at };
        ctx.say({ note: `Commented on wish ${one.n}`, id: one.n, comment: said.n });
        return put(state, { ...one, comments: [...one.comments, said].slice(-ctx.knob("comments")) });
      },
    },
    withdraw: {
      line: "Withdraw a wish you posted; its backers hear of it.",
      args: naming(),
      run: (state, args, ctx) => {
        const one = wishOf(state, args);
        if (one.by !== ctx.who.id) return ctx.refuse(`only ${one.by} withdraws wish ${one.n}`);
        if (!live(one)) return ctx.refuse(`wish ${one.n} is ${one.status} already`);
        tell(ctx, one.backers, `${one.by} withdrew wish ${one.n}: ${one.text}`);
        ctx.say({ note: `Withdrew wish ${one.n}`, id: one.n });
        return put(state, { ...one, status: "withdrawn", withdrawn: ctx.at });
      },
    },
    grant: {
      line: (knob) =>
        judging(
          knob,
          "carried",
          "granted, which says only that the people want it; nobody has built it, and nothing changes",
        ),
      args: naming(),
      run: (state, args, ctx) => ended(state, args, ctx, "granted"),
    },
    decline: {
      line: (knob) => judging(knob, "failed", "declined"),
      args: naming(),
      run: (state, args, ctx) => ended(state, args, ctx, "declined"),
    },
  },
  moves: (state) => (state.n ? [...OPENING, "back", "unback", "comment", "withdraw", "grant", "decline"] : OPENING),
  read: (state, path, who, at, peek) =>
    path[0] === "wishes" ? walk(shaped(state, ballotsOf(peek)), path.slice(1)) : view(state, path, who, at, peek),
});
