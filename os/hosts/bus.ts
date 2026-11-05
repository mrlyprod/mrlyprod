import type { Act, App, Called, Json, Obj, Sent, Who, World } from "../kernel/index.ts";
import { Fault } from "../kernel/index.ts";

export const HOST = "host";

export const ACT = "act";

export const DEPTH = 3;

export type Answer = { ok: boolean; data?: Json };

export type Asked = Sent & { who: Who; at: number };

export type Handler = (wish: Asked, key: string) => Answer | Promise<Answer>;

export type Handlers = Record<string, Handler>;

export type Out = (wish: Asked) => Act | null;

export type Heard = (thrown: unknown, act: Act) => void;

export type Wiring = { apps: readonly App[]; world: () => World; step: (act: Act) => Called; heard?: Heard; out?: Out };

export const answered = (wish: Asked, answer: Answer): Act | null =>
  wish.reply
    ? {
        verb: wish.reply,
        args: { wish: wish.id, ok: answer.ok, ...(answer.data === undefined ? {} : { data: answer.data }) },
        who: wish.who,
        at: wish.at,
        via: HOST,
      }
    : null;

export const declined: Out = (wish) => answered(wish, { ok: false });

const present = (world: World, verb: string) =>
  Object.hasOwn(world.apps, verb.slice(0, Math.max(0, verb.indexOf("."))));

export function follows(done: Called, apps: readonly App[], out: Out, hooks: boolean): Act[] {
  const { world, envelope, line } = done;
  const from = line.verb.slice(0, line.verb.indexOf("."));
  const list: Act[] = [];
  if (envelope.last.ok)
    for (const wish of envelope.wishes) {
      if (wish.kind !== ACT) {
        const reply = out({ ...wish, who: line.who, at: line.at });
        if (reply) list.push(reply);
        continue;
      }
      const { verb, args, plain } = wish.data as { verb: string; args?: Obj; plain?: boolean };
      if (present(world, verb))
        list.push({ verb, args: args ?? {}, who: line.who, at: line.at, ...(plain ? {} : { via: from }) });
    }
  if (hooks)
    for (const id of Object.keys(world.apps)) {
      const next = apps.find((one) => one.id === id)?.after?.(done);
      if (next) list.push({ verb: next.verb, args: next.args ?? {}, who: line.who, at: line.at, via: id });
    }
  return list;
}

export function woken(world: World, apps: readonly App[], who: Who | undefined, at: number): Act[] {
  if (world.actors && !(who && world.actors.includes(who.id))) return [];
  const list: Act[] = [];
  for (const id of Object.keys(world.apps)) {
    const name = apps.find((one) => one.id === id)?.wake?.(world.apps[id], at);
    if (name) list.push({ verb: `${id}.${name}`, args: {}, who, at, via: HOST });
  }
  return list;
}

export const bus =
  ({ apps, world, step, heard, out = declined }: Wiring) =>
  (act: Act): Called => {
    const safe = (one: Act) => {
      try {
        return step(one);
      } catch (thrown) {
        heard?.(thrown, one);
        if (!heard && !(thrown instanceof Fault)) throw thrown;
        return null;
      }
    };
    const run = (list: Act[], depth: number) => {
      for (const next of list) {
        if (depth < 0) {
          heard?.(new Fault("too-deep", `${next.verb} follows ${DEPTH} acts deep, so it was dropped`), next);
          continue;
        }
        const one = safe(next);
        if (one) run(follows(one, apps, out, false), depth - 1);
      }
    };
    const at = Math.max(act.at ?? world().at, world().at);
    const wake = () => run(woken(world(), apps, act.who, at), DEPTH);
    wake();
    const done = step({ ...act, at });
    run(follows(done, apps, out, true), DEPTH);
    wake();
    return done;
  };
