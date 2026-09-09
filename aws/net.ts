import type { S3Client } from "bun";
import { existsSync, readdirSync, renameSync, rmSync } from "node:fs";
import { join } from "node:path";
import { client, DEV_BUCKET, getText, putBytes } from "./s3.ts";

/* WHERE */

export const SOURCE = "mrlyprod/mrlyprod";
const HEAD_KEY = "build/net/head";
const AGENT = "mrlynet-site-builder";
const SRC_DIR = "/tmp/src";
const SHELF_DIR = "/tmp/shelf";
const CACHE_DIR = process.env.BUN_INSTALL_CACHE_DIR || "/tmp/bun/cache";
const BACKOFF = [1000, 3000, 9000];

const shelfRepo = () => (process.env.SHELF_REPO ?? "").trim();

/* CLOCK */

let mark = Date.now();

function log(line: string) {
  const now = Date.now();
  console.log(`${line} ${now - mark}ms`);
  mark = now;
}

/* RETRY */

async function retry<T>(label: string, work: () => Promise<T>): Promise<T> {
  for (let attempt = 0; ; attempt++) {
    try {
      return await work();
    } catch (error) {
      if (attempt === BACKOFF.length) throw error;
      const wait = BACKOFF[attempt] ?? 1000;
      log(`${label} failed, retry ${attempt + 1} in ${wait}ms: ${String((error as Error).message ?? error).slice(0, 160)}`);
      await Bun.sleep(wait);
    }
  }
}

/* EVENT */

export type Source = "push" | "schedule" | "automator" | "manual" | "";

export type Wake = { source: Source; on: "source" | "shelf" | ""; sha: string };

const SOURCES = new Set(["push", "schedule", "automator", "manual"]);
const SHA = /^[0-9a-f]{7,40}$/;

function json(text: string): Record<string, unknown> {
  try {
    const value = JSON.parse(text);
    return value && typeof value === "object" ? (value as Record<string, unknown>) : {};
  } catch {
    return {};
  }
}

export function readEvent(text: string): Wake {
  const outer = json(text);
  const body = typeof outer.body === "string" ? json(outer.body) : outer;
  const word = typeof body.source === "string" ? body.source.trim() : "";
  const repo = typeof body.repo === "string" ? body.repo.trim() : "";
  const named = typeof body.sha === "string" ? body.sha.trim().toLowerCase() : "";
  return {
    source: SOURCES.has(word) ? (word as Source) : "",
    on: repo === SOURCE ? "source" : (repo && repo === shelfRepo() ? "shelf" : ""),
    sha: SHA.test(named) ? named : "",
  };
}

async function payload(request?: Request): Promise<string> {
  if (!request) return "";
  try {
    return await request.text();
  } catch {
    return "";
  }
}

/* HEAD */

type Head = { sha: string; etag: string; shelf: string; shelfEtag: string };

type Mark = { sha: string; etag: string };

const short = (sha: string) => sha.slice(0, 7) || "none";

async function readHead(s3: S3Client): Promise<Head> {
  const text = await getText(s3, HEAD_KEY);
  const [sha = "", etag = "", shelf = "", shelfEtag = ""] = (text ?? "").trim().split("\n");
  return { sha: sha.trim(), etag: etag.trim(), shelf: shelf.trim(), shelfEtag: shelfEtag.trim() };
}

async function writeHead(s3: S3Client, head: Head): Promise<void> {
  const body = `${head.sha}\n${head.etag}\n${head.shelf}\n${head.shelfEtag}\n`;
  await putBytes(s3, HEAD_KEY, body, { type: "text/plain", cacheControl: "no-store" });
}

/* GITHUB */

async function commit(slug: string, etag: string): Promise<Mark | null> {
  return retry(`github ${slug}`, async () => {
    const headers: Record<string, string> = { "user-agent": AGENT, accept: "application/vnd.github+json" };
    if (etag) headers["if-none-match"] = etag;
    const res = await fetch(`https://api.github.com/repos/${slug}/commits/main`, { headers });
    if (res.status === 304) return null;
    if (!res.ok) throw new Error(`github ${slug} ${res.status}: ${(await res.text()).slice(0, 200)}`);
    const body = (await res.json()) as { sha?: string };
    if (!body.sha) throw new Error(`github ${slug}: the commit carries no sha`);
    return { sha: body.sha, etag: res.headers.get("etag") ?? "" };
  });
}

async function unpack(slug: string, ref: string, into: string): Promise<void> {
  await retry(`codeload ${slug} ${short(ref)}`, async () => {
    const stage = `${into}.stage`;
    rmSync(stage, { recursive: true, force: true });
    rmSync(into, { recursive: true, force: true });
    const res = await fetch(`https://codeload.github.com/${slug}/tar.gz/${ref}`, { headers: { "user-agent": AGENT } });
    if (!res.ok) throw new Error(`codeload ${slug} ${ref}: ${res.status}`);
    const files = await new Bun.Archive(new Uint8Array(await res.arrayBuffer())).extract(stage);
    const [top] = readdirSync(stage);
    if (!files || !top) throw new Error(`codeload ${slug} ${ref}: the tarball is empty`);
    renameSync(join(stage, top), into);
    rmSync(stage, { recursive: true, force: true });
  });
}

