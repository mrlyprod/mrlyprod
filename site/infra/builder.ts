import type { S3Client } from "bun";
import { createHash } from "node:crypto";
import { cpSync, existsSync, readFileSync, readdirSync, renameSync, rmSync } from "node:fs";
import { join } from "node:path";
import { client, getText, need, putBytes } from "./s3.ts";

/* WHERE */

const CACHE_DIR = process.env.BUN_INSTALL_CACHE_DIR || "/tmp/bun/cache";
const LAYER_DIR = process.env.NODE_LAYER_DIR || "/opt/node";
const TOKEN_ENV = "GITHUB_READ_TOKEN";
const BACKOFF = [1000, 3000, 9000];
const SHA = /^[0-9a-f]{7,40}$/;

export type Runner = (cmd: string[], cwd: string) => Promise<string>;

export type Config = {
  source: string;
  head: string;
  dir: string;
  agent: string;
  bucket: string;
  folder: string;
  sources?: string[];
  prepare?: (site: string, run: Runner) => Promise<string>;
};

export type Wake = { source: string; on: "source" | ""; sha: string; force: boolean };

export type Head = { sha: string; etag: string };

export type Reply = {
  ok: boolean;
  status: number;
  json: () => Promise<unknown>;
  text: () => Promise<string>;
  headers: { get: (name: string) => string | null };
};

export type Get = (url: string, init: { headers: Record<string, string> }) => Promise<Reply>;

/* CLOCK */

let mark = Date.now();

function log(line: string) {
  const now = Date.now();
  console.log(`${line} ${now - mark}ms`);
  mark = now;
}

const short = (sha: string) => sha.slice(0, 7) || "none";

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

/* JSON */

function json(text: string): Record<string, unknown> {
  try {
    const value = JSON.parse(text);
    return value && typeof value === "object" ? (value as Record<string, unknown>) : {};
  } catch {
    return {};
  }
}

async function payload(request?: Request): Promise<string> {
  if (!request) return "";
  try {
    return await request.text();
  } catch {
    return "";
  }
}

/* MODULES */

export type Modules = "layer" | "stale" | "absent";

export function modules(site: string, layer = LAYER_DIR): Modules {
  const lock = createHash("sha256").update(readFileSync(join(site, "bun.lock"))).digest("hex");
  const held = join(layer, "bun.lock.sha256");
  if (!existsSync(held)) return "absent";
  return readFileSync(held, "utf8").trim() === lock ? "layer" : "stale";
}

/* SPAWN */

async function spawn(cmd: string[], cwd: string, env: Record<string, string>): Promise<string> {
  const child = Bun.spawn(cmd, { cwd, env, stdout: "pipe", stderr: "pipe" });
  const [out, err] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text()]);
  const code = await child.exited;
  if (code !== 0) throw new Error(`${cmd.join(" ")}: exit ${code}\n${(err || out).trim().slice(-1500)}`);
  return out;
}

/* CORE */

