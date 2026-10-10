import type { BuildArtifact, BunPlugin } from "bun";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { config as gitConfig, forest, mime, tree, TREE } from "../kit/git/git.ts";
import { HUGE, IMAGE, ext, reads } from "../kit/git/view.ts";
import { figures as named, front, NAME, plain, summary } from "../kit/md/md.ts";
import { escape } from "../kit/md/text.ts";
import { doors } from "../ui/word.js";
import SITE from "../site.json";
import { llms } from "./map.ts";

/* PATHS */

const org = resolve(import.meta.dir, "..");
const repo = resolve(org, "..");
const DIST = process.env.MRLY_DIST ? resolve(process.env.MRLY_DIST) : join(org, "dist");
const root = (process.env.MRLY_SITE ?? SITE.root).replace(/\/$/, "");
const DRAWS = join(repo, "figures");
const MRLYJS = join(repo, "pkgs", "mrlyjs");
const APPS = join(org, "apps");
const PAGES = join(org, "ui", "pages");
const LOCK = join(org, "lib", "lock.js");
const ARCHIVE = "research/";
const HASHED = /-[0-9a-z]{8}\.[^./]+$/;
const NAMING = { entry: "[name]-[hash].[ext]", chunk: "lib-[hash].[ext]", asset: "[name]-[hash].[ext]" };
const EXPORTS = JSON.parse(readFileSync(join(MRLYJS, "package.json"), "utf8")).exports as Record<string, { default?: string }>;

const read = (file: string) => readFileSync(file, "utf8");
const sha = (data: string | Uint8Array) => createHash("sha256").update(data).digest("hex");
const rel = (file: string) => relative(repo, file);

/* TYPES */

export type Row = { route: string; kind: string; title: string; source: string | null; date: string | null; lead: string; figure: string; hidden: boolean; meta: Record<string, unknown> };

export type Out = { path: string; bytes: Uint8Array | string; type?: string; fixed?: boolean };

type App = { id: string; title: string; kind: string; line?: string };

type Post = { slug: string; route: string; title: string; date: string; lead: string; figure: string; source: string };

type Figure = { name: string; file: string };

export type State = { rows: Row[]; site: Record<string, unknown>; figures: Figure[]; units: string[]; apps: App[]; posts: Post[]; raw: string[]; dropped: number };

export type Ready = State & { boot: string; css: Record<string, string>; assets: Out[] };

/* TEXT */

const lede = (text: string, max = 200) => {
  const all = summary(text, Infinity);
  if (all.length <= max) return all;
  const cut = all.slice(0, max + 1).lastIndexOf(". ");
  return cut > 0 ? all.slice(0, cut + 1) : summary(text, max);
};

const PLAIN = "site-page";

const FIXED: Record<string, string> = { "/": "site-home", "/mrlymath/": "site-math", "/git/": "site-code", "/about/": "site-icon", "/contact/": "site-contact", "/donate/": "site-donate", "/method/": "site-method", "/blog/": "site-blog", "/stats/": "site-stats", "/cart/": "site-cart", "/menu/": "site-menu" };

const PAUSED = [
  { route: "/research/", door: "Research", figure: "site-research" },
  { route: "/research/wiki/", door: "Wiki", figure: "site-wiki" },
  { route: "/demos/", door: "Demos", figure: "site-demos" },
];

const LEADS = {
  blog: "Notes on what lands on this site and in the crates behind it.",
  menu: "Every door of mrly.net.",
  cart: "Coming soon.",
  stats: "The CDN, the Lambdas and the bucket, read from the bucket every minute.",
  math: "A design is one string: one JSON object per named thing, and every other name a view cut from it.",
  settings: "Theme, font, tint and saver, kept in this browser.",
  saver: "A screensaver drawn in the browser from a seed; its url is the share link.",
  paused: "Being rebuilt.",
  missing: "That page does not exist.",
  moved: "Pages that moved, and where they live now. An old link still works: it answers with a permanent redirect to the new page.",
  figures: "Every live figure, drawn with the time it took.",
};

/* FIGURES */

const scanner = new Bun.Transpiler({ loader: "ts" });

