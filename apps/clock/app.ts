import type { Ctx, Knob, Knobs, Obj, Schema, Verb, Who } from "@mrlyprod/os/contract/index.ts";
import { app, sliced } from "@mrlyprod/os/contract/index.ts";
import { notice } from "../shared/people.ts";

export type Alarm = { id: number; at: number; label: string; daily: boolean };

export type Timer = { id: number; at: number; seconds: number; label: string };

export type Rung = { id: number; at: number; kind: "alarm" | "timer"; label: string };

export type Dial = { alarms: Alarm[]; timers: Timer[]; rung: Rung[] };

export type Clock = { next: number; keep: number; dials: Record<string, Dial> };

const KNOBS = {
  alarms: {
    line: "Alarms one person keeps.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 20,
  },
  timers: {
    line: "Timers one person runs.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 20,
  },
  rung: {
    line: "Rung alarms and timers one person's list keeps.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 20,
  },
  seconds: {
    line: "Seconds the longest timer runs.",
    schema: { type: "integer", minimum: 60, maximum: 31_536_000 },
    reality: 86_400,
  },
  label: {
    line: "Characters in an alarm's or a timer's label.",
    schema: { type: "integer", minimum: 8, maximum: 200 },
    reality: 40,
  },
} satisfies Knobs;

export const DAY = 86_400_000;

export const LAST = 8_640_000_000_000_000;

const MINUTE = 60_000;

const MINUTES = 5_256_000;

const WEEKDAYS = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

const EMPTY: Dial = { alarms: [], timers: [], rung: [] };

const iso = (at: number) => new Date(Math.min(at, LAST)).toISOString();

export const time = (at: number) => ({
  at,
  iso: iso(at),
  weekday: WEEKDAYS[new Date(Math.min(at, LAST)).getUTCDay()]!,
  day: Math.floor(at / DAY),
  midnight: (Math.floor(at / DAY) + 1) * DAY,
});

function settled(dial: Dial, at: number, keep: number): { dial: Dial; rung: Rung[] } {
  const rung: Rung[] = [];
  const alarms = dial.alarms.flatMap((alarm) => {
    if (alarm.at > at) return [alarm];
    rung.push({ id: alarm.id, at: alarm.at, kind: "alarm", label: alarm.label });
    return alarm.daily ? [{ ...alarm, at: alarm.at + (Math.floor((at - alarm.at) / DAY) + 1) * DAY }] : [];
  });
  const timers = dial.timers.filter((timer) => {
    if (timer.at > at) return true;
    rung.push({ id: timer.id, at: timer.at, kind: "timer", label: timer.label });
    return false;
  });
  if (!rung.length) return { dial, rung };
  rung.sort((a, b) => a.at - b.at || a.id - b.id);
  return { dial: { alarms, timers, rung: [...dial.rung, ...rung].slice(-keep) }, rung };
}

function ring(dial: Dial, owner: string, state: Clock, ctx: Ctx): Dial {
  const done = settled(dial, ctx.at, state.keep);
  for (const one of done.rung)
    ctx.wish(notice(owner, `${one.kind === "alarm" ? "Alarm" : "Timer done"}: ${one.label}`, true));
  return done.dial;
}

export const fire = (state: Clock, ctx: Ctx): Clock => ({
  ...state,
  dials: Object.fromEntries(Object.entries(state.dials).map(([owner, dial]) => [owner, ring(dial, owner, state, ctx)])),
});

const firing =
  (run: Verb<Clock>["run"]): Verb<Clock>["run"] =>
  (state, args, ctx) =>
    run(fire(state, ctx), args, ctx);

const due = (state: Clock, at: number) =>
  Object.values(state.dials).some(
    (dial) => dial.alarms.some((one) => one.at <= at) || dial.timers.some((one) => one.at <= at),
  );

const dialOf = (state: Clock, me: string) => state.dials[me] ?? EMPTY;

const put = (state: Clock, me: string, dial: Dial): Clock => ({ ...state, dials: { ...state.dials, [me]: dial } });

const labelOf = (value: unknown, fallback: string) => (typeof value === "string" && value.trim() ? value : fallback);

const label = (knob: Knob): Schema => ({ type: "string", minLength: 1, maxLength: knob("label") });

const mine = (state: Clock, id: string, _who: Who | null, at: number) => {
  const { alarms, timers, rung } = settled(dialOf(state, id), at, state.keep).dial;
  return {
    ...time(at),
    alarms: alarms.map((one) => ({ ...one, iso: iso(one.at) })),
    timers: timers.map((one) => ({ ...one, iso: iso(one.at) })),
    rung,
  };
};

const open = (state: Clock, who: Who | null, at: number) => (who ? mine(state, who.id, who, at) : time(at));

