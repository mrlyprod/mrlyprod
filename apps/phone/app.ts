import type { Ctx, Knob, Knobs, Obj, Schema, Verb, Who } from "@mrlyprod/os/contract/index.ts";
import { app, SLICE, sliced } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, mutedBy, notice, reach, someone } from "../shared/people.ts";

export type Status = "ringing" | "active" | "ended" | "declined" | "missed";

export type Said = { from: string; text: string; at: number };

export type Voicemail = { text: string; at: number; heard: boolean };

export type Call = {
  id: number;
  from: string;
  to: string;
  status: Status;
  at: number;
  deadline: number;
  answered: number | null;
  ended: number | null;
  said: Said[];
  voicemail: Voicemail | null;
};

export type Calls = {
  people: string[] | null;
  next: number;
  calls: Record<string, Call>;
  logs: Record<string, number[]>;
  open: number[];
};

const KNOBS = {
  ring: {
    line: "Milliseconds a call rings before it is missed.",
    schema: { type: "integer", minimum: 1000, maximum: 3_600_000 },
    reality: 30_000,
  },
  log: {
    line: "Calls each person's log keeps.",
    schema: { type: "integer", minimum: 5, maximum: 5000 },
    reality: 50,
  },
  text: {
    line: "Characters in one thing said on a call, or in one voicemail.",
    schema: { type: "integer", minimum: 20, maximum: 10_000 },
    reality: 500,
  },
  lines: {
    line: "Things said that one call keeps.",
    schema: { type: "integer", minimum: 5, maximum: 1000 },
    reality: 100,
  },
  voicemail: {
    line: "Whether a caller can leave a voicemail on a call nobody answered.",
    schema: { type: "boolean" },
    reality: true,
  },
} satisfies Knobs;

const ID = 40;

const SHOWN = 5;

const TALK = 10;

export const live = (call: Call) => call.status === "ringing" || call.status === "active";

export const current = (call: Call, at: number): Call =>
  call.status === "ringing" && call.deadline <= at ? { ...call, status: "missed", ended: call.deadline } : call;

const opened = (state: Calls) => state.open.map((id) => state.calls[id]!);

const lineOf = (state: Calls, id: string) => opened(state).find((call) => call.from === id || call.to === id);

const put = (state: Calls, call: Call): Calls => ({
  ...state,
  calls: { ...state.calls, [call.id]: call },
  open: live(call) ? [...new Set([...state.open, call.id])] : state.open.filter((id) => id !== call.id),
});

const shut = (ctx: Ctx, to: string, from: string) =>
  blockedBy(ctx, to, from) || blockedBy(ctx, from, to) || mutedBy(ctx, to, from);

const unanswered = (call: Call, ctx: Ctx) => {
  const offer = ctx.knob<boolean>("voicemail") ? "; you can leave a voicemail" : "";
  if (!shut(ctx, call.to, call.from)) ctx.wish(notice(call.to, `Missed call from ${call.from}`, true));
  ctx.wish(notice(call.from, `${call.to} did not answer call ${call.id}${offer}`, true));
};

function expire(state: Calls, ctx: Ctx): Calls {
  let next = state;
  for (const call of opened(state)) {
    const now = current(call, ctx.at);
    if (now === call) continue;
    unanswered(now, ctx);
    next = put(next, now);
  }
  return next;
}

const ringing =
  (run: Verb<Calls>["run"]): Verb<Calls>["run"] =>
  (state, args, ctx) =>
    run(expire(state, ctx), args, ctx);

const callOf = (state: Calls, id: number, me: string, refuse: Ctx["refuse"]): Call => {
  const call = state.calls[id];
  if (!call || (call.from !== me && call.to !== me))
    return refuse(`call ${id} is not in your log; read phone lists yours`);
  return call;
};

const answering = (state: Calls, args: Obj, ctx: Ctx): Call => {
  const call = callOf(state, Number(args.id), ctx.who.id, ctx.refuse);
  if (call.to !== ctx.who.id) return ctx.refuse(`call ${call.id} is yours to end, not to answer`);
  if (call.status !== "ringing") return ctx.refuse(`call ${call.id} is ${call.status}, not ringing`);
  return call;
};

function logged(state: Calls, call: Call, keep: number): Calls {
  const next = put(state, call);
  const logs = { ...next.logs };
  const calls = { ...next.calls };
  for (const id of [call.from, call.to]) logs[id] = [...(logs[id] ?? []), call.id].slice(-keep);
  for (const key of Object.keys(calls))
    if (!live(calls[key]!) && ![calls[key]!.from, calls[key]!.to].some((id) => logs[id]?.includes(Number(key))))
      delete calls[key];
  return { ...next, calls, logs };
}