const UNITS = readFileSync(join(repo, "pkgs", "bridge", "units.txt"), "utf8").split(/\s+/).filter(Boolean);

const UNIT = /mrlyjs[/_]([a-z]+)/g;

const unitWasm = (unit: string) => join(org, "pkg", unit, `mrlyjs_${unit}_bg.wasm`);

export function roster(): Figure[] {
  return readdirSync(DRAWS)
    .filter((name) => name.endsWith(".ts") && !/\.(test|d)\.ts$/.test(name) && NAME.test(name.slice(0, -3)))
    .sort()
    .flatMap((name) => {
      const file = join(DRAWS, name);
      return scanner.scan(read(file)).exports.includes("default") ? [{ name: name.slice(0, -3), file }] : [];
    });
}

/* SOURCES */

const walk = (dir: string): string[] =>
  readdirSync(dir, { withFileTypes: true }).flatMap((item) => (item.name.startsWith(".") ? [] : item.isDirectory() ? walk(join(dir, item.name)) : [join(dir, item.name)]));

const SLUG = /^[a-z0-9]+(-[a-z0-9]+)*$/;

function posts(): Post[] {
  const home = join(org, "blog");
  const loose = readdirSync(home, { withFileTypes: true }).find((item) => !item.isDirectory() && !item.name.startsWith("."));
  if (loose) throw new Error(`site: blog/${loose.name} sits loose; a post is blog/<slug>/index.md with its files beside it`);
  return readdirSync(home, { withFileTypes: true })
    .filter((item) => item.isDirectory())
    .map(({ name: slug }) => {
      const dir = join(home, slug);
      const file = join(dir, "index.md");
      if (!SLUG.test(slug)) throw new Error(`site: blog/${slug} is not a slug; use lowercase words and digits joined by hyphens`);
      if (!existsSync(file)) throw new Error(`site: blog/${slug} has no index.md`);
      const { data } = front(read(file));
      for (const key of ["title", "date", "lead"]) if (!data[key]) throw new Error(`site: blog/${slug} names no ${key} in its front matter`);
      if (!/^\d{4}-\d{2}-\d{2}$/.test(data.date!)) throw new Error(`site: blog/${slug} dates itself ${data.date}; a date is YYYY-MM-DD`);
      return { slug, route: `/blog/${slug}/`, title: data.title!, date: data.date!, lead: lede(plain(data.lead!)), figure: data.figure || `blog-${slug}`, source: rel(file) };
    })
    .sort((a, b) => b.date.localeCompare(a.date) || a.slug.localeCompare(b.slug));
}

function apps(): App[] {
  const list = JSON.parse(read(join(APPS, "apps.json"))) as App[];
  for (const one of list) {
    if (one.kind === "tool") {
      if (one.id !== "settings") throw new Error(`site: apps/apps.json names the tool ${one.id}, and settings is the only tool`);
      continue;
    }
    if (one.kind !== "saver" && one.kind !== "app") throw new Error(`site: apps/apps.json gives ${one.id} the kind ${one.kind}, and the kinds are tool, saver, app`);
    const lost = ["scene.js", "widget.jsx", "index.jsx"].find((part) => !existsSync(join(APPS, one.id, part)));
    if (lost) throw new Error(`site: apps/apps.json names ${one.id}, and apps/${one.id}/${lost} is missing`);
  }
  return list;
}

/* ROWS */

