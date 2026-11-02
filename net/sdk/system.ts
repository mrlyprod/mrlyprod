import type { Act, App, Called, Json, Obj, Who, Wish, World } from "../kernel/index.ts";
import { below, Fault, app as kernelApp, prefix } from "../kernel/index.ts";

export type { App, Boot, Ctx, Json, Kind, Obj, Schema, Verb, Who } from "../kernel/index.ts";
export { below, canon, digest, stream } from "../kernel/index.ts";

/* HOOKS */

export type Seen<S = Json> = { state: S; me: string; actors: string[]; at: number; draw: () => number; world: World };

export type Step = { verb: string; args: Obj } | null;

export type Bot<S = Json> = (seen: Seen<S>) => Step;

export type After = (done: Called) => Act | null;

export type Hooked<S = Json> = App<S> & { bot?: Bot<S>; after?: After };

export const hooked = (one: App): Hooked => one as Hooked;

/* APPS */

export type Manual = { id: string; title: string; line: string; verbs: Record<string, string> };

export const installed = new Map<string, App>();

export const app = <S>(one: Hooked<S>): App => {
  const made = kernelApp(one);
  installed.set(made.id, made);
  return made;
};

export const pick = <T>(draw: () => number, list: readonly T[]): T => list[below(draw, list.length)]!;

export const manual = (one: App): Manual => ({
  id: one.id,
  title: one.title,
  line: one.line,
  verbs: Object.fromEntries(Object.entries(one.verbs).map(([name, verb]) => [`${prefix(one)}.${name}`, verb.line])),
});

export const manuals = (ids?: readonly string[]): Manual[] =>
  [...installed.values()]
    .filter((one) => !ids || ids.includes(one.id))
    .sort((a, b) => a.id.localeCompare(b.id))
    .map(manual);

/* NAMES */

export const SLICE = "@";

export const named = (id: string) => {
  const words = id.split("-").join(" ");
  return words.charAt(0).toUpperCase() + words.slice(1);
};

/* SLICES */

export const walk = (at: Json | undefined, path: readonly string[]): Json | undefined => {
  for (const key of path) {
    if (at === null || at === undefined || typeof at !== "object") return undefined;
    at = Array.isArray(at) ? at[Number(key)] : at[key];
  }
  return at;
};

export const sliced =
  <S>(
    slice: (state: S, id: string, who: Who | null) => Json | undefined,
    whole?: (state: S, who: Who | null) => Json,
  ) =>
  (state: S, path: string[], who: Who | null): Json | undefined => {
    const [head = "", ...rest] = path;
    if (head.startsWith(SLICE)) return walk(slice(state, head.slice(SLICE.length), who), rest);
    return walk(whole ? whole(state, who) : (state as Json), path);
  };

/* NOTICES */

export const INBOX = "notifications";

export const NOTICE = 140;

export const notice = (to: string, text: string): Wish => ({
  kind: "notice",
  data: { to, text: text.length > NOTICE ? `${text.slice(0, NOTICE - 3)}...` : text },
});

/* THE BUS */

export const NAV = "nav";

export const act = (verb: string, args: Obj = {}): Wish => ({ kind: "act", data: { verb, args } });

export const appOf = (verb: string) => {
  const head = verb.slice(0, verb.lastIndexOf("."));
  return head.slice(head.lastIndexOf(".") + 1);
};

export const verbOf = (verb: string) => verb.slice(verb.lastIndexOf(".") + 1);

export function noticed(done: Called, apps: readonly App[]): Act[] {
  const { world, envelope, line } = done;
  const from = appOf(line.verb);
  const by = { who: line.who, at: line.at };
  const out: Act[] = [];
  if (!from || from === NAV) return out;
  if (envelope.last.ok)
    for (const wish of envelope.wishes) {
      if (wish.kind === "notice" && INBOX in world.apps && from !== INBOX)
        out.push({ verb: `${INBOX}.notice`, args: { ...(wish.data as Obj), app: from }, ...by });
      if (wish.kind === "act") {
        const { verb, args } = wish.data as { verb: string; args?: Obj };
        out.push({ verb, args: args ?? {}, ...by });
      }
    }
  for (const id of Object.keys(world.apps)) {
    const one = apps.find((installed) => installed.id === id);
    const next = one && hooked(one).after?.(done);
    if (next) out.push({ ...next, ...by });
  }
  return out;
}

export const bus =
  (apps: readonly App[], step: (act: Act) => Called) =>
  (act: Act): Called => {
    const done = step(act);
    for (const next of noticed(done, apps))
      try {
        step(next);
      } catch (thrown) {
        if (!(thrown instanceof Fault)) throw thrown;
      }
    return done;
  };
