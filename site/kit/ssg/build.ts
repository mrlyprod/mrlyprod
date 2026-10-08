import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, rmdirSync, statSync, unlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, extname, join, normalize, relative, resolve } from "node:path";
import { deflateSync } from "node:zlib";
import { TREE, collect as gitRoutes, forest, isGit, mirror, print as gitPrint, render as gitRender, type Hooks } from "../git/git.ts";
import { owner } from "../git/view.ts";
import { collect as blogRoutes, isBlog, render as blogRender, type Hooks as Posts } from "./blog.ts";
import { answer, index, type Index } from "./links.ts";
import { check, deeps, fence, mode, packed, render as spaRender, seal, sealed, type Hooks as Clients, type Mode, type Rule } from "./modes.ts";

/* TYPES */

export type Bytes = string | Uint8Array;

export type Output = { path: string; bytes: Bytes; type?: string };

export type Link = { route: string; name?: string; at?: string; source?: string; entry?: string };

export type Route = {
  route: string;
  kind?: string;
  name?: string;
  data?: unknown;
  source?: string;
  inputs?: string[];
  urls?: Link[];
  at?: string;
  hidden?: boolean;
  sitemap?: boolean;
  mode?: Mode;
  entry?: string;
};

export type Node = { name: string; href?: string; nodes?: Node[]; open?: boolean; lazy?: string; icon?: string; figure?: { dark: string; light: string }; text?: string; dates?: string[] };

export type Input = { name: string; path: string; files: string[]; missing: boolean };

export type Bundle = { path: string; out: string; hash: boolean; files: string[]; ext?: string };

export type Icons = { rows: string[]; svg: string };

export type Site = {
  root: string;
  out: string;
  config: Config;
  inputs: Record<string, Input>;
  kit: Bundle | null;
  routes: Route[];
  stamp: string;
  index: Index | null;
  copies: Output[];
  serves: Map<string, string>;
  made: Set<string>;
  ships: Map<string, string>;
  asks?: Set<string>;
  asset: (name: string) => string;
  input: (name: string) => Input;
  bytes: (file: string) => Uint8Array;
};

export type Decl = { path: string; out?: string; hash?: boolean; files?: string[]; ext?: string };

export type Sheet = { name: string; out?: string; files: string[] };

export type Row = { href: string; name?: string; note?: string };

export type Section = { name: string; rows: Row[] };

export type Config = {
  title?: string;
  name?: string;
  root?: string;
  inputs?: Record<string, { path: string; ext?: string; deep?: boolean }>;
  kit?: Decl;
  assets?: Decl[];
  sheets?: Sheet[];
  modes?: Record<string, Rule>;
  manifest?: Record<string, unknown>;
  robots?: { disallow?: string[] };
  llms?: { about?: string; legend?: string; links?: Row[] };
  [key: string]: unknown;
};

export type Picked = { routes: Route[] };

export type Spec = {
  root: string;
  out: string;
  templates?: string[];
  collect: (site: Site) => Promise<Picked> | Picked;
  render: (site: Site, route: Route) => Promise<Output[]> | Output[];
  globals?: (site: Site) => Promise<Output[]> | Output[];
  llms?: (site: Site) => Section[];
  inline?: string[];
  icons?: Icons;
  git?: Hooks;
  blog?: Posts;
  spa?: Clients;
};

export type Record_ = { hash: string; at: string; outputs: string[]; types?: Record<string, string>; sums?: Record<string, string>; asks?: string[]; reads?: string[] };

export type Manifest = Record<string, Record_>;

/* FILES */

const TICK = 250;
const SKIP = new Set(["node_modules", "dist", ".git", ".cache", "target", "data", "pkg"]);

export function walk(dir: string, deep = true): string[] {
  if (!existsSync(dir)) return [];
  const out: string[] = [];
  for (const item of readdirSync(dir, { withFileTypes: true })) {
    if (item.name.startsWith(".")) continue;
    const path = join(dir, item.name);
    if (item.isDirectory()) {
      if (deep && !SKIP.has(item.name)) out.push(...walk(path, deep));
    } else out.push(path);
  }
  return out.sort();
}

const cache = new Map<string, Uint8Array>();

const sums = new Map<string, string>();

export function forget() {
  cache.clear();
  sums.clear();
}

let read: Set<string> | null = null;

const isFile = (file: string) => existsSync(file) && statSync(file).isFile();

