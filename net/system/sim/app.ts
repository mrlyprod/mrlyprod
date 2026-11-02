import type { Obj, Schema } from "../../sdk/system.ts";
import { app, appOf, canon, NOTICE, sliced, verbOf } from "../../sdk/system.ts";
import bot from "./bot.ts";

/* TYPES */

export type Tick = { tick: number; at: number; app: string; verb: string; ok: boolean; note: string | null };

export type Tally = { acts: number; refused: number };

export type Snap = { tick: number; at: number; by: string; bytes: Record<string, number> };

export type Sim = {
  users: string[];
  acts: number;
  refused: number;
  apps: Record<string, Tally>;
  now: { tick: number; at: number };
  snaps: Snap[];
  history: Record<string, Tick[]>;
};

export const KEEP = 50;

export const SNAPS = 20;

const ID = "sim";

const NAME = 64;

/* SCHEMAS */

const ticking: Schema = {
  type: "object",
  properties: {
    app: { type: "string", minLength: 1, maxLength: NAME },
    verb: { type: "string", minLength: 1, maxLength: NAME },
    ok: { type: "boolean" },
    note: { type: "string", maxLength: NOTICE },
  },
  required: ["app", "verb", "ok"],
  additionalProperties: false,
};

const EMPTY: Schema = { type: "object", additionalProperties: false };

/* APP */

export default app<Sim>({
  id: ID,
  title: "Sim",
  line: "This world's own stats: its users, acts and refusals per app, the tick, and snapshots of every app's size.",
  boot: ({ actors }) => ({
    users: actors ?? [],
    acts: 0,
    refused: 0,
    apps: {},
    now: { tick: 0, at: 0 },
    snaps: [],
    history: {},
  }),
  verbs: {
    tick: {
      line: "Count one act of yours: the app, the verb, whether it went through; the host sends one after each act.",
      args: ticking,
      run: (state, args, ctx) => {
        const me = ctx.who.id;
        const id = String(args.app);
        const ok = Boolean(args.ok);
        const was = state.apps[id] ?? { acts: 0, refused: 0 };
        const one: Tick = {
          tick: ctx.tick - 1,
          at: ctx.at,
          app: id,
          verb: String(args.verb),
          ok,
          note: args.note === undefined ? null : String(args.note),
        };
        return {
          ...state,
          acts: state.acts + 1,
          refused: state.refused + (ok ? 0 : 1),
          apps: { ...state.apps, [id]: { acts: was.acts + 1, refused: was.refused + (ok ? 0 : 1) } },
          now: { tick: ctx.tick, at: ctx.at },
          history: { ...state.history, [me]: [...(state.history[me] ?? []), one].slice(-KEEP) },
        };
      },
    },
    snap: {
      line: `Snapshot the world now: the tick, the time and every app's state in bytes; the last ${SNAPS} are kept.`,
      args: EMPTY,
      run: (state, _args, ctx) => {
        const world = (ctx.read("") ?? {}) as Obj;
        const bytes = Object.fromEntries(Object.keys(world).map((id) => [id, canon(world[id]!).length]));
        const snap: Snap = { tick: ctx.tick, at: ctx.at, by: ctx.who.id, bytes };
        return { ...state, now: { tick: ctx.tick, at: ctx.at }, snaps: [...state.snaps, snap].slice(-SNAPS) };
      },
    },
  },
  read: sliced((state: Sim, me: string) => {
    const mine = state.history[me] ?? [];
    return { acts: mine.length, refused: mine.filter((one) => !one.ok).length, history: [...mine].reverse() };
  }),
  bot,
  after: ({ envelope, line }) => {
    const from = appOf(line.verb);
    if (from === ID) return null;
    const note = envelope.last.note;
    return {
      verb: `${ID}.tick`,
      args: {
        app: from,
        verb: verbOf(line.verb),
        ok: envelope.last.ok,
        ...(note ? { note: note.slice(0, NOTICE) } : {}),
      },
    };
  },
});