const row = ({ id, from, to, status, at, answered, ended, said, voicemail }: Call) => ({
  id,
  from,
  to,
  status,
  at,
  answered,
  ended,
  said: said.length,
  voicemail: voicemail !== null,
});

const leftFor = (me: string) => (call: Call) =>
  call.to === me && call.voicemail
    ? [
        {
          call: call.id,
          from: call.from,
          at: call.voicemail.at,
          text: call.voicemail.text,
          heard: call.voicemail.heard,
        },
      ]
    : [];

const view = (state: Calls, id: string, at: number, deep: boolean) => {
  const log = [...(state.logs[id] ?? [])]
    .reverse()
    .flatMap((key) => (state.calls[key] ? [current(state.calls[key]!, at)] : []));
  const left = log.flatMap(leftFor(id));
  const unheard = left.filter((one) => !one.heard);
  const line = log.find(live) ?? null;
  const counts = {
    missed: log.filter((call) => call.status === "missed" && call.to === id).length,
    unheard: unheard.length,
  };
  if (deep)
    return {
      live: line,
      ...counts,
      log: log.map(row),
      voicemails: left,
      calls: Object.fromEntries(log.map((call) => [call.id, call])),
    };
  return {
    live: line && { ...line, said: line.said.slice(-TALK) },
    ...counts,
    log: log.slice(0, SHOWN).map(row),
    voicemails: unheard.slice(0, SHOWN),
    more: "phone/log, phone/voicemails, phone/calls/<id>",
  };
};

const totals = (state: Calls) => ({ calls: state.next - 1, live: state.open.length });

const deepens = (path: string[]) => path.length > (path[0]?.startsWith(SLICE) ? 1 : 0);

const naming: Schema = {
  type: "object",
  properties: { id: { type: "integer", minimum: 1 } },
  required: ["id"],
  additionalProperties: false,
};

const talking = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("text") });

