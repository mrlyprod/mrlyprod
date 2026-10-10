import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { client, del, getText, need, putBytes } from "./s3.ts";
import { kind } from "./types.ts";

/* TYPES */

export type Store = { bucket: string; prefix: string };

export type Block = { prefix: string; guard: string[]; bucket: string; store: Store };

export type Record_ = { hash: string; at: string; outputs: string[]; types?: Record<string, string>; sums?: Record<string, string> };

export type Manifest = Record<string, Record_>;

export type File = { path: string; hash: string; type?: string; fixed?: boolean };

export type Built = { out: string; rows: unknown[]; files: File[] };

const ASSET = "@";
const BATCH = 32;
const TICK = 512;
const WEEK = 7;

export const GONE = "~";

export const today = () => new Date().toISOString().slice(0, 10);

const digest = (...parts: (string | Uint8Array)[]) => parts.reduce((h, part) => h.update(part), createHash("sha256")).digest("hex");

export function block(root: string): Block {
  const found = (JSON.parse(readFileSync(join(root, "site.json"), "utf8")) as { push?: Block }).push;
  if (!found) throw new Error("push: site.json has no push block");
  return found;
}

/* HEADERS */

export const IMMUTABLE = "public, max-age=31536000, immutable";
export const SHORT = "public, max-age=0, s-maxage=60, stale-while-revalidate=300, stale-if-error=86400";

const HASHED = /-[0-9a-z]{8}\.[^./]+$/;

export function headers(files: File[]): (path: string) => string {
  const fixed = new Set(files.filter((one) => one.fixed).map((one) => one.path));
  const loose = [...fixed].filter((path) => !HASHED.test(path));
  if (loose.length) throw new Error(`push: ${loose.join(", ")} would be immutable with no hash in the name`);
  return (path) => (fixed.has(path) ? IMMUTABLE : SHORT);
}

const SHELL = new Set(["index.html", "404.html"]);

/* GUARD */

export const mine = (path: string, guard: string[]) => !guard.some((one) => path.startsWith(one));

type Page = { contents?: { key: string }[]; commonPrefixes?: ({ prefix: string } | string)[]; isTruncated?: boolean; nextContinuationToken?: string | null };

export type Lister = { list: (options: { prefix: string; delimiter: string; maxKeys: number; continuationToken?: string }) => Promise<Page | null> };

export async function sweep(s3: Lister, root: string, guard: string[], at = root, out: string[] = []): Promise<string[]> {
  let token: string | undefined;
  do {
    const page = await s3.list({ prefix: at, delimiter: "/", maxKeys: 1000, continuationToken: token });
    for (const item of page?.contents ?? []) out.push(item.key.slice(root.length));
    for (const one of page?.commonPrefixes ?? []) {
      const next = typeof one === "string" ? one : one.prefix;
      if (!guard.some((each) => next === root + each)) await sweep(s3, root, guard, next, out);
    }
    token = page?.isTruncated ? (page.nextContinuationToken ?? undefined) : undefined;
  } while (token);
  return out;
}

/* MANIFEST */

export function assets(files: File[], old: Manifest, now = today()): Manifest {
  const out: Manifest = {};
  for (const one of files) {
    const was = old[ASSET + one.path];
    out[ASSET + one.path] = { ...(was && was.hash === one.hash ? was : { hash: one.hash, at: now, outputs: [one.path] }), types: one.type ? { [one.path]: one.type } : undefined };
  }
  return out;
}

const each = <T>(manifest: Manifest, pick: (record: Record_) => Record<string, T> | undefined) => new Map(Object.values(manifest).flatMap((record) => Object.entries(pick(record) ?? {})));

export const typing = (manifest: Manifest) => each(manifest, (record) => record.types);

export const sums = (manifest: Manifest) => each(manifest, (record) => record.sums);

export const spread = (manifest: Manifest) => new Map(Object.entries(manifest).flatMap(([key, record]) => record.outputs.map((path) => [path, key] as const)));

export const seal = (bytes: string | Uint8Array, type: string, control: string) => digest(bytes, type, control).slice(0, 16);

export function changes(old: Manifest, next: Manifest, guard: string[], sealed: (path: string) => string, force = false): string[] {
  const had = spread(old);
  const before = sums(old);
  const out: string[] = [];
  for (const [path, name] of spread(next)) {
    if (!mine(path, guard)) continue;
    const was = old[name];
    const now = next[name]!;
    if (!force && was && was.hash === now.hash && had.has(path)) continue;
    const sum = sealed(path);
    now.sums = { ...now.sums, [path]: sum };
    if (before.get(path) !== sum) out.push(path);
  }
  return out;
}

