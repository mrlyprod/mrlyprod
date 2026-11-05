import type { Ctx, Json, Knob, Knobs, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { act, app, sliced, walk } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, notice } from "../shared/people.ts";

export type Choice = "yes" | "no";

export type Decide = "vote" | "ruler" | "lottery";

export type Result = "carried" | "failed" | "short" | "cancelled";

export type Ballot = {
  n: number;
  by: string;
  question: string;
  wish: number | null;
  org: number | null;
  members: string[] | null;
  opened: number;
  closes: number;
  closed: number | null;
  needed: number;
  votes: Record<string, Choice>;
  tally: Record<Choice, number>;
  result: Result | null;
};

export type Votes = {
  ballots: Record<string, Ballot>;
  n: number;
  people: string[] | null;
  decide: Decide;
  secret: boolean;
};

type Named = { by: string; text: string; status: string; decider: string | null };

type Org = { n: number; name: string; dissolved: number | null; members: Record<string, string> };

const KNOBS = {
  decide: {
    line: "Who decides a wish: a ballot, one ruler drawn from the seed, or one person drawn by lot for each wish.",
    schema: { enum: ["vote", "ruler", "lottery"] },
    reality: "vote",
  },
  quorum: {
    line: "Percent of everyone eligible who must vote before a ballot decides anything; below it, the ballot comes up short.",
    schema: { type: "integer", minimum: 0, maximum: 100 },
    reality: 0,
  },
  pass: {
    line: "Percent of the votes cast that yes must exceed to carry a ballot.",
    schema: { type: "integer", minimum: 0, maximum: 99 },
    reality: 50,
  },
  secret: {
    line: "Hide who voted what, and hide the tally until the ballot closes.",
    schema: { type: "boolean" },
    reality: false,
  },
  shortest: {
    line: "Seconds every ballot stays open, even once everyone eligible has voted. Count acts, not clock: at 144 seconds an act across 10 people, 600 seconds is 4 acts in all.",
    schema: { type: "integer", minimum: 60, maximum: 604800 },
    reality: 600,
  },
  longest: {
    line: "Most seconds a ballot may run.",
    schema: { type: "integer", minimum: 3600, maximum: 31536000 },
    reality: 2592000,
  },
  lasting: {
    line: "Seconds a ballot runs when its opener names none.",
    schema: { type: "integer", minimum: 60, maximum: 2592000 },
    reality: 86400,
  },
  opened: {
    line: "Open ballots one person may hold.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 3,
  },
  question: {
    line: "Longest ballot question, in characters.",
    schema: { type: "integer", minimum: 40, maximum: 4000 },
    reality: 280,
  },
} satisfies Knobs;

const WISHES = 1_000_000;

const CHOICES: Choice[] = ["yes", "no"];

const TOP = 20;

const OPENING = ["open"];

const WANTED = "the people want this; nobody has built it";

export function span(ms: number): string {
  const s = Math.ceil(ms / 1000);
  if (s < 120) return `${s} seconds`;
  if (s < 7200) return `${Math.round(s / 60)} minutes`;
  if (s < 172800) return `${Math.round(s / 3600)} hours`;
  return `${Math.round(s / 86400)} days`;
}

const ballotOf = (state: Votes, args: { ballot?: unknown }) => state.ballots[String(args.ballot)]!;

const listed = (state: Votes) => Object.values(state.ballots);

const due = (one: Ballot, ctx: Ctx) => ctx.at >= one.closes;

const voted = (one: Ballot, id: string) => Object.hasOwn(one.votes, id);

const eligible = (state: Votes, one: Pick<Ballot, "members">) => one.members ?? state.people;

const everyone = (state: Votes, one: Ballot) => !!eligible(state, one)?.every((id) => voted(one, id));

const earliest = (one: Ballot, knob: Knob) => one.opened + knob("shortest") * 1000;

const ready = (state: Votes, one: Ballot, ctx: Ctx) =>
  due(one, ctx) || (everyone(state, one) && ctx.at >= earliest(one, ctx.knob));

const least = (knob: Knob) => Math.max(1, knob("shortest"));

const most = (knob: Knob) => Math.max(least(knob), knob("longest"));

const put = (state: Votes, one: Ballot): Votes => ({ ...state, ballots: { ...state.ballots, [one.n]: one } });

const statusOf = (one: Ballot, at: number) => one.result ?? (at >= one.closes ? "due" : "open");

const shownVotes = (state: Votes) => !state.secret;

