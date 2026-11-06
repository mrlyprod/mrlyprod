import { named } from "@mrlyprod/os/contract/index.ts";
import { bus } from "@mrlyprod/os/hosts/bus.ts";
import type { Act, App, Config, Kernel, Line, Who, World } from "@mrlyprod/os/kernel/index.ts";
import { canon, Fault, kernel, resume } from "@mrlyprod/os/kernel/index.ts";
import type { Port } from "../kit/world.ts";
import { plug } from "../kit/world.ts";
import { adopt, KEY, nameOf, PREFIX, pack, read, record, SETTINGS } from "./prefs.ts";

export type User = { id: string; label: string; name: string; hue: string | null; glyph: string | null; kind: string };

export type Kept = { world: World; lines: Line[] };

export type Options = {
  apps: readonly App[];
  config: Config;
  as?: string | null;
  kept?: Kept | null;
};

export type Store = Port & {
  world: () => World;
  lines: () => readonly Line[];
  users: () => User[];
  sign: (id: string) => boolean;
  wipe: () => void;
};

type Card = { name?: string; hue?: string; glyph?: string; kind?: string };

const kindOf = (world: World, id: string) =>
  ((world.apps.people as Record<string, Card> | undefined)?.[id]?.kind as string | undefined) ?? "human";

export const GUEST: Who = { id: "guest", kind: "human" };

export const WATCH = "Sign in as a test user to do this";

const SNAPSHOT = `${PREFIX}world`;

const JOURNAL = `${PREFIX}journal`;

const CAP = 200;

const SET = "settings.set";

const TOP = 4294967295;

export const capOf = () => {
  const text = read(KEY.history);
  const n = Number(text);
  return text && Number.isInteger(n) && n >= 0 ? n : CAP;
};

const put = (key: string, value: unknown) => {
  try {
    localStorage.setItem(key, JSON.stringify(value));
    return true;
  } catch {
    return false;
  }
};

const fitted = (k: Kernel, config: Config, kept: World): World => {
  const fresh = k.boot({ ...config, seed: kept.seed });
  if (canon(Object.keys(fresh.apps).sort()) === canon(Object.keys(kept.apps).sort())) return kept;
  const held = Object.keys(fresh.apps).filter((id) => Object.hasOwn(kept.apps, id));
  const apps = { ...fresh.apps, ...Object.fromEntries(held.map((id) => [id, kept.apps[id]!])) };
  return { ...kept, rules: fresh.rules, actors: fresh.actors, apps };
};

export function create({ apps, config, as = null, kept = null }: Options): Store {
  const k = kernel(apps);
  const actors = config.actors ?? [];
  const lines: Line[] = kept ? kept.lines.filter((line) => line.tick > kept.world.tick) : [];
  const heard = new Set<() => void>();
  let base = kept ? fitted(k, config, kept.world) : k.boot(config);
  let world = resume(k, base, lines);
  let saved = Boolean(kept);
  const fold = (count: number) => {
    base = resume(k, base, lines.slice(0, count));
    lines.splice(0, count);
    saved = put(SNAPSHOT, base);
  };
  const persist = () => {
    const over = lines.length - capOf();
    if (over > 0) fold(over);
    else if (!saved) saved = put(SNAPSHOT, base);
    if (saved && put(JOURNAL, lines)) return;
    fold(lines.length);
    put(JOURNAL, lines);
  };
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
        persist();
        if (act.verb === SET && envelope.last.ok) adopt(String(act.args?.key), String(act.args?.value));
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
    view: (id) => apps.find((one) => one.id === id)?.view ?? null,
    world: () => world,
    lines: () => lines,
    users,
    sign: (id) => {
      if (id !== GUEST.id && !actors.includes(id)) return false;
      me = id === GUEST.id ? GUEST : actor(id);
      ring();
      return true;
    },
    wipe: () => {
      if (Object.hasOwn(world.apps, "logs")) apply({ verb: "logs.clear", who: me, at: stamp() });
      fold(lines.length);
      put(JOURNAL, lines);
      ring();
    },
  };
}

let live: Store | null = null;

export const current = () => live;

function taken(): Kept | null {
  try {
    const world = JSON.parse(localStorage.getItem(SNAPSHOT) ?? "null") as World | null;
    return world && { world, lines: JSON.parse(localStorage.getItem(JOURNAL) ?? "[]") as Line[] };
  } catch {
    return null;
  }
}

export const seeded = (text: string | null) => {
  const n = Number(text ?? "");
  return text && Number.isInteger(n) && n >= 0 && n <= TOP ? n : null;
};

function settle(store: Store, resumed: boolean) {
  const held = () => (store.read("settings/@me").state ?? {}) as Record<string, string>;
  const set = (key: string, value: string) => store.call({ verb: SET, args: { key, value } });
  if (resumed) {
    const mine = held();
    for (const name of [...SETTINGS, ...Object.keys(mine)]) adopt(name, mine[name] ?? "");
  } else {
    for (const [name, value] of Object.entries(pack())) set(name, value);
  }
  record((key, value) => {
    const name = nameOf(key);
    if (name && (held()[name] ?? "") !== value) set(name, value);
  });
}

let started: Promise<Store | null> | null = null;

async function boot(): Promise<Store | null> {
  const { apps, config } = await import("site:world");
  const url = new URL(location.href);
  const asked = seeded(url.searchParams.get("seed"));
  const held = taken();
  const kept = held && (asked === null || asked === held.world.seed) ? held : null;
  const seed = kept?.world.seed ?? asked ?? Math.floor(Math.random() * TOP);
  const as = url.searchParams.get("as");
  const open = (from: Kept | null) => create({ apps, config: { ...config, seed }, as, kept: from });
  let store: Store;
  try {
    store = open(kept);
  } catch {
    store = open(null);
  }
  live = store;
  plug(store);
  if (Object.hasOwn(store.world().apps, "settings")) settle(store, Boolean(kept));
  return store;
}

export const start = () => (started ??= boot());
