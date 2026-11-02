#!/usr/bin/env bun
import { readFileSync, rmSync } from "node:fs";
import { basename } from "node:path";
import { cli, install, site } from "../../hosts/local/apps.ts";
import { simulate } from "../../hosts/local/sim.ts";
import type { Store } from "../../hosts/local/store.ts";
import { boot, configOf, empty, exists, open, place } from "../../hosts/local/store.ts";
import type { Config, Json, Kind, Obj, Who } from "../../kernel/index.ts";
import { Fault, SPEC } from "../../kernel/index.ts";
import { goose } from "../../sim/goose.ts";
import { BEAT } from "../../sim/run.ts";
import { stdio } from "../gym/gym.ts";
import { handler } from "../http/http.ts";
import { serve } from "../mcp/mcp.ts";

/* TYPES */

type Ask = {
  verb: string;
  rest: string[];
  world: string;
  who: Who | null;
  fresh: boolean;
  bots: number;
  rounds: number;
  port: number;
};

const KINDS: Kind[] = ["human", "org", "bot", "agent"];

const NAME = cli(site());

const HELP = `${NAME}: the four calls of MrlyNet, JSON in, JSON out.

  ${NAME} boot [config.json | '{json}'] [--fresh]   start a world; no config boots every app
  ${NAME} list                                      every legal verb now, with its args schema
  ${NAME} call <verb> ['{args}']                    one act; prints the envelope
  ${NAME} read [path] ['{shape}']                   '' is the world, 'snake' one app, 'snake/seed' one leaf
  ${NAME} goose [n]                                 a seeded random player takes n acts (default 1)
  ${NAME} sim [config] [--bots 5] [--rounds 10]       the config's actors (else N bots) act in turn, a second of world time a round; resumes; replays to check
  ${NAME} serve [--port 4100]                          the four calls over HTTP on 127.0.0.1, worlds by name under /api/1/worlds
  ${NAME} mcp                                       serve this world over MCP on stdio; boots it first if new
  ${NAME} gym                                       many in-memory worlds over stdio, one JSON line per batch each way
  ${NAME} apps                                      the apps this site can boot

  --world <name|dir>   the world folder (default: local)
  --as <id>[:kind]     who acts (default local:human; mcp local:agent)`;

/* ARGS */

function whole(text: string | undefined, flag: string): number {
  const n = Number(text);
  if (!Number.isInteger(n) || n < 1) throw new Fault("bad-act", `${flag} takes a whole number above 0`);
  return n;
}

function parse(argv: string[]): Ask {
  const ask: Ask = { verb: "", rest: [], world: "local", who: null, fresh: false, bots: 5, rounds: 10, port: 4100 };
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === "--world") ask.world = argv[++i] ?? "";
    else if (arg === "--as") {
      const [id, kind = "human"] = (argv[++i] ?? "").split(":");
      if (!id || !KINDS.includes(kind as Kind)) throw new Fault("bad-act", `--as takes id[:${KINDS.join("|")}]`);
      ask.who = { id, kind: kind as Kind };
    } else if (arg === "--fresh") ask.fresh = true;
    else if (arg === "--bots") ask.bots = whole(argv[++i], "--bots");
    else if (arg === "--rounds") ask.rounds = whole(argv[++i], "--rounds");
    else if (arg === "--port") ask.port = whole(argv[++i], "--port");
    else if (arg === "--help" || arg === "-h") ask.verb = "help";
    else if (!ask.verb) ask.verb = arg;
    else ask.rest.push(arg);
  }
  return ask;
}

function json(text: string | undefined, what: string): Json | undefined {
  if (text === undefined) return undefined;
  try {
    return JSON.parse(text);
  } catch {
    throw new Fault("bad-act", `${what} must be JSON, not ${text}`);
  }
}

function configFrom(text: string | undefined, ids: string[]): Config {
  if (text === undefined) return { spec: SPEC, name: "local", apps: ids };
  const raw = text.trim().startsWith("{") ? text : readFileSync(text, "utf8");
  return json(raw, "the config") as Config;
}

/* WORLD */

async function opened(ask: Ask): Promise<Store> {
  const dir = place(ask.world);
  if (!exists(dir)) throw new Fault("unknown-world", `no world at ${dir}; run ${NAME} boot first`);
  const config = configOf(dir);
  const ids = config.apps.map((one) => (typeof one === "string" ? one : one.id));
  return open(await install(ids), dir);
}

const stamp = (store: Store) => Math.max(Date.now(), store.world().at);

/* MAIN */

const say = (out: unknown) => process.stdout.write(`${JSON.stringify(out, null, 2)}\n`);