const standing = (state: Votes, one: Ballot, id: string, at: number): string => {
  if (voted(one, id)) return `you voted ${one.votes[id]}`;
  if (!(eligible(state, one)?.includes(id) ?? true))
    return one.org === null
      ? "not eligible: you are not among this world's people"
      : `not eligible: only org ${one.org} members vote`;
  if (one.result) return `closed ${one.result}; you cannot vote`;
  if (at >= one.closes) return "time ran out; vote.close settles it, no more votes";
  return "you can vote";
};

const row = (state: Votes, one: Ballot, at: number, id: string | null = null) => ({
  n: one.n,
  by: one.by,
  question: one.question,
  wish: one.wish,
  ...(one.org === null ? {} : { org: one.org }),
  ...(one.result === "carried" && one.wish !== null ? { means: WANTED } : {}),
  status: statusOf(one, at),
  ...(id ? { you: standing(state, one, id, at) } : {}),
  lasts: span(one.closes - one.opened),
  ...(one.result === null && at < one.closes ? { left: span(one.closes - at) } : {}),
  closes: one.closes,
  voted: Object.keys(one.votes).length,
  eligible: eligible(state, one)?.length ?? null,
  needed: one.needed,
  ...(state.secret && one.result === null ? {} : { tally: one.tally }),
});

const opened = (state: Votes, one: Ballot, at: number, id: string | null) => ({
  ...row(state, one, at, id),
  opened: one.opened,
  closed: one.closed,
  ...(shownVotes(state) ? { votes: one.votes } : {}),
});

const newest = (list: Ballot[]) => [...list].sort((a, b) => b.n - a.n);

const counted = (state: Votes, result: Result | null) => listed(state).filter((one) => one.result === result).length;

const running = (one: Ballot, at: number) => one.result === null && at < one.closes;

const board = (state: Votes, at: number, id: string | null = null) => ({
  decide: state.decide,
  secret: state.secret,
  counts: {
    open: listed(state).filter((one) => running(one, at)).length,
    due: counted(state, null) - listed(state).filter((one) => running(one, at)).length,
    carried: counted(state, "carried"),
    failed: counted(state, "failed"),
    short: counted(state, "short"),
    cancelled: counted(state, "cancelled"),
  },
  live: newest(listed(state).filter((one) => running(one, at)))
    .slice(0, TOP)
    .map((one) => row(state, one, at, id)),
  due: newest(listed(state).filter((one) => one.result === null && at >= one.closes))
    .slice(0, TOP)
    .map((one) => row(state, one, at, id)),
  closed: newest(listed(state).filter((one) => one.result !== null))
    .slice(0, TOP)
    .map((one) => row(state, one, at, id)),
});

const votesOf = (state: Votes, id: string) =>
  Object.fromEntries(
    listed(state)
      .filter((one) => voted(one, id))
      .map((one) => [one.n, one.votes[id]!]),
  );

const openedBy = (state: Votes, id: string) =>
  listed(state)
    .filter((one) => one.by === id)
    .map((one) => one.n);

const own = (state: Votes, id: string, at: number) => ({
  ...board(state, at, id),
  votes: votesOf(state, id),
  opened: openedBy(state, id),
  waiting: listed(state)
    .filter(
      (one) =>
        one.result === null && at < one.closes && !voted(one, id) && (eligible(state, one)?.includes(id) ?? true),
    )
    .map((one) => one.n),
});

const profile = (state: Votes, id: string) => ({
  id,
  opened: openedBy(state, id),
  ...(shownVotes(state) ? { votes: votesOf(state, id) } : {}),
});

const slice = (state: Votes, id: string, who: Who | null, at: number): Json | undefined =>
  who?.id === id || (!who && !state.secret) ? own(state, id, at) : profile(state, id);

const whole = (state: Votes, who: Who | null, at: number): Json => (who ? own(state, who.id, at) : board(state, at));

const view = sliced(slice, whole, true);

const opening = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    question: { type: "string", minLength: 1, maxLength: knob("question") },
    wish: { type: "integer", minimum: 1, maximum: WISHES },
    org: { type: "integer", minimum: 1 },
    seconds: { type: "integer", minimum: least(knob), maximum: most(knob) },
  },
  additionalProperties: false,
});

const naming = (more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { ballot: { type: "integer", minimum: 1 }, ...more },
  required: ["ballot", ...Object.keys(more)],
  additionalProperties: false,
});

function wishNamed(state: Votes, n: number, ctx: Ctx): Named {
  if (ctx.read("wish") === null) return ctx.refuse("this world has no wishes; put a question instead");
  const one = ctx.read(`wish/wishes/${n}`) as Named | null;
  if (!one) return ctx.refuse(`no wish ${n}; read wish for the ones open`);
  if (one.status !== "open") return ctx.refuse(`wish ${n} is ${one.status}`);
  if (state.decide !== "vote") return ctx.refuse(`${one.decider} decides wish ${n} here, not a ballot; ask them`);
  return one;
}

