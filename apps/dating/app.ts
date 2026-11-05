import type { Ctx, Json, Knob, Knobs, Obj, Peek, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, digest, sliced } from "@mrlyprod/os/contract/index.ts";
import { notice, reach } from "../shared/people.ts";

const ID = "dating";

const DAY = 86_400_000;

const PERSON = 40;

const KNOBS = {
  open: { line: "Whether people may join and swipe at all.", schema: { type: "boolean" }, reality: true },
  swipes: {
    line: "Swipes a person gets per day.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 20,
  },
  shown: {
    line: "People shown to swipe on at a time.",
    schema: { type: "integer", minimum: 1, maximum: 50 },
    reality: 5,
  },
  profile: {
    line: "Characters a profile line may hold, and what someone seeks.",
    schema: { type: "integer", minimum: 20, maximum: 2000 },
    reality: 140,
  },
  reveal: { line: "Whether people see who already liked them.", schema: { type: "boolean" }, reality: false },
  mutual: {
    line: "Whether a match needs a like from both; if not, one like is a match unless the other passed.",
    schema: { type: "boolean" },
    reality: true,
  },
  monogamy: { line: "Whether a person holds only one match at a time.", schema: { type: "boolean" }, reality: false },
} satisfies Knobs;

export type Dials = { [K in keyof typeof KNOBS]: (typeof KNOBS)[K]["reality"] };

export type Profile = { line: string; seeking: string };

export type Match = { between: string[]; at: number };

export type Dating = {
  dials: Dials;
  profiles: Record<string, Profile>;
  swipes: Record<string, Record<string, boolean>>;
  matches: Record<string, Match>;
  spent: Record<string, { day: number; n: number }>;
};

const dialed = (knob: Knob) => Object.fromEntries(Object.keys(KNOBS).map((name) => [name, knob(name)])) as Dials;

const pairOf = (a: string, b: string) => [a, b].sort().join("+");

const without = <T>(table: Record<string, T>, id: string) =>
  Object.fromEntries(Object.entries(table).filter(([key]) => key !== id));

const joined = (state: Dating, id: string) => Object.hasOwn(state.profiles, id);

const blocksOf = (peek: Peek, id: string) => (peek(`people/blocks/${id}`) as string[] | null) ?? [];

const apart = (peek: Peek, a: string, b: string) => blocksOf(peek, a).includes(b) || blocksOf(peek, b).includes(a);

const split = (peek: Peek, one: Match) => apart(peek, one.between[0]!, one.between[1]!);

const live = (state: Dating, peek: Peek) => Object.values(state.matches).filter((one) => !split(peek, one));

const heldBy = (state: Dating, id: string, peek: Peek) => live(state, peek).filter((one) => one.between.includes(id));

const otherIn = (one: Match, id: string) => one.between.find((each) => each !== id)!;

const taken = (state: Dating, id: string, peek: Peek) => state.dials.monogamy && heldBy(state, id, peek).length > 0;

const cardOf = (peek: Peek, id: string) => peek(`people/${id}`) as Obj | null;

const nameOf = (peek: Peek, id: string) => {
  const name = cardOf(peek, id)?.name;
  return typeof name === "string" ? name : id;
};

const dayOf = (at: number) => Math.floor(at / DAY);

const used = (state: Dating, id: string, at: number) => {
  const one = state.spent[id];
  return one?.day === dayOf(at) ? one.n : 0;
};

export const left = (state: Dating, id: string, at: number) => Math.max(0, state.dials.swipes - used(state, id, at));

const ranked = (me: string, ids: string[]) =>
  ids
    .map((id) => ({ id, rank: digest(`${me}/${id}`) }))
    .sort((a, b) => a.rank.localeCompare(b.rank))
    .map((one) => one.id);

export const deckOf = (state: Dating, me: string, peek: Peek) => {
  if (!state.dials.open || !joined(state, me) || taken(state, me, peek)) return [];
  const swiped = state.swipes[me] ?? {};
  const open = (id: string) =>
    id !== me && !(id in swiped) && !state.matches[pairOf(me, id)] && !apart(peek, me, id) && !taken(state, id, peek);
  return ranked(me, Object.keys(state.profiles).filter(open)).slice(0, state.dials.shown);
};

const blurb = ({ line, seeking }: Profile): Obj => ({ line, ...(seeking ? { seeking } : {}) });

const brief = (state: Dating, id: string, peek: Peek): Obj => ({
  id,
  name: nameOf(peek, id),
  ...blurb(state.profiles[id]!),
});

const profileOf = (state: Dating, id: string, peek: Peek): Obj => {
  const card = cardOf(peek, id);
  return { ...brief(state, id, peek), ...(card ? { card } : {}) };
};

export const matchesOf = (state: Dating, id: string, peek: Peek) =>
  heldBy(state, id, peek)
    .sort((a, b) => b.at - a.at)
    .map((one) => {
      const other = otherIn(one, id);
      return { id: other, name: nameOf(peek, other), since: one.at };
    });

