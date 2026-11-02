import type { Ctx, Obj, Schema, Verb } from "../../sdk/system.ts";
import { app, notice, sliced } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Answer = "invited" | "accepted" | "declined";

export type Event = {
  id: number;
  owner: string;
  title: string;
  start: number;
  minutes: number;
  guests: Record<string, Answer>;
};

export type Calendar = { people: string[] | null; next: number; events: Record<string, Event> };

export const TITLE = 100;

export const MINUTES = 1440;

export const GUESTS = 8;

export const OWN = 50;

export const KEPT = 30 * 86_400_000;

export const LAST = 8_640_000_000_000_000;

const ID = 40;

/* EVENTS */

const end = (event: Event) => event.start + event.minutes * 60_000;

const pruned = (state: Calendar, at: number): Calendar => ({
  ...state,
  events: Object.fromEntries(Object.entries(state.events).filter(([, event]) => end(event) + KEPT >= at)),
});

const eventOf = (state: Calendar, id: number, refuse: Ctx["refuse"]): Event =>
  state.events[id] ?? refuse(`no event ${id}; it was cancelled or has long passed`);

const owned = (state: Calendar, id: number, me: string, refuse: Ctx["refuse"]): Event => {
  const event = eventOf(state, id, refuse);
  return event.owner === me ? event : refuse(`event ${id} is ${event.owner}'s; only its owner changes it`);
};

const set = (state: Calendar, event: Event | null, id: number): Calendar => {
  const events = { ...state.events };
  if (event) events[id] = event;
  else delete events[id];
  return { ...state, events };
};

const fresh =
  (run: Verb<Calendar>["run"]): Verb<Calendar>["run"] =>
  (state, args, ctx) =>
    run(pruned(state, ctx.at), args, ctx);

function answer(state: Calendar, args: Obj, ctx: Ctx, to: Answer): Calendar {
  const me = ctx.who.id;
  const id = Number(args.id);
  const event = eventOf(state, id, ctx.refuse);
  if (!event.guests[me]) return ctx.refuse(`you are not invited to event ${id}`);
  if (event.guests[me] === to) return ctx.refuse(`you already ${to} event ${id}`);
  ctx.wish(notice(event.owner, `${to === "accepted" ? "Accepted" : "Declined"}: ${event.title}`));
  return set(state, { ...event, guests: { ...event.guests, [me]: to } }, id);
}

const mine = (state: Calendar, id: string) => {
  const events = Object.values(state.events)
    .filter((event) => event.owner === id || event.guests[id])
    .sort((a, b) => a.start - b.start || a.id - b.id);
  return { invites: events.filter((event) => event.guests[id] === "invited"), events };
};

/* SCHEMAS */

const person = (state: Calendar): Schema =>
  state.people ? { enum: state.people } : { type: "string", minLength: 1, maxLength: ID };

const event = (state: Calendar): Schema => ({ type: "integer", minimum: 1, maximum: Math.max(1, state.next - 1) });

const start: Schema = { type: "integer", minimum: 0, maximum: LAST };

const adding = (state: Calendar): Schema => ({
  type: "object",
  properties: {
    title: { type: "string", minLength: 1, maxLength: TITLE },
    start,
    minutes: { type: "integer", minimum: 1, maximum: MINUTES },
    guests: { type: "array", items: person(state), maxItems: GUESTS },
  },
  required: ["title", "start", "minutes"],
  additionalProperties: false,
});

const moving = (state: Calendar): Schema => ({
  type: "object",
  properties: { id: event(state), start },
  required: ["id", "start"],
  additionalProperties: false,
});

const naming = (state: Calendar): Schema => ({
  type: "object",
  properties: { id: event(state) },
  required: ["id"],
  additionalProperties: false,
});

/* APP */

export default app<Calendar>({
  id: "calendar",
  title: "Calendar",
  line: "Events in world time, each with its owner and guests who accept or decline.",
  boot: ({ actors }) => ({ people: actors, next: 1, events: {} }),
  verbs: {
    add: {
      line: `Add an event: a title of 1 to ${TITLE} characters, a start in world milliseconds, 1 to ${MINUTES} minutes, up to ${GUESTS} guests; you keep at most ${OWN}.`,
      args: adding,
      run: fresh((now, args, ctx) => {
        const me = ctx.who.id;
        const title = String(args.title);
        const guests = [...new Set(((args.guests ?? []) as string[]).map(String))];
        if (!title.trim()) return ctx.refuse("the title is only spaces; name the event");
        if (guests.includes(me)) return ctx.refuse("you own the event; invite others");
        if (Object.values(now.events).filter((one) => one.owner === me).length >= OWN)
          return ctx.refuse(`you hold ${OWN} events; cancel one first`);
        for (const guest of guests) ctx.wish(notice(guest, `Invitation: ${title}`));
        const id = now.next;
        const made: Event = {
          id,
          owner: me,
          title,
          start: Number(args.start),
          minutes: Number(args.minutes),
          guests: Object.fromEntries(guests.map((guest) => [guest, "invited" as Answer])),
        };
        return { ...set(now, made, id), next: id + 1 };
      }),
    },
    move: {
      line: "Move an event of yours to another start; its guests hear of it.",
      args: moving,
      run: fresh((state, args, ctx) => {
        const id = Number(args.id);
        const was = owned(state, id, ctx.who.id, ctx.refuse);
        for (const guest of Object.keys(was.guests)) ctx.wish(notice(guest, `Moved: ${was.title}`));
        return set(state, { ...was, start: Number(args.start) }, id);
      }),
    },
    cancel: {
      line: "Cancel an event of yours; its guests hear of it.",
      args: naming,
      run: fresh((state, args, ctx) => {
        const id = Number(args.id);
        const was = owned(state, id, ctx.who.id, ctx.refuse);
        for (const guest of Object.keys(was.guests)) ctx.wish(notice(guest, `Cancelled: ${was.title}`));
        return set(state, null, id);
      }),
    },
    accept: {
      line: "Accept an event you are invited to.",
      args: naming,
      run: fresh((state, args, ctx) => answer(state, args, ctx, "accepted")),
    },
    decline: {
      line: "Decline an event you are invited to.",
      args: naming,
      run: fresh((state, args, ctx) => answer(state, args, ctx, "declined")),
    },
  },
  moves: (state) => (Object.keys(state.events).length ? ["add", "move", "cancel", "accept", "decline"] : ["add"]),
  read: sliced(mine),
  bot,
});
