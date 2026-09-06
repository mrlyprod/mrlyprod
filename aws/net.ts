import type { S3Client } from "bun";
import { existsSync, readdirSync, renameSync, rmSync } from "node:fs";
import { join } from "node:path";
import { client, DEV_BUCKET, getText, putBytes } from "./s3.ts";

/* WHERE */

const SOURCE = "mrlyprod/mrlyprod";
const SHELF = "carlomitchener/carlomitchener";
const HEAD_KEY = "build/net/head";
const AGENT = "mrlynet-builder";
const SRC_DIR = "/tmp/src";
const SHELF_DIR = "/tmp/shelf";
const CACHE_DIR = process.env.BUN_INSTALL_CACHE_DIR || "/tmp/bun/cache";

/* CLOCK */

let mark = Date.now();

function log(line: string) {
  const now = Date.now();
  console.log(`${line} ${now - mark}ms`);
  mark = now;
}

/* HEAD */

type Head = { sha: string; etag: string };

const short = (sha: string) => sha.slice(0, 7) || "none";

async function readHead(s3: S3Client): Promise<Head> {
  const text = await getText(s3, HEAD_KEY);
  const [sha = "", etag = ""] = (text ?? "").trim().split("\n");
  return { sha: sha.trim(), etag: etag.trim() };
}

async function writeHead(s3: S3Client, head: Head): Promise<void> {
  await putBytes(s3, HEAD_KEY, `${head.sha}\n${head.etag}\n`, { type: "text/plain", cacheControl: "no-store" });
}

/* GITHUB */

async function commit(etag: string): Promise<Head | null> {
  const headers: Record<string, string> = { "user-agent": AGENT, accept: "application/vnd.github+json" };
  if (etag) headers["if-none-match"] = etag;
  const res = await fetch(`https://api.github.com/repos/${SOURCE}/commits/main`, { headers });
  if (res.status === 304) return null;
  if (!res.ok) throw new Error(`github ${res.status}: ${(await res.text()).slice(0, 200)}`);
  const body = (await res.json()) as { sha?: string };
  if (!body.sha) throw new Error("github: the commit carries no sha");
  return { sha: body.sha, etag: res.headers.get("etag") ?? "" };
}

async function unpack(slug: string, ref: string, into: string): Promise<void> {
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
}

/* CHILD */

function childEnv(): Record<string, string> {
  return {
    ...(process.env as Record<string, string>),
    HOME: "/tmp",
    BUN_INSTALL_CACHE_DIR: CACHE_DIR,
    MRLY_SHELF: join(SHELF_DIR, "research"),
  };
}

async function run(cmd: string[], cwd: string): Promise<string> {
  const child = Bun.spawn(cmd, { cwd, env: childEnv(), stdout: "pipe", stderr: "pipe" });
  const [out, err] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text()]);
  const code = await child.exited;
  if (code !== 0) throw new Error(`${cmd.join(" ")}: exit ${code}\n${(err || out).trim().slice(-1500)}`);
  return out;
}

/* BUILD */

async function build(s3: S3Client, head: Head): Promise<string> {
  await unpack(SOURCE, head.sha, SRC_DIR);
  log(`source ${short(head.sha)}`);
  await unpack(SHELF, "main", SHELF_DIR);
  if (!existsSync(join(SHELF_DIR, "research", "README.md"))) throw new Error("shelf: no research/README.md in the tarball");
  log("shelf main");
  const site = join(SRC_DIR, "sites", "net");
  await run([process.execPath, "install", "--frozen-lockfile"], site);
  await run([process.execPath, "install", "--frozen-lockfile"], join(SRC_DIR, "pkgs", "js", "mrlyjs"));
  log("install");
  const pkg = await run([process.execPath, "scripts/pkg.ts"], site);
  log(`pkg ${(pkg.trim().split(/\s+/)[0] ?? "").slice(0, 12)}`);
  const out = await run([process.execPath, "run", "push"], site);
  const line = out.split("\n").map((one) => one.trim()).find((one) => one.startsWith("push:")) ?? "push: no count line";
  log(line);
  await writeHead(s3, head);
  log(`head ${short(head.sha)}`);
  return line;
}

/* HANDLER */

async function handler(_request?: Request): Promise<Response> {
  const began = Date.now();
  mark = began;
  const s3 = client(DEV_BUCKET);
  const stored = await readHead(s3);
  const fresh = await commit(stored.etag);
  if (!fresh || fresh.sha === stored.sha) {
    if (fresh && fresh.etag !== stored.etag) await writeHead(s3, fresh);
    const sha = short(fresh?.sha || stored.sha);
    log(`unchanged ${sha}`);
    return new Response(`unchanged ${sha}\n`);
  }
  log(`commit ${short(fresh.sha)}`);
  const line = await build(s3, fresh);
  const total = Date.now() - began;
  console.log(`done ${short(fresh.sha)} ${total}ms`);
  return new Response(`${line} in ${total}ms\n`);
}

export default { fetch: handler };
