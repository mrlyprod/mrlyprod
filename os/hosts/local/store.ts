import {
  appendFileSync,
  closeSync,
  existsSync,
  mkdirSync,
  openSync,
  readFileSync,
  readSync,
  renameSync,
  rmSync,
  statSync,
  truncateSync,
  writeFileSync,
} from "node:fs";
import { join } from "node:path";
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
import { canon, Fault, kernel, parse } from "../../kernel/index.ts";
import type { Answer, Asked, Handlers, Out } from "../bus.ts";
import { answered, bus, HOST } from "../bus.ts";
import { HOME } from "./apps.ts";

export type Store = {
  dir: string;
  config: () => Config;
  world: () => World;
  list: () => { moves: Move[]; apps: Manifest[] };
  call: (act: Act) => Envelope;
  read: (path?: string, shape?: Json, who?: Who | null) => Envelope;
};

export type Faulted = { tick: number; who: Who | null; verb: string; args: Obj; fault: Code | "crash"; note: string };

type Snapshot = { tick: number; world: World };

export const DATA_DIR = join(HOME, "data");

const EVERY = 500;

const STALE = 10_000;

const WAIT = 2;

const NEWLINE = 10;

export const place = (name: string) => (name.includes("/") ? name : join(DATA_DIR, "worlds", name));

const files = (dir: string) => ({
  config: join(dir, "config.json"),
  journal: join(dir, "journal.jsonl"),
  snapshot: join(dir, "snapshot.json"),
  faults: join(dir, "faults.jsonl"),
  outbox: join(dir, "outbox.jsonl"),
  lock: join(dir, "lock"),
});

const bytesOf = (path: string) => (existsSync(path) ? statSync(path).size : 0);

const wholeBytes = (path: string) => (existsSync(path) ? readFileSync(path).lastIndexOf(NEWLINE) + 1 : 0);

const whole = (path: string): Line[] =>
  existsSync(path) ? parse(readFileSync(path).subarray(0, wholeBytes(path)).toString("utf8")) : [];

function tail(path: string, from: number) {
  const fd = openSync(path, "r");
  try {
    const out = Buffer.alloc(bytesOf(path) - from);
    readSync(fd, out, 0, out.length, from);
    return out;
  } finally {
    closeSync(fd);
  }
}

const alive = (pid: number) => {
  try {
    process.kill(pid, 0);
    return true;
  } catch (thrown) {
    return (thrown as NodeJS.ErrnoException).code === "EPERM";
  }
};

function reclaim(lock: string, owner: string) {
  try {
    const pid = existsSync(owner) ? Number(readFileSync(owner, "utf8")) : 0;
    if (pid ? alive(pid) : Date.now() - statSync(lock).mtimeMs <= STALE) return;
    const gone = `${lock}.${process.pid}.${Date.now()}`;
    renameSync(lock, gone);
    rmSync(gone, { recursive: true, force: true });
  } catch {}
}

function locked<T>(dir: string, run: () => T): T {
  const { lock } = files(dir);
  const owner = join(lock, "pid");
  for (;;) {
    try {
      mkdirSync(lock);
      writeFileSync(owner, String(process.pid));
      break;
    } catch (thrown) {
      if ((thrown as NodeJS.ErrnoException).code !== "EEXIST") throw thrown;
      reclaim(lock, owner);
      Bun.sleepSync(WAIT);
    }
  }
  try {
    return run();
  } finally {
    rmSync(lock, { recursive: true, force: true });
  }
}

export const exists = (dir: string) => existsSync(files(dir).config);

export const configOf = (dir: string): Config => JSON.parse(readFileSync(files(dir).config, "utf8"));

export const fault = (dir: string, one: Faulted) => appendFileSync(files(dir).faults, `${canon(one)}\n`);

export const faults = (dir: string): Faulted[] =>
  existsSync(files(dir).faults)
    ? readFileSync(files(dir).faults, "utf8")
        .split("\n")
        .filter(Boolean)
        .map((line) => JSON.parse(line) as Faulted)
    : [];

function applied(k: Kernel, dir: string, from: World, lines: Line[]): World {
  let world = from;
  for (const line of lines) {
    if (line.tick <= world.tick) continue;
    if (line.tick !== world.tick + 1)
      throw new Error(`store: ${files(dir).journal} jumps from ${world.tick} to ${line.tick}`);
    world = k.call(world, line).world;
  }
  return world;
}

