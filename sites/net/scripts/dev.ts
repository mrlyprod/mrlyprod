import { existsSync, statSync, watch } from "node:fs";
import { extname, join, resolve } from "node:path";
import { forget, globals, render, scan, type Output, type Route, type Site } from "../../kit/ssg/build.ts";
import { owner as rawOwner } from "../../kit/git/git.ts";
import { counted, spec } from "./site.ts";

const org = resolve(import.meta.dir, "..");
const cached = join(org, "data", "shelf", "research");
if (!process.env.MRLY_SHELF && existsSync(join(cached, "README.md"))) process.env.MRLY_SHELF = cached;

/* TYPES */

const TYPES: Record<string, string> = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json",
  ".md": "text/markdown; charset=utf-8",
  ".pdf": "application/pdf",
  ".png": "image/png",
  ".svg": "image/svg+xml",
  ".tex": "text/plain; charset=utf-8",
  ".txt": "text/plain; charset=utf-8",
  ".wasm": "application/wasm",
  ".webmanifest": "application/manifest+json",
  ".xml": "application/xml",
};

const type = (path: string) => TYPES[extname(path)] ?? "application/octet-stream";

const send = (item: Output, status = 200) =>
  new Response(item.bytes as string | Uint8Array, { status, headers: { "content-type": item.type ?? type(item.path) } });

/* SCAN */

let site: Site = await scan(spec);
let extra: Map<string, Output> | null = null;

async function assets() {
  if (!extra) extra = new Map((await globals(site, spec)).map((item) => [item.path, item]));
  return extra;
}

function watched() {
  const paths = new Set<string>();
  for (const one of Object.values(site.inputs)) if (!one.missing) paths.add(one.path);
  if (site.kit) paths.add(site.kit.path);
  for (const dir of spec.templates ?? []) paths.add(join(org, dir));
  return [...paths].filter((path) => existsSync(path));
}

let timer: ReturnType<typeof setTimeout> | null = null;

function refresh() {
  if (timer) clearTimeout(timer);
  timer = setTimeout(async () => {
    forget();
    site = await scan(spec);
    extra = null;
  }, 80);
}

for (const path of watched()) watch(path, { recursive: statSync(path).isDirectory() }, refresh);

/* SERVE */

const pages = () => site.routes.filter((route) => route.kind !== "demos");

const disk = (): [string, string][] => [
  ["/ui/", site.kit?.path ?? ""],
  ["/lib/", join(org, "lib")],
  ["/figures/", site.input("figures").path],
  ["/research/", site.input("research").path],
  ["/", site.input("public").path],
];

async function seek(route: Route, want: string) {
  const outputs = await render(site, route, spec);
  return outputs.find((item) => item.path === want) ?? null;
}

async function serve(path: string): Promise<Response | null> {
  const want = path.endsWith("/") ? `${path.slice(1)}index.html` : path.slice(1);
  const route = pages().find((one) => one.route === path);
  if (route) {
    const hit = await seek(route, want);
    if (hit) return send(hit);
  }
  const back = rawOwner(path);
  const holder = back ? pages().find((one) => one.route === back) : null;
  if (holder) {
    const hit = await seek(holder, want);
    if (hit) return send(hit);
  }
  for (const [at, dir] of disk()) {
    if (!dir || !path.startsWith(at)) continue;
    const file = Bun.file(join(dir, path.slice(at.length)));
    if (await file.exists()) return new Response(file);
  }
  const owner = pages()
    .filter((one) => one.route.endsWith("/") && path.startsWith(one.route))
    .sort((a, b) => b.route.length - a.route.length)[0];
  if (owner && owner !== route) {
    const hit = await seek(owner, want);
    if (hit) return send(hit);
  }
  const hit = (await assets()).get(want);
  return hit ? send(hit) : null;
}

async function lost() {
  const route = site.routes.find((one) => one.kind === "missing");
  if (!route) return new Response("not found", { status: 404 });
  const hit = await seek(route, "404.html");
  return hit ? send(hit, 404) : new Response("not found", { status: 404 });
}

/* DEMOS */

const home = site.input("demos").path;
const routes: Record<string, unknown> = {};
for (const file of site.input("demos").files) {
  if (!file.endsWith("/index.html")) continue;
  const name = file.slice(home.length + 1, -"/index.html".length);
  const bundle = (await import(file)).default;
  routes[name ? `/demos/${name}` : "/demos"] = bundle;
  routes[name ? `/demos/${name}/` : "/demos/"] = bundle;
}

/* SERVER */

const server = Bun.serve({
  port: Number(process.env.PORT ?? 3000),
  development: true,
  routes,
  async fetch(req) {
    const path = decodeURIComponent(new URL(req.url).pathname);
    if (!path.endsWith("/") && pages().some((one) => one.route === `${path}/`)) return Response.redirect(`${path}/`, 302);
    return (await serve(path)) ?? (await lost());
  },
});

const count = counted();
console.log(`dev: ${site.routes.length} routes, ${count.papers} papers, ${count.research} research pages, ${count.blog} posts at ${server.url}`);
