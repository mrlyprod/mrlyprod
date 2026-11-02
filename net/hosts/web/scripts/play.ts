import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { createInterface } from "node:readline";
import type { Rng } from "../../../sdk/rand.ts";
import { rng } from "../../../sdk/rand.ts";
import type { Raw, Spec, Value } from "../../../sdk/spec.ts";
import { tidy } from "../../../sdk/spec.ts";
import { REPO, SITE } from "./home.ts";

/* TYPES */

export type Engine = {
  boot: (config: Raw) => unknown;
  moves: (state: unknown) => unknown[];
  step: (state: unknown, act: unknown) => unknown;
  read: (state: unknown, path?: string) => unknown;
};

type Scene = { SPEC?: Spec; PAGE?: { keys?: readonly unknown[] }; tidied?: (value: Raw, units?: Doors) => Value };

type Policy = { pick: (state: unknown, mode: string, rand: Rng) => unknown };

type Mod = Record<string, unknown>;

export type Doors = Record<string, Mod>;

export type App = { id: string; engine: Engine; scene: Scene; policy: Policy | null; bots: string[] };

export type Session = {
  app: App;
  units: Doors;
  seed: number;
  kv: Raw;
  value: Value;
  engine: Engine;
  state: unknown;
  rand: Rng;
};

type Booted = { engine?: Engine; state: unknown };

type Table = typeof STAGE & { seed: number; rand: Rng; units: Doors };

type Dealer = { deal?: (value: Value, table: Table) => unknown; reducer?: (value: Value) => Engine };

type Way = {
  bot?: (session: Session, name: string) => unknown[] | null;
};

export type Answer = Record<string, unknown>;

/* CONSTANTS */

const APPS = join(REPO, "apps");
const CRATES = join(REPO, "crates");
const WIDE = 4294967296;
const STAGE = { w: 1280, h: 800, dpr: 1, lock: 0 };
const BUILT = ["first", "random"];
const MOST = 100000;
const VERBS = ["boot", "moves", "step", "read"];
const USAGE = "usage: bun run play [<app> [--seed N] [key=value ...] [--bot <name>[,<name>] --until-done [--max N]]]";

/* UNITS */

let doors: Promise<Doors> | null = null;

export const units = () =>
  (doors ??= (async () => {
    const out: Doors = {};
    for (const unit of readdirSync(CRATES)) {
      const wasm = join(CRATES, unit, "js", "pkg", `mrly_${unit}_js_bg.wasm`);
      if (!existsSync(join(CRATES, unit, "js", "package.json")) || !existsSync(wasm)) continue;
      const door = (await import(`@mrlyprod/${unit}`)) as Mod & { initSync: (o: { module: Buffer }) => unknown };
      door.initSync({ module: readFileSync(wasm) });
      out[unit] = door;
    }
    return out;
  })());

/* DISCOVERY */

const exported = (text: string, name: string) => new RegExp(`^export (function|const) ${name}\\b`, "m").test(text);

const fns = (mod: Mod) => VERBS.every((name) => typeof mod[name] === "function");

export async function reducers(): Promise<string[]> {
  const out: string[] = [];
  for (const id of readdirSync(APPS).sort()) {
    const file = join(APPS, id, "engine.ts");
    if (!existsSync(file) || !VERBS.every((name) => exported(readFileSync(file, "utf8"), name))) continue;
    if (fns(await import(file))) out.push(id);
  }
  return out;
}

const optional = async (file: string): Promise<Mod | null> => (existsSync(file) ? await import(file) : null);

function players(spec: Spec): string[] {
  const out = new Set<string>();
  for (const row of spec) {
    if (!Array.isArray(row.options)) continue;
    const names = row.options.map((one) => String(Array.isArray(one) ? one[0] : one));
    if (names.includes("me")) for (const name of names) if (name !== "me") out.add(name);
  }
  return [...out];
}

export async function load(id: string): Promise<App> {
  const engine = await optional(join(APPS, id, "engine.ts"));
  if (!engine || !fns(engine)) throw new Error(`play: ${id} has no reducer; try ${(await reducers()).join(", ")}`);
  const scene = ((await optional(join(APPS, id, "scene.ts"))) ?? {}) as Scene;
  const policy = (await optional(join(APPS, id, "policy.ts"))) as Policy | null;
  const own = policy && typeof policy.pick === "function" ? players(scene.SPEC ?? []) : [];
  return { id, engine: engine as unknown as Engine, scene, policy, bots: [...new Set([...own, ...BUILT])] };
}

export async function verbs(app: App, units: Doors) {
  const { list } = await import("../../../sdk/verbs.ts");
  const spec = app.scene.SPEC ?? [];
  const keys = (app.scene.PAGE?.keys ?? []) as never[];
  return list({ id: app.id, spec, keys, value: valued(app, {}, units), units });
}

