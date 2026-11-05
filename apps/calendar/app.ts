import type { Ctx, Json, Knob, Knobs, Obj, Schema, Verb, Who } from "@mrlyprod/os/contract/index.ts";
import { app, canon, sliced, walk } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, notice, someone } from "../shared/people.ts";

export type Answer = "invited" | "accepted" | "declined";

export type Repeat = "daily" | "weekly";

export type Spot = { q: number; r: number; name: string | null };

export type Stood = { at: number; people: string[] };

export type Event = {
  id: number;
  owner: string;
  title: string;
  start: number;
  minutes: number;
  guests: Record<string, Answer>;
  place: Spot | null;
  repeat: Repeat | null;
  until: number | null;
  told: number;
  stood: Stood | null;
};

export type Calendar = { people: string[] | null; next: number; alert: number; events: Record<string, Event> };

const KNOBS = {
  title: {
    line: "Characters in an event's title.",
    schema: { type: "integer", minimum: 8, maximum: 1000 },
    reality: 100,
  },
  minutes: {
    line: "Minutes the longest event lasts.",
    schema: { type: "integer", minimum: 15, maximum: 43_200 },
    reality: 1440,
  },
  guests: {
    line: "Guests one event invites.",
    schema: { type: "integer", minimum: 0, maximum: 256 },
    reality: 8,
  },
  events: {
    line: "Events one person owns.",
    schema: { type: "integer", minimum: 1, maximum: 1000 },
    reality: 50,
  },
  keep: {
    line: "Days an event stays after it ends.",
    schema: { type: "integer", minimum: 0, maximum: 3650 },
    reality: 30,
  },
  alert: {
    line: "Minutes before an event starts that its people get a notice.",
    schema: { type: "integer", minimum: 0, maximum: 1440 },
    reality: 15,
  },
  stood: {
    line: "People an event's roll records as standing at its place when it starts.",
    schema: { type: "integer", minimum: 1, maximum: 4096 },
    reality: 256,
  },
} satisfies Knobs;

export const LAST = 8_640_000_000_000_000;

const MINUTE = 60_000;

const DAY = 86_400_000;

const STEP: Record<Repeat, number> = { daily: DAY, weekly: 7 * DAY };

const ID = 40;

const SHOWN = 8;

const WAITING = 4;

const IN = 5_256_000;

const STAMP = /^\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}(:\d{2}(\.\d+)?)?(Z|[+-]\d{2}:\d{2})?)?$/;

const ZONED = /^\d{4}-\d{2}-\d{2}$|(Z|[+-]\d{2}:\d{2})$/;

const iso = (at: number) => new Date(Math.min(at, LAST)).toISOString();

const when = (at: number) => iso(at).slice(0, 16).replace("T", " ");

const length = (event: Event) => event.minutes * MINUTE;

const occurrence = (event: Event, at: number) => {
  if (!event.repeat || at <= event.start + length(event)) return event.start;
  const step = STEP[event.repeat];
  const start = event.start + Math.ceil((at - length(event) - event.start) / step) * step;
  if (event.until === null || start <= event.until) return start;
  return event.start + Math.floor(Math.max(0, event.until - event.start) / step) * step;
};

const last = (event: Event) => {
  if (!event.repeat) return event.start + length(event);
  return event.until === null ? Number.POSITIVE_INFINITY : event.until + length(event);
};

const pruned = (state: Calendar, at: number, keep: number): Calendar => ({
  ...state,
  events: Object.fromEntries(Object.entries(state.events).filter(([, event]) => last(event) + keep * DAY >= at)),
});

const eventOf = (state: Calendar, id: number, refuse: Ctx["refuse"]): Event =>
  state.events[id] ?? refuse(`no event ${id}; it was cancelled or has long passed`);