function rows(list: App[], blog: Post[], git: string): Row[] {
  const out: Row[] = [];
  const row = (route: string, kind: string, title: string, more: Partial<Row> = {}) =>
    out.push({ route, kind, title, source: null, date: null, figure: FIXED[route] ?? PLAIN, hidden: false, meta: {}, ...more, lead: lede(more.lead ?? "") });
  row("/", "home", SITE.title, { lead: SITE.tagline });
  for (const file of readdirSync(join(org, "pages")).filter((name) => name.endsWith(".md")).sort()) {
    const slug = basename(file, ".md");
    const { data, body } = front(read(join(org, "pages", file)));
    row(`/${slug}/`, "prose", data.title ?? slug, { source: `site/pages/${file}`, lead: lede(data.lead ? plain(data.lead) : body), figure: data.figure || FIXED[`/${slug}/`] || PLAIN, meta: { was: "page" } });
  }
  row("/blog/", "hub", "Blog", { lead: LEADS.blog, meta: { was: "blog" } });
  for (const post of blog) row(post.route, "prose", post.title, { source: post.source, date: post.date, lead: post.lead, figure: post.figure, meta: { was: "post", date: post.date } });
  for (const app of list) {
    const tool = app.kind === "tool";
    row(`/${app.id}/`, tool ? app.id : "app", app.title, { lead: app.line ?? (tool ? LEADS.settings : app.kind === "saver" ? LEADS.saver : app.title), figure: `app-${app.id}`, meta: { id: app.id, kind: app.kind, line: app.line ?? null } });
  }
  row("/menu/", "hub", "Menu", { lead: LEADS.menu, meta: { was: "menu" } });
  row("/git/", "git", "Code", { lead: `The source of ${git}: every tracked file, browsable, with its raw bytes under /raw/.`, meta: { deep: true } });
  row("/mrlymath/", "prose", "MrlyMath", { source: "pkgs/mrlyrs/NAMES.md", lead: LEADS.math, meta: { was: "math" } });
  for (const one of PAUSED) row(one.route, "paused", one.door, { lead: LEADS.paused, figure: one.figure, meta: { door: one.door } });
  row("/cart/", "cart", "Cart", { lead: LEADS.cart, hidden: true });
  row("/stats/", "stats", "Stats", { lead: LEADS.stats, hidden: true });
  row("/redirects/", "moved", "Redirects", { lead: LEADS.moved, hidden: true });
  row("/404.html", "missing", "Nothing here", { lead: LEADS.missing, hidden: true });
  row("/figures/", "figures", "Figures", { lead: LEADS.figures, hidden: true });
  return out;
}

/* STATE */

let held: State | null = null;

export function state(): State {
  if (held) return held;
  const git = gitConfig(org, SITE.git)!;
  const figures = roster();
  const known = new Set(figures.map((one) => one.name));
  const list = apps();
  const blog = posts();
  const all = rows(list, blog, git.name);
  const seen = new Set<string>();
  for (const row of all) {
    if (seen.has(row.route)) throw new Error(`site: two rows claim ${row.route}`);
    seen.add(row.route);
    if (!known.has(row.figure)) throw new Error(`site: ${row.route} wears the figure ${row.figure}, and figures/${row.figure}.ts is no live figure`);
    if (!row.source) continue;
    if (!existsSync(join(repo, row.source))) throw new Error(`site: ${row.route} reads ${row.source}, which does not exist`);
    for (const name of named(read(join(repo, row.source)))) if (!known.has(name)) throw new Error(`site: ${row.source} shows the figure ${name}, and figures/${name}.ts is no live figure`);
  }
  const code = [...figures.map((one) => one.file), ...walk(APPS).filter((file) => /\.[jt]sx?$/.test(file) && !/\.test\./.test(file))];
  const used = new Set(code.flatMap((file) => [...read(file).matchAll(UNIT)].map((hit) => hit[1]!)));
  const units = UNITS.filter((unit) => used.has(unit)).sort();
  for (const unit of units) if (!existsSync(unitWasm(unit))) throw new Error(`site: a live figure or app imports mrlyjs/${unit}, and site/pkg/${unit}/ does not hold it; scripts/wasm.sh copies the live units there`);
  const moves = JSON.parse(read(join(org, "redirects.json"))) as Record<string, { to: string; since: string }>;
  const kept = Object.fromEntries(Object.entries(moves).filter(([, row]) => seen.has(row.to)));
  const site = {
    ...Object.fromEntries(["title", "company", "tagline", "since", "prefix", "tint", "menu", "cart", "socials", "contact", "tree", "heroes", "tiles"].map((key) => [key, SITE[key as keyof typeof SITE]])),
    root,
    doors: doors(SITE.tree),
    apps: list,
    redirects: kept,
    git: { slug: git.slug, branch: git.branch },
  };
  const raw = tree(git.root).filter((path) => !path.startsWith(ARCHIVE));
  held = { rows: all, site, figures, units, apps: list, posts: blog, raw, dropped: Object.keys(moves).length - Object.keys(kept).length };
  return held;
}

