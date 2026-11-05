import type { Ctx, Json, Knobs, Obj, Schema, Who } from "@mrlyprod/os/contract/index.ts";
import { app, below, sliced, stream, walk } from "@mrlyprod/os/contract/index.ts";
import { blockedBy, known, notice, reach, someone } from "../shared/people.ts";
import { pay } from "../wallet/app.ts";

export type Clinic = {
  n: number;
  name: string;
  by: string;
  practitioners: string[];
  invited: string[];
  opened: number;
  closed: number | null;
};

export type Visit = {
  n: number;
  clinic: number;
  with: string;
  patient: string;
  start: number;
  booked: number;
  cancelled: number | null;
};

export type Care = {
  people: string[] | null;
  shown: number;
  clinics: Record<string, Clinic>;
  visits: Record<string, Visit>;
  count: { clinics: number; visits: number };
};

const KNOBS = {
  name: {
    line: "Characters in a clinic's name.",
    schema: { type: "integer", minimum: 8, maximum: 200 },
    reality: 40,
  },
  slot: {
    line: "Minutes in one visit slot.",
    schema: { type: "integer", minimum: 1, maximum: 1440 },
    reality: 15,
  },
  ahead: {
    line: "Days ahead a visit can be booked.",
    schema: { type: "integer", minimum: 1, maximum: 3650 },
    reality: 90,
  },
  upcoming: {
    line: "Visits one patient has booked ahead.",
    schema: { type: "integer", minimum: 1, maximum: 100 },
    reality: 3,
  },
  shown: {
    line: "Clinics and visits ahead one read shows.",
    schema: { type: "integer", minimum: 1, maximum: 500 },
    reality: 20,
  },
  fee: {
    line: "Coins a patient pays the practitioner on booking; 0 means care is free.",
    schema: { type: "integer", minimum: 0, maximum: 100_000 },
    reality: 0,
  },
  clinics: {
    line: "Clinics a world of known people starts with, each with one practitioner.",
    schema: { type: "integer", minimum: 0, maximum: 12 },
    reality: 2,
  },
} satisfies Knobs;

const MINUTE = 60_000;

const DAY_MINUTES = 1440;

const ID = 40;

const NAMES = [
  "Riverside Clinic",
  "Hillside Clinic",
  "Harbour Clinic",
  "Market Street Clinic",
  "Old Town Clinic",
  "Northgate Clinic",
  "Eastfield Clinic",
  "Westbrook Clinic",
  "Southside Clinic",
  "Lakeview Clinic",
  "Orchard Clinic",
  "Bridge Clinic",
];

function seeded(seed: number, actors: string[] | null, many: number): Record<string, Clinic> {
  if (!actors?.length) return {};
  const first = below(stream(seed).draw, actors.length);
  const out: Record<string, Clinic> = {};
  for (let i = 0; i < Math.min(many, NAMES.length); i++) {
    const by = actors[(first + i) % actors.length]!;
    out[i + 1] = { n: i + 1, name: NAMES[i]!, by, practitioners: [by], invited: [], opened: 0, closed: null };
  }
  return out;
}

const iso = (at: number) => new Date(at).toISOString();

export const slot = (at: number, minutes: number) => Math.ceil(at / (minutes * MINUTE)) * minutes * MINUTE;

function clinicOf(state: Care, args: Obj, ctx: Ctx): Clinic {
  const one = known(state.clinics, String(args.clinic), "clinic", ctx);
  if (one.closed !== null) ctx.refuse(`${one.name} has closed`);
  return one;
}

const ahead = (one: Visit, at: number) => one.cancelled === null && one.start > at;

const cancelled = (visits: Record<string, Visit>, at: number, hit: (one: Visit) => boolean) =>
  Object.fromEntries(
    Object.entries(visits).map(([key, one]) => [key, ahead(one, at) && hit(one) ? { ...one, cancelled: at } : one]),
  );

const opened = (state: Care) =>
  Object.values(state.clinics)
    .filter((one) => one.closed === null)
    .map((one) => ({ n: one.n, name: one.name, by: one.by, practitioners: one.practitioners }));

const seen = (state: Care, id: string) =>
  Object.values(state.visits).filter((one) => one.patient === id || one.with === id);

const mine = (state: Care, id: string, _who: Who | null, at: number) => ({
  at,
  iso: iso(at),
  clinics: opened(state).slice(0, state.shown),
  practises: Object.values(state.clinics)
    .filter((one) => one.closed === null && one.practitioners.includes(id))
    .map((one) => one.n),
  asked: Object.values(state.clinics)
    .filter((one) => one.closed === null && one.invited.includes(id))
    .map((one) => one.n),
  ahead: seen(state, id)
    .filter((one) => ahead(one, at))
    .sort((a, b) => a.start - b.start || a.n - b.n)
    .slice(0, state.shown)
    .map((one) => ({ ...one, iso: iso(one.start) })),
  visited: seen(state, id).length,
});

