import type { Act, Called, Json, Knobs, Peek, Who } from "@mrlyprod/os/contract/index.ts";
import { app, below, digest, sliced, stream } from "@mrlyprod/os/contract/index.ts";
import type { Body } from "../body/needs.ts";
import { levelsOf, spent } from "../body/needs.ts";

const ID = "mind";
const HOUR = 3_600_000;
const DAY = 24 * HOUR;
const SPAN = 16 * HOUR;
const CAP = 1400;
const HABITS = 3;
const SOON = 3;
const SHOWN = 4;
const MANY = 64;
const NAME = 40;
const EDGE = 100;
const HALF = 0.5;
const DOWN = -50;
const LOW = -15;
const UP = 15;
const BRIGHT = 50;

export const REACHES = ["to", "with", "person"];

const range = (minimum: number, maximum: number) => ({ type: "integer", minimum, maximum }) as const;

const KNOBS = {
  on: {
    line: "Whether people remember and feel; off, the front page shows notes instead.",
    schema: { type: "boolean" },
    reality: true,
  },
  keep: { line: "Days a day stays in full, diary and all.", schema: range(1, 30), reality: 2 },
  fade: { line: "Days a day stays as one line after that, then it is forgotten.", schema: range(1, 365), reality: 14 },
  diary: { line: "Most characters one diary entry keeps.", schema: range(100, 2000), reality: 600 },
  settle: { line: "Hours for a mood to drift halfway back to a person's nature.", schema: range(1, 168), reality: 24 },
  lonely: { line: "Hours without anyone before a person misses people.", schema: range(1, 720), reality: 36 },
  cheer: { line: "Mood a gift, a friend, company or a message brings.", schema: range(0, 50), reality: 5 },
  sting: { line: "Mood a refusal, a bug, a missed event or waking worn out costs.", schema: range(0, 50), reality: 2 },
  spread: { line: "How far natures differ, in mood points either side of 0.", schema: range(0, 100), reality: 40 },
  faces: { line: "Most people shown as close to you.", schema: range(0, 20), reality: 5 },
} satisfies Knobs;

type Dials = Record<Exclude<keyof typeof KNOBS, "on">, number> & { on: boolean };

export type Day = {
  acts: number;
  refused: number;
  did: Record<string, number>;
  met: string[];
  gave: number;
  got: number;
  diary?: string;
};

type Face = { at: number; n: number };

export type Person = {
  mood: number;
  at: number;
  last: number | null;
  met: number | null;
  days: Record<string, Day>;
  faces: Record<string, Face>;
};

export type Mind = {
  seed: number;
  start: number;
  people: string[] | null;
  dials: Dials;
  persons: Record<string, Person>;
};

type Alarm = { at: number; label: string; daily?: boolean };

type Planned = {
  owner: string;
  title: string;
  start: number;
  guests: Record<string, string>;
  place: { name: string | null } | null;
  repeat: string | null;
  until: number | null;
  stood: { at: number; people: string[] } | null;
};

const FAMILIES: [string, string[]][] = [
  ["cooked", ["body.cook", "body.feast"]],
  ["ate", ["body.eat"]],
  ["slept", ["body.sleep"]],
  ["played", ["body.play", "music.", "tv.", "podcasts."]],
  ["gave", ["market.give", "wallet.give"]],
  ["posted", ["news.post", "messages.post", "wish.", "photos.", "sites.", "ads."]],
  ["wrote", ["messages.", "mail.", "notes.", "docs.", "books.", "wiki.", "papers.", "code.", "slides."]],
  ["met", ["maps.", "people.", "calendar.", "dating.", "phone."]],
  ["bought", ["market.", "wallet.", "stocks."]],
];

const GIFTS = new Set(["market.give", "wallet.give", "body.feast"]);

const CHEERS = new Set([...GIFTS, "people.friend", "body.play", "messages.send", "messages.reply", "maps.invite"]);

const EVERY: Record<string, number> = { daily: DAY, weekly: 7 * DAY };

const familyOf = (verb: string) =>
  FAMILIES.find(([, heads]) =>
    heads.some((head) => (head.endsWith(".") ? verb.startsWith(head) : verb === head)),
  )?.[0] ?? null;

const clamp = (n: number) => Math.max(-EDGE, Math.min(EDGE, n));

