import type { Schema } from "../spec/index.ts";
import { SPEC } from "../spec/index.ts";
import { check } from "./check.ts";
import type { Json, Obj } from "./json.ts";
import { canon, copy } from "./json.ts";
import { below, mix, seedOf, stream } from "./rand.ts";

/* TYPES */

export { SPEC };

export type Kind = "human" | "org" | "bot" | "agent";

export type Who = { id: string; handle?: string; kind: Kind };

export type Act = { verb: string; args?: Obj; who?: Who; at?: number };

export type Wish = { kind: string; data: Json; reply?: { verb: string } };

export type Notice = { from: Who; verb: string; data: Json };

export type Ctx = {
  who: Who;
  at: number;
  tick: number;
  rules: Obj;
  draw: () => number;
  read: (path: string) => Json | null;
  wish: (wish: Wish) => void;
  refuse: (note: string) => never;
};

export type Boot = { seed: number; config: Obj; actors: string[] | null; rules: Obj };

export type Pay = { from?: string; to: string; amount: number };

export type Paid = { from: string; to: string; amount: number };

export type Money<S = Json> = { move: (state: S, paid: Paid & { at: number; verb: string }) => S | string };

export type Verb<S = Json> = {
  line: string;
  args?: Schema | ((state: S) => Schema);
  pay?: (state: S, args: Obj, ctx: Ctx) => Pay | null;
  run: (state: S, args: Obj, ctx: Ctx) => S;
};

export type App<S = Json> = {
  id: string;
  user?: string;
  title: string;
  line: string;
  boot: (boot: Boot) => S;
  verbs: Record<string, Verb<S>>;
  moves?: (state: S) => string[];
  read?: (state: S, path: string[], who: Who | null) => Json | undefined;
  beat?: string;
  money?: Money<S>;
};

export type Entry = { id: string; config?: Obj };

export type Config = {
  spec?: string;
  name: string;
  time?: "timeline" | "stream";
  seed?: number;
  apps: (string | Entry)[];
  actors?: string[];
  rules?: Obj;
};

export type Last = { ok: boolean; note: string | null };

export type World = {
  spec: string;
  name: string;
  time: "timeline" | "stream";
  seed: number;
  tick: number;
  at: number;
  rand: number;
  route: { app: string | null };
  actors: string[] | null;
  rules: Obj;
  apps: Obj;
  last: Last;
};

export type Move = { verb: string; line: string; args: Schema };

export type Manifest = {
  id: string;
  user: string | null;
  title: string;
  line: string;
  verbs: string[];
  beat: string | null;
};

export type Envelope = {
  spec: string;
  tick: number;
  route: { app: string | null };
  state: Json | null;
  moves: string[];
  beat: string | null;
  last: Last;
  wishes: Wish[];
  notices: Notice[];
  paid?: Paid;
};

export type Line = { tick: number; at: number; who: Who; verb: string; args: Obj; n: number };

export type Called = { world: World; envelope: Envelope; line: Line };

export type Code =
  | "bad-act"
  | "bad-config"
  | "unknown-verb"
  | "unknown-world"
  | "no-iden"
  | "refused"
  | "spec-mismatch";

export class Fault extends Error {
  constructor(
    readonly code: Code,
    readonly note: string,
  ) {
    super(`${code}: ${note}`);
  }
}

class Refused extends Error {
  constructor(readonly note: string) {
    super(note);
  }
}

const KINDS: Kind[] = ["human", "org", "bot", "agent"];

const OWN = "nav";

const BANK = "wallet";

const ARGS: Schema = { type: "object", additionalProperties: false };

const NOBODY: Who = { id: "nobody", kind: "agent" };

/* NAMES */

export const prefix = (app: Pick<App, "id" | "user">) => (app.user ? `${app.user}.${app.id}` : app.id);

function distance(a: string, b: string): number {
  const row = Array.from({ length: b.length + 1 }, (_, i) => i);
  for (let i = 1; i <= a.length; i++) {
    let prev = row[0];
    row[0] = i;
    for (let j = 1; j <= b.length; j++) {
      const keep = row[j];
      row[j] = Math.min(row[j] + 1, row[j - 1] + 1, prev + (a[i - 1] === b[j - 1] ? 0 : 1));
      prev = keep;
    }
  }
  return row[b.length];
}

export function nearest(word: string, list: string[]): string | null {
  let best: string | null = null;
  let score = Number.POSITIVE_INFINITY;
  for (const one of list) {
    const d = distance(word, one);
    if (d < score) [best, score] = [one, d];
  }
  return best;
}

