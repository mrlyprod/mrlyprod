import type { Act, App, Config, Kind, Line, Move, Who, World } from "../kernel/index.ts";
import { Fault, kernel, replay } from "../kernel/index.ts";
import { kindOf } from "../sdk/kinds.ts";
import { bus, named } from "../sdk/system.ts";
import type { Port } from "../sdk/world.ts";
import { plug } from "../sdk/world.ts";
import { PREFIX } from "./prefs.ts";

/* TYPES */

export type User = { id: string; label: string; name: string; hue: string | null; glyph: string | null; kind: Kind };

export type Table = { world: () => World; list: () => { moves: Move[] }; call: (act: Act) => unknown };

export type Players = (table: Table, who: readonly Who[], at: number) => number;

export type Options = {
  apps: readonly App[];
  config: Config;
  as?: string | null;
  lines?: readonly Line[];
  players?: Players;
};

export type Store = Port & {
  world: () => World;
  lines: () => readonly Line[];
  users: () => User[];
  sign: (id: string) => boolean;
  round: (at?: number) => number;
};

type Kept = { seed: number; who: string; lines: Line[] };

type Card = { name?: string; hue?: string; glyph?: string };

export const GUEST: Who = { id: "guest", kind: "human" };

export const BEAT = 6000;

export const WATCH = "Sign in as a test user to do this";

const HANDOVER = `${PREFIX}world`;

const TOP = 4294967295;

/* STORE */

export function create({ apps, config, as = null, lines: kept = [], players }: Options): Store {
  const k = kernel(apps);
  const actors = config.actors ?? [];
  const lines: Line[] = [...kept];
  const heard = new Set<() => void>();
  let world = kept.length ? replay(k, config, kept) : k.boot(config);
  const actor = (id: string): Who => ({ id, kind: kindOf(world, id) });
  let me: Who = as && actors.includes(as) ? actor(as) : GUEST;
  const ring = () => {
    for (const one of heard) one();
  };
  const step = (act: Act) => {
    const done = k.call(world, act);
    world = done.world;
    lines.push(done.line);
    return done;
  };
  const apply = bus(apps, step);
  const stamp = () => Math.max(Date.now(), world.at);
  const users = (): User[] => {
    const cards = (k.read(world, "people").state ?? {}) as Record<string, Card>;
    return [
      { id: GUEST.id, label: "Guest", name: "Guest", hue: null, glyph: null, kind: GUEST.kind },
      ...actors.map((id) => ({
        id,
        label: named(id),
        name: cards[id]?.name ?? named(id),
        hue: cards[id]?.hue ?? null,
        glyph: cards[id]?.glyph ?? null,
        kind: kindOf(world, id),
      })),
    ];
  };
  return {
    subscribe: (one) => {
      heard.add(one);
      return () => {
        heard.delete(one);
      };
    },
    read: (path = "", shape) => k.read(world, path, shape, me),
    list: () => k.list(world),
    call: (act) => {
      const who = act.who ?? me;
      try {
        const { envelope } = apply({ ...act, who, at: act.at ?? stamp() });
        ring();
        return { ok: envelope.last.ok, note: envelope.last.note, envelope };
      } catch (thrown) {
        if (!(thrown instanceof Fault)) throw thrown;
        const watching = thrown.code === "refused" && !actors.includes(who.id);
        return { ok: false, note: watching ? WATCH : thrown.note, envelope: null };
      }
    },
    who: () => me,
    actor: () => actors.includes(me.id),
    at: () => world.at,
    world: () => world,
    lines: () => lines,
    users,
    sign: (id) => {
      if (id !== GUEST.id && !actors.includes(id)) return false;
      me = id === GUEST.id ? GUEST : actor(id);
      ring();
      return true;
    },
    round: (at = stamp()) => {
      if (!players) return 0;
      const who = actors.filter((id) => id !== me.id).map(actor);
      const acts = players({ world: () => world, list: () => k.list(world), call: apply }, who, at);
      if (acts) ring();
      return acts;
    },
  };
}

/* HANDOVER */

let live: Store | null = null;

export const current = () => live;

export function handover() {
  if (!live) return;
  try {
    const kept: Kept = { seed: live.world().seed, who: live.who().id, lines: [...live.lines()] };
    sessionStorage.setItem(HANDOVER, JSON.stringify(kept));
  } catch {}
}

function taken(): Kept | null {
  try {
    const text = sessionStorage.getItem(HANDOVER);
    sessionStorage.removeItem(HANDOVER);
    return text ? (JSON.parse(text) as Kept) : null;
  } catch {
    return null;
  }
}

/* START */

export const seeded = (text: string | null) => {
  const n = Number(text ?? "");
  return text && Number.isInteger(n) && n >= 0 && n <= TOP ? n : null;
};

export const paced = (store: Pick<Store, "round">, hidden: () => boolean) => () => (hidden() ? 0 : store.round());

let started: Promise<Store | null> | null = null;

async function boot(): Promise<Store | null> {
  const { apps, config, players } = await import("site:world");
  if (!config) return null;
  const url = new URL(location.href);
  const kept = taken();
  const seed = kept?.seed ?? seeded(url.searchParams.get("seed")) ?? Math.floor(Math.random() * TOP);
  const as = url.searchParams.get("as") ?? kept?.who ?? null;
  const open = (lines?: Line[]) => create({ apps, config: { ...config, seed }, as, lines, players });
  let store: Store;
  try {
    store = open(kept?.lines);
  } catch {
    store = open();
  }
  live = store;
  plug(store);
  setInterval(
    paced(store, () => document.hidden),
    BEAT,
  );
  return store;
}

export const start = () => (started ??= boot());