function load(k: Kernel, dir: string): World {
  const f = files(dir);
  const lines: Line[] = parse(tail(f.journal, 0).subarray(0, wholeBytes(f.journal)).toString("utf8"));
  const snap: Snapshot | null = existsSync(f.snapshot) ? JSON.parse(readFileSync(f.snapshot, "utf8")) : null;
  return applied(k, dir, snap && snap.tick <= lines.length ? snap.world : k.boot(configOf(dir)), lines);
}

function snapshot(dir: string, world: World) {
  const f = files(dir);
  writeFileSync(`${f.snapshot}.next`, canon({ tick: world.tick, world }));
  renameSync(`${f.snapshot}.next`, f.snapshot);
}

export function open(apps: readonly App[], dir: string, handlers: Handlers = {}): Store {
  const k = kernel(apps);
  const { journal, outbox } = files(dir);
  let bytes = wholeBytes(journal);
  let world = load(k, dir);
  const fresh = () => {
    if (bytesOf(journal) === bytes) return;
    const added = tail(journal, bytes);
    const whole = added.subarray(0, added.lastIndexOf(NEWLINE) + 1);
    world = applied(k, dir, world, parse(whole.toString("utf8")));
    bytes += whole.length;
  };
  const logged = (thrown: unknown, act: Act) =>
    fault(dir, {
      tick: world.tick,
      who: act.who ?? null,
      verb: act.verb,
      args: act.args ?? {},
      fault: thrown instanceof Fault ? thrown.code : "crash",
      note: thrown instanceof Fault ? thrown.note : String((thrown as Error)?.message ?? thrown),
    });
  const step = (act: Act) => {
    const done = k.call(world, act);
    const text = `${canon(done.line)}\n`;
    appendFileSync(journal, text);
    bytes += Buffer.byteLength(text);
    world = done.world;
    if (world.tick % EVERY === 0) snapshot(dir, world);
    return done;
  };
  const name = configOf(dir).name;
  const handle = (wish: Asked): Act | null => {
    const handler = Object.hasOwn(handlers, wish.kind) ? handlers[wish.kind] : undefined;
    if (!handler) return answered(wish, { ok: false });
    let answer: Answer | Promise<Answer>;
    try {
      answer = handler(wish, `${name}/${wish.id}`);
    } catch {
      return null;
    }
    if (!(answer instanceof Promise)) return answered(wish, answer);
    answer
      .then((late) => {
        const act = answered(wish, late);
        if (act) reply(act);
      })
      .catch(() => null);
    return null;
  };
  const reply = (act: Act) =>
    locked(dir, () => {
      try {
        drive(settle(act));
      } catch (thrown) {
        logged(thrown, act);
      }
    });
  const out: Out = (wish) => {
    appendFileSync(outbox, `${canon(wish)}\n`);
    return handle(wish);
  };
  const drive = bus({ apps, world: () => world, step, heard: logged, out });
  const settle = (act: Act): Act => {
    fresh();
    if (bytesOf(journal) > bytes) truncateSync(journal, bytes);
    return { ...act, at: Math.max(act.at ?? world.at, world.at) };
  };
  const pending = !existsSync(outbox)
    ? []
    : locked(dir, () => {
        fresh();
        const done = new Set(
          whole(journal)
            .filter((line) => line.via === HOST && typeof line.args.wish === "string")
            .map((line) => line.args.wish as string),
        );
        return whole(outbox)
          .map((one) => one as unknown as Asked)
          .filter((one) => one.reply && !done.has(one.id));
      });
  for (const wish of pending) {
    const act = handle(wish);
    if (act) reply(act);
  }
  return {
    dir,
    config: () => configOf(dir),
    world: () => {
      fresh();
      return world;
    },
    list: () => {
      fresh();
      return k.list(world);
    },
    read: (path = "", shape, who = null) => {
      fresh();
      return k.read(world, path, shape, who);
    },
    call: (act) =>
      locked(dir, () => {
        try {
          return drive(settle(act)).envelope;
        } catch (thrown) {
          logged(thrown, act);
          throw thrown;
        }
      }),
  };
}

export function boot(apps: readonly App[], dir: string, config: Config, handlers: Handlers = {}): Store {
  const k = kernel(apps);
  k.boot(config);
  const f = files(dir);
  mkdirSync(dir, { recursive: true });
  writeFileSync(f.config, `${JSON.stringify(config, null, 2)}\n`);
  writeFileSync(f.journal, "");
  writeFileSync(f.faults, "");
  rmSync(f.snapshot, { force: true });
  rmSync(f.outbox, { force: true });
  return open(apps, dir, handlers);
}

export const empty = (dir: string) =>
  !existsSync(files(dir).journal) || !readFileSync(files(dir).journal, "utf8").trim();
