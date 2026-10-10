import { existsSync, readFileSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import shell from "../ui/index.html";
import { decide } from "../kit/edge.ts";
import { decode } from "../kit/git/view.ts";
import { kind } from "../kit/types.ts";
import { globals, ready, typed } from "./site.ts";

const org = resolve(import.meta.dir, "..");
if (process.cwd() !== org) throw new Error("dev: run from site/ (bun run --cwd mrlyprod/site dev), where bunfig.toml hands the plugin to Bun");
const made = await ready();
const files = new Map([...made.assets, ...globals(made)].map((one) => [`/${one.path}`, one]));
files.set("/boot.js", files.get(made.boot)!);
const reply = (body: BodyInit, type: string) => new Response(body, { headers: { "content-type": type } });
const SHELL = "/_shell";

const server = Bun.serve({
  port: Number(process.env.PORT ?? 3000),
  hostname: "127.0.0.1",
  development: { hmr: true },
  routes: { [SHELL]: shell },
  async fetch(request) {
    const url = new URL(request.url);
    const step = decide(decode(url.pathname));
    if (step.redirect) return Response.redirect(`${step.redirect}${url.search}`, 301);
    const path = step.uri!;
    if (path === "/index.html" || path === "/404.html") return reply((await (await fetch(new URL(SHELL, server.url))).text()).replaceAll(`${org}/`, "/"), "text/html; charset=utf-8");
    const raw = path.startsWith("/raw/") && !path.startsWith("/raw/research/");
    const disk = raw ? join(org, "..", path.slice(5)) : join(org, "public", path);
    const found = !path.includes("..") && existsSync(disk) && statSync(disk).isFile() ? readFileSync(disk) : null;
    if (found) return reply(found, raw ? typed(path, found) : kind(disk));
    const hit = files.get(path);
    return hit ? reply(hit.bytes, hit.type ?? kind(path)) : new Response("not found", { status: 404 });
  },
});

console.log(`dev: ${made.rows.length} rows at ${server.url}`);
