import type { BunPlugin } from "bun";
import { createHash } from "node:crypto";
import { dirname, join, relative, resolve } from "node:path";
import { page, type Bytes, type Config, type Output, type Route, type Site, type Spec } from "./build.ts";

/* TYPES */

export type Mode = "ssg" | "spa" | "ssr";

export type Rule = Mode | { mode: Mode; deep?: boolean };

export type Shell = { route: string; entry: string; html: string; script: string; ahead: string[] };

export type Pack = { files: Map<string, Uint8Array>; entries: Map<string, string>; edges: Map<string, string[]> };

export type Hooks = {
  entries: (site: Site) => string[];
  page: (site: Site, route: Route, shell: Shell, out: Output[]) => Bytes;
  root?: string;
  plugins?: (site: Site) => BunPlugin[];
  place?: (path: string) => string;
};

/* RULES */

const WORDS = new Set(["ssg", "spa", "ssr"]);

const word = (rule: Rule) => (typeof rule === "string" ? rule : rule.mode);

const sunk = (rule: Rule) => typeof rule !== "string" && rule.deep === true;

export function rules(config: Config): [string, Rule][] {
  const list = Object.entries(config.modes ?? {});
  for (const [prefix, rule] of list) {
    if (!prefix.startsWith("/") || !prefix.endsWith("/")) throw new Error(`ssg: modes names ${prefix}, and a prefix starts and ends with /`);
    if (!WORDS.has(word(rule))) throw new Error(`ssg: modes gives ${prefix} the mode ${JSON.stringify(word(rule))}, and a mode is ssg, spa or ssr`);
    if (sunk(rule) && word(rule) !== "spa") throw new Error(`ssg: modes makes ${prefix} deep, and only an spa route is`);
  }
  return list.sort((a, b) => b[0].length - a[0].length);
}

export function mode(site: Site, route: Route): Mode {
  if (route.mode) {
    if (!WORDS.has(route.mode)) throw new Error(`ssg: ${route.route} carries the mode ${JSON.stringify(route.mode)}, and a mode is ssg, spa or ssr`);
    return route.mode;
  }
  if (!site.config.modes) return "ssg";
  for (const [prefix, rule] of rules(site.config)) if (route.route.startsWith(prefix)) return word(rule);
  return "ssg";
}

export const deeps = (config: Config): string[] => (config.modes ? rules(config).filter(([, rule]) => sunk(rule)).map(([prefix]) => prefix) : []);

export function fence(walls: string[], paths: string[]) {
  for (const wall of walls) {
    const under = wall.slice(1);
    for (const path of paths) {
      if (path.startsWith(under) && path !== `${under}index.html`) throw new Error(`ssg: ${path} lands under the deep shell ${wall}, where every path answers ${under}index.html`);
    }
  }
}

/* ENTRIES */

function claims(route: Route): { route: string; entry: string }[] {
  const list = route.entry ? [{ route: route.route, entry: route.entry }] : (route.urls ?? []).map((one) => ({ route: one.route, entry: one.entry ?? "" }));
  if (!list.length || list.some((one) => !one.entry)) throw new Error(`ssg: ${route.route} is spa and names no client entry`);
  return list;
}

export function check(site: Site, spec: Spec) {
  for (const route of site.routes) {
    const how = mode(site, route);
    if (how === "ssr") throw new Error(`ssg: ${route.route} is ssr, and a build has no request to render it at; dev serves it, no origin does yet`);
    if (how !== "spa") continue;
    claims(route);
    if (!spec.spa) throw new Error(`ssg: ${route.route} is spa, and the spec carries no spa hooks`);
  }
  for (const wall of deeps(site.config)) {
    const held = site.routes.find((one) => one.route === wall);
    if (!held || mode(site, held) !== "spa") throw new Error(`ssg: modes makes ${wall} deep, and no spa route sits there`);
  }
}

/* BUNDLE */

const IMPORTS = /\bimport\s*["']([^"']+)["']|\bfrom\s*["']([^"']+)["']/g;

const MODULE = /<script[^>]*type="module"[^>]*>/;

const SRC = /src="([^"]+)"/;

