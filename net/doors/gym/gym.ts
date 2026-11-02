import { createInterface } from "node:readline";
import { HOME, install } from "../../hosts/local/apps.ts";
import type { Act, App, Config, Envelope, Json, Line, Move, Who, World } from "../../kernel/index.ts";
import { Fault, kernel, nearest } from "../../kernel/index.ts";
import { bus } from "../../sdk/system.ts";

/* TYPES */

export type Ask =
  | { op: "boot"; configs: Config[] }
  | { op: "boot"; config: Config; seeds: number[] }
  | { op: "call"; acts: (Act | null)[] }
  | { op: "list" }
  | { op: "read"; path?: string; shape?: Json }
  | { op: "lines" }
  | { op: "close" };

export type Failed = { error: { code: string; note: string } };

export type Reply = (Envelope | Move[] | Line[] | Failed)[] | Failed;

export type Gym = {
  answer: (line: string) => Promise<string>;
  worlds: () => (World | null)[];
  closed: () => boolean;
};

type Raw = Record<string, unknown>;

export const BOT: Who = { id: "gym", kind: "bot" };

export const BEAT = 1000;

const OPS = ["boot", "call", "list", "read", "lines", "close"];

/* ASKS */

const isObj = (value: unknown): value is Raw => !!value && typeof value === "object" && !Array.isArray(value);

const failed = (thrown: unknown): Failed =>
  thrown instanceof Fault
    ? { error: { code: thrown.code, note: thrown.note } }
    : { error: { code: "crash", note: String((thrown as Error)?.message ?? thrown) } };

function configsOf(ask: Raw): unknown[] {
  if (Array.isArray(ask.configs)) return ask.configs;
  if (isObj(ask.config) && Array.isArray(ask.seeds)) return ask.seeds.map((seed) => ({ ...(ask.config as Raw), seed }));
  throw new Fault("bad-act", "boot takes configs: [config, ...], or a config and seeds: [n, ...]");
}

function idsOf(configs: unknown[]): string[] {
  const ids = new Set<string>();
  for (const config of configs)
    if (isObj(config) && Array.isArray(config.apps))
      for (const entry of config.apps) {
        const id = isObj(entry) ? entry.id : entry;
        if (typeof id === "string") ids.add(id);
      }
  return [...ids];
}

function parsed(line: string): Raw {
  let ask: unknown;
  try {
    ask = JSON.parse(line);
  } catch {
    ask = null;
  }
  if (!isObj(ask)) throw new Fault("bad-act", "send one JSON object a line: { op, ... }");
  return ask;
}

/* GYM */

export function gym(home = HOME): Gym {
  const apps = new Map<string, App>();
  const tried = new Set<string>();
  let k = kernel([]);
  let worlds: (World | null)[] = [];
  let lines: Line[][] = [];
  let closed = false;

  async function need(ids: string[]) {
    const missing = ids.filter((id) => !tried.has(id));
    if (!missing.length) return;
    for (const id of missing) tried.add(id);
    for (const one of await install(missing, home)) apps.set(one.id, one);
    k = kernel([...apps.values()]);
  }

  const front = (world: World) => k.read(world, world.route.app ?? "");

  const each = <T>(fn: (world: World, i: number) => T): (T | Failed)[] =>
    worlds.map((world, i) => {
      try {
        if (!world) throw new Fault("unknown-world", `world ${i} did not boot`);
        return fn(world, i);
      } catch (thrown) {
        return failed(thrown);
      }
    });

  async function boot(ask: Raw): Promise<Reply> {
    const configs = configsOf(ask);
    await need(idsOf(configs));
    const booted = configs.map((config): World | Failed => {
      try {
        return k.boot(config as Config);
      } catch (thrown) {
        return failed(thrown);
      }
    });
    worlds = booted.map((one) => ("error" in one ? null : one));
    lines = booted.map(() => []);
    return booted.map((one) => ("error" in one ? one : front(one)));
  }

  function call(acts: unknown): Reply {
    if (!Array.isArray(acts) || acts.length !== worlds.length)
      throw new Fault("bad-act", `call takes acts: one per world, ${worlds.length} of them, null to skip one`);
    return each((world, i) => {
      const act = acts[i];
      if (act === null) return front(world);
      if (!isObj(act)) throw new Fault("bad-act", "an act is { verb, args, who, at }");
      const step = (one: Act) => {
        const done = k.call(worlds[i]!, one);
        worlds[i] = done.world;
        lines[i]!.push(done.line);
        return done;
      };
      return bus(
        [...apps.values()],
        step,
      )({
        ...(act as Act),
        who: (act.who as Who | undefined) ?? BOT,
        at: (act.at as number | undefined) ?? world.at + BEAT,
      }).envelope;
    });
  }

  function read(ask: Raw): Reply {
    const path = ask.path ?? "";
    if (typeof path !== "string") throw new Fault("bad-act", "path is a string: '' every app, 'snake' one app");
    return each((world) => k.read(world, path, ask.shape as Json | undefined));
  }

  async function reply(ask: Raw): Promise<Reply> {
    switch (ask.op) {
      case "boot":
        return boot(ask);
      case "call":
        return call(ask.acts);
      case "list":
        return each((world) => k.list(world).moves);
      case "read":
        return read(ask);
      case "lines":
        return each((_, i) => lines[i]!);
      case "close":
        closed = true;
        worlds = [];
        lines = [];
        return [];
      default:
        throw new Fault("unknown-verb", `the gym has no op ${ask.op}; nearest: ${nearest(String(ask.op), OPS)}`);
    }
  }

  async function answer(line: string): Promise<string> {
    try {
      return JSON.stringify(await reply(parsed(line)));
    } catch (thrown) {
      return JSON.stringify(failed(thrown));
    }
  }

  return { answer, worlds: () => [...worlds], closed: () => closed };
}

/* STDIO */

export async function stdio() {
  const one = gym();
  for await (const line of createInterface({ input: process.stdin, crlfDelay: Number.POSITIVE_INFINITY })) {
    if (!line.trim()) continue;
    process.stdout.write(`${await one.answer(line)}\n`);
    if (one.closed()) break;
  }
  process.stdin.destroy();
}