const held = (state: Mind, id: string) => (Object.hasOwn(state.persons, id) ? state.persons[id]! : null);

function natureOf(state: Mind, id: string) {
  const { draw } = stream(Number.parseInt(digest(`${state.seed}/${id}`).slice(0, 8), 16));
  return below(draw, 2 * state.dials.spread + 1) - state.dials.spread;
}

function moodOf(state: Mind, one: Person | null, id: string, at: number) {
  const nature = natureOf(state, id);
  if (!one) return nature;
  return nature + (one.mood - nature) * HALF ** (Math.max(0, at - one.at) / (state.dials.settle * HOUR));
}

const dayOf = (state: Mind, at: number) => Math.floor((at - state.start) / DAY) + 1;

const hourOf = (at: number) => new Date(at).toISOString().slice(11, 16);

const clockOf = (state: Mind, at: number) => `${hourOf(at)} day ${dayOf(state, at)}`;

const listed = (ids: string[]) =>
  ids.length > SHOWN ? `${ids.slice(0, SHOWN).join(", ")} and ${ids.length - SHOWN} more` : ids.join(", ");

const timesOf = (n: number) => (n === 1 ? "once" : n === 2 ? "twice" : `${n} times`);

const blank = (): Day => ({ acts: 0, refused: 0, did: {}, met: [], gave: 0, got: 0 });

function pruned(one: Person, dials: Dials, today: number, at: number): Person {
  const days: Record<string, Day> = {};
  for (const [key, day] of Object.entries(one.days)) {
    const n = Number(key);
    if (n < today - dials.keep - dials.fade) continue;
    const { diary: _diary, ...plain } = day;
    days[key] = n < today - dials.keep ? plain : day;
  }
  const span = (dials.keep + dials.fade) * DAY;
  const faces = Object.fromEntries(Object.entries(one.faces).filter(([, face]) => at - face.at <= span));
  return { ...one, days, faces };
}

function edited(state: Mind, now: Person | undefined, id: string, at: number, edit: (one: Person, day: Day) => void) {
  const base = now ?? held(state, id);
  const one: Person = base ? structuredClone(base) : { mood: 0, at, last: null, met: null, days: {}, faces: {} };
  one.mood = moodOf(state, base, id, at);
  one.at = at;
  const key = String(dayOf(state, at));
  one.days[key] ??= blank();
  edit(one, one.days[key]);
  one.mood = clamp(one.mood);
  return pruned(one, state.dials, dayOf(state, at), at);
}

function meet(one: Person, day: Day, ids: string[], at: number) {
  for (const id of ids) {
    if (!day.met.includes(id)) day.met.push(id);
    one.faces[id] = { at, n: (one.faces[id]?.n ?? 0) + 1 };
    one.met = at;
  }
}

function tallyOf(n: number, day: Day) {
  const parts = [`${day.acts} acts`];
  if (day.met.length) parts.push(`met ${listed(day.met)}`);
  if (day.gave || day.got) parts.push(`gave ${day.gave}, got ${day.got}`);
  if (day.refused) parts.push(`${day.refused} refused`);
  return `day ${n}: ${parts.join("; ")}`;
}

function missedOf(peek: Peek, id: string, at: number) {
  const events = Object.values((peek("calendar/events") ?? {}) as Record<string, Planned>);
  return events
    .filter((one) => one.owner === id || one.guests[id] === "accepted")
    .filter((one) => one.place && one.stood && at - one.stood.at <= DAY && !one.stood.people.includes(id))
    .map((one) => one.title);
}

function worn(peek: Peek, id: string, at: number) {
  const body = peek("body") as Body | null;
  return !!body?.dials && spent(levelsOf(body, id, at));
}

const nextOf = (start: number, every: number, at: number) =>
  every && start < at ? start + Math.ceil((at - start) / every) * every : start;

