import type { Schema } from "../contract/spec/index.ts";
import { SPEC } from "../contract/spec/index.ts";
import { check } from "./check.ts";
import type { Json, Obj } from "./json.ts";
import { canon, copy, walk } from "./json.ts";
import { below, mix, seedOf, stream } from "./rand.ts";

export { SPEC };

export type Who = { id: string; kind: string };

export type Act = { verb: string; args?: Obj; who?: Who; at?: number; via?: string };

export type Wish = { kind: string; data: Json; reply?: string };

export type Sent = Wish & { id: string };

export type Knob = <T extends Json = number>(name: string) => T;

export type Ctx = {
  who: Who;
  via: string | null;
  at: number;
  draw: () => number;
  refuse: (note: string) => never;
  say: (value: Json) => void;
  see: (path: string) => Json | null;
  read: (path: string) => Json | null;
  call: (verb: string, args?: Obj) => Json | null;
  wish: (wish: Wish) => void;
  knob: Knob;
};

export type Boot = { seed: number; start: number; config: Obj; actors: string[] | null; knob: Knob };

export type Verb<S = Json> = {
  line: string | ((knob: Knob) => string);
  args?: Schema | ((knob: Knob) => Schema);
  inner?: boolean;
  run: (state: S, args: Obj, ctx: Ctx) => S;
};

export type Setting = { line: string; schema: Schema; reality: Json };

export type Knobs = Record<string, Setting>;

export type Peek = (path: string) => Json | null;

export type App<S = Json> = {
  id: string;
  title: string;
  line: string;
  boot: (boot: Boot) => S;
  verbs: Record<string, Verb<S>>;
  read?: (state: S, path: string[], who: Who | null, at: number, peek: Peek) => Json | undefined;
  moves?: (state: S) => string[];
  knobs?: Knobs;
  wake?: (state: S, at: number) => string | null;
  after?: (done: Called) => Act | null;
};

export type Entry = { id: string; config?: Obj };

export type Config = {
  spec?: string;
  name: string;
  seed?: number;
  start?: number;
  pace?: number;
  front?: string[];
  apps: (string | Entry)[];
  actors?: string[];
  rules?: Obj;
};

export type Last = { ok: boolean; note: string | null };

export type World = {
  spec: string;
  name: string;
  seed: number;
  tick: number;
  at: number;
  rand: number;
  actors: string[] | null;
  rules: Obj;
  apps: Obj;
  last: Last;
};

export type Move = { verb: string; line: string; args: Schema };

export type Manifest = { id: string; title: string; line: string; verbs: string[] };

export type Envelope = {
  spec: string;
  tick: number;
  state: Json | null;
  moves: string[];
  last: Last;
  wishes: Sent[];
  said?: Json;
};

export type Line = { tick: number; at: number; who: Who; verb: string; args: Obj; via?: string };

export type Called = { world: World; envelope: Envelope; line: Line };

export type Code =
  | "bad-act"
  | "bad-config"
  | "unknown-verb"
  | "unknown-world"
  | "refused"
  | "spec-mismatch"
  | "too-deep";

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

const ARGS: Schema = { type: "object", additionalProperties: false };

const NOBODY: Who = { id: "nobody", kind: "agent" };

const NAME = /^[a-z][a-z0-9]*$/;

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

export const reader =
  (rules: Obj, id: string): Knob =>
  <T extends Json = number>(name: string) => {
    const key = name.includes(".") ? name : `${id}.${name}`;
    return (Object.hasOwn(rules, key) ? rules[key] : null) as T;
  };

const parts = (path: string) => path.split("/").filter(Boolean);

function kept(shape: Json): Obj {
  if (!Array.isArray(shape)) return shape && typeof shape === "object" ? shape : {};
  const [head] = shape;
  if (shape.length === 1 && head && typeof head === "object") return kept(head);
  return Object.fromEntries(shape.map((key) => [String(key), true]));
}

function isTable(value: Obj, keep: Obj): boolean {
  const names = Object.keys(keep);
  const rows = Object.values(value);
  const record = (one: Json | undefined) => one !== null && typeof one === "object";
  return names.length > 0 && !names.some((key) => Object.hasOwn(value, key)) && rows.length > 0 && rows.every(record);
}