export default app<Calls>({
  id: "phone",
  title: "Phone",
  line: "Calls between two users: ring, accept or decline, talk, end; voicemail when nobody answers; a private log each.",
  boot: ({ actors }) => ({ people: actors, next: 1, calls: {}, logs: {}, open: [] }),
  knobs: KNOBS,
  verbs: {
    ring: {
      line: (knob) =>
        `Call someone; they are told at once. A call not accepted by its deadline, ${knob("ring") / 1000} seconds away, is missed.`,
      args: {
        type: "object",
        properties: { to: { type: "string", minLength: 1, maxLength: ID } },
        required: ["to"],
        additionalProperties: false,
      },
      run: ringing((state, args, ctx) => {
        const me = ctx.who.id;
        const to = someone(state.people, String(args.to), ctx);
        if (to === me) return ctx.refuse("a call is between two people; call someone else");
        reach(ctx, to);
        const mineNow = lineOf(state, me);
        if (mineNow) return ctx.refuse(`you are on call ${mineNow.id}; end it first`);
        if (lineOf(state, to)) return ctx.refuse(`${to} cannot take a call now; try later, or send a message`);
        const call: Call = {
          id: state.next,
          from: me,
          to,
          status: "ringing",
          at: ctx.at,
          deadline: ctx.at + ctx.knob("ring"),
          answered: null,
          ended: null,
          said: [],
          voicemail: null,
        };
        ctx.wish(notice(to, `${me} is calling you: call ${call.id}; accept or decline it`));
        ctx.say({ note: `Ringing ${to}: call ${call.id}`, id: call.id, deadline: call.deadline });
        return { ...logged(state, call, ctx.knob("log")), next: state.next + 1 };
      }),
    },
    accept: {
      line: "Accept a call ringing for you.",
      args: naming,
      run: ringing((state, args, ctx) => {
        const call = answering(state, args, ctx);
        reach(ctx, call.from);
        ctx.wish(notice(call.from, `${call.to} accepted call ${call.id}; say something`));
        ctx.say({ note: `On call ${call.id} with ${call.from}`, id: call.id });
        return put(state, { ...call, status: "active", answered: ctx.at });
      }),
    },
    decline: {
      line: "Decline a call ringing for you.",
      args: naming,
      run: ringing((state, args, ctx) => {
        const call = answering(state, args, ctx);
        const offer = ctx.knob<boolean>("voicemail") ? "; you can leave a voicemail" : "";
        ctx.wish(notice(call.from, `${call.to} declined call ${call.id}${offer}`));
        ctx.say({ note: `Declined call ${call.id}`, id: call.id });
        return put(state, { ...call, status: "declined", ended: ctx.at });
      }),
    },
    say: {
      line: (knob) =>
        `Say something on your active call: 1 to ${knob("text")} characters. The other person is told at once; a call keeps its last ${knob("lines")}.`,
      args: (knob) => ({
        type: "object",
        properties: { text: talking(knob) },
        required: ["text"],
        additionalProperties: false,
      }),
      run: ringing((state, args, ctx) => {
        const me = ctx.who.id;
        const text = String(args.text);
        if (!text.trim()) return ctx.refuse("the text is only spaces; say something");
        const call = lineOf(state, me);
        if (!call) return ctx.refuse("you are not on a call; ring someone first");
        if (call.status === "ringing")
          return ctx.refuse(
            call.to === me ? `call ${call.id} is ringing for you; accept it first` : `call ${call.id} is still ringing`,
          );
        const other = call.from === me ? call.to : call.from;
        reach(ctx, other);
        ctx.wish(notice(other, `${me} on call ${call.id}: ${text}`));
        ctx.say({ note: `Said on call ${call.id}`, id: call.id });
        const said = [...call.said, { from: me, text, at: ctx.at }].slice(-ctx.knob("lines"));
        return put(state, { ...call, said });
      }),
    },
    end: {
      line: "Hang up a call of yours: an active one ends, one still ringing is missed.",
      args: naming,
      run: ringing((state, args, ctx) => {
        const me = ctx.who.id;
        const call = callOf(state, Number(args.id), me, ctx.refuse);
        if (!live(call)) return ctx.refuse(`call ${call.id} is already ${call.status}`);
        if (call.status === "ringing" && call.to === me)
          return ctx.refuse(`call ${call.id} is ringing for you; accept or decline it`);
        if (call.status === "ringing") ctx.wish(notice(call.to, `Missed call from ${call.from}`));
        else ctx.wish(notice(call.from === me ? call.to : call.from, `${me} ended call ${call.id}`));
        ctx.say({ note: `Ended call ${call.id}`, id: call.id });
        return put(state, { ...call, status: call.status === "active" ? "ended" : "missed", ended: ctx.at });
      }),
    },
    voicemail: {
      line: (knob) =>
        `Leave a voicemail on a call you made that nobody answered: 1 to ${knob("text")} characters, one per call.`,
      args: (knob) => ({
        type: "object",
        properties: { ...naming.properties, text: talking(knob) },
        required: ["id", "text"],
        additionalProperties: false,
      }),
      run: ringing((state, args, ctx) => {
        const me = ctx.who.id;
        const text = String(args.text);
        if (!ctx.knob<boolean>("voicemail")) return ctx.refuse("this world has no voicemail; send a message instead");
        if (!text.trim()) return ctx.refuse("the text is only spaces; say something");
        const call = callOf(state, Number(args.id), me, ctx.refuse);
        if (call.from !== me) return ctx.refuse(`only the caller leaves a voicemail on call ${call.id}`);
        if (call.status !== "missed" && call.status !== "declined")
          return ctx.refuse(`call ${call.id} is ${call.status}; a voicemail is for a call nobody answered`);
        if (call.voicemail) return ctx.refuse(`call ${call.id} already holds your voicemail`);
        reach(ctx, call.to);
        ctx.wish(notice(call.to, `Voicemail from ${me}: ${text}`));
        ctx.say({ note: `Left a voicemail on call ${call.id}`, id: call.id });
        return put(state, { ...call, voicemail: { text, at: ctx.at, heard: false } });
      }),
    },
    listen: {
      line: "Mark the voicemail left for you on a call as heard; read phone/voicemails holds them all.",
      args: naming,
      run: ringing((state, args, ctx) => {
        const call = callOf(state, Number(args.id), ctx.who.id, ctx.refuse);
        if (call.to !== ctx.who.id || !call.voicemail) return ctx.refuse(`call ${call.id} holds no voicemail for you`);
        if (call.voicemail.heard) return ctx.refuse(`you already heard the voicemail on call ${call.id}`);
        ctx.say({ note: `Voicemail from ${call.from}`, id: call.id, text: call.voicemail.text });
        return put(state, { ...call, voicemail: { ...call.voicemail, heard: true } });
      }),
    },
    settle: {
      inner: true,
      line: "Move every call past its deadline to missed and tell both sides; the phone does this by itself after any act.",
      run: (state, _args, ctx) => expire(state, ctx),
    },
  },
  moves: (state) =>
    state.next > 1
      ? ["ring", "accept", "decline", "say", "end", "voicemail", "listen", ...(state.open.length ? ["settle"] : [])]
      : ["ring"],
  read: (state, path, who, at) =>
    sliced(
      (one: Calls, id: string, _who: Who | null, now: number) => view(one, id, now, deepens(path)),
      (one: Calls, by: Who | null, now: number) => (by ? view(one, by.id, now, deepens(path)) : totals(one)),
    )(state, path, who, at),
  wake: (state, at) => (opened(state).some((call) => current(call, at) !== call) ? "settle" : null),
});