function clubOf(ctx: Ctx, n: number): Org {
  if (ctx.read("people") === null) return ctx.refuse("this world has no people, so no orgs; put it to everyone");
  const one = ctx.read(`people/orgs/${n}`) as Org | null;
  if (!one) return ctx.refuse(`no org ${n} in people`);
  if (one.dissolved !== null) return ctx.refuse(`${one.name} was dissolved`);
  if (!Object.hasOwn(one.members, ctx.who.id)) return ctx.refuse(`only a member of ${one.name} opens its ballots`);
  return one;
}

function announce(state: Votes, one: Ballot, ctx: Ctx) {
  const me = ctx.who.id;
  const time = span(one.closes - ctx.at);
  const club = one.org === null ? "" : ` for org ${one.org}`;
  for (const id of eligible(state, one) ?? [])
    if (id !== me) ctx.wish(notice(id, `Ballot ${one.n}${club} from ${me}, open ${time}: ${one.question}`));
}

const wishWord = (one: Ballot) => {
  if (one.wish === null) return "";
  if (one.result === "carried") return `; wish ${one.wish} marked granted: ${WANTED}`;
  if (one.result === "failed") return `; wish ${one.wish} declined`;
  return `; wish ${one.wish} stays open`;
};

const ending = (one: Ballot) =>
  `Ballot ${one.n} ${one.result}, ${one.tally.yes} yes and ${one.tally.no} no${one.result === "short" ? `, ${one.needed} votes needed` : ""}${wishWord(one)}`;

function reported(one: Ballot, ctx: Ctx) {
  const told = new Set([one.by, ...Object.keys(one.votes)]);
  if (one.result === "short" && one.wish !== null) told.add(String(ctx.read(`wish/wishes/${one.wish}/by`)));
  const text = `${ending(one)}: ${one.question}`;
  for (const id of told)
    if (id !== ctx.who.id && (id === one.by || !blockedBy(ctx, id, one.by))) ctx.wish(notice(id, text, true));
}

function decided(one: Ballot, ctx: Ctx) {
  if (one.wish === null || ctx.read(`wish/wishes/${one.wish}/status`) !== "open") return;
  if (one.result === "carried") ctx.wish(act("wish.grant", { wish: one.wish }));
  if (one.result === "failed") ctx.wish(act("wish.decline", { wish: one.wish }));
}

function outcome(one: Ballot, knob: Knob): Result {
  const cast = one.tally.yes + one.tally.no;
  if (!cast || cast < one.needed) return "short";
  return one.tally.yes * 100 > cast * knob("pass") ? "carried" : "failed";
}

function closed(state: Votes, one: Ballot, ctx: Ctx): Votes {
  const next: Ballot = { ...one, result: outcome(one, ctx.knob), closed: ctx.at };
  reported(next, ctx);
  decided(next, ctx);
  ctx.say({ note: `${ending(next)}.`, id: next.n, result: next.result });
  return put(state, next);
}