const owned = (state: Calendar, id: number, me: string, refuse: Ctx["refuse"]): Event => {
  const event = eventOf(state, id, refuse);
  return event.owner === me ? event : refuse(`event ${id} is not yours; only its owner changes it`);
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
    run(pruned(state, ctx.at, ctx.knob("keep")), args, ctx);

const at = (place: Spot | null) => {
  if (!place) return "";
  const cell = `(${place.q}, ${place.r})`;
  return place.name ? `, ${place.name} ${cell}` : `, ${cell}`;
};

const told = (event: Event, start: number) => `${event.title}, ${when(start)}${at(event.place)}`;

function placed(args: Obj, ctx: Ctx): Spot | null {
  if (args.place !== undefined) {
    const one = ctx.read(`maps/places/${args.place}`) as { q: number; r: number; name: string } | null;
    if (!one) return ctx.refuse(`no place ${args.place} on the map; read maps for places, or give q and r`);
    return { q: one.q, r: one.r, name: one.name };
  }
  if (args.q === undefined && args.r === undefined) return null;
  if (args.q === undefined || args.r === undefined) return ctx.refuse("a cell takes both q and r");
  const q = Number(args.q);
  const r = Number(args.r);
  const radius = ctx.read("maps/radius") as number | null;
  if (radius === null) return ctx.refuse("this world has no map; leave the place out");
  if (Math.abs(q) > radius || Math.abs(r) > radius || Math.abs(q + r) > radius)
    return ctx.refuse(`(${q}, ${r}) is off the map; q, r and q + r stay within ${radius} of the centre`);
  return { q, r, name: null };
}

function reachable(guest: string, me: string, ctx: Ctx) {
  if (guest === me) ctx.refuse("you own the event; invite others");
  if (blockedBy(ctx, guest, me)) ctx.refuse(`${guest} is not taking this from you`);
}

const ending = (event: Event, start: number, now: number) => start + length(event) >= now;

const answerOf = (event: Event, id: string) => (Object.hasOwn(event.guests, id) ? event.guests[id] : undefined);

const named = (event: Event, to: Answer) =>
  Object.keys(event.guests)
    .filter((guest) => event.guests[guest] === to)
    .sort();

const rsvp = (event: Event) => {
  const [yes, no, waiting] = [named(event, "accepted"), named(event, "declined"), named(event, "invited")];
  return { yes, no, waiting, counts: { yes: yes.length, no: no.length, waiting: waiting.length } };
};

const tally = (event: Event) => {
  const { yes, no, waiting } = rsvp(event).counts;
  return `${yes} yes, ${no} no, ${waiting} waiting`;
};

const clashes = (state: Calendar, me: string, event: Event, now: number) => {
  const start = occurrence(event, now);
  return Object.values(state.events).filter((other) => {
    if (other.id === event.id) return false;
    if (other.owner !== me && answerOf(other, me) !== "accepted") return false;
    const theirs = occurrence(other, start);
    return theirs < start + length(event) && start < theirs + length(other);
  });
};

const warned = (list: Event[]) =>
  list.length ? `; clashes with ${list.map((one) => `event ${one.id}: ${one.title}`).join(", ")}` : "";

function moment(value: unknown, ctx: Ctx, name: string): number {
  if (typeof value === "number" && Number.isInteger(value) && value >= 0 && value <= LAST) return value;
  if (typeof value === "string" && /^\d{1,16}$/.test(value.trim())) {
    const ms = Number(value.trim());
    if (ms <= LAST) return ms;
  }
  if (typeof value === "string" && STAMP.test(value.trim())) {
    const text = value.trim();
    const at = Date.parse(ZONED.test(text) ? text : `${text}Z`);
    if (at >= 0 && at <= LAST) return at;
  }
  return ctx.refuse(`${name} is an ISO time like ${iso(ctx.at)} or world milliseconds, not ${canon(value as Json)}`);
}

function startOf(args: Obj, ctx: Ctx): number | null {
  if (args.in !== undefined && args.start !== undefined) return ctx.refuse("say when once: start or in, not both");
  if (args.in !== undefined) return ctx.at + Number(args.in) * MINUTE;
  return args.start === undefined ? null : moment(args.start, ctx, "start");
}

function answer(state: Calendar, args: Obj, ctx: Ctx, to: Answer): Calendar {
  const me = ctx.who.id;
  const id = Number(args.id);
  const event = eventOf(state, id, ctx.refuse);
  const was = answerOf(event, me);
  if (!was) return ctx.refuse(`you are not invited to event ${id}`);
  if (was === to) return ctx.refuse(`you already ${to} event ${id}`);
  if (!ending(event, occurrence(event, ctx.at), ctx.at)) return ctx.refuse(`event ${id} is over`);
  const made: Event = { ...event, guests: { ...event.guests, [me]: to } };
  ctx.wish(notice(event.owner, `${me} ${to} event ${id} (${tally(made)}): ${event.title}`));
  const clash = to === "accepted" ? clashes(state, me, made, ctx.at) : [];
  ctx.say({
    note: `You ${to} event ${id}: ${event.title}; ${tally(made)}${warned(clash)}`,
    id,
    clash: clash.map((one) => one.id),
  });
  return set(state, made, id);
}

const due = (state: Calendar, now: number, lead: number) =>
  Object.values(state.events).filter((event) => {
    const start = occurrence(event, now);
    return start !== event.told && start - lead <= now && ending(event, start, now);
  });

const rollDue = (state: Calendar, now: number) =>
  Object.values(state.events).filter((event) => {
    if (!event.place) return false;
    const start = occurrence(event, now);
    return start <= now && ending(event, start, now) && event.stood?.at !== start;
  });

const rollOf = (event: Event, start: number, ctx: Ctx, where: Record<string, { q: number; r: number }>): Stood => {
  const walking = (ctx.read("maps/walking") ?? {}) as Record<string, number>;
  const ours = new Set([event.owner, ...Object.keys(event.guests)]);
  const there = Object.keys(where)
    .filter((id) => where[id]!.q === event.place!.q && where[id]!.r === event.place!.r)
    .filter((id) => !(Object.hasOwn(walking, id) && walking[id]! > ctx.at))
    .sort();
  const people = [...there.filter((id) => ours.has(id)), ...there.filter((id) => !ours.has(id))];
  return { at: start, people: people.slice(0, ctx.knob("stood")) };
};

const summed = (event: Event, id: string, now: number) => {
  const next = occurrence(event, now);
  return {
    id: event.id,
    title: event.title,
    owner: event.owner,
    next,
    iso: iso(next),
    minutes: event.minutes,
    place: event.place,
    repeat: event.repeat,
    answer: event.owner === id ? "owner" : answerOf(event, id)!,
    over: !ending(event, next, now),
    rsvp: rsvp(event),
    stood: event.stood,
  };
};

const held = (state: Calendar, id: string) =>
  Object.values(state.events).filter((event) => event.owner === id || answerOf(event, id));

const mine = (state: Calendar, id: string, _who: Who | null, now: number) => {
  const ahead = held(state, id)
    .filter((event) => ending(event, occurrence(event, now), now))
    .map((event) => summed(event, id, now))
    .sort((a, b) => a.next - b.next || a.id - b.id);
  return {
    at: now,
    iso: iso(now),
    invites: ahead.filter((event) => event.answer === "invited").slice(0, WAITING),
    events: ahead.filter((event) => event.answer === "owner" || event.answer === "accepted").slice(0, SHOWN),
    held: held(state, id).length,
  };
};

const everything = (state: Calendar, id: string, now: number): Json =>
  Object.fromEntries(
    held(state, id).map((event) => [event.id, { ...summed(event, id, now), start: event.start, guests: event.guests }]),
  );

const open = (state: Calendar, who: Who | null, now: number): Json =>
  who ? mine(state, who.id, who, now) : { at: now, iso: iso(now), events: Object.keys(state.events).length };

const looked = (state: Calendar, path: string[], who: Who | null, now: number): Json | undefined =>
  path[0] === "all" && who
    ? walk(everything(state, who.id, now), path.slice(1))
    : sliced(mine, open)(state, path, who, now);

const person: Schema = { type: "string", minLength: 1, maxLength: ID };

const event: Schema = { type: "integer", minimum: 1 };

const start: Schema = { line: "An ISO time like 2030-06-01T09:00:00Z, or world milliseconds." };

const soon: Schema = { type: "integer", minimum: 0, maximum: IN };

const where: Schema["properties"] = {
  place: { type: "integer", minimum: 1 },
  q: { type: "integer" },
  r: { type: "integer" },
};

const adding = (knob: Knob): Schema => ({
  type: "object",
  properties: {
    title: { type: "string", minLength: 1, maxLength: knob("title") },
    start,
    in: soon,
    minutes: { type: "integer", minimum: 1, maximum: knob("minutes") },
    guests: { type: "array", items: person, maxItems: knob("guests") },
    ...where,
    repeat: { enum: ["daily", "weekly"] },
    until: start,
  },
  required: ["title", "minutes"],
  additionalProperties: false,
});

const moving: Schema = {
  type: "object",
  properties: { id: event, start, in: soon, ...where },
  required: ["id"],
  additionalProperties: false,
};

const inviting: Schema = {
  type: "object",
  properties: { id: event, guest: person },
  required: ["id", "guest"],
  additionalProperties: false,
};

const naming: Schema = {
  type: "object",
  properties: { id: event },
  required: ["id"],
  additionalProperties: false,
};

export default app<Calendar>({
  id: "calendar",
  title: "Calendar",
  line: "Your events in world time and the time now: a place on the map, a daily or weekly repeat, guests who accept or decline, and a notice before each one starts. Each event shows who said yes, no or nothing, and who stood at its place when it began. Every event of yours: calendar/all.",
  boot: ({ actors, knob }) => ({ people: actors, next: 1, alert: knob("alert") * MINUTE, events: {} }),
  knobs: KNOBS,
  verbs: {
    add: {
      line: (knob) =>
        `Add an event: a title of 1 to ${knob("title")} characters, a start as an ISO time or world milliseconds, or in some minutes from now (read calendar for the time now), 1 to ${knob("minutes")} minutes, up to ${knob("guests")} guests; a place on the map by id or q and r; a daily or weekly repeat, ending at until if given. You own at most ${knob("events")}.`,
      args: adding,
      run: fresh((now, args, ctx) => {
        const me = ctx.who.id;
        const title = String(args.title);
        const begins = startOf(args, ctx) ?? ctx.refuse(`say when: start or in; it is ${iso(ctx.at)}`);
        const minutes = Number(args.minutes);
        const repeat = args.repeat === undefined ? null : (String(args.repeat) as Repeat);
        const guests = [...new Set(((args.guests ?? []) as string[]).map((one) => someone(now.people, one, ctx)))];
        const most = ctx.knob("events");
        if (!title.trim()) return ctx.refuse("the title is only spaces; name the event");
        if (!repeat && begins + minutes * MINUTE < ctx.at)
          return ctx.refuse(`that is over already; it is ${iso(ctx.at)}, world time ${ctx.at}`);
        if (args.until !== undefined && !repeat)
          return ctx.refuse("until ends a repeat; add a repeat, or leave it out");
        const until = args.until === undefined ? null : moment(args.until, ctx, "until");
        if (until !== null && until < begins) return ctx.refuse("until comes before the start; pick a later one");
        for (const guest of guests) reachable(guest, me, ctx);
        if (Object.values(now.events).filter((one) => one.owner === me).length >= most)
          return ctx.refuse(`you hold ${most} events; cancel one first`);
        const id = now.next;
        const made: Event = {
          id,
          owner: me,
          title,
          start: begins,
          minutes,
          guests: Object.fromEntries(guests.map((guest) => [guest, "invited" as Answer])),
          place: placed(args, ctx),
          repeat,
          until,
          told: -1,
          stood: null,
        };
        for (const guest of guests)
          ctx.wish(notice(guest, `${me} invites you to event ${id} (calendar.accept): ${told(made, begins)}`));
        const clash = clashes(now, me, made, ctx.at);
        ctx.say({
          note: `Added event ${id}: ${told(made, begins)}; invited ${guests.join(", ") || "nobody"}${warned(clash)}`,
          id,
          iso: iso(begins),
          invited: guests,
          clash: clash.map((one) => one.id),
        });
        return { ...set(now, made, id), next: id + 1 };
      }),
    },
    move: {
      line: "Move an event of yours to another start, another place, or both; a repeat moves whole. Its guests hear of it.",
      args: moving,
      run: fresh((state, args, ctx) => {
        const id = Number(args.id);
        const was = owned(state, id, ctx.who.id, ctx.refuse);
        const place = placed(args, ctx);
        const begins = startOf(args, ctx);
        if (begins === null && !place) return ctx.refuse("say what changes: a start or in, or a place or q and r");
        const made: Event = {
          ...was,
          start: begins ?? was.start,
          place: place ?? was.place,
          told: -1,
          stood: null,
        };
        const me = ctx.who.id;
        const next = occurrence(made, ctx.at);
        for (const guest of Object.keys(was.guests)) ctx.wish(notice(guest, `Moved event ${id}: ${told(made, next)}`));
        const clash = clashes(state, me, made, ctx.at);
        ctx.say({
          note: `Moved event ${id}: ${told(made, next)}${warned(clash)}`,
          id,
          iso: iso(next),
          clash: clash.map((one) => one.id),
        });
        return set(state, made, id);
      }),
    },
    invite: {
      line: (knob) => `Invite someone to an event of yours; an event holds at most ${knob("guests")} guests.`,
      args: inviting,
      run: fresh((state, args, ctx) => {
        const me = ctx.who.id;
        const id = Number(args.id);
        const guest = someone(state.people, String(args.guest), ctx);
        const was = owned(state, id, me, ctx.refuse);
        const most = ctx.knob("guests");
        reachable(guest, me, ctx);
        const had = answerOf(was, guest);
        if (had) return ctx.refuse(`${guest} is on event ${id} already: ${had}`);
        if (Object.keys(was.guests).length >= most) return ctx.refuse(`event ${id} holds ${most} guests already`);
        ctx.wish(
          notice(guest, `${me} invites you to event ${id} (calendar.accept): ${told(was, occurrence(was, ctx.at))}`),
        );
        ctx.say({ note: `Invited ${guest} to event ${id}`, id, guest });
        return set(state, { ...was, guests: { ...was.guests, [guest]: "invited" } }, id);
      }),
    },
    cancel: {
      line: "Cancel an event of yours; its guests hear of it.",
      args: naming,
      run: fresh((state, args, ctx) => {
        const id = Number(args.id);
        const was = owned(state, id, ctx.who.id, ctx.refuse);
        for (const guest of Object.keys(was.guests)) ctx.wish(notice(guest, `Cancelled event ${id}: ${was.title}`));
        ctx.say({ note: `Cancelled event ${id}: ${was.title}`, id });
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
    tick: {
      inner: true,
      line: "Tell everyone of the events about to start; it happens by itself whenever anyone acts.",
      run: fresh((state, _args, ctx) => {
        const hit = due(state, ctx.at, state.alert);
        const noted = hit.reduce<Calendar>((now, one) => {
          const begins = occurrence(one, ctx.at);
          const going = [one.owner, ...Object.keys(one.guests).filter((guest) => one.guests[guest] !== "declined")];
          for (const person of going) ctx.wish(notice(person, `Soon, event ${one.id}: ${told(one, begins)}`, true));
          return set(now, { ...one, told: begins }, one.id);
        }, state);
        const where = ctx.read("maps/where") as Record<string, { q: number; r: number }> | null;
        const rolls = rollDue(noted, ctx.at);
        ctx.say({ note: `Told of ${hit.length} events; took the roll of ${rolls.length}` });
        return rolls.reduce<Calendar>((now, one) => {
          const begins = occurrence(one, ctx.at);
          const stood = where ? rollOf(one, begins, ctx, where) : { at: begins, people: [] };
          return set(now, { ...one, stood }, one.id);
        }, noted);
      }),
    },
  },
  moves: (state) =>
    Object.keys(state.events).length ? ["add", "move", "invite", "cancel", "accept", "decline", "tick"] : ["add"],
  read: looked,
  wake: (state, at) => (rollDue(state, at).length || due(state, at, state.alert).length ? "tick" : null),
});
