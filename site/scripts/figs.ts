import type { S3Client } from "bun";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { client, need } from "../kit/s3.ts";

/* WHERE */

const ORG = resolve(import.meta.dir, "..");
const PRESS = resolve(ORG, "../../data/mrlyprod/figures");
const CACHE = process.env.MRLY_FIGURES_CACHE || "/tmp/mrly-figures";
const MARK = ".keys.json";
const BATCH = 32;
const TRIES = 3;
const LEFT = 60000;

/* TYPES */

export type Pin = { key: string; files: string[] };

export type Lock = Record<string, string | Pin>;

export type Want = { name: string; key: string; from: string; to: string };

export type Get = (want: Want) => Promise<Uint8Array>;

export type Options = { root?: string; press?: string; cache?: string; get?: Get };

export type Made = { dir: string; files: number; placed: number; fetched: number; removed: number };

/* ROWS */

const pressed = (name: string, key: string): Want[] => [
  { name, key, from: `${name}-dark.webp`, to: `${name}-dark.webp` },
  { name, key, from: `${name}-light.webp`, to: `${name}-light.webp` },
  { name, key, from: `${name}.png`, to: `${name}-dark.png` },
];

function wants(lock: Lock, desk: Record<string, string>, held: (key: string) => boolean): Want[] {
  const out: Want[] = [];
  for (const name of [...new Set([...Object.keys(lock), ...Object.keys(desk)])].sort()) {
    const row = lock[name];
    if (row && typeof row !== "string") {
      out.push(...row.files.map((file) => ({ name, key: row.key, from: file, to: file })));
      continue;
    }
    const key = desk[name] && held(desk[name]) ? desk[name] : row;
    if (key) out.push(...pressed(name, key));
  }
  return out;
}

function lockOf(path: string): unknown {
  if (!existsSync(path)) return {};
  try {
    return JSON.parse(readFileSync(path, "utf8"));
  } catch {
    throw new Error(`figures: ${path} is not JSON`);
  }
}

function marksOf(path: string): Record<string, string> {
  try {
    return JSON.parse(readFileSync(path, "utf8"));
  } catch {
    return {};
  }
}

/* FETCH */

function door(): Get {
  let s3: S3Client | null = null;
  return async ({ key, from }) => {
    s3 ??= client(need("MRLYPROD_BUCKET"));
    return Buffer.from(await s3.file(`figures/${key}/${from}`).arrayBuffer());
  };
}

async function attempt<T>(work: () => Promise<T>): Promise<T> {
  for (let n = 1; ; n++) {
    try {
      return await work();
    } catch (error) {
      if (n >= TRIES) throw error;
      await Bun.sleep(300 * n);
    }
  }
}

const beside = (path: string) => join(dirname(path), `.${basename(path)}.${process.pid}.${Math.random().toString(36).slice(2, 10)}.tmp`);

const put = (path: string, body: Uint8Array | string) => {
  mkdirSync(dirname(path), { recursive: true });
  const tmp = beside(path);
  writeFileSync(tmp, body);
  renameSync(tmp, path);
};

const abandoned = (dir: string, name: string) => {
  try {
    return /^\..*\.tmp$/.test(name) && Date.now() - statSync(join(dir, name)).mtimeMs > LEFT;
  } catch {
    return false;
  }
};

/* PLACE */

async function run(options: Options): Promise<Made> {
  const root = options.root ?? ORG;
  const press = options.press ?? PRESS;
  const cache = options.cache ?? CACHE;
  const get = options.get ?? door();
  const dir = join(root, "data", "figures");
  const lock = lockOf(join(root, "figures.lock")) as Lock;
  const desk = lockOf(join(press, "figures.lock")) as Record<string, string>;
  const all = wants(lock, desk, (key) => existsSync(join(press, "store", key)));
  if (!all.length) throw new Error(`figures: ${join(root, "figures.lock")} names no figure; the figures console writes it`);
  mkdirSync(dir, { recursive: true });
  const next = Object.fromEntries(all.map((want) => [want.to, `${want.key}/${want.from}`]));
  const held = Object.fromEntries(Object.entries(marksOf(join(dir, MARK))).filter(([to]) => Object.hasOwn(next, to) && existsSync(join(dir, to))));
  let saved = existsSync(join(dir, MARK)) ? readFileSync(join(dir, MARK), "utf8") : "";
  const save = () => {
    const text = `${JSON.stringify(held, null, 1)}\n`;
    if (text === saved) return;
    put(join(dir, MARK), text);
    saved = text;
  };
  const todo = all.filter((want) => held[want.to] !== next[want.to]);
  const stale = readdirSync(dir, { withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name !== MARK && !Object.hasOwn(next, entry.name) && (!entry.name.startsWith(".") || abandoned(dir, entry.name)))
    .map((entry) => entry.name);
  let fetched = 0;
  const source = async (want: Want) => {
    const home = [join(press, "store"), join(press, "pins"), cache].map((base) => join(base, want.key, want.from)).find((file) => existsSync(file));
    if (home) return home;
    const to = join(cache, want.key, want.from);
    try {
      put(to, await attempt(() => get(want)));
    } catch (error) {
      throw new Error(`figures: ${want.name} ${want.from} is not in the local store and s3 gave none: ${error instanceof Error ? error.message : String(error)}`);
    }
    fetched++;
    return to;
  };
  for (let i = 0; i < todo.length; i += BATCH) {
    const batch = todo.slice(i, i + BATCH);
    for (const want of batch) delete held[want.to];
    save();
    await Promise.all(
      batch.map(async (want) => {
        const to = join(dir, want.to);
        const tmp = beside(to);
        copyFileSync(await source(want), tmp);
        renameSync(tmp, to);
      }),
    );
    for (const want of batch) held[want.to] = next[want.to]!;
    save();
  }
  for (const name of stale) rmSync(join(dir, name), { force: true });
  save();
  return { dir, files: all.length, placed: todo.length, fetched, removed: stale.length };
}

let queue: Promise<unknown> = Promise.resolve();

export function ensureFigures(options: Options = {}): Promise<Made> {
  const next = queue.then(
    () => run(options),
    () => run(options),
  );
  queue = next;
  return next;
}

export const said = (made: Made) => `figures ${made.files} files, ${made.placed} placed, ${made.fetched} fetched, ${made.removed} removed, ${made.dir}`;

/* MAIN */

if (import.meta.main) console.log(said(await ensureFigures()));