/* PLAN */

function complete(stored: Head): boolean {
  return Boolean(stored.sha) && (!shelfRepo() || Boolean(stored.shelf));
}

function seen(wake: Wake, stored: Head): string {
  if (!wake.sha || !complete(stored)) return "";
  if (wake.on === "source" && wake.sha === stored.sha) return short(wake.sha);
  if (wake.on === "shelf" && wake.sha === stored.shelf) return `shelf ${short(wake.sha)}`;
  return "";
}

async function shelfMark(wake: Wake, stored: Head): Promise<Mark> {
  const slug = shelfRepo();
  if (!slug) return { sha: "", etag: "" };
  if (wake.on === "shelf" && wake.sha) return { sha: wake.sha, etag: "" };
  return (await commit(slug, stored.shelf ? stored.shelfEtag : "")) ?? { sha: stored.shelf, etag: stored.shelfEtag };
}

async function freshen(wake: Wake, stored: Head): Promise<Head> {
  const source =
    wake.on === "source" && wake.sha
      ? { sha: wake.sha, etag: "" }
      : ((await commit(SOURCE, stored.sha ? stored.etag : "")) ?? { sha: stored.sha, etag: stored.etag });
  const shelf = await shelfMark(wake, stored);
  return { sha: source.sha, etag: source.etag, shelf: shelf.sha, shelfEtag: shelf.etag };
}

/* CHILD */

function childEnv(): Record<string, string> {
  const env: Record<string, string> = {
    ...(process.env as Record<string, string>),
    HOME: "/tmp",
    BUN_INSTALL_CACHE_DIR: CACHE_DIR,
  };
  if (shelfRepo()) env.MRLY_SHELF = join(SHELF_DIR, "research");
  else delete env.MRLY_SHELF;
  return env;
}

async function run(cmd: string[], cwd: string): Promise<string> {
  const child = Bun.spawn(cmd, { cwd, env: childEnv(), stdout: "pipe", stderr: "pipe" });
  const [out, err] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text()]);
  const code = await child.exited;
  if (code !== 0) throw new Error(`${cmd.join(" ")}: exit ${code}\n${(err || out).trim().slice(-1500)}`);
  return out;
}

/* BUILD */

async function build(s3: S3Client, next: Head): Promise<string> {
  await unpack(SOURCE, next.sha, SRC_DIR);
  log(`source ${short(next.sha)}`);
  const slug = shelfRepo();
  if (slug) {
    await unpack(slug, next.shelf || "main", SHELF_DIR);
    if (!existsSync(join(SHELF_DIR, "research", "README.md"))) throw new Error("shelf: no research/README.md in the tarball");
    log(`shelf ${short(next.shelf)}`);
  }
  const site = join(SRC_DIR, "sites", "net");
  await run([process.execPath, "install", "--frozen-lockfile"], join(SRC_DIR, "sites", "kit"));
  await run([process.execPath, "install", "--frozen-lockfile"], site);
  log("install");
  const pkg = await run([process.execPath, "scripts/pkg.ts"], site);
  log(`pkg ${(pkg.trim().split(/\s+/)[0] ?? "").slice(0, 12)}`);
  const out = await run([process.execPath, "run", "push"], site);
  const line = out.split("\n").map((one) => one.trim()).find((one) => one.startsWith("push:")) ?? "push: no count line";
  log(line);
  await writeHead(s3, next);
  log(`head ${short(next.sha)} ${short(next.shelf)}`);
  return line;
}

/* HANDLER */

async function handler(request?: Request): Promise<Response> {
  const began = Date.now();
  mark = began;
  const wake = readEvent(await payload(request));
  const s3 = client(DEV_BUCKET);
  const stored = await readHead(s3);
  const known = seen(wake, stored);
  if (known) {
    log(`seen ${known}`);
    return new Response(`seen ${known}\n`);
  }
  const next = await freshen(wake, stored);
  if (next.sha === stored.sha && next.shelf === stored.shelf) {
    if (next.etag !== stored.etag || next.shelfEtag !== stored.shelfEtag) await writeHead(s3, next);
    log(`unchanged ${short(next.sha)}`);
    return new Response(`unchanged ${short(next.sha)}\n`);
  }
  log(`${wake.source || "poll"} ${short(next.sha)} shelf ${short(next.shelf)}`);
  const line = await build(s3, next);
  const total = Date.now() - began;
  console.log(`done ${short(next.sha)} ${total}ms`);
  return new Response(`${line} in ${total}ms\n`);
}

export default { fetch: handler };
