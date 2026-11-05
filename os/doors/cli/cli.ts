#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { knobsOf } from "../../contract/index.ts";
import { cli, handlers, install, site } from "../../hosts/local/apps.ts";
import type { Store } from "../../hosts/local/store.ts";
import { boot, configOf, empty, exists, open, place } from "../../hosts/local/store.ts";
import type { Config, Json, Obj, Who } from "../../kernel/index.ts";
import { Fault, SPEC } from "../../kernel/index.ts";
import { stdio } from "../gym/gym.ts";
import { handler } from "../http/http.ts";
import { serve } from "../mcp/mcp.ts";
import { nowhere, tui } from "../tui/tui.ts";

type Ask = {
  verb: string;
  rest: string[];
  world: string;
  who: Who | null;
  fresh: boolean;
  port: number;
};

function here() {
  try {
    return site();
  } catch (thrown) {
    process.stderr.write(`${JSON.stringify({ error: { code: "bad-config", note: (thrown as Error).message } })}\n`);
    process.exit(1);
  }
}

const SITE = here();

const NAME = cli(SITE);

const TITLE = SITE.title;

const HELP = `${NAME}: the four calls of ${TITLE}, JSON in, JSON out.

  ${NAME} boot [config.json | '{json}'] [--fresh]   start a world; no config boots every app
  ${NAME} list                                      every legal verb now, with its args schema
  ${NAME} call <verb> ['{args}']                    one act; prints the envelope
  ${NAME} read [path] ['{shape}']                   '' is the world, 'snake' one app, 'snake/seed' one leaf
  ${NAME} serve [--port 4100]                          the four calls over HTTP on 127.0.0.1, worlds by name under /api/1/worlds
  ${NAME} mcp                                       serve this world over MCP on stdio; boots it first if new
  ${NAME} tui                                       a prompt over this world, or one command a line from a pipe; boots it first if new
  ${NAME} gym                                       many in-memory worlds over stdio, one JSON line per batch each way
  ${NAME} apps                                      the apps this site can boot
  ${NAME} knobs                                     every rule of a world that can be turned, with reality's value

  --world <name|dir>   the world folder (default: local)
  --as <id>[:kind]     who acts (default local:human; mcp local:agent)`;

function whole(text: string | undefined, flag: string): number {
  const n = Number(text);
  if (!Number.isInteger(n) || n < 1) throw new Fault("bad-act", `${flag} takes a whole number above 0`);
  return n;
}

function parse(argv: string[]): Ask {
  const ask: Ask = { verb: "", rest: [], world: "local", who: null, fresh: false, port: 4100 };
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === "--world") ask.world = argv[++i] ?? "";
    else if (arg === "--as") {
      const [id, kind = "human"] = (argv[++i] ?? "").split(":");
      if (!id || !kind) throw new Fault("bad-act", "--as takes id[:kind]");
      ask.who = { id, kind };
    } else if (arg === "--fresh") ask.fresh = true;
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

async function opened(ask: Ask): Promise<Store> {
  const dir = place(ask.world);
  if (!exists(dir)) throw new Fault("unknown-world", `no world at ${dir}; run ${NAME} boot first`);
  const config = configOf(dir);
  const ids = config.apps.map((one) => (typeof one === "string" ? one : one.id));
  return open(await install(ids), dir, await handlers());
}

const stamp = (store: Store) => {
  const { at } = store.world();
  const { pace } = store.config();
  return pace ? at + pace : Math.max(Date.now(), at);
};

const door = (store: Store) => ({ list: store.list, call: store.call, read: store.read, at: () => stamp(store) });

const say = (out: unknown) => process.stdout.write(`${JSON.stringify(out, null, 2)}\n`);

async function main(argv: string[]) {
  const ask = parse(argv);
  const me = (kind: string): Who => ask.who ?? { id: "local", kind };
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
    case "knobs":
      say(knobsOf(await install()));
      return;
    case "boot": {
      const dir = place(ask.world);
      if (exists(dir) && !empty(dir) && !ask.fresh)
        throw new Fault("bad-config", `${dir} already holds a journal; add --fresh to start over`);
      const all = await install();
      const config = configFrom(
        ask.rest[0],
        all.map((one) => one.id),
      );
      const world = boot(all, dir, config).world();
      say({ name: world.name, tick: world.tick, at: world.at, apps: Object.keys(world.apps) });
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
    case "serve": {
      const kept = new Map<string, Store>();
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
              return { id: config.name, world: door(store) };
            },
            open: async (id) => {
              const dir = place(id);
              if (!/^[a-z0-9][a-z0-9-]*$/.test(id) || !exists(dir)) return null;
              if (!kept.has(id)) kept.set(id, await opened({ ...ask, world: id }));
              return door(kept.get(id)!);
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
      const version = JSON.parse(readFileSync(new URL("../../package.json", import.meta.url), "utf8")).version ?? SPEC;
      await serve(door(store), {
        name: NAME,
        version,
        who: me("agent"),
        front: () => store.config().front ?? [],
        note: `${NAME} is ${TITLE}'s world, and you are ${me("agent").id}. Each tool is a legal verb now; the list shifts as the world does. read with no path is your front page: who you are, the time, what is new, every app. read shows each app as you may see it. A refused act is an outcome, never an error: its note says why.`,
      });
      return;
    }
    case "tui": {
      const shown = { name: NAME, title: TITLE, who: me("human") };
      const dir = place(ask.world);
      const all = exists(dir) ? null : await install();
      if (all && !all.length) {
        await tui(nowhere(), shown);
        return;
      }
      if (all)
        boot(
          all,
          dir,
          configFrom(
            undefined,
            all.map((one) => one.id),
          ),
        );
      const store = await opened(ask);
      await tui(door(store), {
        ...shown,
        front: () => store.config().front ?? [],
      });
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
