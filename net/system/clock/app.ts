import type { Ctx, Schema, Verb } from "../../sdk/system.ts";
import { app, notice, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Alarm = { id: number; at: number; label: string; daily: boolean };

export type Timer = { id: number; at: number; seconds: number; label: string };

export type Rung = { id: number; at: number; kind: "alarm" | "timer"; label: string };

export type Dial = { alarms: Alarm[]; timers: Timer[]; rung: Rung[] };

export type Clock = { next: number; dials: Record<string, Dial> };

export const LABEL = 40;

export const SET = 20;

export const RUNG = 20;

export const SECONDS = 86_400;

export const DAY = 86_400_000;

export const LAST = 8_640_000_000_000_000;

/* FIRING */

const EMPTY: Dial = { alarms: [], timers: [], rung: [] };

function ring(dial: Dial, owner: string, at: number, ctx: Ctx): Dial {
  const rung: Rung[] = [];
  const alarms = dial.alarms.flatMap((alarm) => {
    if (alarm.at > at) return [alarm];
    rung.push({ id: alarm.id, at: alarm.at, kind: "alarm", label: alarm.label });
    ctx.wish(notice(owner, `Alarm: ${alarm.label}`));
    return alarm.daily ? [{ ...alarm, at: alarm.at + (Math.floor((at - alarm.at) / DAY) + 1) * DAY }] : [];
  });
  const timers = dial.timers.filter((timer) => {
    if (timer.at > at) return true;
    rung.push({ id: timer.id, at: timer.at, kind: "timer", label: timer.label });
    ctx.wish(notice(owner, `Timer done: ${timer.label}`));
    return false;
  });
  if (!rung.length) return dial;
  return { alarms, timers, rung: [...dial.rung, ...rung.sort((a, b) => a.at - b.at || a.id - b.id)].slice(-RUNG) };
}

export const fire = (state: Clock, at: number, ctx: Ctx): Clock => ({
  ...state,
  dials: Object.fromEntries(Object.entries(state.dials).map(([owner, dial]) => [owner, ring(dial, owner, at, ctx)])),
});

const firing =
  (run: Verb<Clock>["run"]): Verb<Clock>["run"] =>
  (state, args, ctx) =>
    run(fire(state, ctx.at, ctx), args, ctx);

const dialOf = (state: Clock, me: string) => state.dials[me] ?? EMPTY;

const put = (state: Clock, me: string, dial: Dial): Clock => ({ ...state, dials: { ...state.dials, [me]: dial } });

const labelOf = (value: unknown, fallback: string) => (typeof value === "string" && value.trim() ? value : fallback);

/* SCHEMAS */

const label: Schema = { type: "string", minLength: 1, maxLength: LABEL };

/* APP */

export default app<Clock>({
  id: "clock",
  title: "Clock",
  line: "Alarms and timers per user, ringing on world time; each act first rings whatever is due.",
  boot: () => ({ next: 1, dials: {} }),
  verbs: {
    alarm: {
      line: `Set an alarm at a world time in milliseconds, daily if asked; you keep at most ${SET}.`,
      args: {
        type: "object",
        properties: { at: { type: "integer", minimum: 0, maximum: LAST }, label, daily: { type: "boolean" } },
        required: ["at"],
        additionalProperties: false,
      },
      run: firing((state, args, ctx) => {
        const me = ctx.who.id;
        const dial = dialOf(state, me);
        const at = Number(args.at);
        if (at <= ctx.at) return ctx.refuse(`an alarm rings in the future; the world's time is ${ctx.at}`);
        if (dial.alarms.length >= SET) return ctx.refuse(`you keep ${SET} alarms; cancel one first`);
        const alarm: Alarm = { id: state.next, at, label: labelOf(args.label, "Alarm"), daily: args.daily === true };
        return { ...put(state, me, { ...dial, alarms: [...dial.alarms, alarm] }), next: state.next + 1 };
      }),
    },
    timer: {
      line: `Start a timer of 1 to ${SECONDS} seconds; you run at most ${SET}.`,
      args: {
        type: "object",
        properties: { seconds: { type: "integer", minimum: 1, maximum: SECONDS }, label },
        required: ["seconds"],
        additionalProperties: false,
      },
      run: firing((state, args, ctx) => {
        const me = ctx.who.id;
        const dial = dialOf(state, me);
        const seconds = Number(args.seconds);
        if (dial.timers.length >= SET) return ctx.refuse(`you run ${SET} timers; cancel one first`);
        const timer: Timer = {
          id: state.next,
          at: ctx.at + seconds * 1000,
          seconds,
          label: labelOf(args.label, "Timer"),
        };
        return { ...put(state, me, { ...dial, timers: [...dial.timers, timer] }), next: state.next + 1 };
      }),
    },
    cancel: {
      line: "Cancel an alarm or a timer of yours.",
      args: (state) => ({
        type: "object",
        properties: { id: { type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) } },
        required: ["id"],
        additionalProperties: false,
      }),
      run: firing((state, args, ctx) => {
        const me = ctx.who.id;
        const id = Number(args.id);
        const owner = Object.keys(state.dials).find((one) =>
          [...state.dials[one]!.alarms, ...state.dials[one]!.timers].some((each) => each.id === id),
        );
        if (!owner) return ctx.refuse(`no alarm or timer ${id} is set; it rang or was cancelled`);
        if (owner !== me) return ctx.refuse(`alarm or timer ${id} is ${owner}'s`);
        const dial = dialOf(state, me);
        return put(state, me, {
          ...dial,
          alarms: dial.alarms.filter((one) => one.id !== id),
          timers: dial.timers.filter((one) => one.id !== id),
        });
      }),
    },
    clear: {
      line: "Clear your list of what has rung.",
      run: firing((state, _args, ctx) => put(state, ctx.who.id, { ...dialOf(state, ctx.who.id), rung: [] })),
    },
    tick: {
      line: "Ring every alarm and timer due by now, for everyone.",
      run: firing((state) => state),
    },
  },
  read: sliced((state: Clock, id: string) => dialOf(state, id)),
  beat: "tick",
  bot,
});