function soonOf(state: Mind, peek: Peek, id: string, at: number) {
  const until = Math.max(state.start + (dayOf(state, at) - 1) * DAY + SPAN, at + HOUR);
  const due: [number, string][] = [];
  for (const one of (peek(`clock/dials/${id}/alarms`) ?? []) as Alarm[]) {
    const when = nextOf(one.at, one.daily ? DAY : 0, at);
    if (when >= at && when <= until) due.push([when, `${hourOf(when)} alarm: ${one.label}`]);
  }
  for (const one of Object.values((peek("calendar/events") ?? {}) as Record<string, Planned>)) {
    if (one.owner !== id && !["accepted", "invited"].includes(one.guests[id] ?? "")) continue;
    const when = nextOf(one.start, EVERY[one.repeat ?? ""] ?? 0, at);
    if (when < at || when > until || (one.until !== null && when > one.until)) continue;
    const where = one.place?.name ? ` at ${one.place.name}` : "";
    due.push([when, `${hourOf(when)} ${one.title}${where}${one.owner === id ? "" : `, with ${one.owner}`}`]);
  }
  return due
    .sort((a, b) => a[0] - b[0])
    .slice(0, SOON)
    .map(([, line]) => line);
}

function feelOf(state: Mind, one: Person | null, id: string, at: number, peek: Peek, last: Day | null) {
  const missed = missedOf(peek, id, at);
  const tired = worn(peek, id, at);
  const mood = clamp(moodOf(state, one, id, at) - state.dials.sting * (missed.length + (tired ? 1 : 0)));
  const why = missed.length ? ` after missing ${missed[0]}` : last?.refused ? " after yesterday" : "";
  const words =
    mood <= DOWN
      ? `You feel miserable${why}.`
      : mood <= LOW
        ? `You feel low${why}.`
        : mood < UP
          ? "You feel steady."
          : mood < BRIGHT
            ? "You feel good."
            : "You feel bright.";
  const since = one?.met ?? null;
  const alone = (at - (since ?? state.start)) / HOUR > state.dials.lonely;
  const missing =
    since === null
      ? " You miss people: you have met nobody yet."
      : ` You miss people: nobody since day ${dayOf(state, since)}.`;
  return `${words}${tired ? " You are worn out." : ""}${alone ? missing : ""}`;
}

function sinceOf(state: Mind, one: Person | null, at: number) {
  const now = `It is ${clockOf(state, at)}.`;
  if (!one?.last) return `${now} You have not acted here yet.`;
  return `${now} You last acted ${Math.round((at - one.last) / HOUR)}h ago, at ${clockOf(state, one.last)}.`;
}

function habitsOf(one: Person | null, from: number) {
  const sums: Record<string, number> = {};
  for (const [key, day] of Object.entries(one?.days ?? {}))
    if (Number(key) >= from) for (const [family, n] of Object.entries(day.did)) sums[family] = (sums[family] ?? 0) + n;
  return Object.entries(sums)
    .sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))
    .slice(0, HABITS)
    .map(([family, n]) => `${family} ${timesOf(n)}`);
}

function fitted(page: { yesterday: string; before: string[] } & Record<string, Json>) {
  const before = [...page.before];
  const size = () => JSON.stringify({ ...page, before }).length;
  while (size() > CAP && before.length) before.pop();
  const over = size() - CAP;
  return {
    ...page,
    before,
    yesterday: over > 0 ? page.yesterday.slice(0, Math.max(0, page.yesterday.length - over)) : page.yesterday,
  };
}

const mine = (state: Mind, id: string, _who: Who | null, at: number, peek: Peek): Json | undefined => {
  if (!state.dials.on) return undefined;
  const one = held(state, id);
  const { keep, fade, faces } = state.dials;
  const today = dayOf(state, at);
  const days = one?.days ?? {};
  const past = Object.keys(days)
    .map(Number)
    .filter((n) => n < today && n >= today - keep - fade)
    .sort((a, b) => b - a);
  const lineOf = (n: number) => {
    const day = days[n]!;
    return n >= today - keep && day.diary ? `${tallyOf(n, day)}. You wrote: ${day.diary}` : tallyOf(n, day);
  };
  const [last, ...rest] = past;
  return fitted({
    day: today,
    since: sinceOf(state, one, at),
    feel: feelOf(state, one, id, at, peek, last === undefined ? null : days[last]!),
    soon: soonOf(state, peek, id, at),
    yesterday: last === undefined ? "Nothing yet: this is your first day." : lineOf(last),
    before: rest.map(lineOf),
    close: Object.entries(one?.faces ?? {})
      .sort(([, a], [, b]) => b.n - a.n || b.at - a.at)
      .slice(0, faces)
      .map(([who, face]) => `${who}: ${timesOf(face.n)}, last at ${clockOf(state, face.at)}`),
    habits: habitsOf(one, today - keep),
  });
};