/* PLUGIN */

const thunks = (list: [string, string][]) => `{ ${list.map(([key, file]) => `${JSON.stringify(key)}: () => import(${JSON.stringify(file)})`).join(", ")} }`;

const pageFiles = (): [string, string][] => (existsSync(PAGES) ? readdirSync(PAGES).filter((name) => name.endsWith(".jsx")).sort().map((name) => [name.slice(0, -4), join(PAGES, name)]) : []);

const appFiles = (list: App[]): [string, string][] => list.flatMap((one) => (existsSync(join(APPS, one.id, "index.jsx")) ? [[one.id, join(APPS, one.id, "index.jsx")] as [string, string]] : []));

export function modules(ready: Ready): Record<string, string> {
  return {
    "site:routes": `export const site = ${JSON.stringify(ready.site)};\nexport const rows = ${JSON.stringify(ready.rows)};`,
    "site:pages": `export const pages = ${thunks(pageFiles())};`,
    "site:figures": [...ready.units.map((unit) => `import ${unit} from ${JSON.stringify(unitWasm(unit))};`), `export const units = { ${ready.units.join(", ")} };`, `export const figures = ${thunks(ready.figures.map((one) => [one.name, one.file]))};`].join("\n"),
    "site:apps": `export const apps = ${thunks(appFiles(ready.apps))};\nexport const lock = () => import(${JSON.stringify(LOCK)});`,
    "site:css": `export const css = ${JSON.stringify(ready.css)};`,
    "apps:scenes": `export const scenes = ${thunks(ready.apps.filter((one) => one.kind === "saver").map((one) => [one.id, join(APPS, one.id, "scene.js")]))};`,
  };
}