const whole = (state: Care, who: Who | null, at: number): Json =>
  who ? mine(state, who.id, who, at) : { at, iso: iso(at), clinics: opened(state) };

function looked(state: Care, path: string[], who: Who | null, at: number): Json | undefined {
  if (path[0] === "clinics") return walk(opened(state), path.slice(1));
  if (path[0] === "visits" && who) return walk(seen(state, who.id), path.slice(1));
  return sliced(mine, whole)(state, path, who, at);
}

const person: Schema = { type: "string", minLength: 1, maxLength: ID };

const clinicArgs = (more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { clinic: { type: "integer", minimum: 1 }, ...more },
  required: ["clinic", ...Object.keys(more)],
  additionalProperties: false,
});

const told = (ctx: Ctx, to: string, text: string, plain: string) => {
  if (to === ctx.who.id) return;
  ctx.wish(blockedBy(ctx, to, ctx.who.id) ? notice(to, plain, true) : notice(to, text));
};

export default app<Care>({
  id: "health",
  title: "Health",
  line: "Clinics and their practitioners, and your own visits, booked in world time; nobody else sees them. All open clinics: health/clinics. All your visits: health/visits.",
  boot: ({ seed, actors, knob }) => {
    const clinics = seeded(seed, actors, knob("clinics"));
    return {
      people: actors,
      shown: knob("shown"),
      clinics,
      visits: {},
      count: { clinics: Object.keys(clinics).length, visits: 0 },
    };
  },
  knobs: KNOBS,
  verbs: {
    open: {
      line: (knob) =>
        `Open a clinic and practise there: a name of 1 to ${knob("name")} characters no open clinic holds.`,
      args: (knob) => ({
        type: "object",
        properties: { name: { type: "string", minLength: 1, maxLength: knob("name") } },
        required: ["name"],
        additionalProperties: false,
      }),
      run: (state, args, ctx) => {
        const name = String(args.name).trim();
        if (!name) return ctx.refuse("a name needs more than spaces");
        const held = Object.values(state.clinics).find(
          (one) => one.closed === null && one.name.toLowerCase() === name.toLowerCase(),
        );
        if (held) return ctx.refuse(`clinic ${held.n} is already called ${held.name}; pick another name`);
        const n = state.count.clinics + 1;
        const clinic: Clinic = {
          n,
          name,
          by: ctx.who.id,
          practitioners: [ctx.who.id],
          invited: [],
          opened: ctx.at,
          closed: null,
        };
        ctx.say({ note: `Opened clinic ${n}: ${name}`, id: n });
        return { ...state, clinics: { ...state.clinics, [n]: clinic }, count: { ...state.count, clinics: n } };
      },
    },
    hire: {
      line: "As the one who runs a clinic, ask someone to practise there; they hear of it and join with health.join.",
      args: clinicArgs({ person }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = clinicOf(state, args, ctx);
        const person = someone(state.people, String(args.person), ctx);
        if (one.by !== me) return ctx.refuse(`only ${one.by}, who runs ${one.name}, asks practitioners`);
        if (one.practitioners.includes(person)) return ctx.refuse(`${person} already practises at ${one.name}`);
        if (one.invited.includes(person)) return ctx.refuse(`${person} is already asked to ${one.name}; wait for them`);
        reach(ctx, person);
        const made = `${me} asks you to practise at clinic ${one.n}, ${one.name}: health.join {"clinic":${one.n}}`;
        told(ctx, person, made, made);
        ctx.say({ note: `Asked ${person} to practise at ${one.name}; they join if they agree`, id: one.n });
        return {
          ...state,
          clinics: { ...state.clinics, [one.n]: { ...one, invited: [...one.invited, person] } },
        };
      },
    },
    join: {
      line: "Accept a clinic's ask to practise there.",
      args: clinicArgs(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = clinicOf(state, args, ctx);
        if (one.practitioners.includes(me)) return ctx.refuse(`you already practise at ${one.name}`);
        if (!one.invited.includes(me)) return ctx.refuse(`${one.name} has not asked you; ${one.by} runs it`);
        told(ctx, one.by, `${me} now practises at ${one.name}`, `${me} now practises at ${one.name}`);
        ctx.say({ note: `You now practise at ${one.name}`, id: one.n });
        return {
          ...state,
          clinics: {
            ...state.clinics,
            [one.n]: {
              ...one,
              practitioners: [...one.practitioners, me],
              invited: one.invited.filter((id) => id !== me),
            },
          },
        };
      },
    },
    leave: {
      line: "Stop practising at a clinic; your visits ahead there are cancelled and their patients told, and the last to leave closes it.",
      args: clinicArgs(),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = clinicOf(state, args, ctx);
        if (!one.practitioners.includes(me)) return ctx.refuse(`you do not practise at ${one.name}`);
        const practitioners = one.practitioners.filter((id) => id !== me);
        const shut = !practitioners.length;
        const next = one.by === me ? (practitioners[0] ?? me) : one.by;
        const clinic: Clinic = { ...one, practitioners, by: next, closed: shut ? ctx.at : null };
        const hit = (visit: Visit) => visit.clinic === one.n && (shut || visit.with === me);
        for (const visit of Object.values(state.visits))
          if (ahead(visit, ctx.at) && hit(visit))
            told(
              ctx,
              visit.patient,
              `Your visit ${visit.n} at ${one.name} is cancelled: ${me} left`,
              `Your visit ${visit.n} at ${one.name} is cancelled`,
            );
        if (!shut && next !== one.by)
          told(ctx, next, `${me} left ${one.name}; you run it now`, `You run ${one.name} now`);
        ctx.say({ note: shut ? `Left ${one.name}; it is closed` : `Left ${one.name}`, id: one.n });
        return {
          ...state,
          clinics: { ...state.clinics, [one.n]: clinic },
          visits: cancelled(state.visits, ctx.at, hit),
        };
      },
    },
    book: {
      line: (knob) => {
        const fee = knob("fee");
        const cost = fee ? ` It costs ${fee} coins, paid to them on booking and not refunded.` : "";
        return `Book a visit with a practitioner, in 1 minute to ${knob("ahead")} days, at the next ${knob("slot")}-minute slot; up to ${knob("upcoming")} ahead each. They hear of it.${cost}`;
      },
      args: (knob) =>
        clinicArgs({ with: person, in: { type: "integer", minimum: 1, maximum: knob("ahead") * DAY_MINUTES } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = clinicOf(state, args, ctx);
        const doctor = String(args.with);
        const most = ctx.knob("upcoming");
        if (doctor === me) return ctx.refuse("you cannot book a visit with yourself");
        if (!one.practitioners.includes(doctor)) return ctx.refuse(`${doctor} does not practise at ${one.name}`);
        if (ctx.knob("fee")) pay(ctx, doctor, ctx.knob("fee"));
        reach(ctx, doctor);
        const start = slot(ctx.at + Number(args.in) * MINUTE, ctx.knob("slot"));
        const live = Object.values(state.visits).filter((visit) => ahead(visit, ctx.at));
        if (live.filter((visit) => visit.patient === me).length >= most)
          return ctx.refuse(`you have ${most} visits ahead; cancel one first`);
        if (live.some((visit) => visit.start === start && [visit.with, visit.patient].includes(doctor)))
          return ctx.refuse(`${doctor} has no opening at that time; pick another`);
        if (live.some((visit) => visit.start === start && [visit.with, visit.patient].includes(me)))
          return ctx.refuse("you are booked at that time; pick another");
        const n = state.count.visits + 1;
        const booked = `${me} booked visit ${n} with you at ${one.name}, ${iso(start).slice(0, 16)}`;
        told(ctx, doctor, booked, booked);
        ctx.say({ note: `Booked visit ${n} with ${doctor} at ${one.name}, ${iso(start).slice(0, 16)}`, id: n, start });
        const visit: Visit = { n, clinic: one.n, with: doctor, patient: me, start, booked: ctx.at, cancelled: null };
        return { ...state, visits: { ...state.visits, [n]: visit }, count: { ...state.count, visits: n } };
      },
    },
    cancel: {
      line: "Cancel a visit ahead, as its patient or its practitioner; the other hears of it.",
      args: {
        type: "object",
        properties: { visit: { type: "integer", minimum: 1 } },
        required: ["visit"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const one = state.visits[String(args.visit)];
        if (!one || ![one.patient, one.with].includes(ctx.who.id))
          return ctx.refuse(`visit ${Number(args.visit)} is not yours; only its patient and practitioner cancel it`);
        if (one.cancelled !== null) return ctx.refuse(`visit ${one.n} is already cancelled`);
        if (one.start <= ctx.at) return ctx.refuse(`visit ${one.n} has begun; it cannot be cancelled`);
        const other = ctx.who.id === one.patient ? one.with : one.patient;
        told(ctx, other, `${ctx.who.id} cancelled visit ${one.n}`, `Visit ${one.n} was cancelled`);
        ctx.say({ note: `Cancelled visit ${one.n}`, id: one.n });
        return { ...state, visits: { ...state.visits, [one.n]: { ...one, cancelled: ctx.at } } };
      },
    },
  },
  moves: (state) => [
    "open",
    ...(state.count.clinics ? ["hire", "join", "leave", "book"] : []),
    ...(state.count.visits ? ["cancel"] : []),
  ],
  read: looked,
});