const whole = (state: Mind, who: Who | null, at: number, peek: Peek): Json =>
  who
    ? (mine(state, who.id, who, at, peek) ?? null)
    : Object.fromEntries(
        (state.people ?? Object.keys(state.persons)).map((id) => [
          id,
          Math.round(moodOf(state, held(state, id), id, at)),
        ]),
      );

function noted({ world, line, envelope }: Called): Act | null {
  if (line.via || line.verb.startsWith(`${ID}.`) || world.rules[`${ID}.on`] === false) return null;
  const said = envelope.said;
  const fed = said && typeof said === "object" && !Array.isArray(said) && Array.isArray(said.fed) ? said.fed : [];
  const named = [...REACHES.map((key) => line.args[key]), ...fed].filter(
    (one): one is string =>
      typeof one === "string" &&
      one !== line.who.id &&
      one.length <= NAME &&
      (!world.actors || world.actors.includes(one)),
  );
  const bug = line.verb === "feedback.send" && line.args.severity === "bug";
  return {
    verb: `${ID}.note`,
    args: { verb: line.verb, ok: envelope.last.ok, to: [...new Set(named)].slice(0, MANY), bug },
  };
}

export default app<Mind>({
  id: ID,
  title: "Mind",
  line: "What you remember and how you feel: yesterday, the days before, the people close to you, what is due soon.",
  knobs: KNOBS,
  boot: ({ seed, start, knob, actors }) => ({
    seed,
    start,
    people: actors,
    dials: Object.fromEntries(Object.keys(KNOBS).map((name) => [name, knob(name)])) as Dials,
    persons: {},
  }),
  verbs: {
    sleep: {
      line: (knob) =>
        `Before you sleep, write what you want to remember tomorrow, up to ${knob("diary")} characters; it opens your next front page.`,
      args: (knob) => ({
        type: "object",
        properties: { diary: { type: "string", minLength: 1, maxLength: knob("diary") } },
        required: ["diary"],
        additionalProperties: false,
      }),
      run: (state, args, ctx) => {
        if (!state.dials.on) return ctx.refuse("nobody keeps a diary in this world; write it in notes instead");
        const diary = String(args.diary).trim();
        if (!diary) return ctx.refuse("write something to remember");
        const one = edited(state, undefined, ctx.who.id, ctx.at, (_one, day) => {
          day.diary = (day.diary ? `${day.diary} ${diary}` : diary).slice(-state.dials.diary);
        });
        ctx.say({ note: "Kept for tomorrow: it opens your next front page." });
        return { ...state, persons: { ...state.persons, [ctx.who.id]: one } };
      },
    },
    note: {
      inner: true,
      line: "Keep the world's record of what someone did and who they touched; it happens by itself after each act.",
      args: {
        type: "object",
        properties: {
          verb: { type: "string", maxLength: 200 },
          ok: { type: "boolean" },
          to: { type: "array", items: { type: "string", maxLength: NAME }, maxItems: MANY },
          bug: { type: "boolean" },
        },
        required: ["verb", "ok", "to", "bug"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        if (!state.dials.on) return state;
        const me = ctx.who.id;
        const verb = String(args.verb);
        const ok = args.ok === true;
        const to = ok
          ? (args.to as string[]).filter((id) => id !== me && (!state.people || state.people.includes(id)))
          : [];
        const family = ok ? familyOf(verb) : null;
        const { cheer, sting } = state.dials;
        const persons: Record<string, Person> = {};
        const touch = (id: string, edit: (one: Person, day: Day) => void) => {
          persons[id] = edited(state, persons[id], id, ctx.at, edit);
        };
        touch(me, (one, day) => {
          day.acts++;
          if (!ok) day.refused++;
          if (!ok) one.mood -= sting;
          if (args.bug === true) one.mood -= sting;
          if (family) day.did[family] = (day.did[family] ?? 0) + 1;
          if (GIFTS.has(verb) && to.length) day.gave++;
          one.last = ctx.at;
          meet(one, day, to, ctx.at);
        });
        for (const id of to)
          touch(id, (one, day) => {
            if (GIFTS.has(verb)) day.got++;
            if (CHEERS.has(verb)) one.mood += cheer;
            meet(one, day, [me], ctx.at);
          });
        return { ...state, persons: { ...state.persons, ...persons } };
      },
    },
  },
  read: sliced(mine, whole),
  after: noted,
});