export function probe(file: string): boolean {
  read?.add(file);
  return isFile(file);
}

export function bytes(file: string): Uint8Array {
  read?.add(file);
  const hit = cache.get(file);
  if (hit) return hit;
  const data = new Uint8Array(readFileSync(file));
  cache.set(file, data);
  return data;
}

const digest = (parts: Bytes[]) => {
  const h = createHash("sha256");
  for (const part of parts) h.update(part);
  return h.digest("hex");
};

function sum(file: string): string {
  const hit = sums.get(file);
  if (hit) return hit;
  const made = digest([bytes(file)]);
  sums.set(file, made);
  return made;
}

const short = (text: string) => text.slice(0, 8);

const escape = (text: string) =>
  text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");

/* SCAN */

function inputs(root: string, config: Config): Record<string, Input> {
  const found: Record<string, Input> = {};
  for (const [name, decl] of Object.entries(config.inputs ?? {})) {
    const path = resolve(root, decl.path);
    const all = existsSync(path) && statSync(path).isFile() ? [path] : walk(path, decl.deep ?? false);
    const files = decl.ext ? all.filter((f) => f.endsWith(decl.ext!)) : all;
    found[name] = { name, path, files, missing: !existsSync(path) };
  }
  return found;
}

const CODE: Record<string, "ts" | "tsx" | "js" | "jsx"> = { ".ts": "ts", ".mts": "ts", ".tsx": "tsx", ".js": "js", ".mjs": "js", ".jsx": "jsx" };

export function graph(roots: string[]): string[] {
  const seen = new Set<string>();
  const visit = (file: string) => {
    if (seen.has(file)) return;
    seen.add(file);
    const loader = CODE[extname(file)];
    if (!loader) return;
    const text = new TextDecoder().decode(bytes(file));
    for (const { path } of new Bun.Transpiler({ loader }).scanImports(text)) if (path.startsWith(".")) visit(Bun.resolveSync(path, dirname(file)));
  };
  for (const root of roots) visit(root);
  return [...seen].sort();
}

export const drawn = (spec: Spec): string[] => graph([join(import.meta.dir, "build.ts"), ...(spec.templates ?? []).map((file) => resolve(spec.root, file))]);

function templates(spec: Spec): string {
  const root = resolve(spec.root);
  const parts: Bytes[] = [];
  for (const file of drawn(spec)) parts.push(relative(root, file), bytes(file));
  return digest(parts);
}

const outDir = (decl: Decl) => (decl.out ?? "ui").replace(/^\/|\/$/g, "");

function bundle(root: string, decl: Decl): Bundle {
  const path = resolve(root, decl.path);
  const named = decl.files ?? walk(path, false).map((f) => f.slice(path.length + 1));
  const files = decl.ext ? named.filter((f) => f.endsWith(decl.ext!)) : named;
  return { path, out: outDir(decl), hash: decl.hash ?? false, files };
}

const decls = (config: Config): Decl[] => [...(config.kit ? [config.kit] : []), ...(config.assets ?? [])];

/* ASSETS */

