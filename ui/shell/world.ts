import { named } from "@mrlyprod/os/contract/index.ts";
import { bus } from "@mrlyprod/os/hosts/bus.ts";
import type { Act, App, Config, Line, Move, Who, World } from "@mrlyprod/os/kernel/index.ts";
import { Fault, kernel, replay } from "@mrlyprod/os/kernel/index.ts";
import type { Port } from "../kit/world.ts";
import { plug } from "../kit/world.ts";
import { PREFIX } from "./prefs.ts";

export type User = { id: string; label: string; name: string; hue: string | null; glyph: string | null; kind: string };

export type Options = {
  apps: readonly App[];
  config: Config;
  as?: string | null;
  lines?: readonly Line[];
};

export type Store = Port & {
  world: () => World;
  lines: () => readonly Line[];
  users: () => User[];
  sign: (id: string) => boolean;
};

type Kept = { seed: number; who: string; lines: Line[] };

type Card = { name?: string; hue?: string; glyph?: string; kind?: string };

const kindOf = (world: World, id: string) =>
  ((world.apps.people as Record<string, Card> | undefined)?.[id]?.kind as string | undefined) ?? "human";

export const GUEST: Who = { id: "guest", kind: "human" };

export const WATCH = "Sign in as a test user to do this";

const HANDOVER = `${PREFIX}world`;

const TOP = 4294967295;

export function create({ apps, config, as = null, lines: kept = [] }: Options): Store {
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
  const apply = bus({ apps, world: () => world, step });
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
    read: (path = "", shape) => k.read(world, path, shape, actors.includes(me.id) ? me : null),
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
  };
}

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

export const seeded = (text: string | null) => {
  const n = Number(text ?? "");
  return text && Number.isInteger(n) && n >= 0 && n <= TOP ? n : null;
};

let started: Promise<Store | null> | null = null;

async function boot(): Promise<Store | null> {
  const { apps, config } = await import("site:world");
  if (!config) return null;
  const url = new URL(location.href);
  const kept = taken();
  const seed = kept?.seed ?? seeded(url.searchParams.get("seed")) ?? Math.floor(Math.random() * TOP);
  const as = url.searchParams.get("as") ?? kept?.who ?? null;
  const open = (lines?: Line[]) => create({ apps, config: { ...config, seed }, as, lines });
  let store: Store;
  try {
    store = open(kept?.lines);
  } catch {
    store = open();
  }
  live = store;
  plug(store);
  return store;
}

export const start = () => (started ??= boot());
