import { existsSync, readFileSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { router } from "@mrlyprod/os/hosts/cloud/edge.ts";
import { kind } from "@mrlyprod/os/hosts/cloud/mime.ts";
import { decode } from "../../kit/git/view.ts";
import shell from "../../shell/index.html";
import { globals, ready, sheet, typed, withSite } from "./build.ts";
import { HOME, REPO, SITE } from "./home.ts";

if (process.cwd() !== REPO)
  throw new Error("dev: run from the repo root (bun run dev), where bunfig.toml hands the plugin to Bun");
const { decide, join: joined } = router(JSON.parse(readFileSync(join(HOME, "redirects.json"), "utf8")));
const made = await ready();
const files = new Map([...made.assets, ...globals(made)].map((one) => [`/${one.path}`, one]));
files.set("/boot.js", files.get(made.boot)!);
const looks = new Map(made.designs.map((one) => [made.pages[one.name], one]));
const reply = (body: string | Uint8Array, type: string) =>
  new Response(body as BodyInit, { headers: { "content-type": type } });
const SHELL = "/_shell";
const STATS = "/stats/stats.json";
const FIXTURE = join(REPO, "..", "apps", "stats", "fixture.json");
const stamp = (at: unknown, gap: number) =>
  typeof at === "number"
    ? at + gap
    : typeof at === "string" && !Number.isNaN(Date.parse(at))
      ? new Date(Date.parse(at) + gap * 1000).toISOString()
      : at;
const slide = (value: unknown, gap: number): unknown => {
  if (Array.isArray(value)) return value.map((one) => slide(one, gap));
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(
    Object.entries(value).map(([key, one]) => [key, key === "at" ? stamp(one, gap) : slide(one, gap)]),
  );
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
    if (path === "/index.html" || path === "/404.html")
      return reply(
        withSite((await (await fetch(new URL(SHELL, server.url))).text()).replaceAll(`${REPO}/`, "/"), made),
        "text/html; charset=utf-8",
      );
    const raw = path.startsWith("/raw/");
    const disk = raw ? join(resolve(HOME, SITE.git.root ?? "."), path.slice(5)) : join(HOME, "public", path);
    const found = !path.includes("..") && existsSync(disk) && statSync(disk).isFile() ? readFileSync(disk) : null;
    if (found) return reply(found, raw ? typed(path, found) : kind(disk));
    const dressed = looks.get(path);
    if (dressed) return reply((await sheet(dressed, new Map()))[0]!.bytes, kind(path));
    const hit = files.get(path);
    if (!hit && path === STATS) return reply(sample(), "application/json");
    return hit ? reply(hit.bytes, hit.type ?? kind(path)) : new Response("not found", { status: 404 });
  },
});

console.log(`dev: ${made.rows.length} rows at ${server.url}`);