export function lambda(config: Config) {
  const SOURCES = new Set(config.sources ?? ["push", "schedule", "manual"]);

  /* EVENT */

  function readEvent(text: string): Wake {
    const outer = json(text);
    const event = outer.event && typeof outer.event === "object" ? (outer.event as Record<string, unknown>) : outer;
    const body = typeof event.body === "string" ? json(event.body) : event;
    const word = typeof body.source === "string" ? body.source.trim() : "";
    const repo = typeof body.repo === "string" ? body.repo.trim() : "";
    const named = typeof body.sha === "string" ? body.sha.trim().toLowerCase() : "";
    return {
      source: SOURCES.has(word) ? word : "",
      on: repo === config.source ? "source" : "",
      sha: SHA.test(named) ? named : "",
      force: body.force === true && word === "manual",
    };
  }

  /* HEAD */

  function parseHead(text: string | null): Head {
    const lines = (text ?? "").trim().split("\n").map((one) => one.trim());
    return { sha: lines[0] ?? "", etag: lines[1] ?? "" };
  }

  const headText = (head: Head): string => `${head.sha}\n${head.etag}\n`;

  const readHead = async (s3: S3Client): Promise<Head> => parseHead(await getText(s3, config.head));

  async function writeHead(s3: S3Client, head: Head): Promise<void> {
    await putBytes(s3, config.head, headText(head), { type: "text/plain", cacheControl: "no-store" });
  }

  /* GITHUB */

  function headers(): Record<string, string> {
    const sent: Record<string, string> = { "user-agent": config.agent, accept: "application/vnd.github+json" };
    const token = (process.env[TOKEN_ENV] ?? "").trim();
    if (token) sent.authorization = `Bearer ${token}`;
    return sent;
  }

  async function commit(slug: string, etag: string, get: Get = fetch): Promise<Head | null> {
    return retry(`github ${slug}`, async () => {
      const sent: Record<string, string> = headers();
      if (etag) sent["if-none-match"] = etag;
      const res = await get(`https://api.github.com/repos/${slug}/commits/main`, { headers: sent });
      if (res.status === 304) return null;
      if (!res.ok) throw new Error(`github ${slug} ${res.status}: ${(await res.text()).slice(0, 200)}`);
      const body = (await res.json()) as { sha?: string };
      if (!body.sha) throw new Error(`github ${slug}: the commit carries no sha`);
      return { sha: body.sha, etag: res.headers.get("etag") ?? "" };
    });
  }

  async function onMain(slug: string, sha: string, get: Get = fetch): Promise<boolean> {
    try {
      const res = await get(`https://api.github.com/repos/${slug}/compare/${sha}...main`, { headers: headers() });
      if (!res.ok) {
        log(`refused ${short(sha)} on ${slug}: compare ${res.status}`);
        return false;
      }
      const body = (await res.json()) as { status?: string };
      const state = typeof body?.status === "string" ? body.status : "";
      if (state === "ahead" || state === "identical") return true;
      log(`refused ${short(sha)} on ${slug}: ${state || "no status"}, not an ancestor of main`);
      return false;
    } catch (error) {
      log(`refused ${short(sha)} on ${slug}: compare failed, ${String((error as Error)?.message ?? error).slice(0, 160)}`);
      return false;
    }
  }

  async function unpack(slug: string, ref: string, into: string): Promise<void> {
    if (ref !== "main" && !SHA.test(ref)) throw new Error(`codeload ${slug}: ${ref} is not a sha`);
    await retry(`codeload ${slug} ${short(ref)}`, async () => {
      const stage = `${into}.stage`;
      rmSync(stage, { recursive: true, force: true });
      rmSync(into, { recursive: true, force: true });
      const res = await fetch(`https://codeload.github.com/${slug}/tar.gz/${ref}`, { headers: { "user-agent": config.agent } });
      if (!res.ok) throw new Error(`codeload ${slug} ${ref}: ${res.status}`);
      const files = await new Bun.Archive(new Uint8Array(await res.arrayBuffer())).extract(stage);
      const [top] = readdirSync(stage);
      if (!files || !top) throw new Error(`codeload ${slug} ${ref}: the tarball is empty`);
      renameSync(join(stage, top), into);
      rmSync(stage, { recursive: true, force: true });
    });
  }

  /* PLAN */

  function seen(wake: Wake, stored: Head): string {
    if (wake.force || !wake.sha || !stored.sha) return "";
    return wake.on === "source" && wake.sha === stored.sha ? short(wake.sha) : "";
  }

  async function freshen(wake: Wake, stored: Head, get: Get = fetch): Promise<Head> {
    if (wake.on === "source" && wake.sha && (await onMain(config.source, wake.sha, get))) return { sha: wake.sha, etag: "" };
    return (await commit(config.source, stored.sha ? stored.etag : "", get)) ?? stored;
  }

  /* CHILD */

  const childEnv = (): Record<string, string> => ({
    ...(process.env as Record<string, string>),
    HOME: "/tmp",
    BUN_INSTALL_CACHE_DIR: CACHE_DIR,
  });

  const run: Runner = (cmd, cwd) => spawn(cmd, cwd, childEnv());

  async function install(site: string): Promise<string> {
    const state = modules(site);
    if (state === "layer") {
      const target = join(site, "node_modules");
      rmSync(target, { recursive: true, force: true });
      cpSync(join(LAYER_DIR, "node_modules"), target, { recursive: true });
      return "modules layer";
    }
    await run([process.execPath, "install", "--frozen-lockfile"], site);
    return state === "stale" ? "modules layer stale, installed" : "modules installed";
  }

  /* BUILD */

  async function build(s3: S3Client, next: Head, force: boolean): Promise<string> {
    await unpack(config.source, next.sha, config.dir);
    log(`source ${short(next.sha)}`);
    const site = join(config.dir, config.folder);
    log(await install(site));
    if (config.prepare) {
      const said = await config.prepare(site, run);
      if (said) log(said);
    }
    const out = await run([process.execPath, "run", "push", ...(force ? ["--force"] : [])], site);
    const line = out.split("\n").map((one) => one.trim()).find((one) => one.startsWith("push:")) ?? "push: no count line";
    log(line);
    await writeHead(s3, next);
    log(`head ${short(next.sha)}`);
    return line;
  }

  /* HANDLER */

  async function once(wake: Wake): Promise<string> {
    const began = Date.now();
    mark = began;
    const s3 = client(need(config.bucket));
    const stored = await readHead(s3);
    const known = seen(wake, stored);
    if (known) {
      log(`seen ${known}`);
      return `seen ${known}`;
    }
    const next = await freshen(wake, stored);
    if (!next.sha) throw new Error(`${config.source}: no sha in the event, in the head or from github`);
    if (!wake.force && next.sha === stored.sha) {
      if (next.etag !== stored.etag) await writeHead(s3, next);
      log(`unchanged ${short(next.sha)}`);
      return `unchanged ${short(next.sha)}`;
    }
    log(`${wake.source || "poll"} ${short(next.sha)}${wake.force ? " forced" : ""}`);
    const line = await build(s3, next, wake.force);
    const total = Date.now() - began;
    console.log(`done ${short(next.sha)} ${total}ms`);
    return `${line} in ${total}ms`;
  }

  async function handler(request?: Request): Promise<Response> {
    return new Response(`${await once(readEvent(await payload(request)))}\n`);
  }

  return { fetch: handler, once, readEvent, parseHead, headText, onMain, seen, freshen };
}
