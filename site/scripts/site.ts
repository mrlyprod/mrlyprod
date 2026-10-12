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
const MRLYJS = join(repo, "pkgs", "mrlyjs");
const APPS = join(org, "apps");
const BLOG = join(APPS, "blog");
const RESEARCH = join(repo, "research");
const SHELLS = ["app", "prose"].map((name) => join(org, "ui", `${name}.jsx`));
const LOCK = join(org, "lib", "lock.js");
const HASHED = /-[0-9a-z]{8}\.[^./]+$/;
const NAMING = { entry: "[name]-[hash].[ext]", chunk: "lib-[hash].[ext]", asset: "[name]-[hash].[ext]" };
const EXPORTS = JSON.parse(readFileSync(join(MRLYJS, "package.json"), "utf8")).exports as Record<string, { default?: string }>;

const read = (file: string) => readFileSync(file, "utf8");
const sha = (data: string | Uint8Array) => createHash("sha256").update(data).digest("hex");
const rel = (file: string) => relative(repo, file);

/* TYPES */

export type Row = { route: string; kind: "app" | "prose"; title: string; source: string | null; date: string | null; lead: string; figure: string | null; hidden: boolean; meta: Record<string, unknown> };

export type Out = { path: string; bytes: Uint8Array | string; type?: string; fixed?: boolean };

export type Menu = "play" | "read" | "tool";

export type App = { id: string; title: string; line: string; menu?: Menu; saver?: true; hidden?: true; deep?: true; source?: string; route?: string; page: string | null; icon: boolean };

type Post = { slug: string; route: string; title: string; date: string; lead: string; source: string };

export type Figure = { name: string; file: string };

export type Topic = { slug: string; title: string; figure: string | null; figures: string[] };

export type State = { rows: Row[]; site: Record<string, unknown>; figures: Figure[]; units: string[]; apps: App[]; posts: Post[]; topics: Topic[]; raw: string[]; dropped: number };

export type Ready = State & { boot: string; css: Record<string, string>; assets: Out[] };

/* TEXT */

const lede = (text: string, max = 200) => {
  const all = summary(text, Infinity);
  if (all.length <= max) return all;
  const cut = all.slice(0, max + 1).lastIndexOf(". ");
  return cut > 0 ? all.slice(0, cut + 1) : summary(text, max);
};

/* FIGURES */

const scanner = new Bun.Transpiler({ loader: "ts" });

const jsx = new Bun.Transpiler({ loader: "jsx" });

const UNITS = readFileSync(join(repo, "pkgs", "bridge", "units.txt"), "utf8").split(/\s+/).filter(Boolean);

const UNIT = /mrlyjs[/_]([a-z]+)/g;

const unitWasm = (unit: string) => join(org, "pkg", unit, `mrlyjs_${unit}_bg.wasm`);

const folders = (dir: string) => (existsSync(dir) ? readdirSync(dir, { withFileTypes: true }).filter((item) => item.isDirectory() && !item.name.startsWith(".")).map((item) => item.name).sort() : []);

const drawn = (dir: string) => (existsSync(dir) ? readdirSync(dir).filter((name) => name.endsWith(".ts") && !/\.(test|d)\.ts$/.test(name)).sort() : []);

export function roster(): Figure[] {
  const found = [
    ...folders(APPS).map((id) => ({ name: id, file: join(APPS, id, "icon.js") })),
    ...folders(BLOG).map((slug) => ({ name: `blog-${slug}`, file: join(BLOG, slug, "icon.js") })),
    ...folders(RESEARCH).flatMap((topic) => drawn(join(RESEARCH, topic, "figures")).map((name) => ({ name: name.slice(0, -3), file: join(RESEARCH, topic, "figures", name) }))),
  ].filter((one) => existsSync(one.file));
  const seen = new Map<string, string>();
  for (const one of found) {
    if (!NAME.test(one.name)) throw new Error(`site: ${rel(one.file)} draws the figure ${one.name}, and a figure name is lowercase words and digits joined by hyphens`);
    if (seen.has(one.name)) throw new Error(`site: two figures take the name ${one.name}: ${seen.get(one.name)} and ${rel(one.file)}`);
    seen.set(one.name, rel(one.file));
    if (!scanner.scan(read(one.file)).exports.includes("default")) throw new Error(`site: ${rel(one.file)} exports no default draw`);
  }
  return found.sort((a, b) => a.name.localeCompare(b.name));
}

