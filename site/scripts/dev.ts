import { existsSync, readFileSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import shell from "../src/shell/index.html";
import { decide, joined } from "../infra/edge.ts";
import { decode } from "../src/lib/git/view.ts";
import { kind } from "../infra/mime.ts";
import { globals, ready, typed, withTint } from "./build.ts";

const org = resolve(import.meta.dir, "..");
if (process.cwd() !== org) throw new Error("dev: run from site/ (bun run --cwd site dev), where bunfig.toml hands the plugin to Bun");
const made = await ready();
const files = new Map([...made.assets, ...globals(made)].map((one) => [`/${one.path}`, one]));
files.set("/boot.js", files.get(made.boot)!);
const reply = (body: string | Uint8Array, type: string) => new Response(body as BodyInit, { headers: { "content-type": type } });
const SHELL = "/_shell";
const STATS = "/stats/stats.json";
const FIXTURE = join(org, "src", "pages", "stats", "fixture.json");
const stamp = (at: unknown, gap: number) => (typeof at === "number" ? at + gap : typeof at === "string" && !Number.isNaN(Date.parse(at)) ? new Date(Date.parse(at) + gap * 1000).toISOString() : at);
const slide = (value: unknown, gap: number): unknown => {
  if (Array.isArray(value)) return value.map((one) => slide(one, gap));
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(Object.entries(value).map(([key, one]) => [key, key === "at" ? stamp(one, gap) : slide(one, gap)]));
};
const sample = () => {
  const data = JSON.parse(readFileSync(FIXTURE, "utf8"));
  return JSON.stringify(slide(data, Math.floor(Date.now() / 1000) - data.at));
};

const server = Bun.serve({
  port: Number(process.env.PORT ?? 3000),
  hostname: "127.0.0.1",
  development: { hmr: true },
  routes: { [SHELL]: shell },
  async fetch(request): Promise<Response> {
    const url = new URL(request.url);
    const step = decide(decode(url.pathname));
    if (step.redirect) return Response.redirect(joined(step.redirect, url.search), 301);
    const path = step.uri!;
    if (path === "/index.html" || path === "/404.html") return reply(withTint((await (await fetch(new URL(SHELL, server.url))).text()).replaceAll(`${org}/`, "/")), "text/html; charset=utf-8");
    const raw = path.startsWith("/raw/");
    const disk = raw ? join(org, "..", path.slice(5)) : join(org, "public", path);
    const found = !path.includes("..") && existsSync(disk) && statSync(disk).isFile() ? readFileSync(disk) : null;
    if (found) return reply(found, raw ? typed(path, found) : kind(disk));
    const hit = files.get(path);
    if (!hit && path === STATS) return reply(sample(), "application/json");
    return hit ? reply(hit.bytes, hit.type ?? kind(path)) : new Response("not found", { status: 404 });
  },
});

console.log(`dev: ${made.rows.length} rows at ${server.url}`);
