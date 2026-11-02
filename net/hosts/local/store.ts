import { appendFileSync, existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import type {
  Act,
  App,
  Code,
  Config,
  Envelope,
  Json,
  Kernel,
  Line,
  Manifest,
  Move,
  Obj,
  Who,
  World,
} from "../../kernel/index.ts";
import { canon, kernel, parse } from "../../kernel/index.ts";
import { bus } from "../../sdk/system.ts";
import { REPO } from "./apps.ts";

/* TYPES */

export type Store = {
  dir: string;
  world: () => World;
  list: () => { moves: Move[]; apps: Manifest[] };
  call: (act: Act) => Envelope;
  read: (path?: string, shape?: Json, who?: Who | null) => Envelope;
};

export type Faulted = { tick: number; who: Who | null; verb: string; args: Obj; fault: Code; note: string };

type Snapshot = { tick: number; world: World };

const HERE = import.meta.dir;

export const DATA_DIR = join(REPO, "data", relative(REPO, HERE));

const EVERY = 500;

/* FILES */

export const place = (name: string) => (name.includes("/") ? name : join(DATA_DIR, "worlds", name));

const files = (dir: string) => ({
  config: join(dir, "config.json"),
  journal: join(dir, "journal.jsonl"),
  snapshot: join(dir, "snapshot.json"),
  faults: join(dir, "faults.jsonl"),
});

export const exists = (dir: string) => existsSync(files(dir).config);

export const configOf = (dir: string): Config => JSON.parse(readFileSync(files(dir).config, "utf8"));

/* FAULTS */

export const fault = (dir: string, one: Faulted) => appendFileSync(files(dir).faults, `${canon(one)}\n`);

export const faults = (dir: string): Faulted[] =>
  existsSync(files(dir).faults)
    ? readFileSync(files(dir).faults, "utf8")
        .split("\n")
        .filter(Boolean)
        .map((line) => JSON.parse(line) as Faulted)
    : [];

/* OPEN */

function load(k: Kernel, dir: string): World {
  const f = files(dir);
  const config = configOf(dir);
  const lines: Line[] = existsSync(f.journal) ? parse(readFileSync(f.journal, "utf8")) : [];
  const snap: Snapshot | null = existsSync(f.snapshot) ? JSON.parse(readFileSync(f.snapshot, "utf8")) : null;
  let world = snap && snap.tick <= lines.length ? snap.world : k.boot(config);
  for (const line of lines) {
    if (line.tick <= world.tick) continue;
    if (line.tick !== world.tick + 1) throw new Error(`store: ${f.journal} jumps from ${world.tick} to ${line.tick}`);
    world = k.call(world, { verb: line.verb, args: line.args, who: line.who, at: line.at }).world;
  }
  return world;
}

function snapshot(dir: string, world: World) {
  const f = files(dir);
  writeFileSync(`${f.snapshot}.next`, canon({ tick: world.tick, world }));
  renameSync(`${f.snapshot}.next`, f.snapshot);
}

export function open(apps: readonly App[], dir: string): Store {
  const k = kernel(apps);
  let world = load(k, dir);
  const step = (act: Act) => {
    const done = k.call(world, act);
    appendFileSync(files(dir).journal, `${canon(done.line)}\n`);
    world = done.world;
    if (world.tick % EVERY === 0) snapshot(dir, world);
    return done;
  };
  const drive = bus(apps, step);
  return {
    dir,
    world: () => world,
    list: () => k.list(world),
    read: (path = "", shape, who = null) => k.read(world, path, shape, who),
    call: (act) => drive(act).envelope,
  };
}

export function boot(apps: readonly App[], dir: string, config: Config): Store {
  const k = kernel(apps);
  k.boot(config);
  const f = files(dir);
  mkdirSync(dir, { recursive: true });
  writeFileSync(f.config, `${JSON.stringify(config, null, 2)}\n`);
  writeFileSync(f.journal, "");
  writeFileSync(f.faults, "");
  rmSync(f.snapshot, { force: true });
  return open(apps, dir);
}

export const empty = (dir: string) =>
  !existsSync(files(dir).journal) || !readFileSync(files(dir).journal, "utf8").trim();