const IMPORT = /((?:\bfrom|\bimport)\s*\(?\s*)(["'])(\.\.?\/[^"']+)\2/g;

const URLS = /(\burl\(\s*)(["']?)([^"')]+)\2(\s*\))/g;

const LOADS = /(@import\s+)(["'])([^"']+)\2/g;

const OUTSIDE = /^(?:[a-z][a-z0-9+.-]*:|[/#])/i;

function place(one: Bundle, assets: Map<string, string>, copies: Output[], serves: Map<string, string>) {
  const known = new Set(one.files);
  const busy = new Set<string>();
  const point = (name: string, target: string): string | null => {
    if (OUTSIDE.test(target)) return null;
    const dep = normalize(join(dirname(name), target));
    if (known.has(dep)) return visit(dep);
    const placed = serves.get(join(one.path, dep));
    if (placed) return placed;
    if (existsSync(join(one.path, dep))) throw new Error(`ssg: ${name} names ${dep}, which the bundle's files do not list`);
    return null;
  };
  const visit = (name: string): string => {
    const hit = assets.get(name);
    if (hit) return hit;
    if (busy.has(name)) throw new Error(`ssg: ${name} imports itself around a cycle`);
    busy.add(name);
    const file = join(one.path, name);
    if (!existsSync(file)) throw new Error(`ssg: asset missing: ${file}`);
    let body: Bytes = bytes(file);
    if (one.hash && /\.m?js$/.test(name)) {
      const text = typeof body === "string" ? body : new TextDecoder().decode(body);
      body = text.replace(IMPORT, (whole, head: string, quote: string, target: string) => {
        const to = point(name, target);
        return to ? `${head}${quote}${to}${quote}` : whole;
      });
    }
    if (one.hash && /\.css$/.test(name)) {
      const text = typeof body === "string" ? body : new TextDecoder().decode(body);
      body = text
        .replace(URLS, (whole, head: string, quote: string, target: string, tail: string) => {
          const to = point(name, target);
          return to ? `${head}${quote}${to}${quote}${tail}` : whole;
        })
        .replace(LOADS, (whole, head: string, quote: string, target: string) => {
          const to = point(name, target);
          return to ? `${head}${quote}${to}${quote}` : whole;
        });
    }
    const data = typeof body === "string" ? new TextEncoder().encode(body) : body;
    const stem = name.replace(/(\.[^.]+)$/, "");
    const path = one.hash ? `${one.out}/${stem}-${short(digest([data]))}${extname(name)}` : `${one.out}/${name}`;
    assets.set(name, `/${path}`);
    serves.set(join(one.path, name), `/${path}`);
    copies.push({ path, bytes: data });
    busy.delete(name);
    return `/${path}`;
  };
  for (const name of one.files) visit(name);
}

function sheets(list: Sheet[], assets: Map<string, string>, copies: Output[], serves: Map<string, string>): Output[] {
  const spent = new Set<string>();
  const made: Output[] = [];
  for (const one of list) {
    const parts = one.files.map((name, n) => {
      const href = assets.get(name);
      const copy = href && copies.find((item) => `/${item.path}` === href);
      if (!copy) throw new Error(`ssg: the sheet ${one.name} names ${name}, which no bundle places`);
      const body = typeof copy.bytes === "string" ? copy.bytes : new TextDecoder().decode(copy.bytes);
      if (n && /@(?:import|charset)\b/.test(body)) throw new Error(`ssg: ${name} carries an @import or an @charset, which only the first member of the sheet ${one.name} may`);
      spent.add(copy.path);
      return copy.bytes;
    });
    const data = new Uint8Array(Buffer.concat(parts.map((part) => (typeof part === "string" ? Buffer.from(part) : part))));
    const path = `${outDir({ path: "", out: one.out })}/${one.name.replace(/(\.[^.]+)$/, "")}-${short(digest([data]))}${extname(one.name)}`;
    made.push({ path, bytes: data });
  }
  for (const [name, href] of [...assets]) if (spent.has(href.slice(1))) assets.delete(name);
  for (const [file, href] of [...serves]) if (spent.has(href.slice(1))) serves.delete(file);
  list.forEach((one, n) => assets.set(one.name, `/${made[n]!.path}`));
  return [...copies.filter((item) => !spent.has(item.path)), ...made];
}

export async function scan(spec: Spec): Promise<Site> {
  const root = resolve(spec.root);
  const config = JSON.parse(readFileSync(join(root, "site.json"), "utf8")) as Config;
  const found = inputs(root, config);
  const list = decls(config).map((decl) => bundle(root, decl));
  const assets = new Map<string, string>();
  const placed: Output[] = [];
  const serves = new Map<string, string>();
  for (const one of list) place(one, assets, placed, serves);
  const copies = sheets(config.sheets ?? [], assets, placed, serves);
  const site: Site = {
    root,
    out: resolve(spec.out),
    config,
    inputs: found,
    kit: config.kit ? list[0] : null,
    routes: [],
    stamp: "",
    index: null,
    copies,
    serves,
    made: new Set(copies.map((one) => one.path)),
    ships: new Map(),
    asset: (name) => {
      const hit = assets.get(name);
      if (!hit) throw new Error(`ssg: no asset named ${name}`);
      return hit;
    },
    input: (name) => {
      const hit = found[name];
      if (!hit) throw new Error(`ssg: ${name} is not declared under inputs in site.json`);
      return hit;
    },
    bytes,
  };
  const picked = await spec.collect(site);
  site.routes = picked.routes;
  const written = blogRoutes(site);
  if (written.routes.length) site.routes = [...site.routes, ...written.routes];
  const repo = gitRoutes(site, spec.git);
  if (repo.routes.length) site.routes = [...site.routes, ...repo.routes];
  site.index = index(site, spec.git);
  site.stamp = digest([templates(spec), JSON.stringify(config), JSON.stringify([...assets]), year()]);
  return site;
}

/* FINGERPRINT */

export function label(site: Site, file: string): string {
  let base = "";
  let name = "";
  for (const one of Object.values(site.inputs)) {
    if (one.path.length <= base.length) continue;
    if (file !== one.path && !file.startsWith(`${one.path}/`)) continue;
    base = one.path;
    name = one.name;
  }
  if (!base) return basename(file);
  return file === base ? name : `${name}/${file.slice(base.length + 1)}`;
}

export function fingerprint(site: Site, route: Route, spec?: Spec, asks: string[] = [], reads: string[] = []): string {
  const pack = sealed(site, route);
  if (isGit(route)) return pack ? digest([gitPrint(site, route, spec?.git), pack]).slice(0, 16) : gitPrint(site, route, spec?.git);
  const files = route.inputs ?? (route.source ? [route.source] : []);
  const named = files.map((file) => [label(site, file), file] as const).sort((a, b) => (a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0));
  const parts: Bytes[] = [route.route, route.kind ?? "", JSON.stringify(route.data ?? null), site.stamp];
  if (pack) parts.push(pack);
  if (asks.length && site.index) parts.push(JSON.stringify(asks.map((ask) => [ask, answer(site.index!, ask)])));
  for (const [name, file] of named) {
    parts.push(name);
    if (!existsSync(file)) {
      parts.push("gone");
      continue;
    }
    if (statSync(file).isDirectory()) for (const inner of walk(file)) parts.push(relative(file, inner), bytes(inner));
    else parts.push(bytes(file));
  }
  for (const name of reads) {
    const file = resolve(site.root, name);
    parts.push(name, isFile(file) ? sum(file) : "gone");
  }
  return digest(parts).slice(0, 16);
}

/* RENDER */

export async function render(site: Site, route: Route, spec: Spec): Promise<Output[]> {
  if (isGit(route) && !spec.git?.page) return [];
  if (mode(site, route) === "spa") return spaRender(site, route, spec);
  if (isGit(route)) return gitRender(site, route, spec);
  if (isBlog(route)) return spec.blog?.page ? blogRender(site, route, spec) : [];
  return await spec.render(site, route);
}

/* ICONS */

const SIGNATURE = Uint8Array.from([137, 80, 78, 71, 13, 10, 26, 10]);

const TABLE = new Uint32Array(256).map((_, n) => {
  let c = n;
  for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
  return c >>> 0;
});

function crc32(data: Uint8Array): number {
  let c = 0xffffffff;
  for (const b of data) c = TABLE[(c ^ b) & 0xff] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
}

function chunk(type: string, body: Uint8Array): Uint8Array {
  const out = new Uint8Array(12 + body.length);
  const view = new DataView(out.buffer);
  view.setUint32(0, body.length);
  out.set(new TextEncoder().encode(type), 4);
  out.set(body, 8);
  view.setUint32(8 + body.length, crc32(out.subarray(4, 8 + body.length)));
  return out;
}

function png(size: number, dark: (x: number, y: number) => boolean): Uint8Array {
  const raw = new Uint8Array(size * (size + 1));
  for (let y = 0; y < size; y++) for (let x = 0; x < size; x++) raw[y * (size + 1) + 1 + x] = dark(x, y) ? 0 : 255;
  const head = new Uint8Array(13);
  const view = new DataView(head.buffer);
  view.setUint32(0, size);
  view.setUint32(4, size);
  head[8] = 8;
  const parts = [SIGNATURE, chunk("IHDR", head), chunk("IDAT", new Uint8Array(deflateSync(raw))), chunk("IEND", new Uint8Array(0))];
  const out = new Uint8Array(parts.reduce((n, part) => n + part.length, 0));
  let at = 0;
  for (const part of parts) {
    out.set(part, at);
    at += part.length;
  }
  return out;
}

function ico(image: Uint8Array, size: number): Uint8Array {
  const out = new Uint8Array(22 + image.length);
  const view = new DataView(out.buffer);
  view.setUint16(2, 1, true);
  view.setUint16(4, 1, true);
  out[6] = size;
  out[7] = size;
  view.setUint16(10, 1, true);
  view.setUint16(12, 8, true);
  view.setUint32(14, image.length, true);
  view.setUint32(18, 22, true);
  out.set(image, 22);
  return out;
}

function icons({ rows, svg }: Icons): Output[] {
  const n = rows.length;
  const mark = (size: number) => png(size, (x, y) => rows[Math.floor((y * n) / size)][Math.floor((x * n) / size)] === "1");
  return [
    { path: "favicon.svg", bytes: svg },
    { path: "favicon.png", bytes: mark(40) },
    { path: "favicon.ico", bytes: ico(mark(32), 32) },
    { path: "apple-touch-icon.png", bytes: mark(180) },
    { path: "icon-192.png", bytes: mark(192) },
    { path: "icon-512.png", bytes: mark(512) },
  ];
}

/* GLOBALS */

const clean = (root: string) => (root ?? "").replace(/\/$/, "");

const AGENTS = ["GPTBot", "ClaudeBot", "Claude-Web", "CCBot", "Google-Extended", "anthropic-ai", "PerplexityBot"];

function links(site: Site, spec: Spec): Link[] {
  const out: Link[] = [];
  for (const route of site.routes) {
    if (route.hidden && !route.sitemap) continue;
    const away = mirror(site, route, spec.git);
    const list = route.urls ?? (route.route.endsWith("/") ? [{ route: route.route, name: route.name }] : []);
    for (const one of list) {
      if (away && owner(one.route)) continue;
      out.push({ route: one.route, name: one.name ?? one.route, at: one.at || route.at });
    }
  }
  return out.sort((a, b) => a.route.localeCompare(b.route));
}

export const shown = (site: Site): Link[] =>
  site.routes
    .filter((route) => !route.hidden)
    .flatMap((route) => route.urls ?? (route.route.endsWith("/") ? [{ route: route.route, name: route.name }] : []))
    .map((one) => ({ route: one.route, name: one.name ?? one.route }))
    .sort((a, b) => a.route.localeCompare(b.route));

const XML = `<?xml version="1.0" encoding="UTF-8"?>\n`;

const SCHEMA = "http://www.sitemaps.org/schemas/sitemap/0.9";

const LONE = "pages";

const segment = (route: string) => route.split("/")[1]!;

function sitemaps(shown: Link[], root: string): Output[] {
  const count = new Map<string, number>();
  for (const one of shown) count.set(segment(one.route), (count.get(segment(one.route)) ?? 0) + 1);
  if ((count.get(LONE) ?? 0) > 1) throw new Error(`ssg: /${LONE}/ holds ${count.get(LONE)} urls, and sitemap-${LONE}.xml is the lone pages' map`);
  const children = new Map<string, Link[]>();
  for (const one of shown) {
    const at = segment(one.route);
    const name = at && count.get(at)! > 1 ? at : LONE;
    if (!children.has(name)) children.set(name, []);
    children.get(name)!.push(one);
  }
  const out: Output[] = [];
  const index: string[] = [];
  for (const name of [...children.keys()].sort()) {
    const list = children.get(name)!;
    const dates = list.map((l) => l.at || today());
    const urls = list.map((l, n) => `<url><loc>${escape(root + l.route)}</loc><lastmod>${dates[n]}</lastmod></url>`);
    out.push({ path: `sitemap-${name}.xml`, bytes: `${XML}<urlset xmlns="${SCHEMA}">\n${urls.join("\n")}\n</urlset>\n` });
    index.push(`<sitemap><loc>${escape(`${root}/sitemap-${name}.xml`)}</loc><lastmod>${dates.reduce((a, b) => (b > a ? b : a))}</lastmod></sitemap>`);
  }
  return [{ path: "sitemap.xml", bytes: `${XML}<sitemapindex xmlns="${SCHEMA}">\n${index.join("\n")}\n</sitemapindex>\n` }, ...out];
}

function robots(site: Site, root: string): string {
  const deny = (site.config.robots?.disallow ?? []).map((path) => `Disallow: ${path}`);
  const lines: string[] = [];
  for (const agent of AGENTS) lines.push(`User-agent: ${agent}`, "Allow: /", "");
  lines.push("User-agent: *", "Allow: /", ...deny, "");
  lines.push(`Sitemap: ${root}/sitemap.xml`, "");
  return lines.join("\n");
}

function llms(site: Site, root: string, spec: Spec): string {
  const decl = site.config.llms ?? {};
  const known = new Set<string>();
  for (const route of site.routes) {
    known.add(route.route);
    for (const one of route.urls ?? []) known.add(one.route);
  }
  const rows = (list: Row[]) =>
    list
      .filter((one) => known.has(one.href.replace(/#.*/, "")))
      .map((one) => `- [${one.name ?? one.href}](${root}${one.href})${one.note ? `: ${one.note}` : ""}`)
      .join("\n");
  const sections = (spec.llms?.(site) ?? []).map((one) => [one.name, rows(one.rows)]).filter(([, list]) => list).map(([name, list]) => `## ${name}\n\n${list}`);
  const blocks = [`# ${site.config.title ?? site.config.name ?? ""}`, `> ${root}`, decl.about, decl.legend, rows(decl.links ?? []), ...sections];
  return `${blocks.filter(Boolean).join("\n\n")}\n`;
}

export const PACK = "@spa";

export async function globals(site: Site, spec: Spec, lean = false): Promise<Output[]> {
  const out: Output[] = [...site.copies];
  if (!lean) out.push(...(await packed(site, spec)));
  const root = clean(site.config.root as string);
  const maps = sitemaps(links(site, spec), root);
  out.push(...maps);
  out.push({ path: "robots.txt", bytes: robots(site, root) });
  if (site.config.llms) out.push({ path: "llms.txt", bytes: llms(site, root, spec) });
  if (site.config.manifest) out.push({ path: "manifest.webmanifest", bytes: JSON.stringify(site.config.manifest, null, 2) + "\n" });
  if (spec.icons) out.push(...icons(spec.icons));
  const wood = forest(site, spec.git);
  if (wood) out.push({ path: TREE, bytes: JSON.stringify(wood), type: "application/json" });
  const pub = site.config.inputs?.public ? site.input("public") : null;
  if (pub) for (const file of walk(pub.path)) out.push({ path: file.slice(pub.path.length + 1), bytes: bytes(file) });
  if (spec.globals) out.push(...(await spec.globals(site)));
  const paths = [...site.made, ...out.map((one) => one.path)];
  for (const one of maps) if (paths.indexOf(one.path) !== paths.lastIndexOf(one.path)) throw new Error(`ssg: ${one.path} is the sitemap's, and another output claims it`);
  return out;
}

/* GUARD */

const SCRIPT = /<script\b([^>]*)>([\s\S]*?)<\/script>/g;

const TYPED = /\btype\s*=\s*["']?([^"'\s>]*)/;

const RUNS = new Set(["", "module", "text/javascript", "application/javascript", "text/ecmascript", "application/ecmascript"]);

export function guard(path: string, html: string, known: Set<string>) {
  if (path.startsWith("raw/")) return;
  for (const [, attrs, body] of html.matchAll(SCRIPT)) {
    if (/\bsrc\s*=/.test(attrs)) continue;
    if (!RUNS.has((attrs.match(TYPED)?.[1] ?? "").trim().toLowerCase())) continue;
    if (known.has(body)) continue;
    throw new Error(`ssg: ${path} carries an inline script the boot list does not know: ${body.slice(0, 60)}`);
  }
}

/* WRITE */

const today = () => new Date().toISOString().slice(0, 10);

const year = () => String(new Date().getFullYear());

function loose(site: Site, route: Route, files: Set<string>): string[] {
  const held = route.inputs ?? (route.source ? [route.source] : []);
  return [...files]
    .filter((file) => !held.some((one) => file === one || file.startsWith(`${one}/`)))
    .map((file) => relative(site.root, file))
    .sort();
}

function put(out: string, item: Output): boolean {
  const path = join(out, item.path);
  const body = typeof item.bytes === "string" ? new TextEncoder().encode(item.bytes) : item.bytes;
  if (isFile(path)) {
    const old = new Uint8Array(readFileSync(path));
    if (old.length === body.length && Buffer.compare(old, body) === 0) return false;
  } else if (existsSync(path)) rmSync(path, { recursive: true });
  const dir = dirname(path);
  if (!existsSync(dir) || isFile(dir)) {
    for (let at = dir; at.length > out.length; at = dirname(at)) if (isFile(at)) unlinkSync(at);
    mkdirSync(dir, { recursive: true });
  }
  writeFileSync(path, body);
  return true;
}

/* BUILD */

function typed(outputs: Output[]): Record<string, string> | undefined {
  const out: Record<string, string> = {};
  for (const item of outputs) if (item.type) out[item.path] = item.type;
  return Object.keys(out).length ? out : undefined;
}

export async function build(spec: Spec, options: { manifest?: string; force?: boolean; verify?: boolean; tick?: (done: number, total: number) => void } = {}) {
  const site = await scan(spec);
  check(site, spec);
  await seal(site, spec);
  const walls = deeps(site.config);
  const path = options.manifest ? resolve(spec.root, options.manifest) : "";
  const old: Manifest = path && existsSync(path) ? JSON.parse(readFileSync(path, "utf8")) : {};
  const next: Manifest = {};
  const kept = new Set<string>();
  const known = new Set(spec.inline ?? []);
  const verify = options.verify ?? true;
  let rendered = 0;
  let written = 0;
  let walked = 0;
  for (const route of site.routes) {
    if (++walked % TICK === 0) options.tick?.(walked, site.routes.length);
    if (isGit(route) && !spec.git?.page) continue;
    if (isBlog(route) && !spec.blog?.page) continue;
    const first = fingerprint(site, route, spec, old[route.route]?.asks, old[route.route]?.reads);
    const was = old[route.route];
    const same = !options.force && !!was && was.hash === first;
    if (was && same && (!verify || was.outputs.every((p) => existsSync(join(site.out, p))))) {
      next[route.route] = was;
      route.at = route.at || was.at;
      fence(walls, was.outputs);
      for (const p of was.outputs) {
        kept.add(p);
        site.made.add(p);
      }
      continue;
    }
    site.asks = new Set();
    read = new Set();
    const outputs = await render(site, route, spec);
    const asks = [...site.asks].sort();
    const reads = loose(site, route, read);
    site.asks = undefined;
    read = null;
    const settled = asks.join("\n") === (was?.asks ?? []).join("\n") && reads.join("\n") === (was?.reads ?? []).join("\n");
    const hash = settled ? first : fingerprint(site, route, spec, asks, reads);
    fence(walls, outputs.map((o) => o.path));
    for (const item of outputs) {
      if (item.path.endsWith(".html")) guard(item.path, typeof item.bytes === "string" ? item.bytes : new TextDecoder().decode(item.bytes), known);
      if (put(site.out, item)) written++;
    }
    rendered++;
    const at = route.at || (was && same ? was.at : today());
    next[route.route] = { hash, at, outputs: outputs.map((o) => o.path), types: typed(outputs), asks: asks.length ? asks : undefined, reads: reads.length ? reads : undefined };
    route.at = at;
    for (const item of outputs) {
      kept.add(item.path);
      site.made.add(item.path);
    }
  }
  const shared = await globals(site, spec);
  fence(walls, shared.map((o) => o.path));
  for (const item of shared) {
    if (put(site.out, item)) written++;
    kept.add(item.path);
  }
  const pack = (await packed(site, spec)).map((item) => item.path);
  if (pack.length) next[PACK] = { hash: digest(pack).slice(0, 16), at: old[PACK]?.at ?? today(), outputs: pack };
  let removed = 0;
  const drop = (p: string) => {
    const file = join(site.out, p);
    if (!isFile(file)) return;
    unlinkSync(file);
    removed++;
    let dir = dirname(file);
    while (dir !== site.out && existsSync(dir) && readdirSync(dir).length === 0) {
      rmdirSync(dir);
      dir = dirname(dir);
    }
  };
  for (const record of Object.values(old)) for (const p of record.outputs) if (!kept.has(p)) drop(p);
  for (const out of new Set(decls(site.config).map(outDir))) {
    for (const file of walk(join(site.out, out))) {
      const p = relative(site.out, file);
      if (!kept.has(p)) drop(p);
    }
  }
  if (path) {
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, JSON.stringify(next, null, 2) + "\n");
  }
  return { site, manifest: next, rendered, written, removed, shared };
}

/* HELPERS */

export const page = (route: string) => (route === "/" ? "index.html" : `${route.replace(/^\/|\/$/g, "")}/index.html`);

const jsonText = (data: unknown) => JSON.stringify(data).replace(/</g, "\\u003c");

export const jsonScript = (data: unknown) => `<script type="application/ld+json">${jsonText(data)}</script>`;

export { escape, digest, short, today };