export default app<Votes>({
  id: "vote",
  title: "Vote",
  line: "Ballots on wishes and questions: yes or no, one vote each, a closing time, a tally anyone reads. A ballot only records what people want; it changes nothing in the world. Read vote for what is live and what you owe a vote on, vote/ballots/3 for one ballot.",
  boot: ({ actors, knob }) => ({
    ballots: {},
    n: 0,
    people: actors,
    decide: actors?.length ? (knob("decide") as Decide) : "vote",
    secret: knob("secret"),
  }),
  knobs: KNOBS,
  verbs: {
    open: {
      line: (knob) =>
        `Put a question, or a wish by its number, to a vote; everyone hears of it. Name an org you belong to and only its members vote and hear of it. It runs ${span(least(knob) * 1000)} to ${span(most(knob) * 1000)}, ${span(knob("lasting") * 1000)} if you name no seconds; ${knob("opened")} open each.`,
      args: (knob) => opening(knob),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const named = args.wish === undefined ? null : wishNamed(state, Number(args.wish), ctx);
        const wish = named ? Number(args.wish) : null;
        const club = args.org === undefined ? null : clubOf(ctx, Number(args.org));
        if (club && named) return ctx.refuse("a wish is everyone's; an org's ballot takes a question");
        const question = String(args.question ?? named?.text ?? "").trim();
        if (!question) return ctx.refuse("ask a question, or name a wish by its number");
        const live = listed(state).filter((one) => one.result === null);
        const cap = ctx.knob("opened");
        if (live.filter((one) => one.by === me).length >= cap)
          return ctx.refuse(`you have ${cap} ballots open; close or cancel one first`);
        const twin = wish === null ? undefined : live.find((one) => one.wish === wish);
        if (twin) return ctx.refuse(`ballot ${twin.n} already decides wish ${wish}`);
        const n = state.n + 1;
        const asked = args.seconds ?? Math.min(most(ctx.knob), Math.max(least(ctx.knob), ctx.knob("lasting")));
        const ballot: Ballot = {
          n,
          by: me,
          question,
          wish,
          org: club?.n ?? null,
          members: club ? Object.keys(club.members) : null,
          opened: ctx.at,
          closes: ctx.at + Number(asked) * 1000,
          closed: null,
          needed: Math.ceil(
            ((club ? Object.keys(club.members).length : (state.people?.length ?? 0)) * ctx.knob("quorum")) / 100,
          ),
          votes: {},
          tally: { yes: 0, no: 0 },
          result: null,
        };
        announce(state, ballot, ctx);
        ctx.say({
          note: `Opened ballot ${n}, closing in ${span(ballot.closes - ctx.at)}${wish === null ? "" : ` on wish ${wish}`}`,
          id: n,
          closes: ballot.closes,
        });
        return { ...put(state, ballot), n };
      },
    },
    cast: {
      line: (knob) =>
        `Vote yes or no on an open ballot; one vote each, final. Once everyone eligible has voted it may close, but no ballot closes within ${span(knob("shortest") * 1000)} of opening.`,
      args: naming({ choice: { enum: CHOICES } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = ballotOf(state, args);
        const choice = args.choice as Choice;
        if (one.result) return ctx.refuse(`ballot ${one.n} is closed: ${one.result}`);
        if (due(one, ctx)) return ctx.refuse(`ballot ${one.n} ran out of time; settle it with vote.close`);
        if (one.members && !one.members.includes(me))
          return ctx.refuse(`ballot ${one.n} is for org ${one.org}; only those in it when it opened vote`);
        if (voted(one, me)) return ctx.refuse(`you already voted ${one.votes[me]} on ballot ${one.n}; one vote each`);
        const next = {
          ...one,
          votes: { ...one.votes, [me]: choice },
          tally: { ...one.tally, [choice]: one.tally[choice] + 1 },
        };
        const after = put(state, next);
        ctx.say({ note: `Voted ${choice} on ballot ${one.n}`, id: one.n });
        return ready(after, next, ctx) ? closed(after, next, ctx) : after;
      },
    },
    close: {
      line: `Settle a ballot once its time has come or everyone eligible has voted; anyone may. Enough yes carries it; no votes at all leave it short and its wish open. A carried wish is marked granted, which says only that ${WANTED}.`,
      args: naming(),
      run: (state, args, ctx) => {
        const one = ballotOf(state, args);
        if (one.result) return ctx.refuse(`ballot ${one.n} is already closed: ${one.result}`);
        if (ready(state, one, ctx)) return closed(state, one, ctx);
        const until = everyone(state, one) ? earliest(one, ctx.knob) : one.closes;
        return ctx.refuse(`ballot ${one.n} is open for ${span(until - ctx.at)} more`);
      },
    },
    cancel: {
      line: "Cancel a ballot you opened, while no one has voted on it.",
      args: naming(),
      run: (state, args, ctx) => {
        const one = ballotOf(state, args);
        if (one.by !== ctx.who.id) return ctx.refuse(`only ${one.by} cancels ballot ${one.n}`);
        if (one.result) return ctx.refuse(`ballot ${one.n} is already closed: ${one.result}`);
        const cast = Object.keys(one.votes).length;
        if (cast) return ctx.refuse(`${cast} people voted on ballot ${one.n}; it runs to its close`);
        ctx.say({ note: `Cancelled ballot ${one.n}`, id: one.n });
        return put(state, { ...one, result: "cancelled", closed: ctx.at });
      },
    },
  },
  moves: (state) => (state.n ? [...OPENING, "cast", "close", "cancel"] : OPENING),
  read: (state, path, who, at) => {
    const [head, n, ...rest] = path;
    if (head !== "ballots") return view(state, path, who, at);
    if (n === undefined)
      return Object.fromEntries(listed(state).map((one) => [one.n, row(state, one, at, who?.id ?? null)]));
    const one = Object.hasOwn(state.ballots, n) ? state.ballots[n] : undefined;
    return one && walk(opened(state, one, at, who?.id ?? null), rest);
  },
});
