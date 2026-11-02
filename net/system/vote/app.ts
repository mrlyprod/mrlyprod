import type { Ctx, Schema } from "../../sdk/system.ts";
import { app, sliced } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Choice = "yes" | "no";

export type Result = "carried" | "failed";

export type Ballot = {
  n: number;
  by: string;
  question: string;
  wish: number | null;
  opened: number;
  closes: number;
  votes: Record<string, Choice>;
  tally: Record<Choice, number>;
  result: Result | null;
};

export type Votes = { ballots: Record<string, Ballot>; n: number };

export const QUESTION = 280;

export const SHORTEST = 10;

export const LONGEST = 30 * 24 * 3600;

export const OPENED = 3;

const WISHES = 1_000_000;

const CHOICES: Choice[] = ["yes", "no"];

/* TIME */

export function span(ms: number): string {
  const s = Math.ceil(ms / 1000);
  if (s < 120) return `${s} seconds`;
  if (s < 7200) return `${Math.round(s / 60)} minutes`;
  if (s < 172800) return `${Math.round(s / 3600)} hours`;
  return `${Math.round(s / 86400)} days`;
}

/* BALLOTS */

const ballotOf = (state: Votes, args: { ballot?: unknown }) => state.ballots[String(args.ballot)]!;

const due = (one: Ballot, ctx: Ctx) => ctx.at >= one.closes;

const mine = (state: Votes, id: string) => ({
  votes: Object.fromEntries(
    Object.values(state.ballots)
      .filter((one) => id in one.votes)
      .map((one) => [one.n, one.votes[id]!]),
  ),
  opened: Object.values(state.ballots)
    .filter((one) => one.by === id)
    .map((one) => one.n),
});

/* SCHEMAS */

const opening: Schema = {
  type: "object",
  properties: {
    question: { type: "string", minLength: 1, maxLength: QUESTION },
    wish: { type: "integer", minimum: 1, maximum: WISHES },
    seconds: { type: "integer", minimum: SHORTEST, maximum: LONGEST },
  },
  required: ["question", "seconds"],
  additionalProperties: false,
};

const naming = (state: Votes, more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { ballot: { type: "integer", minimum: 1, maximum: Math.max(1, state.n) }, ...more },
  required: ["ballot", ...Object.keys(more)],
  additionalProperties: false,
});

/* APP */

export default app<Votes>({
  id: "vote",
  title: "Vote",
  line: "Ballots on wishes: yes or no, one vote each, a closing time in world time, a tally anyone reads.",
  boot: () => ({ ballots: {}, n: 0 }),
  verbs: {
    open: {
      line: `Put a question, or a wish by its number, to a vote that closes ${SHORTEST} seconds to 30 days from now.`,
      args: opening,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const question = String(args.question).trim();
        const wish = args.wish === undefined ? null : Number(args.wish);
        if (!question) return ctx.refuse("the question is only spaces; ask something");
        const live = Object.values(state.ballots).filter((one) => one.result === null);
        if (live.filter((one) => one.by === me).length >= OPENED)
          return ctx.refuse(`you have ${OPENED} ballots open; close one first`);
        const twin = wish === null ? undefined : live.find((one) => one.wish === wish);
        if (twin) return ctx.refuse(`ballot ${twin.n} already decides wish ${wish}`);
        const n = state.n + 1;
        const ballot: Ballot = {
          n,
          by: me,
          question,
          wish,
          opened: ctx.at,
          closes: ctx.at + Number(args.seconds) * 1000,
          votes: {},
          tally: { yes: 0, no: 0 },
          result: null,
        };
        return { n, ballots: { ...state.ballots, [n]: ballot } };
      },
    },
    cast: {
      line: "Vote yes or no on an open ballot; one vote each, final.",
      args: (state) => naming(state, { choice: { enum: CHOICES } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = ballotOf(state, args);
        const choice = args.choice as Choice;
        if (due(one, ctx)) return ctx.refuse(`ballot ${one.n} has closed; its tally is final`);
        if (me in one.votes) return ctx.refuse(`you already voted ${one.votes[me]} on ballot ${one.n}; one vote each`);
        const next = {
          ...one,
          votes: { ...one.votes, [me]: choice },
          tally: { ...one.tally, [choice]: one.tally[choice] + 1 },
        };
        return { ...state, ballots: { ...state.ballots, [one.n]: next } };
      },
    },
    close: {
      line: "Close a ballot whose time has come: more yes than no carries it, anything else fails it.",
      args: (state) => naming(state),
      run: (state, args, ctx) => {
        const one = ballotOf(state, args);
        if (one.result) return ctx.refuse(`ballot ${one.n} is already closed: ${one.result}`);
        if (!due(one, ctx)) return ctx.refuse(`ballot ${one.n} is open for ${span(one.closes - ctx.at)} more`);
        const result: Result = one.tally.yes > one.tally.no ? "carried" : "failed";
        return { ...state, ballots: { ...state.ballots, [one.n]: { ...one, result } } };
      },
    },
  },
  moves: (state) => (state.n ? ["open", "cast", "close"] : ["open"]),
  read: sliced(mine),
  bot,
});