export const mrlyjs: BunPlugin = {
  name: "mrlyjs",
  setup(build) {
    build.onResolve({ filter: /^mrlyjs\// }, ({ path }) => {
      const door = EXPORTS[`./${path.slice(7)}`]?.default;
      return door ? { path: join(MRLYJS, door) } : undefined;
    });
  },
};

/* SOURCES */

const walk = (dir: string): string[] =>
  readdirSync(dir, { withFileTypes: true }).flatMap((item) => (item.name.startsWith(".") ? [] : item.isDirectory() ? walk(join(dir, item.name)) : [join(dir, item.name)]));

const SLUG = /^[a-z0-9]+(-[a-z0-9]+)*$/;

function posts(): Post[] {
  return folders(BLOG)
    .map((slug) => {
      const file = join(BLOG, slug, "index.md");
      if (!SLUG.test(slug)) throw new Error(`site: apps/blog/${slug} is not a slug; use lowercase words and digits joined by hyphens`);
      if (!existsSync(file)) throw new Error(`site: apps/blog/${slug} has no index.md`);
      const { data } = front(read(file));
      for (const key of ["title", "date", "lead"]) if (!data[key]) throw new Error(`site: apps/blog/${slug} names no ${key} in its front matter`);
      if (!/^\d{4}-\d{2}-\d{2}$/.test(data.date!)) throw new Error(`site: apps/blog/${slug} dates itself ${data.date}; a date is YYYY-MM-DD`);
      return { slug, route: `/blog/${slug}/`, title: data.title!, date: data.date!, lead: lede(plain(data.lead!)), source: rel(file) };
    })
    .sort((a, b) => b.date.localeCompare(a.date) || a.slug.localeCompare(b.slug));
}

function topics(known: Set<string>): Topic[] {
  return folders(RESEARCH)
    .filter((slug) => slug !== "shared" && existsSync(join(RESEARCH, slug, "README.md")))
    .flatMap((slug) => {
      const { data } = front(read(join(RESEARCH, slug, "README.md")));
      if (!data.title) return [];
      if (!SLUG.test(slug)) throw new Error(`site: research/${slug} is not a slug; use lowercase words and digits joined by hyphens`);
      if (data.slug && data.slug !== slug) throw new Error(`site: research/${slug}/README.md names the slug ${data.slug}, and the folder is ${slug}`);
      if (data.figure && !known.has(data.figure)) throw new Error(`site: research/${slug}/README.md wears the figure ${data.figure}, which no roster file draws`);
      const figures = drawn(join(RESEARCH, slug, "figures")).map((name) => name.slice(0, -3)).filter((name) => known.has(name));
      return [{ slug, title: data.title, figure: data.figure || null, figures }];
    })
    .sort((a, b) => a.title.localeCompare(b.title) || a.slug.localeCompare(b.slug));
}

const MENUS = ["play", "read", "tool"];

const FIELDS = new Set(["title", "line", "menu", "saver", "hidden", "deep", "source", "route"]);

const PAGES = ["index.jsx", "page.md"];

