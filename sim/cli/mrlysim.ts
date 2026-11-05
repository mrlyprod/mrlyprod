#!/usr/bin/env bun
import { readFileSync, rmSync } from "node:fs";
import { basename, join, relative, resolve } from "node:path";
import { cli, install, site } from "@mrlyprod/os/hosts/local/apps.ts";
import type { Store } from "@mrlyprod/os/hosts/local/store.ts";
import { configOf, exists, open } from "@mrlyprod/os/hosts/local/store.ts";
import type { Config, Obj, Who } from "@mrlyprod/os/kernel/index.ts";
import { Fault, SPEC } from "@mrlyprod/os/kernel/index.ts";
import { goose } from "../goose.ts";
import { simulate } from "../report.ts";
import { BEAT } from "../run.ts";
import { generate } from "../worlds.ts";

const HERE = import.meta.dir;
const SIM = resolve(HERE, "..");
const REPO = resolve(SIM, "..");
const DATA_DIR = join(REPO, "data", relative(REPO, HERE));
const DOOR = Bun.resolveSync("@mrlyprod/os/doors/cli/cli.ts", HERE);
const NAME = cli(site(SIM));
const OWN = ["generate", "goose", "sim"];

const HELP = `${NAME}: simulated worlds over the engine. Scripted players, no API money.

  ${NAME} generate [seed]                        a world config drawn from the knobs by a seed
  ${NAME} sim [config] [--bots 5] [--rounds 10]  the config's actors (else N bots) act in turn, a second of world time a round; resumes; checks replay against the live world
  ${NAME} goose [n]                              a seeded random player takes n acts (default 1)
  ${NAME} <other>                                any engine verb on Sim's apps: boot, list, call, read, mcp, apps, knobs

  --world <name|dir>   the world folder (default: local, under ${DATA_DIR})
  --as <id>[:kind]     who acts`;

type Ask = { verb: string; rest: string[]; dir: string; who: Who | null; fresh: boolean; bots: number; rounds: number };

const place = (name: string) => (name.includes("/") ? resolve(name) : join(DATA_DIR, "worlds", name));

function whole(text: string | undefined, flag: string): number {
  const n = Number(text);
  if (!Number.isInteger(n) || n < 1) throw new Fault("bad-act", `${flag} takes a whole number above 0`);
  return n;
}

function parse(argv: string[]): Ask {
  const ask: Ask = { verb: "", rest: [], dir: place("local"), who: null, fresh: false, bots: 5, rounds: 10 };
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i]!;
    if (arg === "--world") ask.dir = place(argv[++i] ?? "");
    else if (arg === "--as") {
      const [id = "", kind = "bot"] = (argv[++i] ?? "").split(":");
      ask.who = { id, kind: kind as Who["kind"] };
    } else if (arg === "--fresh") ask.fresh = true;
    else if (arg === "--bots") ask.bots = whole(argv[++i], "--bots");
    else if (arg === "--rounds") ask.rounds = whole(argv[++i], "--rounds");
    else if (!ask.verb) ask.verb = arg;
    else ask.rest.push(arg);
  }
  return ask;
}

function configFrom(text: string | undefined, ids: string[]): Config {
  if (text === undefined) return { spec: SPEC, name: "local", apps: ids };
  const raw = text.trim().startsWith("{") ? text : readFileSync(text, "utf8");
  return JSON.parse(raw) as Config;
}

const idsOf = (config: Config) => config.apps.map((one) => (typeof one === "string" ? one : one.id));

const say = (out: unknown) => process.stdout.write(`${JSON.stringify(out, null, 2)}\n`);

async function forward(argv: string[], ask: Ask) {
  const has = argv.includes("--world");
  const args = has ? argv.map((one, i) => (argv[i - 1] === "--world" ? ask.dir : one)) : ["--world", ask.dir, ...argv];
  const child = Bun.spawn(["bun", DOOR, ...args], {
    env: { ...process.env, MRLY_ROOT: SIM },
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  });
  process.exit(await child.exited);
}

function geese(ask: Ask, store: Store) {
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
    const { at, rules } = store.world();
    const pace = rules["world.pace"];
    const stamp = typeof pace === "number" ? at + pace : Math.max(Date.now(), at);
    const envelope = store.call({ ...act, who, at: stamp });
    done.push({ verb: act.verb, args: (act.args ?? {}) as Obj, ...envelope.last });
  }
  return { tick: store.world().tick, acts: done };
}

async function main(argv: string[]) {
  const ask = parse(argv);
  if (!ask.verb || ask.verb === "help") return void process.stdout.write(`${HELP}\n`);
  if (!OWN.includes(ask.verb)) return forward(argv, ask);
  if (ask.verb === "generate") return say(generate(Number(ask.rest[0] ?? 0), await install(undefined, SIM)));
  const { dir } = ask;
  if (ask.verb === "goose") {
    if (!exists(dir)) throw new Fault("unknown-world", `no world at ${dir}; run ${NAME} boot first`);
    const store = open(await install(idsOf(configOf(dir)), SIM), dir);
    return say(geese(ask, store));
  }
  if (exists(dir) && ask.fresh) rmSync(dir, { recursive: true, force: true });
  const known = exists(dir) ? configOf(dir) : null;
  const all = await install(known ? idsOf(known) : undefined, SIM);
  const config = known ?? {
    ...configFrom(
      ask.rest[0],
      all.map((one) => one.id),
    ),
    name: basename(dir),
  };
  say(simulate(all, dir, config, { bots: ask.bots, rounds: ask.rounds, beat: BEAT }));
}

if (import.meta.main)
  await main(process.argv.slice(2)).catch((thrown: unknown) => {
    const error =
      thrown instanceof Fault
        ? { code: thrown.code, note: thrown.note }
        : { code: "crash", note: String((thrown as Error)?.message ?? thrown) };
    process.stderr.write(`${JSON.stringify({ error })}\n`);
    process.exit(1);
  });
