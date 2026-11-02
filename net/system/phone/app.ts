import type { Ctx, Schema, Verb } from "../../sdk/system.ts";
import { app, notice, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Status = "ringing" | "active" | "ended" | "declined" | "missed";

export type Call = {
  id: number;
  from: string;
  to: string;
  status: Status;
  at: number;
  answered: number | null;
  ended: number | null;
};

export type Calls = {
  people: string[] | null;
  next: number;
  calls: Record<string, Call>;
  logs: Record<string, number[]>;
};

export const RING = 30_000;

export const LOG = 50;

const ID = 40;

/* CALLS */

export const live = (call: Call) => call.status === "ringing" || call.status === "active";

const lineOf = (state: Calls, id: string) =>
  Object.values(state.calls).find((call) => live(call) && (call.from === id || call.to === id));

function expire(state: Calls, at: number, ctx: Ctx): Calls {
  const late = Object.values(state.calls).filter((call) => call.status === "ringing" && call.at + RING <= at);
  if (!late.length) return state;
  const calls = { ...state.calls };
  for (const call of late) {
    calls[call.id] = { ...call, status: "missed", ended: call.at + RING };
    ctx.wish(notice(call.to, `Missed call from ${call.from}`));
  }
  return { ...state, calls };
}

const ringing =
  (run: Verb<Calls>["run"]): Verb<Calls>["run"] =>
  (state, args, ctx) =>
    run(expire(state, ctx.at, ctx), args, ctx);

const callOf = (state: Calls, id: number, me: string, refuse: Ctx["refuse"]): Call => {
  const call = state.calls[id];
  if (!call) return refuse(`no call ${id}; it has left the logs`);
  if (call.from !== me && call.to !== me) return refuse(`call ${id} is between ${call.from} and ${call.to}`);
  return call;
};

const set = (state: Calls, call: Call): Calls => ({ ...state, calls: { ...state.calls, [call.id]: call } });

function logged(state: Calls, call: Call): Calls {
  const logs = { ...state.logs };
  const calls: Record<string, Call> = { ...state.calls, [call.id]: call };
  for (const id of [call.from, call.to]) logs[id] = [...(logs[id] ?? []), call.id].slice(-LOG);
  for (const key of Object.keys(calls))
    if (!live(calls[key]!) && ![calls[key]!.from, calls[key]!.to].some((id) => logs[id]?.includes(Number(key))))
      delete calls[key];
  return { ...state, calls, logs };
}

const mine = (state: Calls, id: string) => {
  const log = [...(state.logs[id] ?? [])].reverse().flatMap((key) => (state.calls[key] ? [state.calls[key]!] : []));
  return {
    live: lineOf(state, id) ?? null,
    missed: log.filter((call) => call.status === "missed" && call.to === id).length,
    log,
  };
};

/* SCHEMAS */

const naming = (state: Calls): Schema => ({
  type: "object",
  properties: { id: { type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) } },
  required: ["id"],
  additionalProperties: false,
});

/* APP */

export default app<Calls>({
  id: "phone",
  title: "Phone",
  line: "Calls between two users, no audio: ring, accept or decline, end; a log per user, missed calls counted.",
  boot: ({ actors }) => ({ people: actors, next: 1, calls: {}, logs: {} }),
  verbs: {
    ring: {
      line: `Call someone; a call nobody accepts within ${RING / 1000} seconds is missed.`,
      args: (state) => ({
        type: "object",
        properties: { to: state.people ? { enum: state.people } : { type: "string", minLength: 1, maxLength: ID } },
        required: ["to"],
        additionalProperties: false,
      }),
      run: ringing((state, args, ctx) => {
        const me = ctx.who.id;
        const to = String(args.to);
        if (to === me) return ctx.refuse("a call is between two people; call someone else");
        const mineNow = lineOf(state, me);
        if (mineNow) return ctx.refuse(`you are on call ${mineNow.id}; end it first`);
        if (lineOf(state, to)) return ctx.refuse(`${to} is on another call`);
        const call: Call = { id: state.next, from: me, to, status: "ringing", at: ctx.at, answered: null, ended: null };
        return { ...logged(state, call), next: state.next + 1 };
      }),
    },
    accept: {
      line: "Accept a call ringing for you.",
      args: naming,
      run: ringing((state, args, ctx) => {
        const call = callOf(state, Number(args.id), ctx.who.id, ctx.refuse);
        if (call.to !== ctx.who.id) return ctx.refuse(`only ${call.to} answers call ${call.id}`);
        if (call.status !== "ringing") return ctx.refuse(`call ${call.id} is ${call.status}, not ringing`);
        return set(state, { ...call, status: "active", answered: ctx.at });
      }),
    },
    decline: {
      line: "Decline a call ringing for you.",
      args: naming,
      run: ringing((state, args, ctx) => {
        const call = callOf(state, Number(args.id), ctx.who.id, ctx.refuse);
        if (call.to !== ctx.who.id) return ctx.refuse(`only ${call.to} answers call ${call.id}`);
        if (call.status !== "ringing") return ctx.refuse(`call ${call.id} is ${call.status}, not ringing`);
        return set(state, { ...call, status: "declined", ended: ctx.at });
      }),
    },
    end: {
      line: "Hang up a call of yours: an active one ends, one still ringing is missed.",
      args: naming,
      run: ringing((state, args, ctx) => {
        const call = callOf(state, Number(args.id), ctx.who.id, ctx.refuse);
        if (!live(call)) return ctx.refuse(`call ${call.id} is already ${call.status}`);
        if (call.status === "ringing" && call.to === ctx.who.id)
          return ctx.refuse(`call ${call.id} is ringing for you; accept or decline it`);
        if (call.status === "ringing") ctx.wish(notice(call.to, `Missed call from ${call.from}`));
        return set(state, { ...call, status: call.status === "active" ? "ended" : "missed", ended: ctx.at });
      }),
    },
  },
  moves: (state) => (state.next > 1 ? ["ring", "accept", "decline", "end"] : ["ring"]),
  read: sliced(mine),
  bot,
});