function apps(): App[] {
  const list = folders(APPS).map((id): App => {
    const at = (...parts: string[]) => join(APPS, id, ...parts);
    if (!existsSync(at("app.json"))) throw new Error(`site: apps/${id}/ holds no app.json; every folder of apps/ is an app`);
    const one = JSON.parse(read(at("app.json"))) as Omit<App, "id" | "page" | "icon">;
    const say = (text: string) => new Error(`site: apps/${id}/app.json ${text}`);
    if (!/^[a-z][a-z0-9]*$/.test(id)) throw say("sits in a folder that is no root word; an id is lowercase letters and digits");
    const extra = Object.keys(one).find((key) => !FIELDS.has(key));
    if (extra) throw say(`sets ${extra}, and the fields are ${[...FIELDS].join(", ")}`);
    if (!one.title || !one.line) throw say("needs a title and a line");
    if (one.menu !== undefined && !MENUS.includes(one.menu)) throw say(`files itself under ${one.menu}, and the menus are ${MENUS.join(", ")}`);
    if (one.menu !== undefined && one.hidden) throw say(`is hidden and files itself under ${one.menu}; a hidden app is on no menu`);
    for (const key of ["saver", "hidden", "deep"] as const) if (one[key] !== undefined && one[key] !== true) throw say(`sets ${key} to ${one[key]}; it is true or absent`);
    if (one.route !== undefined && !one.route.startsWith("/")) throw say(`routes to ${one.route}, which is not rooted`);
    if (one.source !== undefined && !existsSync(join(repo, one.source))) throw say(`reads ${one.source}, which does not exist`);
    const pages = [...PAGES.filter((name) => existsSync(at(name))), ...(one.source ? ["source"] : [])];
    if (pages.length !== 1) throw say(`needs exactly one page of index.jsx, page.md or a source, and has ${pages.length ? pages.join(" and ") : "none"}`);
    const lost = one.saver ? ["scene.js", "widget.jsx"].find((name) => !existsSync(at(name))) : undefined;
    if (lost) throw say(`is a saver, and apps/${id}/${lost} is missing`);
    const icon = existsSync(at("icon.js"));
    if (!icon && !one.hidden) throw say("shows on the site, and apps/" + id + "/icon.js is missing; only a hidden app goes without an icon");
    return { id, ...one, page: pages[0] === "source" ? null : pages[0]!, icon };
  });
  if (!list.some((one) => one.id === "home")) throw new Error("site: apps/home/ is missing, and home is /");
  const desk = list.find((one) => one.id === SITE.desk);
  if (!desk?.saver) throw new Error(`site: site.json names the desk ${SITE.desk}, which is no saver`);
  return list.sort((a, b) => Number(b.id === "home") - Number(a.id === "home") || a.id.localeCompare(b.id));
}

export const savers = (list: App[]) => list.filter((one) => one.saver).sort((a, b) => Number(b.id === SITE.desk) - Number(a.id === SITE.desk) || a.title.localeCompare(b.title));

/* ROWS */