const likers = (state: Dating, me: string, peek: Peek) =>
  Object.keys(state.profiles).filter(
    (id) =>
      state.swipes[id]?.[me] === true &&
      !(id in (state.swipes[me] ?? {})) &&
      !state.matches[pairOf(me, id)] &&
      !apart(peek, me, id),
  );

function tip(state: Dating, id: string, at: number, peek: Peek): string | null {
  if (!state.dials.open) return "Dating is closed in this world; write to people in messages instead";
  if (!joined(state, id)) return "join with a line to appear here and start swiping";
  if (taken(state, id, peek)) return "this world allows one match at a time; unmatch to swipe again";
  if (!left(state, id, at)) return "no swipes left today; more tomorrow";
  return deckOf(state, id, peek).length ? null : "nobody new to swipe on; check back later";
}

const mine = (state: Dating, id: string, at: number, peek: Peek): Obj => {
  const note = tip(state, id, at, peek);
  const shown = (others: string[]) => others.map((other) => brief(state, other, peek));
  return {
    profile: joined(state, id) ? blurb(state.profiles[id]!) : null,
    swipesLeft: left(state, id, at),
    next: shown(deckOf(state, id, peek)),
    matches: matchesOf(state, id, peek),
    ...(state.dials.reveal ? { likedBy: shown(likers(state, id, peek)) } : {}),
    ...(note ? { tip: note } : {}),
  };
};

const slice = (state: Dating, id: string, who: Who | null, at: number, peek: Peek): Json | undefined => {
  if (who?.id === id) return mine(state, id, at, peek);
  if (!joined(state, id) || (who && apart(peek, who.id, id))) return undefined;
  return profileOf(state, id, peek);
};

const whole = (state: Dating, who: Who | null, at: number, peek: Peek): Json =>
  who
    ? mine(state, who.id, at, peek)
    : {
        open: state.dials.open,
        matches: live(state, peek).length,
        people: Object.keys(state.profiles).map((id) => profileOf(state, id, peek)),
      };

const settled = (state: Dating, ctx: Ctx): Dating => ({
  ...state,
  matches: Object.fromEntries(Object.entries(state.matches).filter(([, one]) => !split(ctx.read, one))),
});

const closed = (state: Dating, ctx: Ctx) => {
  if (!state.dials.open) ctx.refuse("Dating is closed in this world; write to people in messages instead");
};

function checked(state: Dating, me: string, person: string, ctx: Ctx) {
  if (!joined(state, me)) ctx.refuse("you have not joined; join with a line first");
  if (person === me) ctx.refuse("that is you; pick someone from next in read dating");
  if (!joined(state, person)) ctx.refuse(`${person} has not joined Dating; pick someone from next in read dating`);
  reach(ctx, person);
  if (state.matches[pairOf(me, person)]) ctx.refuse(`you already match with ${person}; write to them in messages`);
  if (Object.hasOwn(state.swipes[me] ?? {}, person))
    ctx.refuse(`you already swiped on ${person}; pick someone from next`);
}

function free(state: Dating, me: string, person: string, ctx: Ctx) {
  if (taken(state, me, ctx.read)) ctx.refuse("you hold a match and this world allows one at a time; unmatch first");
  if (taken(state, person, ctx.read))
    ctx.refuse(`${person} is not in your next; pick someone from next in read dating`);
}

const nudge = (id: string, ctx: Ctx) => {
  const talk = ctx.read("messages") === null ? "" : ` Write to them in messages: send to ${id}.`;
  return `You matched with ${nameOf(ctx.read, id)}.${talk}`;
};

function matched(state: Dating, a: string, b: string, ctx: Ctx): Dating {
  ctx.wish(notice(a, nudge(b, ctx)));
  ctx.wish(notice(b, nudge(a, ctx)));
  return { ...state, matches: { ...state.matches, [pairOf(a, b)]: { between: [a, b].sort(), at: ctx.at } } };
}

const meets = (state: Dating, me: string, person: string) => {
  const back = state.swipes[person]?.[me];
  return state.dials.mutual ? back === true : back !== false;
};

const passes = (marks: Record<string, boolean>) =>
  Object.fromEntries(Object.entries(marks).filter(([, like]) => !like));

const unliked = (swipes: Dating["swipes"], me: string) =>
  Object.fromEntries(
    Object.entries(swipes).map(([id, marks]) => [
      id,
      id === me ? passes(marks) : marks[me] === true ? without(marks, me) : marks,
    ]),
  );

const marked = (state: Dating, me: string, person: string, like: boolean, at: number): Dating => ({
  ...state,
  swipes: { ...state.swipes, [me]: { ...state.swipes[me], [person]: like } },
  spent: { ...state.spent, [me]: { day: dayOf(at), n: used(state, me, at) + 1 } },
});