const major = (spec: string) => spec.split(".")[0];

/* RULES */

const picked = (value: Json): value is { pick: Json[] } =>
  !!value && typeof value === "object" && !Array.isArray(value) && Array.isArray((value as Obj).pick);

export function settle(rules: Obj, seed: number): Obj {
  const out: Obj = {};
  for (const key of Object.keys(rules).sort()) {
    const value = rules[key];
    if (!picked(value)) {
      out[key] = copy(value);
      continue;
    }
    if (!value.pick.length) throw new Fault("bad-config", `rule ${key} picks from nothing`);
    out[key] = copy(value.pick[below(stream(seedOf(seed, `rules/${key}`)).draw, value.pick.length)]);
  }
  return out;
}

/* KERNEL */

export type Kernel = ReturnType<typeof kernel>;

export const app = <S>(one: App<S>): App => one as unknown as App;

export function kernel(installed: readonly App[]) {
  const table = new Map<string, App>();
  for (const one of installed) {
    if (!/^[a-z][a-z0-9]*$/.test(one.id))
      throw new Error(`kernel: app id ${JSON.stringify(one.id)} must be lowercase letters and digits`);
    if (one.id === OWN) throw new Error(`kernel: ${OWN} is the kernel's own`);
    if (table.has(one.id)) throw new Error(`kernel: two apps are named ${one.id}`);
    for (const name of Object.keys(one.verbs))
      if (!/^[a-z][a-z0-9]*$/.test(name))
        throw new Error(`kernel: ${one.id} verb ${JSON.stringify(name)} must be lowercase letters and digits`);
    if (one.beat && !one.verbs[one.beat]) throw new Error(`kernel: ${one.id} beats ${one.beat}, which it lacks`);
    table.set(one.id, one);
  }

  const app = (world: World, id: string) => {
    const one = table.get(id);
    if (!one || !(id in world.apps)) throw new Fault("unknown-verb", `no app ${id} in ${world.name}`);
    return one;
  };

  const legal = (one: App, state: Json): string[] => {
    const all = Object.keys(one.verbs);
    if (!one.moves) return all;
    const now = new Set(one.moves(state));
    return all.filter((name) => now.has(name));
  };

  const argsOf = (verb: Verb, state: Json): Schema =>
    typeof verb.args === "function" ? verb.args(state) : (verb.args ?? ARGS);

  const ids = (world: World) => Object.keys(world.apps);

  const opening = (world: World): Schema => ({
    type: "object",
    properties: { app: { enum: ids(world) } },
    required: ["app"],
    additionalProperties: false,
  });

  function boot(config: Config): World {
    if (!config || typeof config !== "object") throw new Fault("bad-config", "a config is an object");
    const spec = config.spec ?? SPEC;
    if (major(spec) !== major(SPEC)) throw new Fault("spec-mismatch", `this kernel speaks ${SPEC}, the config ${spec}`);
    if (typeof config.name !== "string" || !/^[a-z0-9][a-z0-9-]*$/.test(config.name))
      throw new Fault("bad-config", "name must be lowercase letters, digits and hyphens");
    const seed = config.seed ?? 0;
    if (!Number.isInteger(seed) || seed < 0 || seed > 4294967295)
      throw new Fault("bad-config", "seed must be 0..4294967295");
    const time = config.time ?? "timeline";
    if (time !== "timeline" && time !== "stream") throw new Fault("bad-config", "time must be timeline or stream");
    if (!Array.isArray(config.apps) || !config.apps.length)
      throw new Fault("bad-config", "apps must name at least one app");
    const actors = config.actors ? [...config.actors] : null;
    const rules = settle(config.rules ?? {}, seed);
    const apps: Obj = {};
    for (const raw of config.apps) {
      const entry: Entry = typeof raw === "string" ? { id: raw } : raw;
      const one = table.get(entry.id);
      if (!one) {
        const near = nearest(entry.id, [...table.keys()]);
        throw new Fault("bad-config", `no app ${entry.id} is installed${near ? `; nearest: ${near}` : ""}`);
      }
      if (entry.id in apps) throw new Fault("bad-config", `${entry.id} is named twice`);
      apps[entry.id] = copy(one.boot({ seed: seedOf(seed, entry.id), config: entry.config ?? {}, actors, rules }));
    }
    return {
      spec: SPEC,
      name: config.name,
      time,
      seed,
      tick: 0,
      at: 0,
      rand: mix(seed),
      route: { app: Object.keys(apps)[0] },
      actors,
      rules,
      apps,
      last: { ok: true, note: null },
    };
  }

  function list(world: World): { moves: Move[]; apps: Manifest[] } {
    const moves: Move[] = [];
    const apps: Manifest[] = [];
    for (const id of ids(world)) {
      const one = app(world, id);
      const state = world.apps[id];
      const head = prefix(one);
      for (const name of legal(one, state)) {
        const verb = one.verbs[name];
        moves.push({ verb: `${head}.${name}`, line: verb.line, args: argsOf(verb, state) });
      }
      apps.push({
        id,
        user: one.user ?? null,
        title: one.title,
        line: one.line,
        verbs: Object.keys(one.verbs).map((name) => `${head}.${name}`),
        beat: one.beat ? `${head}.${one.beat}` : null,
      });
    }
    moves.push({ verb: `${OWN}.open`, line: "Bring an app to the front.", args: opening(world) });
    return { moves, apps };
  }

  const names = (world: World) => [
    ...ids(world).flatMap((id) => {
      const one = app(world, id);
      return legal(one, world.apps[id]).map((name) => `${prefix(one)}.${name}`);
    }),
    `${OWN}.open`,
  ];

  function envelope(world: World, state: Json | null, wishes: Wish[] = [], paid?: Paid): Envelope {
    const id = world.route.app;
    const one = id ? table.get(id) : undefined;
    return {
      spec: SPEC,
      tick: world.tick,
      route: world.route,
      state,
      moves: names(world),
      beat: one?.beat ? `${prefix(one)}.${one.beat}` : null,
      last: world.last,
      wishes,
      notices: [],
      ...(paid ? { paid } : {}),
    };
  }

  function walk(world: World, path: string[], who: Who | null = null): Json | undefined {
    if (!path.length) return Object.fromEntries(ids(world).map((id) => [id, walk(world, [id], who) ?? null]));
    const [id, ...rest] = path;
    if (!(id in world.apps)) return undefined;
    const one = table.get(id);
    const state = world.apps[id];
    if (one?.read) return one.read(state, rest, who);
    let at: Json | undefined = state;
    for (const key of rest) {
      if (at === null || typeof at !== "object") return undefined;
      at = Array.isArray(at) ? at[Number(key)] : at[key];
    }
    return at;
  }

  function rawAt(world: World, path: string[]): Json | undefined {
    let at: Json | undefined = world.apps;
    for (const key of path) {
      if (at === null || typeof at !== "object") return undefined;
      at = Array.isArray(at) ? at[Number(key)] : at[key];
    }
    return at;
  }

  function shaped(value: Json | undefined, shape: Json | undefined): Json | null {
    if (value === undefined) return null;
    if (shape === undefined || shape === true || value === null || typeof value !== "object") return value;
    const keep: Obj = Array.isArray(shape)
      ? Object.fromEntries(shape.map((key) => [String(key), true]))
      : shape && typeof shape === "object"
        ? (shape as Obj)
        : {};
    if (Array.isArray(value)) return value.map((one) => shaped(one, keep));
    const out: Obj = {};
    for (const key of Object.keys(keep)) if (key in value) out[key] = shaped(value[key], keep[key]);
    return out;
  }

  function read(world: World, path = "", shape?: Json, who: Who | null = null): Envelope {
    const parts = path.split("/").filter(Boolean);
    return envelope(world, shaped(walk(world, parts, who), shape));
  }

  function resolve(world: World, verb: string): { id: string | null; name: string } {
    const cut = verb.lastIndexOf(".");
    const head = cut < 0 ? "" : verb.slice(0, cut);
    const name = verb.slice(cut + 1);
    if (head === OWN) return { id: null, name };
    for (const id of ids(world)) if (prefix(app(world, id)) === head) return { id, name };
    const near = nearest(
      verb,
      list(world)
        .apps.flatMap((one) => one.verbs)
        .concat(`${OWN}.open`),
    );
    throw new Fault("unknown-verb", `no app owns ${verb}${near ? `; nearest: ${near}` : ""}`);
  }

  function settled(world: World, pay: Pay, by: Who, at: number, verb: string): { paid: Paid; bank: Json } {
    const money = table.get(BANK)?.money;
    if (!money || !(BANK in world.apps)) throw new Refused(`this world has no ${BANK}, so nothing can be paid`);
    const paid: Paid = { from: pay.from ?? by.id, to: pay.to, amount: pay.amount };
    if (paid.from !== by.id && paid.to !== by.id) throw new Refused(`${by.id} is on neither side of this payment`);
    const bank = money.move(world.apps[BANK], { ...paid, at, verb });
    if (typeof bank === "string") throw new Refused(bank);
    return { paid, bank };
  }

  function who(act: Act): Who {
    const one = act.who ?? NOBODY;
    if (!one || typeof one.id !== "string" || !one.id || !KINDS.includes(one.kind))
      throw new Fault("bad-act", `who must be { id, kind } with kind ${KINDS.join(", ")}`);
    return one;
  }

  function call(world: World, act: Act): Called {
    if (!act || typeof act !== "object" || typeof act.verb !== "string")
      throw new Fault("bad-act", "an act is { verb, args, who, at }");
    const args = act.args ?? {};
    if (!args || typeof args !== "object" || Array.isArray(args)) throw new Fault("bad-act", "args must be an object");
    const by = who(act);
    if (world.actors && !world.actors.includes(by.id))
      throw new Fault("refused", `${by.id} may watch ${world.name}, not act in it`);
    const at = act.at ?? world.at;
    if (!Number.isInteger(at) || at < world.at)
      throw new Fault("bad-act", `at must be an integer of milliseconds, not before ${world.at}`);
    canon(args);
    const { id, name } = resolve(world, act.verb);
    const line: Line = { tick: world.tick + 1, at, who: by, verb: act.verb, args, n: 1 };
    const done = (next: World, state: Json | null, wishes: Wish[] = [], paid?: Paid): Called => ({
      world: next,
      envelope: envelope(next, state, wishes, paid),
      line,
    });
    const refuse = (note: string): Called => {
      const next = { ...world, tick: world.tick + 1, at, last: { ok: false, note } };
      return done(next, id ? (walk(world, [id], by) ?? null) : null);
    };
    if (id === null) {
      if (name !== "open") throw new Fault("unknown-verb", `the kernel has ${OWN}.open, not ${act.verb}`);
      const note = check(opening(world), args);
      if (note) throw new Fault("bad-act", note);
      const route = { app: String(args.app) };
      return done(
        { ...world, tick: world.tick + 1, at, route, last: { ok: true, note: null } },
        walk(world, [route.app], by) ?? null,
      );
    }
    const one = app(world, id);
    const verb = one.verbs[name];
    if (!verb) {
      const near = nearest(name, Object.keys(one.verbs));
      throw new Fault(
        "unknown-verb",
        `${prefix(one)} has no ${name}${near ? `; nearest: ${prefix(one)}.${near}` : ""}`,
      );
    }
    const state = world.apps[id];
    const note = check(argsOf(verb, state), args);
    if (note) throw new Fault("bad-act", `${act.verb}: ${note}`);
    const now = legal(one, state);
    if (!now.includes(name)) {
      const near = nearest(name, now);
      return refuse(
        `${act.verb} is not legal now${near ? `; try ${prefix(one)}.${near}` : `; ${id} has no legal verb`}`,
      );
    }
    const rand = stream(world.rand);
    const wishes: Wish[] = [];
    const ctx: Ctx = {
      who: by,
      at,
      tick: line.tick,
      rules: world.rules,
      draw: rand.draw,
      read: (path) => copy(rawAt(world, path.split("/").filter(Boolean)) ?? null),
      wish: (wish) => {
        wishes.push(copy(wish));
      },
      refuse: (why) => {
        throw new Refused(why);
      },
    };
    let after: Json;
    let paid: Paid | undefined;
    let apps = world.apps;
    try {
      const pay = verb.pay?.(state, args, ctx) ?? null;
      if (pay) {
        const money = settled(world, pay, by, at, act.verb);
        paid = money.paid;
        apps = { ...apps, [BANK]: copy(money.bank) };
      }
      after = copy(verb.run(id === BANK ? apps[id] : state, args, ctx));
    } catch (thrown) {
      if (thrown instanceof Refused) return refuse(thrown.note);
      throw thrown;
    }
    const next: World = {
      ...world,
      tick: line.tick,
      at,
      rand: rand.state(),
      route: { app: id },
      apps: { ...apps, [id]: after },
      last: { ok: true, note: null },
    };
    return done(next, walk(next, [id], by) ?? null, wishes, paid);
  }

  return { boot, list, call, read, installed: () => [...table.values()] };
}