export const staged = (paths: string[], header: (path: string) => string) => [0, 1, 2].map((tier) => paths.filter((path) => (SHELL.has(path) ? 2 : header(path) === IMMUTABLE ? 0 : 1) === tier));

const days = (from: string, to: string) => (Date.parse(to) - Date.parse(from)) / 86_400_000;

export function keep(old: Manifest, want: Map<string, string>, now = today()): Manifest {
  const out: Manifest = {};
  for (const [path, name] of spread(old)) {
    if (want.has(path)) continue;
    const was = old[name]!;
    const parked = name.startsWith(GONE);
    if (!parked || days(was.at, now) < WEEK) out[GONE + path] = { hash: was.hash, at: parked ? was.at : now, outputs: [path] };
  }
  return out;
}

/* LOCAL */

export const holding = () => process.env.DRY === "1";

const where = (store: Store) => join(process.env.DRY_DIR ?? join(tmpdir(), "push", store.prefix), "manifest.json");

/* PUSH */

export async function push(root: string, build: () => Promise<Built>, options: { dry?: boolean; force?: boolean } = {}) {
  const conf = block(root);
  const local = holding();
  const site = local ? null : client(need(conf.bucket));
  const store = local ? null : conf.store.bucket === conf.bucket ? site : client(need(conf.store.bucket));
  const key = `${conf.store.prefix}/manifest.json`;
  const found = store ? await getText(store, key) : existsSync(where(conf.store)) ? readFileSync(where(conf.store), "utf8") : null;
  const old: Manifest = found ? JSON.parse(found) : {};
  const done = await build();
  const next = assets(done.files, old);
  const want = spread(next);
  const types = typing(next);
  const type = (path: string) => types.get(path) ?? kind(path);
  const cache = headers(done.files);
  const tiers = staged(changes(old, next, conf.guard, (path) => seal(readFileSync(join(done.out, path)), type(path), cache(path)), options.force), cache);
  const upload = tiers.flat();
  const seen = found || !site ? [...spread(old).keys()] : await sweep(site, conf.prefix, conf.guard);
  const parked = keep(old, want);
  const remove = seen.filter((path) => mine(path, conf.guard) && !want.has(path) && !(GONE + path in parked));
  const text = `${JSON.stringify({ ...next, ...parked }, null, 2)}\n`;
  if (options.dry) {
    for (const path of tiers[0]!) console.log(`immutable ${conf.prefix + path}`);
    console.log(`tiers: ${tiers[0]!.length} immutable first, ${tiers[1]!.length} at max-age 0, then the shell ${tiers[2]!.join(" ") || "unchanged"}; ${remove.length} to delete, ${Object.keys(parked).length} kept a week`);
  } else if (site && store) {
    let sent = 0;
    for (const tier of tiers) {
      for (let i = 0; i < tier.length; i += BATCH) {
        const batch = tier.slice(i, i + BATCH);
        await Promise.all(batch.map((path) => putBytes(site, conf.prefix + path, new Uint8Array(readFileSync(join(done.out, path))), { type: type(path), cacheControl: cache(path) })));
        const was = sent;
        sent += batch.length;
        if (Math.floor(sent / TICK) > Math.floor(was / TICK) || sent === upload.length) console.log(`upload ${sent}/${upload.length}`);
      }
    }
    if (remove.length) {
      console.log(`delete ${remove.length}`);
      await del(site, remove.map((path) => conf.prefix + path), BATCH);
    }
    await putBytes(store, key, text, { type: "application/json", cacheControl: SHORT });
  } else {
    mkdirSync(dirname(where(conf.store)), { recursive: true });
    writeFileSync(where(conf.store), text);
  }
  return { rows: done.rows.length, uploaded: upload.length, deleted: remove.length, kept: Object.keys(parked).length };
}

/* MAIN */

export async function main(root: string, build: () => Promise<Built>): Promise<void> {
  const dry = process.argv.includes("--dry");
  const done = await push(root, build, { dry, force: process.argv.includes("--force") });
  console.log(`push${dry ? " --dry" : ""}${holding() ? " --local" : ""}: ${done.rows} rows, ${done.uploaded} uploaded, ${done.deleted} deleted, ${done.kept} kept a week, ${block(root).guard.join(", ")} guarded`);
}
