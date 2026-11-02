import type { Ctx, Obj, Schema } from "../../sdk/system.ts";
import { app, notice, sliced } from "../../sdk/system.ts";
import { bot } from "./bot.ts";

/* TYPES */

export type Clinic = {
  n: number;
  name: string;
  by: string;
  practitioners: string[];
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
  clinics: Record<string, Clinic>;
  visits: Record<string, Visit>;
  count: { clinics: number; visits: number };
};

export const NAME = 40;

export const SLOT = 15 * 60 * 1000;

export const AHEAD = 90 * 24 * 60;

export const UPCOMING = 3;

export const SHOWN = 20;

const ID = 40;

/* CLINICS */

export const slot = (at: number) => Math.ceil(at / SLOT) * SLOT;

function clinicOf(state: Care, args: Obj, ctx: Ctx): Clinic {
  const one = state.clinics[String(args.clinic)]!;
  if (one.closed !== null) ctx.refuse(`${one.name} has closed`);
  return one;
}

const ahead = (one: Visit, at: number) => one.cancelled === null && one.start > at;

const cancelled = (visits: Record<string, Visit>, at: number, hit: (one: Visit) => boolean) =>
  Object.fromEntries(
    Object.entries(visits).map(([key, one]) => [key, ahead(one, at) && hit(one) ? { ...one, cancelled: at } : one]),
  );

const mine = (state: Care, id: string) => ({
  practises: Object.values(state.clinics)
    .filter((one) => one.practitioners.includes(id))
    .map((one) => one.n),
  visits: Object.values(state.visits)
    .filter((one) => one.patient === id || one.with === id)
    .slice(-SHOWN),
});

/* SCHEMAS */

const someone = (state: Care): Schema =>
  state.people ? { enum: state.people } : { type: "string", minLength: 1, maxLength: ID };

const clinicArgs = (state: Care, more: Schema["properties"] = {}): Schema => ({
  type: "object",
  properties: { clinic: { type: "integer", minimum: 1, maximum: Math.max(1, state.count.clinics) }, ...more },
  required: ["clinic", ...Object.keys(more)],
  additionalProperties: false,
});

/* APP */