async function main(argv: string[]) {
  const ask = parse(argv);
  const me = (kind: Kind): Who => ask.who ?? { id: "local", kind };
  switch (ask.verb) {
    case "":
    case "help":
      process.stdout.write(`${HELP}\n`);
      return;
    case "apps": {
      const apps = await install();
      say(apps.map((one) => ({ id: one.id, title: one.title, line: one.line })));
      return;
    }
    case "boot": {
      const dir = place(ask.world);
      if (exists(dir) && !empty(dir) && !ask.fresh)
        throw new Fault("bad-config", `${dir} already holds a journal; add --fresh to start over`);
      const all = await install();
      const config = configFrom(
        ask.rest[0],
        all.map((one) => one.id),
      );
      const store = boot(all, dir, config);
      say(store.read(store.world().route.app ?? ""));
      return;
    }
    case "list":
      say((await opened(ask)).list());
      return;
    case "call": {
      const store = await opened(ask);
      const [verb, args] = ask.rest;
      if (!verb) throw new Fault("bad-act", `${NAME} call <verb> ['{args}']`);
      say(store.call({ verb, args: (json(args, "args") ?? {}) as Obj, who: me("human"), at: stamp(store) }));
      return;
    }
    case "read": {
      const store = await opened(ask);
      say(store.read(ask.rest[0] ?? "", json(ask.rest[1], "the shape"), me("human")));
      return;
    }
    case "goose": {
      const store = await opened(ask);
      const n = Number(ask.rest[0] ?? 1);
      if (!Number.isInteger(n) || n < 1) throw new Fault("bad-act", "goose takes a whole number of acts");
      let s = store.world().rand ^ 0x5bd1e995;
      const draw = () => {
        s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
        return s;
      };
      const who = ask.who ?? { id: "goose", kind: "bot" as const };
      const done: { verb: string; args: Obj; ok: boolean; note: string | null }[] = [];
      for (let i = 0; i < n; i++) {
        const act = goose(store.list().moves, draw);
        if (!act) break;
        const envelope = store.call({ ...act, who, at: stamp(store) });
        done.push({ verb: act.verb, args: act.args ?? {}, ...envelope.last });
      }
      say({ tick: store.world().tick, acts: done });
      return;
    }
    case "sim": {
      const dir = place(ask.world);
      if (exists(dir) && ask.fresh) rmSync(dir, { recursive: true, force: true });
      const known = exists(dir) ? configOf(dir) : null;
      const all = await install(known?.apps.map((one) => (typeof one === "string" ? one : one.id)));
      const config = known ?? {
        ...configFrom(
          ask.rest[0],
          all.map((one) => one.id),
        ),
        name: basename(dir),
      };
      say(simulate(all, dir, config, { bots: ask.bots, rounds: ask.rounds, beat: BEAT }));
      return;
    }
    case "serve": {
      const kept = new Map<string, Store>();
      const wrap = (store: Store) => ({ list: store.list, call: store.call, read: store.read, at: () => stamp(store) });
      const server = Bun.serve({
        hostname: "127.0.0.1",
        port: ask.port,
        fetch: handler(
          {
            boot: async (config) => {
              const dir = place(String(config?.name ?? ""));
              if (exists(dir) && !empty(dir))
                throw new Fault("bad-config", `a world named ${config.name} already exists`);
              const ids = Array.isArray(config?.apps)
                ? config.apps.map((one) => (typeof one === "string" ? one : one.id))
                : [];
              const store = boot(await install(ids), dir, config);
              kept.set(config.name, store);
              return { id: config.name, world: wrap(store) };
            },
            open: async (id) => {
              const dir = place(id);
              if (!/^[a-z0-9][a-z0-9-]*$/.test(id) || !exists(dir)) return null;
              if (!kept.has(id)) kept.set(id, await opened({ ...ask, world: id }));
              return wrap(kept.get(id)!);
            },
          },
          me("agent"),
        ),
      });
      process.stderr.write(`${NAME}: the four calls on http://127.0.0.1:${server.port}/api/1/worlds\n`);
      return;
    }
    case "mcp": {
      const dir = place(ask.world);
      if (!exists(dir)) {
        const all = await install();
        boot(
          all,
          dir,
          configFrom(
            undefined,
            all.map((one) => one.id),
          ),
        );
      }
      const store = await opened(ask);
      const version =
        JSON.parse(readFileSync(new URL("../../../package.json", import.meta.url), "utf8")).version ?? SPEC;
      await serve(
        { list: store.list, call: store.call, read: store.read, at: () => stamp(store) },
        {
          name: NAME,
          version,
          who: me("agent"),
          note: `${NAME} is MrlyNet's world: every app is a pure reducer over one JSON world. Each tool is a legal verb now; the list shifts as the world does. Use read to look before you act. A refused act is an outcome, never an error: its note names the nearest legal verb.`,
        },
      );
      return;
    }
    case "gym":
      await stdio();
      return;
    default:
      throw new Fault("unknown-verb", `${NAME} has no ${ask.verb}; try ${NAME} help`);
  }
}

if (import.meta.main)
  await main(process.argv.slice(2)).catch((thrown) => {
    const error =
      thrown instanceof Fault
        ? { code: thrown.code, note: thrown.note }
        : { code: "crash", note: String((thrown as Error)?.message ?? thrown) };
    process.stderr.write(`${JSON.stringify({ error })}\n`);
    process.exit(1);
  });