const matching = (knob: Knob) =>
  [
    knob("mutual")
      ? "A like stays hidden until both of you like each other."
      : "One like is a match unless they passed on you.",
    knob("monogamy") ? "This world allows one match at a time." : "",
  ]
    .join(" ")
    .trim();

const joining = (most: number): Schema => ({
  type: "object",
  properties: {
    line: { type: "string", minLength: 1, maxLength: most },
    seeking: { type: "string", maxLength: most },
  },
  required: ["line"],
  additionalProperties: false,
});

const someone = { type: "string", minLength: 1, maxLength: PERSON } satisfies Schema;

const swiping: Schema = {
  type: "object",
  properties: { person: someone, like: { type: "boolean" } },
  required: ["person", "like"],
  additionalProperties: false,
};

const naming: Schema = {
  type: "object",
  properties: { person: someone },
  required: ["person"],
  additionalProperties: false,
};

const EMPTY: Schema = { type: "object", additionalProperties: false };

export default app<Dating>({
  id: ID,
  title: "Dating",
  line: "Opt in with a profile and swipe on others; a mutual like is a match, and the next step is messages.",
  boot: ({ knob }) => ({
    dials: dialed(knob),
    profiles: {},
    swipes: {},
    matches: {},
    spent: {},
  }),
  verbs: {
    join: {
      line: (knob) =>
        `Opt in, or change your profile: a line of 1 to ${knob("profile")} characters, and what you seek if you like. Nobody appears here without this.`,
      args: (knob) => joining(knob("profile")),
      run: (state, args, ctx) => {
        closed(state, ctx);
        const me = ctx.who.id;
        const line = String(args.line).trim();
        if (!line) return ctx.refuse("the line is only spaces; say something about yourself");
        const seeking = String(args.seeking ?? state.profiles[me]?.seeking ?? "").trim();
        const now = settled(state, ctx);
        ctx.say({ note: joined(now, me) ? "Your profile changed" : "You joined Dating" });
        return { ...now, profiles: { ...now.profiles, [me]: { line, seeking } } };
      },
    },
    leave: {
      line: "Leave Dating: your profile, likes and matches go, and your matches are told. Passes stay, also if you come back.",
      args: EMPTY,
      run: (state, _args, ctx) => {
        const me = ctx.who.id;
        const now = settled(state, ctx);
        if (!joined(now, me)) return ctx.refuse("you are not in Dating; join to opt in");
        for (const one of heldBy(now, me, ctx.read))
          ctx.wish(notice(otherIn(one, me), `${nameOf(ctx.read, me)} left Dating, so your match is over.`));
        ctx.say({ note: "You left Dating" });
        return {
          ...now,
          profiles: without(now.profiles, me),
          swipes: unliked(now.swipes, me),
          matches: Object.fromEntries(Object.entries(now.matches).filter(([, one]) => !one.between.includes(me))),
        };
      },
    },
    swipe: {
      line: (knob) =>
        `Swipe on someone from next in read dating: like true or false, ${knob("swipes")} swipes a day. ${matching(knob)}`,
      args: swiping,
      run: (state, args, ctx) => {
        closed(state, ctx);
        const me = ctx.who.id;
        const them = String(args.person);
        const like = args.like === true;
        const now = settled(state, ctx);
        checked(now, me, them, ctx);
        if (!left(now, me, ctx.at))
          return ctx.refuse(`no swipes left today, at ${now.dials.swipes} a day; more tomorrow`);
        if (like) free(now, me, them, ctx);
        const next = marked(now, me, them, like, ctx.at);
        if (like && meets(now, me, them)) {
          ctx.say({ note: `You matched with ${them}`, person: them, matched: true });
          return matched(next, me, them, ctx);
        }
        ctx.say({ note: `${like ? "Liked" : "Passed on"} ${them}`, person: them, matched: false });
        if (like && now.dials.reveal)
          ctx.wish(notice(them, `${nameOf(ctx.read, me)} (${me}) liked you; swipe on them in dating to answer.`));
        return next;
      },
    },
    unmatch: {
      line: "End a match: they are told, and neither of you comes up for the other again.",
      args: naming,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const them = String(args.person);
        const now = settled(state, ctx);
        const key = pairOf(me, them);
        if (!now.matches[key]) return ctx.refuse(`you have no match with ${them}; see matches in read dating`);
        ctx.wish(notice(them, `${nameOf(ctx.read, me)} unmatched you.`));
        ctx.say({ note: `You unmatched ${them}`, person: them });
        return {
          ...now,
          matches: without(now.matches, key),
          swipes: {
            ...now.swipes,
            [me]: { ...now.swipes[me], [them]: false },
            [them]: { ...now.swipes[them], [me]: false },
          },
        };
      },
    },
  },
  moves: (state) => (Object.keys(state.profiles).length ? ["join", "leave", "swipe", "unmatch"] : ["join"]),
  read: sliced(slice, whole, true),
  knobs: KNOBS,
});