/* PAGE */

const salt = (rand: Rng) => Math.floor(rand() * WIDE);

const picked = (value: Value, keys: string[]) => Object.fromEntries(keys.map((key) => [key, value[key]]));

function valued(app: App, kv: Raw, units: Doors): Value {
  if (app.scene.tidied) return app.scene.tidied(kv, units);
  if (!app.scene.SPEC) return kv as Value;
  return tidy(app.scene.SPEC, kv, units);
}

const known = (app: App, units: Doors) => (app.scene.SPEC ? Object.keys(valued(app, {}, units)) : null);

const plain = (app: App, value: Value, units: Doors): Value => {
  const keys = known(app, units);
  return keys ? picked(value, keys) : value;
};

const WAYS: Record<string, Way> = {
  snake: {
    bot: (session, name) => {
      const state = session.state as { queue: number[] };
      const e = session.engine as unknown as Engine & {
        toward: (dir: number) => unknown;
        STARVE: unknown;
        TICK: unknown;
      };
      const dir = Number(session.app.policy!.pick(state, name, session.rand));
      return [...(state.queue.length ? [] : [dir < 0 ? e.STARVE : e.toward(dir)]), e.TICK];
    },
  },
};

async function dealt(app: App, value: Value, seed: number, units: Doors, rand: Rng): Promise<Booted> {
  const { deal, reducer } = app.engine as unknown as Dealer;
  if (!deal) return { state: app.engine.boot({ ...plain(app, value, units), seed: salt(rand) }) };
  return { engine: reducer?.(value), state: deal(value, { ...STAGE, seed, rand, units }) };
}

/* SESSION */

export async function open(id: string, seed = 0, kv: Raw = {}): Promise<Session> {
  const app = await load(id);
  const doors = await units();
  return boot({ app, units: doors, seed, kv, value: {}, engine: app.engine, state: null, rand: Math.random });
}

async function boot(session: Session): Promise<Session> {
  const { app, units, seed } = session;
  const keys = known(app, units);
  const odd = keys ? Object.keys(session.kv).filter((key) => !keys.includes(key)) : [];
  if (odd.length) throw new Error(`play: ${app.id} takes no ${odd.join(", ")}; it takes ${keys!.join(", ")}`);
  if (!Number.isInteger(seed) || seed < 0 || seed >= WIDE) throw new Error(`play: seed ${seed} is not 0..${WIDE - 1}`);
  const value = valued(app, session.kv, units);
  const rand = rng(seed);
  const booted = await dealt(app, value, seed, units, rand);
  const engine = booted.engine ?? app.engine;
  return { ...session, value: { ...plain(app, value, units), seed }, engine, state: booted.state, rand };
}

const shown = (session: Session): Answer => ({
  app: session.app.id,
  seed: session.seed,
  value: session.value,
  read: session.engine.read(session.state),
});

function same(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (!a || !b || typeof a !== "object" || typeof b !== "object") return false;
  if (Array.isArray(a) !== Array.isArray(b)) return false;
  const ka = Object.keys(a);
  const kb = Object.keys(b);
  return ka.length === kb.length && ka.every((k) => same((a as Raw)[k], (b as Raw)[k]));
}

function built(session: Session, name: string): unknown[] | null {
  const all = session.engine.moves(session.state);
  if (!all.length) return null;
  return [name === "first" ? all[0] : all[Math.floor(session.rand() * all.length)]];
}

function chosen(session: Session, names: string[], played: number): string {
  const turn = (session.state as { turn?: unknown } | null)?.turn;
  return typeof turn === "number" && turn >= 1 ? names[(turn - 1) % names.length] : names[played % names.length];
}

function acts(session: Session, name: string): unknown[] | null {
  if (BUILT.includes(name) || !session.app.policy) return built(session, name);
  const way = WAYS[session.app.id]?.bot;
  if (way) return way(session, name);
  const act = session.app.policy.pick(session.state, name, session.rand);
  return act && typeof act === "object" ? [act] : null;
}

export function bots(session: Session, bot: string): string[] {
  const names = bot.split(",").map((one) => one.trim());
  const bad = names.filter((name) => !session.app.bots.includes(name));
  if (!names.length || bad.length)
    throw new Error(`play: ${session.app.id} has no bot ${bad.join(", ")}; it has ${session.app.bots.join(", ")}`);
  return names;
}

export function play(session: Session, bot: string, n: number): { session: Session; steps: number } {
  const names = bots(session, bot);
  let state = session.state;
  let steps = 0;
  while (steps < n) {
    const now = { ...session, state };
    if (!session.engine.moves(state).length) break;
    const todo = acts(now, chosen(now, names, steps));
    if (!todo) break;
    for (const act of todo) state = session.engine.step(state, act);
    steps++;
  }
  return { session: { ...session, state }, steps };
}