const trim = (path: string) => path.replace(/^\.\//, "");

const text = (body: Uint8Array) => new TextDecoder().decode(body);

const lands = (root: string, file: string) => {
  const at = relative(root, file);
  return at.endsWith(".html") ? at : at.replace(/\.[^./]+$/, ".js");
};

async function pack(site: Site, spec: Spec): Promise<Pack> {
  const hooks = spec.spa;
  if (!hooks) throw new Error("ssg: an spa route needs the spec's spa hooks");
  const root = resolve(site.root, hooks.root ?? ".");
  const list = hooks.entries(site);
  const files = new Map<string, Uint8Array>();
  const edges = new Map<string, string[]>();
  const entries = new Map<string, string>();
  if (!list.length) return { files, entries, edges };
  const built = await Bun.build({
    entrypoints: list,
    root,
    splitting: true,
    minify: true,
    define: { "process.env.NODE_ENV": '"production"' },
    naming: { chunk: "lib-[hash].[ext]", asset: "[name]-[hash].[ext]" },
    plugins: hooks.plugins?.(site) ?? [],
  });
  if (!built.success) throw new Error(`ssg: the spa entries failed to bundle\n${built.logs.join("\n")}`);
  for (const item of built.outputs) files.set(trim(item.path), new Uint8Array(await item.arrayBuffer()));
  for (const [path, body] of files) {
    if (!path.endsWith(".js")) continue;
    const deps: string[] = [];
    for (const [, bare, named] of text(body).matchAll(IMPORTS)) {
      const dep = join(dirname(path), bare ?? named);
      if (files.has(dep) && !deps.includes(dep)) deps.push(dep);
    }
    edges.set(path, deps);
  }
  for (const file of list) {
    const made = lands(root, file);
    if (!files.has(made)) throw new Error(`ssg: the spa entry ${relative(site.root, file)} bundled to no ${made}; an entry lives under the spa root`);
    entries.set(file, made);
  }
  return { files, entries, edges };
}

const packs = new WeakMap<Site, Promise<Pack>>();

export function bundle(site: Site, spec: Spec): Promise<Pack> {
  let hit = packs.get(site);
  if (!hit) {
    hit = pack(site, spec);
    packs.set(site, hit);
  }
  return hit;
}

export function near(site: Site, spec: Spec, path: string): boolean {
  if (packs.has(site)) return true;
  const hooks = spec.spa;
  if (!hooks) return false;
  const root = resolve(site.root, hooks.root ?? ".");
  const place = hooks.place ?? ((at: string) => at);
  return hooks.entries(site).some((file) => place(lands(root, file)) === path);
}

function closure(edges: Map<string, string[]>, entry: string): string[] {
  const seen = new Set<string>();
  const queue = [...(edges.get(entry) ?? [])];
  while (queue.length) {
    const next = queue.shift()!;
    if (seen.has(next)) continue;
    seen.add(next);
    queue.push(...(edges.get(next) ?? []));
  }
  return [...seen];
}

/* MOVED */

const REF = /(["'])(\.\.?\/[^"'\s]+)\1/g;

function rebase(done: Pack, place: (path: string) => string, path: string, to: string, body: string): string {
  if (dirname(to) === dirname(path)) return body;
  return body.replace(REF, (whole, quote: string, ref: string) => {
    const dep = join(dirname(path), ref);
    if (!done.files.has(dep)) return whole;
    const next = relative(dirname(to), place(dep));
    return `${quote}${next.startsWith(".") ? next : `./${next}`}${quote}`;
  });
}

/* RENDER */

const shells = (done: Pack) => new Set([...done.entries.values()].filter((path) => path.endsWith(".html")));

const spas = (site: Site) => site.routes.some((route) => mode(site, route) === "spa");

export async function packed(site: Site, spec: Spec): Promise<Output[]> {
  if (!spec.spa || !spas(site)) return [];
  const done = await bundle(site, spec);
  const place = spec.spa.place ?? ((path: string) => path);
  const pages = shells(done);
  const out: Output[] = [];
  for (const [path, body] of done.files) {
    if (pages.has(path)) continue;
    const to = place(path);
    out.push({ path: to, bytes: /\.(?:js|css)$/.test(path) && dirname(to) !== dirname(path) ? rebase(done, place, path, to, text(body)) : body });
  }
  return out;
}

const seals = new WeakMap<Site, string>();

export async function seal(site: Site, spec: Spec) {
  if (!spec.spa || !spas(site)) return;
  const done = await bundle(site, spec);
  const hash = createHash("sha256");
  for (const [path, body] of done.files) hash.update(path).update(body);
  seals.set(site, hash.digest("hex"));
}

export const sealed = (site: Site, route: Route) => (mode(site, route) === "spa" ? (seals.get(site) ?? "") : "");

export async function render(site: Site, route: Route, spec: Spec): Promise<Output[]> {
  const list = claims(route);
  const done = await bundle(site, spec);
  const hooks = spec.spa!;
  const place = hooks.place ?? ((path: string) => path);
  const pages = shells(done);
  const out: Output[] = [];
  for (const one of list) {
    const at = done.entries.get(one.entry);
    if (!at) throw new Error(`ssg: ${one.route} names ${relative(site.root, one.entry)}, which the spa entries do not list`);
    const raw = pages.has(at) ? text(done.files.get(at)!) : "";
    const src = raw.match(MODULE)?.[0].match(SRC)?.[1];
    const main = raw ? (src ? join(dirname(at), src) : "") : at;
    const html = rebase(done, place, at, page(one.route), raw);
    const shell: Shell = { route: one.route, entry: one.entry, html, script: main ? `/${place(main)}` : "", ahead: closure(done.edges, main).map((dep) => `/${place(dep)}`) };
    out.push({ path: page(one.route), bytes: hooks.page(site, route, shell, out) });
  }
  return out;
}