export const plugin = (get: () => Ready): BunPlugin => ({
  name: "site",
  setup(build) {
    build.onResolve({ filter: /^mrlyjs\// }, ({ path }) => {
      const door = EXPORTS[`./${path.slice(7)}`]?.default;
      return door ? { path: join(MRLYJS, door) } : undefined;
    });
    build.onResolve({ filter: /(?:^|\/)pkg\/[a-z]+\/mrlyjs_[a-z]+(?:_bg\.wasm|\.js)$/ }, ({ path }) => ({ path: join(org, "pkg", ...path.split("/").slice(-2)) }));
    build.onResolve({ filter: /^\// }, ({ path }) => (existsSync(path) ? undefined : { path, external: true }));
    build.onResolve({ filter: /^(site|apps):/ }, ({ path }) => ({ path, namespace: "site" }));
    build.onLoad({ filter: /.*/, namespace: "site" }, ({ path }) => {
      const text = modules(get())[path];
      if (text === undefined) throw new Error(`site: no virtual module ${path}`);
      return { contents: text, loader: "js" };
    });
  },
});

const fonts = (seen: Map<string, string>): BunPlugin => ({
  name: "fonts",
  setup(build) {
    build.onResolve({ filter: /\.woff2$/ }, ({ path, importer }) => {
      const file = resolve(dirname(importer), path);
      if (!seen.has(file)) seen.set(file, `/fonts/${basename(file, ".woff2")}-${sha(readFileSync(file)).slice(0, 8)}.woff2`);
      return { path: seen.get(file)!, external: true };
    });
  },
});

/* PRIME */

async function outs(list: BuildArtifact[]): Promise<Out[]> {
  return Promise.all(list.map(async (one) => ({ path: one.path.replace(/^\.\//, ""), bytes: new Uint8Array(await one.arrayBuffer()) })));
}

const LICENSES = [...readdirSync(join(org, "ui", "fonts")).filter((name) => name.startsWith("LICENSE-")).map((name) => join(org, "ui", "fonts", name)), join(org, "kit", "code", "seti", "LICENSE-seti.txt")];

const faces = (seen: Map<string, string>): Out[] => [...seen].map(([file, url]) => ({ path: url.slice(1), bytes: readFileSync(file) }));

const licenses = (): Out[] => LICENSES.map((file) => ({ path: `fonts/${basename(file)}`, bytes: readFileSync(file) }));

async function prime(seen: Map<string, string>) {
  const boot = join(org, "ui", "boot.js");
  if (scanner.scanImports(read(boot)).length) throw new Error("site: ui/boot.js imports nothing; it runs alone as the blocking classic script of the shell");
  const sheets = { git: join(org, "ui", "git.css"), mrly: join(org, "lib", "mrly.css") };
  const built = await Bun.build({ entrypoints: [boot, ...Object.values(sheets)], minify: true, publicPath: "/", naming: NAMING, plugins: [fonts(seen)] });
  if (!built.success) throw new Error(`site: the boot script and the sheets failed to bundle\n${built.logs.join("\n")}`);
  const made = await outs(built.outputs);
  const find = (stem: string, ext: string) => made.find((one) => one.path.startsWith(`${stem}-`) && one.path.endsWith(ext))!.path;
  return { boot: `/${find("boot", ".js")}`, css: Object.fromEntries(Object.keys(sheets).map((name) => [name, `/${find(name, ".css")}`])), assets: made };
}

let primed: Promise<Ready> | null = null;

let done: Ready | null = null;

export function ready(): Promise<Ready> {
  const seen = new Map<string, string>();
  return (primed ??= prime(seen).then((made) => (done = { ...state(), ...made, assets: [...made.assets, ...faces(seen), ...licenses()] })));
}

export default plugin(() => {
  if (!done) throw new Error("site: await ready() before a bundle asks for a site: module; a Bun.build inside a plugin never returns");
  return done;
});

/* GLOBALS */

const XML = `<?xml version="1.0" encoding="UTF-8"?>\n`;

const SCHEMA = "http://www.sitemaps.org/schemas/sitemap/0.9";

const AGENTS = ["GPTBot", "ClaudeBot", "Claude-Web", "CCBot", "Google-Extended", "anthropic-ai", "PerplexityBot"];

type Url = { route: string; date: string | null };

function sitemaps(urls: Url[]): Out[] {
  const segment = (route: string) => route.split("/")[1]!;
  const count = new Map<string, number>();
  for (const one of urls) count.set(segment(one.route), (count.get(segment(one.route)) ?? 0) + 1);
  if ((count.get("pages") ?? 0) > 1) throw new Error("site: /pages/ holds more than one url, and sitemap-pages.xml is the lone pages' map");
  const groups = new Map<string, Url[]>();
  for (const one of urls) {
    const name = count.get(segment(one.route))! > 1 ? segment(one.route) : "pages";
    groups.set(name, [...(groups.get(name) ?? []), one]);
  }
  const last = (list: Url[]) => list.map((one) => one.date ?? "").reduce((a, b) => (b > a ? b : a), "");
  const mod = (date: string | null) => (date ? `<lastmod>${date}</lastmod>` : "");
  const names = [...groups.keys()].sort();
  const maps = names.map((name) => ({ path: `sitemap-${name}.xml`, bytes: `${XML}<urlset xmlns="${SCHEMA}">\n${groups.get(name)!.map((one) => `<url><loc>${escape(root + encodeURI(one.route))}</loc>${mod(one.date)}</url>`).join("\n")}\n</urlset>\n` }));
  const index = names.map((name) => `<sitemap><loc>${escape(`${root}/sitemap-${name}.xml`)}</loc>${mod(last(groups.get(name)!) || null)}</sitemap>`);
  return [{ path: "sitemap.xml", bytes: `${XML}<sitemapindex xmlns="${SCHEMA}">\n${index.join("\n")}\n</sitemapindex>\n` }, ...maps];
}

function robots() {
  const lines = AGENTS.flatMap((agent) => [`User-agent: ${agent}`, "Allow: /", ""]);
  return [...lines, "User-agent: *", "Allow: /", "", `Sitemap: ${root}/sitemap.xml`, ""].join("\n");
}

export function globals(ready: State): Out[] {
  const git = gitConfig(org, SITE.git)!;
  const shown = ready.rows.filter((row) => !row.hidden);
  return [
    { path: "routes.json", bytes: `${JSON.stringify({ site: ready.site, rows: ready.rows })}\n` },
    { path: "llms.txt", bytes: llms(SITE.title, root, SITE.llms, ready.rows, new Set(ready.raw)) },
    ...sitemaps([...shown.map((row) => ({ route: row.route, date: row.date })), ...ready.raw.map((path) => ({ route: `/raw/${path}`, date: null }))]),
    { path: "robots.txt", bytes: robots() },
    { path: TREE, bytes: JSON.stringify(forest(git, ready.raw)), type: "application/json" },
    { path: "manifest.webmanifest", bytes: `${JSON.stringify(SITE.manifest, null, 2)}\n` },
    { path: "redirects.json", bytes: `${JSON.stringify(ready.site.redirects, null, 2)}\n` },
  ];
}

/* COPIES */

export const typed = (path: string, bytes: Uint8Array) => mime(path, bytes.length <= HUGE && !IMAGE.has(ext(path)) && ext(path) !== "pdf" && reads(bytes) !== null);

function copies(ready: State): Out[] {
  const raw = ready.raw.map((path) => {
    const bytes = readFileSync(join(repo, path));
    return { path: `raw/${path}`, bytes, type: typed(path, bytes) };
  });
  const pub = walk(join(org, "public")).map((file) => ({ path: relative(join(org, "public"), file), bytes: readFileSync(file) }));
  return [...raw, ...pub];
}

/* BUILD */

export type Built = { out: string; rows: Row[]; files: { path: string; hash: string; type?: string; fixed?: boolean }[]; raw: number; dropped: number; units: string[] };

export async function build(out = DIST): Promise<Built> {
  const seen = new Map<string, string>();
  const made = await prime(seen);
  const ready: Ready = { ...state(), ...made };
  const entries = [join(org, "ui", "index.html"), ...pageFiles().map(([, file]) => file), join(org, "lib", "git.js"), ...ready.figures.map((one) => one.file), ...appFiles(ready.apps).map(([, file]) => file), LOCK];
  const built = await Bun.build({ entrypoints: entries, splitting: true, minify: true, publicPath: "/", naming: NAMING, define: { "process.env.NODE_ENV": '"production"' }, plugins: [plugin(() => ready), fonts(seen)] });
  if (!built.success) throw new Error(`site: the bundle failed\n${built.logs.join("\n")}`);
  const bundle = await outs(built.outputs);
  const page = bundle.find((one) => one.path.endsWith(".html"))!;
  const shell = new TextDecoder().decode(page.bytes as Uint8Array);
  if (!shell.includes('<script src="/boot.js"></script>')) throw new Error("site: ui/index.html must load /boot.js as its first, classic script");
  page.path = "index.html";
  page.bytes = shell.replace('<script src="/boot.js"></script>', `<script src="${made.boot}"></script>`);
  const loose = [...bundle, ...made.assets].filter((one) => one !== page && !HASHED.test(one.path));
  if (loose.length) throw new Error(`site: the bundle named ${loose.map((one) => one.path).join(", ")} without a hash`);
  const hashed = [...faces(seen), ...made.assets, ...bundle.filter((one) => one !== page)].map((one) => ({ ...one, fixed: true }));
  const all: Out[] = [...copies(ready), ...globals(ready), ...licenses(), ...hashed, page, { path: "404.html", bytes: page.bytes }];
  rmSync(out, { recursive: true, force: true });
  const files = new Map<string, Built["files"][number]>();
  for (const one of all) {
    const file = join(out, one.path);
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, one.bytes);
    files.set(one.path, { path: one.path, hash: sha(one.bytes).slice(0, 16), type: one.type, fixed: one.fixed });
  }
  return { out, rows: ready.rows, files: [...files.values()], raw: ready.raw.length, dropped: ready.dropped, units: ready.units };
}

/* MAIN */

if (import.meta.main) {
  const done = await build();
  const size = done.files.reduce((sum, one) => sum + statSync(join(done.out, one.path)).size, 0);
  console.log(`site: ${done.rows.length} rows, ${done.raw} raw files, ${done.files.length} files (${(size / 1e6).toFixed(1)} MB) in ${done.out.startsWith(`${org}/`) ? relative(org, done.out) : done.out}, wasm ${done.units.join(" ")}, ${done.dropped} redirects dropped for a target that is no route`);
}