const count = (n: unknown, fallback: number) => {
  const k = n === undefined ? fallback : Number(n);
  if (!Number.isInteger(k) || k < 0) throw new Error(`play: n must be a whole number, not ${JSON.stringify(n)}`);
  return k;
};

function sense(line: string): unknown {
  try {
    return JSON.parse(line);
  } catch {
    throw new Error(`play: ${line.trim()} is not JSON`);
  }
}

export async function answer(session: Session, line: string): Promise<{ session: Session; out: Answer }> {
  try {
    const ask = sense(line);
    if (!ask || typeof ask !== "object" || Array.isArray(ask)) throw new Error("play: send one JSON object a line");
    const one = ask as Raw;
    if (one.moves) return { session, out: { moves: session.engine.moves(session.state) } };
    if (one.read) return { session, out: { read: session.engine.read(session.state) } };
    if ("step" in one) {
      const legal = session.engine.moves(session.state);
      if (!legal.some((act) => same(act, one.step)))
        throw new Error(`play: ${JSON.stringify(one.step)} is no legal move; ask {"moves":true}`);
      const next = { ...session, state: session.engine.step(session.state, one.step) };
      return { session: next, out: { read: next.engine.read(next.state) } };
    }
    if (typeof one.bot === "string") {
      const done = play(session, one.bot, count(one.n, 1));
      return { session: done.session, out: { steps: done.steps, read: done.session.engine.read(done.session.state) } };
    }
    if ("seed" in one) {
      const next = await boot({ ...session, seed: Number(one.seed) });
      return { session: next, out: shown(next) };
    }
    if (one.set && typeof one.set === "object") {
      const next = await boot({ ...session, kv: { ...session.kv, ...(one.set as Raw) } });
      return { session: next, out: shown(next) };
    }
    throw new Error(`play: no ask in ${line.trim()}; send moves, step, bot, read, seed or set`);
  } catch (thrown) {
    return { session, out: { error: String((thrown as Error)?.message ?? thrown) } };
  }
}

/* MAIN */

const json = (out: unknown) =>
  JSON.stringify(out, (_key, v) =>
    ArrayBuffer.isView(v) && !(v instanceof DataView)
      ? Array.from(v as unknown as ArrayLike<number>)
      : typeof v === "bigint"
        ? String(v)
        : v,
  );

const say = (out: unknown) => process.stdout.write(`${json(out)}\n`);

type Ask = { id: string; seed: number; kv: Raw; bot: string | null; until: boolean; max: number };

export function parse(argv: string[]): Ask {
  const ask: Ask = { id: "", seed: 0, kv: {}, bot: null, until: false, max: MOST };
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === "--until-done") ask.until = true;
    else if (arg === "--seed") ask.seed = Number(argv[++i]);
    else if (arg === "--max") ask.max = Number(argv[++i]);
    else if (arg === "--bot") ask.bot = argv[++i] ?? "";
    else if (arg.startsWith("--")) throw new Error(USAGE);
    else if (arg.includes("=")) {
      const [key, ...rest] = arg.split("=");
      if (key === "seed") ask.seed = Number(rest.join("="));
      else ask.kv[key] = rest.join("=");
    } else if (!ask.id) ask.id = arg;
    else throw new Error(USAGE);
  }
  if (ask.until !== (ask.bot !== null)) throw new Error(USAGE);
  if (!Number.isInteger(ask.max) || ask.max < 0) throw new Error(USAGE);
  return ask;
}

async function main(argv: string[]) {
  Bun.plugin({
    name: "play",
    setup: (build) => {
      build.module("site:routes", () => ({
        contents: `export const site = ${JSON.stringify(SITE)};\nexport const rows = [];`,
        loader: "js",
      }));
    },
  });
  const ask = parse(argv);
  if (!ask.id) {
    const doors = await units();
    for (const id of await reducers()) {
      const app = await load(id);
      say({ app: id, bots: app.bots, verbs: await verbs(app, doors) });
    }
    return;
  }
  let session = await open(ask.id, ask.seed, ask.kv);
  if (ask.bot !== null) {
    const done = play(session, ask.bot, ask.max);
    say({ ...shown(done.session), bot: ask.bot, steps: done.steps });
    return;
  }
  say(shown(session));
  for await (const line of createInterface({ input: process.stdin, crlfDelay: Number.POSITIVE_INFINITY })) {
    if (!line.trim()) continue;
    const next = await answer(session, line);
    session = next.session;
    say(next.out);
  }
}

if (import.meta.main)
  await main(process.argv.slice(2)).catch((thrown) => {
    say({ error: String((thrown as Error)?.message ?? thrown) });
    process.exit(1);
  });