const whenOf = (args: Obj, ctx: Ctx) => {
  const now = `it is ${iso(ctx.at)}, world time ${ctx.at}`;
  if (args.at === undefined && args.in === undefined)
    return ctx.refuse(`say when: at, in world milliseconds, or in, in minutes from now; ${now}`);
  const at = args.at === undefined ? ctx.at + Number(args.in) * MINUTE : Number(args.at);
  return at > ctx.at ? at : ctx.refuse(`an alarm rings in the future; ${now}`);
};

export default app<Clock>({
  id: "clock",
  title: "Clock",
  line: "The world's time, and your alarms and timers, which ring by themselves as soon as they are due.",
  boot: ({ knob }) => ({ next: 1, keep: knob("rung"), dials: {} }),
  knobs: KNOBS,
  verbs: {
    alarm: {
      line: (knob) =>
        `Set an alarm: at a world time in milliseconds (read clock for the time now), or in some minutes from now; daily if asked; you keep at most ${knob("alarms")}. You get a notice when it rings.`,
      args: (knob) => ({
        type: "object",
        properties: {
          at: { type: "integer", minimum: 0, maximum: LAST },
          in: { type: "integer", minimum: 1, maximum: MINUTES },
          label: label(knob),
          daily: { type: "boolean" },
        },
        additionalProperties: false,
      }),
      run: firing((state, args, ctx) => {
        const me = ctx.who.id;
        const dial = dialOf(state, me);
        const at = whenOf(args, ctx);
        const most = ctx.knob("alarms");
        if (dial.alarms.length >= most) return ctx.refuse(`you keep ${most} alarms; cancel one first`);
        const alarm: Alarm = { id: state.next, at, label: labelOf(args.label, "Alarm"), daily: args.daily === true };
        ctx.say({ note: `Alarm ${alarm.id} rings at ${iso(at)}: ${alarm.label}`, id: alarm.id, at });
        return { ...put(state, me, { ...dial, alarms: [...dial.alarms, alarm] }), next: state.next + 1 };
      }),
    },
    timer: {
      line: (knob) =>
        `Start a timer of 1 to ${knob("seconds")} seconds of world time; you run at most ${knob("timers")}. You get a notice when it is done.`,
      args: (knob) => ({
        type: "object",
        properties: { seconds: { type: "integer", minimum: 1, maximum: knob("seconds") }, label: label(knob) },
        required: ["seconds"],
        additionalProperties: false,
      }),
      run: firing((state, args, ctx) => {
        const me = ctx.who.id;
        const dial = dialOf(state, me);
        const seconds = Number(args.seconds);
        const most = ctx.knob("timers");
        if (dial.timers.length >= most) return ctx.refuse(`you run ${most} timers; cancel one first`);
        const timer: Timer = {
          id: state.next,
          at: ctx.at + seconds * 1000,
          seconds,
          label: labelOf(args.label, "Timer"),
        };
        ctx.say({ note: `Timer ${timer.id} is done at ${iso(timer.at)}: ${timer.label}`, id: timer.id, at: timer.at });
        return { ...put(state, me, { ...dial, timers: [...dial.timers, timer] }), next: state.next + 1 };
      }),
    },
    cancel: {
      line: "Cancel an alarm or a timer of yours.",
      args: () => ({
        type: "object",
        properties: { id: { type: "integer", minimum: 1 } },
        required: ["id"],
        additionalProperties: false,
      }),
      run: firing((state, args, ctx) => {
        const me = ctx.who.id;
        const id = Number(args.id);
        const dial = dialOf(state, me);
        if (![...dial.alarms, ...dial.timers].some((each) => each.id === id))
          return ctx.refuse(`you have no alarm or timer ${id}; it rang, was cancelled, or is not yours`);
        ctx.say({ note: `Cancelled ${id}`, id });
        return put(state, me, {
          ...dial,
          alarms: dial.alarms.filter((one) => one.id !== id),
          timers: dial.timers.filter((one) => one.id !== id),
        });
      }),
    },
    clear: {
      line: "Clear your list of what has rung.",
      run: firing((state, _args, ctx) => {
        ctx.say({ note: "Cleared what has rung" });
        return put(state, ctx.who.id, { ...dialOf(state, ctx.who.id), rung: [] });
      }),
    },
    tick: {
      inner: true,
      line: "Ring every alarm and timer due by now, for everyone; it happens by itself whenever anyone acts.",
      run: firing((state, _args, ctx) => {
        ctx.say({ note: "Rang everything due" });
        return state;
      }),
    },
  },
  read: sliced(mine, open),
  wake: (state, at) => (due(state, at) ? "tick" : null),
});