function rows(list: App[], blog: Post[]): Row[] {
  const out: Row[] = [];
  for (const app of list) {
    const route = app.route ?? (app.id === "home" ? "/" : `/${app.id}/`);
    const source = app.page === "page.md" ? rel(join(APPS, app.id, "page.md")) : (app.source ?? null);
    const meta = { id: app.id, ...(app.menu ? { menu: app.menu } : {}), ...(app.saver ? { saver: true } : {}), ...(app.deep ? { deep: true } : {}), ...(source ? { prose: app.source ? "source" : "page" } : {}) };
    out.push({ route, kind: app.page === "index.jsx" ? "app" : "prose", title: app.title, source, date: null, lead: lede(plain(app.line)), figure: app.icon ? app.id : null, hidden: Boolean(app.hidden), meta });
    if (app.id === "blog") for (const post of blog) out.push({ route: post.route, kind: "prose", title: post.title, source: post.source, date: post.date, lead: post.lead, figure: `blog-${post.slug}`, hidden: false, meta: { prose: "post", date: post.date } });
  }
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
  const research = topics(known);
  const all = rows(list, blog);
  const seen = new Set<string>();
  for (const row of all) {
    if (seen.has(row.route)) throw new Error(`site: two rows claim ${row.route}`);
    seen.add(row.route);
    if (row.figure && !known.has(row.figure)) throw new Error(`site: ${row.route} wears the figure ${row.figure}, which no roster file draws`);
    if (!row.source) continue;
    if (!existsSync(join(repo, row.source))) throw new Error(`site: ${row.route} reads ${row.source}, which does not exist`);
    for (const name of named(read(join(repo, row.source)))) if (!known.has(name)) throw new Error(`site: ${row.source} shows the figure ${name}, which no roster file draws`);
  }
  const code = [...figures.map((one) => one.file), ...walk(APPS).filter((file) => /\.[jt]sx?$/.test(file) && !/\.test\./.test(file))];
  const used = new Set(code.flatMap((file) => [...read(file).matchAll(UNIT)].map((hit) => hit[1]!)));
  const units = UNITS.filter((unit) => used.has(unit)).sort();
  for (const unit of units) if (!existsSync(unitWasm(unit))) throw new Error(`site: a live figure or app imports mrlyjs/${unit}, and site/pkg/${unit}/ does not hold it; scripts/wasm.sh copies the live units there`);
  const moves = JSON.parse(read(join(org, "redirects.json"))) as Record<string, { to: string; since: string }>;
  const kept = Object.fromEntries(Object.entries(moves).filter(([, row]) => seen.has(row.to)));
  const site = {
    ...Object.fromEntries(["title", "company", "tagline", "since", "prefix", "tint", "menu", "cart", "socials", "contact", "tree"].map((key) => [key, SITE[key as keyof typeof SITE]])),
    root,
    doors: doors(all.filter((row) => row.meta.id).map((row) => ({ name: row.title, href: row.route }))),
    savers: savers(list).map(({ id, title }) => ({ id, title })),
    redirects: kept,
    git: { slug: git.slug, branch: git.branch },
  };
  const raw = tree(git.root);
  held = { rows: all, site, figures, units, apps: list, posts: blog, topics: research, raw, dropped: Object.keys(moves).length - Object.keys(kept).length };
  return held;
}

/* PLUGIN */

const thunks = (list: [string, string][]) => `{ ${list.map(([key, file]) => `${JSON.stringify(key)}: () => import(${JSON.stringify(file)})`).join(", ")} }`;

const appFiles = (list: App[]): [string, string][] => list.flatMap((one) => (one.page === "index.jsx" ? [[one.id, join(APPS, one.id, "index.jsx")] as [string, string]] : []));

export const own = (list: App[]) => list.filter((one) => one.page === "index.jsx" && jsx.scan(read(join(APPS, one.id, "index.jsx"))).exports.includes("render")).map((one) => one.id);

export function modules(ready: Ready): Record<string, string> {
  return {
    "site:routes": `export const site = ${JSON.stringify(ready.site)};\nexport const rows = ${JSON.stringify(ready.rows)};`,
    "site:figures": [...ready.units.map((unit) => `import ${unit} from ${JSON.stringify(unitWasm(unit))};`), `export const units = { ${ready.units.join(", ")} };`, `export const figures = ${thunks(ready.figures.map((one) => [one.name, one.file]))};`].join("\n"),
    "site:apps": `export const apps = ${thunks(appFiles(ready.apps))};\nexport const own = new Set(${JSON.stringify(own(ready.apps))});\nexport const lock = () => import(${JSON.stringify(LOCK)});`,
    "site:css": `export const css = ${JSON.stringify(ready.css)};`,
    "apps:scenes": `export const scenes = ${thunks(savers(ready.apps).map((one) => [one.id, join(APPS, one.id, "scene.js")]))};`,
  };
}

export const plugin = (get: () => Ready): BunPlugin => ({
  name: "site",
  setup(build) {
    mrlyjs.setup(build);
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
    { path: "llms.txt", bytes: llms(SITE.title, root, SITE.llms, ready.rows, new Set(ready.raw), ready.topics) },
    { path: "research.json", bytes: `${JSON.stringify({ topics: ready.topics })}\n` },
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
  const entries = [join(org, "ui", "index.html"), ...SHELLS, join(org, "lib", "git.js"), ...ready.figures.map((one) => one.file), ...appFiles(ready.apps).map(([, file]) => file), LOCK];
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