function shaped(value: Json | undefined, shape: Json | undefined): Json | null {
  if (value === undefined) return null;
  if (shape === undefined || shape === true || value === null || typeof value !== "object") return value;
  const keep = kept(shape);
  if (Array.isArray(value)) return value.map((one) => shaped(one, keep));
  const every = Object.hasOwn(keep, "*") ? keep["*"] : undefined;
  if (every === undefined && isTable(value, keep)) {
    return Object.fromEntries(Object.entries(value).map(([key, one]) => [key, shaped(one, keep)]));
  }
  const out: Obj = {};
  for (const key of every === undefined ? Object.keys(keep) : Object.keys(value))
    if (Object.hasOwn(value, key)) out[key] = shaped(value[key], Object.hasOwn(keep, key) ? keep[key] : every);
  return out;
}

export type Kernel = ReturnType<typeof kernel>;

export const app = <S>(one: App<S>): App => one as unknown as App;

export function kernel(installed: readonly App[]) {
  const table = new Map<string, App>();
  for (const one of installed) {
    if (!NAME.test(one.id))
      throw new Error(`kernel: app id ${JSON.stringify(one.id)} must be lowercase letters and digits`);
    if (table.has(one.id)) throw new Error(`kernel: two apps are named ${one.id}`);
    for (const name of Object.keys(one.verbs))
      if (!NAME.test(name))
        throw new Error(`kernel: ${one.id} verb ${JSON.stringify(name)} must be lowercase letters and digits`);
    table.set(one.id, one);
  }

  const ids = (world: World) => Object.keys(world.apps);

  const app = (world: World, id: string) => {
    const one = table.get(id);
    if (!one || !Object.hasOwn(world.apps, id)) throw new Fault("unknown-verb", `no app ${id} in ${world.name}`);
    return one;
  };

  const legal = (one: App, state: Json): string[] => {
    const all = Object.keys(one.verbs);
    if (!one.moves) return all;
    const now = new Set(one.moves(state));
    return all.filter((name) => now.has(name));
  };

  const argsOf = (verb: Verb, knob: Knob): Schema =>
    typeof verb.args === "function" ? verb.args(knob) : (verb.args ?? ARGS);

  const lineOf = (verb: Verb, knob: Knob) => (typeof verb.line === "function" ? verb.line(knob) : verb.line);

  function ruled(given: Obj, apps: string[]): Obj {
    const rules = { ...given };
    for (const key of Object.keys(rules)) {
      const cut = key.indexOf(".");
      if (cut < 0) throw new Fault("bad-config", `rule ${key} names no app; a rule is app.knob`);
      const id = key.slice(0, cut);
      const name = key.slice(cut + 1);
      const one = table.get(id);
      if (!one || !apps.includes(id)) throw new Fault("bad-config", `rule ${key}: no app ${id} in this world`);
      const knobs = one.knobs ?? {};
      if (!Object.hasOwn(knobs, name)) {
        const near = nearest(name, Object.keys(knobs));
        throw new Fault(
          "bad-config",
          `rule ${key}: ${id} has no knob ${name}${near ? `; nearest: ${id}.${near}` : `; ${id} has no knobs`}`,
        );
      }
      const note = check(knobs[name].schema, rules[key], key);
      if (note) throw new Fault("bad-config", `rule ${note}`);
    }
    for (const id of apps)
      for (const [name, knob] of Object.entries(table.get(id)?.knobs ?? {}))
        if (!Object.hasOwn(rules, `${id}.${name}`)) rules[`${id}.${name}`] = copy(knob.reality);
    return Object.fromEntries(
      Object.keys(rules)
        .sort()
        .map((key) => [key, rules[key]]),
    );
  }

  function boot(config: Config): World {
    if (!config || typeof config !== "object") throw new Fault("bad-config", "a config is an object");
    const spec = config.spec ?? SPEC;
    if (major(spec) !== major(SPEC)) throw new Fault("spec-mismatch", `this kernel speaks ${SPEC}, the config ${spec}`);
    if (typeof config.name !== "string" || !/^[a-z0-9][a-z0-9-]*$/.test(config.name))
      throw new Fault("bad-config", "name must be lowercase letters, digits and hyphens");
    const seed = config.seed ?? 0;
    if (!Number.isInteger(seed) || seed < 0 || seed > 4294967295)
      throw new Fault("bad-config", "seed must be 0..4294967295");
    const start = config.start ?? 0;
    if (!Number.isInteger(start) || start < 0) throw new Fault("bad-config", "start must be whole milliseconds");
    if (config.pace !== undefined && (!Number.isInteger(config.pace) || config.pace < 1))
      throw new Fault("bad-config", "pace must be whole milliseconds above 0");
    if (
      config.front !== undefined &&
      !(Array.isArray(config.front) && config.front.every((one) => typeof one === "string"))
    )
      throw new Fault("bad-config", "front must be a list of paths");
    if (!Array.isArray(config.apps)) throw new Fault("bad-config", "apps must be a list of apps");
    const actors = config.actors ? [...config.actors] : null;
    const entries = config.apps.map((raw): Entry => (typeof raw === "string" ? { id: raw } : raw));
    const named: string[] = [];
    for (const entry of entries) {
      if (!table.has(entry.id)) {
        const near = nearest(entry.id, [...table.keys()]);
        throw new Fault("bad-config", `no app ${entry.id} is installed${near ? `; nearest: ${near}` : ""}`);
      }
      if (named.includes(entry.id)) throw new Fault("bad-config", `${entry.id} is named twice`);
      named.push(entry.id);
    }
    const rules = ruled(settle(config.rules ?? {}, seed), named);
    const apps: Obj = {};
    for (const entry of entries)
      apps[entry.id] = copy(
        table.get(entry.id)!.boot({
          seed: seedOf(seed, entry.id),
          start,
          config: entry.config ?? {},
          actors,
          knob: reader(rules, entry.id),
        }),
      );
    return {
      spec: SPEC,
      name: config.name,
      seed,
      tick: 0,
      at: start,
      rand: mix(seed),
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
      const knob = reader(world.rules, id);
      for (const name of legal(one, world.apps[id])) {
        const verb = one.verbs[name];
        if (!verb.inner) moves.push({ verb: `${id}.${name}`, line: lineOf(verb, knob), args: argsOf(verb, knob) });
      }
      apps.push({ id, title: one.title, line: one.line, verbs: Object.keys(one.verbs).map((name) => `${id}.${name}`) });
    }
    return { moves, apps };
  }

  const names = (world: World) =>
    ids(world).flatMap((id) => {
      const one = app(world, id);
      return legal(one, world.apps[id])
        .filter((name) => !one.verbs[name].inner)
        .map((name) => `${id}.${name}`);
    });

  function envelope(world: World, state: Json | null, wishes: Sent[] = [], said?: Json): Envelope {
    return {
      spec: SPEC,
      tick: world.tick,
      state: copy(state),
      moves: names(world),
      last: world.last,
      wishes,
      ...(said === undefined ? {} : { said }),
    };
  }

  function look(world: World, path: string[], who: Who | null, at: number): Json | undefined {
    if (!path.length) return Object.fromEntries(ids(world).map((id) => [id, look(world, [id], who, at) ?? null]));
    const [id, ...rest] = path;
    if (!Object.hasOwn(world.apps, id)) return undefined;
    const one = table.get(id);
    const state = world.apps[id];
    if (!one?.read) return walk(state, rest);
    return one.read(state, rest, who, at, (to) => copy(walk(world.apps, parts(to)) ?? null));
  }

  function read(world: World, path = "", shape?: Json, who: Who | null = null): Envelope {
    return envelope(world, shaped(look(world, parts(path), who, world.at), shape));
  }

  function resolve(world: World, verb: string): { id: string; name: string } {
    const cut = verb.indexOf(".");
    const id = verb.slice(0, cut);
    const name = verb.slice(cut + 1);
    if (cut > 0 && Object.hasOwn(world.apps, id) && table.has(id)) return { id, name };
    const near = nearest(
      verb,
      ids(world).flatMap((one) => Object.keys(table.get(one)?.verbs ?? {}).map((each) => `${one}.${each}`)),
    );
    throw new Fault("unknown-verb", `no app owns ${verb}${near ? `; nearest: ${near}` : ""}`);
  }

  function who(act: Act): Who {
    const one = act.who ?? NOBODY;
    if (!one || typeof one.id !== "string" || !one.id || typeof one.kind !== "string" || !one.kind)
      throw new Fault("bad-act", "who must be { id, kind }, both strings");
    return { id: one.id, kind: one.kind };
  }

  function call(world: World, act: Act): Called {
    if (!act || typeof act !== "object" || typeof act.verb !== "string")
      throw new Fault("bad-act", "an act is { verb, args, who, at }");
    const given = act.args ?? {};
    if (!given || typeof given !== "object" || Array.isArray(given))
      throw new Fault("bad-act", "args must be an object");
    const by = who(act);
    if (world.actors && !world.actors.includes(by.id))
      throw new Fault("refused", `${by.id} may watch ${world.name}, not act in it`);
    const at = act.at ?? world.at;
    if (!Number.isInteger(at) || at < world.at)
      throw new Fault("bad-act", `at must be an integer of milliseconds, not before ${world.at}`);
    let args: Obj;
    try {
      args = JSON.parse(canon(given));
    } catch (thrown) {
      throw new Fault("bad-act", `args cannot be kept: ${(thrown as Error).message}`);
    }
    const { id, name } = resolve(world, act.verb);
    const line: Line = {
      tick: world.tick + 1,
      at,
      who: by,
      verb: act.verb,
      args,
      ...(act.via ? { via: act.via } : {}),
    };
    const one = app(world, id);
    const verb = one.verbs[name];
    if (!verb) {
      const near = nearest(name, Object.keys(one.verbs));
      throw new Fault("unknown-verb", `${id} has no ${name}${near ? `; nearest: ${id}.${near}` : ""}`);
    }
    const note = check(argsOf(verb, reader(world.rules, id)), args);
    if (note) throw new Fault("bad-act", `${act.verb}: ${note}`);
    const refuse = (why: string): Called => {
      const next = { ...world, tick: line.tick, at, last: { ok: false, note: why } };
      return { world: next, envelope: envelope(next, look(next, [id], by, at) ?? null), line };
    };
    if (verb.inner && !act.via) return refuse(`${act.verb} runs only when another app asks for it`);
    const rand = stream(world.rand);
    const live: Obj = { ...world.apps };
    const wishes: Sent[] = [];
    const running: string[] = [];
    let broken: string | null = null;
    const halt = (why: string): never => {
      broken ??= why;
      throw new Refused(why);
    };
    const now = (): World => ({ ...world, at, apps: live });
    const run = (target: string, chosen: Verb, input: Obj, via: string | null): Json | undefined => {
      let said: Json | undefined;
      running.push(target);
      const ctx: Ctx = {
        who: by,
        via,
        at,
        draw: rand.draw,
        refuse: halt,
        say: (value) => {
          said = copy(value);
        },
        see: (path) => copy(look(now(), parts(path), by, at) ?? null),
        read: (path) => copy(walk(live, parts(path)) ?? null),
        call: (to, more = {}) => nested(target, to, more),
        wish: (wish) => {
          wishes.push({ ...copy(wish), id: `${line.tick}.${wishes.length}` });
        },
        knob: reader(world.rules, target),
      };
      try {
        live[target] = copy(chosen.run(live[target], input, ctx));
      } finally {
        running.pop();
      }
      return said;
    };
    const nested = (caller: string, to: string, given: Obj): Json | null => {
      const cut = to.indexOf(".");
      const target = to.slice(0, cut);
      const want = to.slice(cut + 1);
      const other = cut > 0 && Object.hasOwn(live, target) ? table.get(target) : undefined;
      if (!other) return halt(`this world has no app for ${to}`);
      if (running.includes(target)) throw new Error(`kernel: ${caller} calls ${to} while ${target} is mid-act`);
      const chosen = other.verbs[want];
      if (!chosen) return halt(`${target} has no ${want}`);
      const input = copy(given);
      const bad = check(argsOf(chosen, reader(world.rules, target)), input);
      if (bad) return halt(`${to}: ${bad}`);
      if (!legal(other, live[target]).includes(want)) return halt(`${to} is not legal now`);
      return run(target, chosen, input, caller) ?? null;
    };
    const allowed = legal(one, world.apps[id]);
    if (!allowed.includes(name)) {
      const near = nearest(name, allowed);
      return refuse(`${act.verb} is not legal now${near ? `; try ${id}.${near}` : `; ${id} has no legal verb`}`);
    }
    let said: Json | undefined;
    try {
      said = run(id, verb, args, act.via ?? null);
    } catch (thrown) {
      if (thrown instanceof Refused) return refuse(broken ?? thrown.note);
      throw thrown;
    }
    if (broken) return refuse(broken);
    const next: World = {
      ...world,
      tick: line.tick,
      at,
      rand: rand.state(),
      apps: live,
      last: { ok: true, note: null },
    };
    return { world: next, envelope: envelope(next, look(next, [id], by, at) ?? null, wishes, said), line };
  }

  return { boot, list, call, read, apps: () => [...table.values()] };
}