export default app<Care>({
  id: "health",
  title: "Health",
  line: "Clinics and their practitioners, and visits booked in world time.",
  boot: ({ actors }) => ({ people: actors, clinics: {}, visits: {}, count: { clinics: 0, visits: 0 } }),
  verbs: {
    open: {
      line: `Open a clinic and practise there: a name of 1 to ${NAME} characters no open clinic holds.`,
      args: {
        type: "object",
        properties: { name: { type: "string", minLength: 1, maxLength: NAME } },
        required: ["name"],
        additionalProperties: false,
      },
      run: (state, args, ctx) => {
        const name = String(args.name).trim();
        if (!name) return ctx.refuse("a name needs more than spaces");
        const held = Object.values(state.clinics).find(
          (one) => one.closed === null && one.name.toLowerCase() === name.toLowerCase(),
        );
        if (held) return ctx.refuse(`clinic ${held.n} is already called ${held.name}; pick another name`);
        const n = state.count.clinics + 1;
        const clinic: Clinic = { n, name, by: ctx.who.id, practitioners: [ctx.who.id], opened: ctx.at, closed: null };
        return { ...state, clinics: { ...state.clinics, [n]: clinic }, count: { ...state.count, clinics: n } };
      },
    },
    hire: {
      line: "As the one who runs a clinic, add a practitioner.",
      args: (state) => clinicArgs(state, { person: someone(state) }),
      run: (state, args, ctx) => {
        const one = clinicOf(state, args, ctx);
        const person = String(args.person);
        if (one.by !== ctx.who.id) return ctx.refuse(`only ${one.by}, who runs ${one.name}, adds practitioners`);
        if (one.practitioners.includes(person)) return ctx.refuse(`${person} already practises at ${one.name}`);
        return {
          ...state,
          clinics: { ...state.clinics, [one.n]: { ...one, practitioners: [...one.practitioners, person] } },
        };
      },
    },
    leave: {
      line: "Stop practising at a clinic; your visits ahead there are cancelled, and the last to leave closes it.",
      args: (state) => clinicArgs(state),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = clinicOf(state, args, ctx);
        if (!one.practitioners.includes(me)) return ctx.refuse(`you do not practise at ${one.name}`);
        const practitioners = one.practitioners.filter((id) => id !== me);
        const shut = !practitioners.length;
        const clinic: Clinic = {
          ...one,
          practitioners,
          by: one.by === me ? (practitioners[0] ?? me) : one.by,
          closed: shut ? ctx.at : null,
        };
        const visits = cancelled(
          state.visits,
          ctx.at,
          (visit) => visit.clinic === one.n && (shut || visit.with === me),
        );
        return { ...state, clinics: { ...state.clinics, [one.n]: clinic }, visits };
      },
    },
    book: {
      line: `Book a visit with a practitioner, in 1 minute to 90 days, at the next ${SLOT / 60000}-minute slot; up to ${UPCOMING} ahead each.`,
      args: (state) => clinicArgs(state, { with: someone(state), in: { type: "integer", minimum: 1, maximum: AHEAD } }),
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const one = clinicOf(state, args, ctx);
        const doctor = String(args.with);
        if (doctor === me) return ctx.refuse("you cannot book a visit with yourself");
        if (!one.practitioners.includes(doctor)) return ctx.refuse(`${doctor} does not practise at ${one.name}`);
        const start = slot(ctx.at + Number(args.in) * 60000);
        const live = Object.values(state.visits).filter((visit) => ahead(visit, ctx.at));
        if (live.filter((visit) => visit.patient === me).length >= UPCOMING)
          return ctx.refuse(`you have ${UPCOMING} visits ahead; cancel one first`);
        if (live.some((visit) => visit.start === start && [visit.with, visit.patient].includes(doctor)))
          return ctx.refuse(`${doctor} is booked at that time; pick another`);
        if (live.some((visit) => visit.start === start && [visit.with, visit.patient].includes(me)))
          return ctx.refuse("you are booked at that time; pick another");
        const n = state.count.visits + 1;
        ctx.wish(notice(doctor, `${me} booked visit ${n} with you at ${one.name}`));
        const visit: Visit = { n, clinic: one.n, with: doctor, patient: me, start, booked: ctx.at, cancelled: null };
        return { ...state, visits: { ...state.visits, [n]: visit }, count: { ...state.count, visits: n } };
      },
    },
    cancel: {
      line: "Cancel a visit ahead, as its patient or its practitioner.",
      args: (state) => ({
        type: "object",
        properties: { visit: { type: "integer", minimum: 1, maximum: Math.max(1, state.count.visits) } },
        required: ["visit"],
        additionalProperties: false,
      }),
      run: (state, args, ctx) => {
        const one = state.visits[String(args.visit)]!;
        if (![one.patient, one.with].includes(ctx.who.id))
          return ctx.refuse(`only ${one.patient} and ${one.with} cancel visit ${one.n}`);
        if (one.cancelled !== null) return ctx.refuse(`visit ${one.n} is already cancelled`);
        if (one.start <= ctx.at) return ctx.refuse(`visit ${one.n} has begun; it cannot be cancelled`);
        const other = ctx.who.id === one.patient ? one.with : one.patient;
        ctx.wish(notice(other, `${ctx.who.id} cancelled visit ${one.n}`));
        return { ...state, visits: { ...state.visits, [one.n]: { ...one, cancelled: ctx.at } } };
      },
    },
  },
  moves: (state) => [
    "open",
    ...(state.count.clinics ? ["hire", "leave", "book"] : []),
    ...(state.count.visits ? ["cancel"] : []),
  ],
  read: sliced(mine),
  bot,
});
