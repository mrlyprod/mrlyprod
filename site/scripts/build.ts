import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import type { BuildArtifact, BunPlugin } from "bun";
import { rawKind } from "../infra/mime.ts";
import SITE from "../site.json";
import { appicon } from "../src/lib/appicon.ts";
import { ext, HUGE, IMAGE, reads } from "../src/lib/git/view.ts";
import { front, NAME, figures as named, plain, summary } from "../src/lib/md/md.ts";
import { esc } from "../src/lib/md/text.ts";
import { rng } from "../src/lib/rand.ts";
import { tintCss } from "../src/lib/theme/tint.ts";
import { forest, config as gitConfig, TREE, tree } from "./git.ts";
import { llms, type Shelf } from "./llms.ts";

/* NAMES */

export const NAMES = {
  index: "index.tsx",
  scene: "scene.ts",
  widget: "widget.tsx",
  icon: "icon.ts",
  page: "page.tsx",
  lock: "lock.ts",
  tiles: "tiles.ts",
  boot: "boot.ts",
  app: "app.tsx",
  prose: "prose.tsx",
  git: "git.ts",
} as const;

type Name = keyof typeof NAMES;

const under = (dir: string, name: Name) => join(dir, NAMES[name]);

export const spells = <N extends Name>(file: string | null, name: N): file is (typeof NAMES)[N] => file === NAMES[name];

/* PATHS */

const org = resolve(import.meta.dir, "..");
const repo = resolve(org, "..");
const DIST = process.env.MRLY_DIST ? resolve(process.env.MRLY_DIST) : join(org, "dist");
const root = (process.env.MRLY_SITE ?? SITE.root).replace(/\/$/, "");
const MRLY = join(repo, "bindings", "js");
const NPM = "@mrlyprod/mrly";
const ROOTS = ["pages", "apps"].map((name) => join(org, "src", name));
const BLOG = join(org, "src", "pages", "blog");
const RESEARCH = join(repo, "research");
const LAUNCHPAD = join(org, "launchpad.json");
const shellFile = (name: Name) => under(join(org, "src", "shell"), name);
const SHELLS = [shellFile("app"), shellFile("prose")];
const LOCK = shellFile("lock");
const TINT = '<style id="tint"></style>';
const HASHED = /-[0-9a-z]{8}\.[^./]+$/;
const NAMING = { entry: "[name]-[hash].[ext]", chunk: "lib-[hash].[ext]", asset: "[name]-[hash].[ext]" };
const MANIFEST = JSON.parse(readFileSync(join(MRLY, "package.json"), "utf8")) as {
  version: string;
  exports: Record<string, { default?: string }>;
};
const EXPORTS = MANIFEST.exports;
const IMPORTS = JSON.parse(readFileSync(join(org, "package.json"), "utf8")).imports as Record<string, string>;

const read = (file: string) => readFileSync(file, "utf8");
const sha = (data: string | Uint8Array) => createHash("sha256").update(data).digest("hex");
const rel = (file: string) => relative(repo, file);

/* TYPES */

export type Row = {
  route: string;
  kind: "app" | "prose";
  title: string;
  source: string | null;
  date: string | null;
  lead: string;
  figure: string | null;
  hidden: boolean;
  meta: { id?: string; scene?: true; deep?: true; prose?: "source" | "page" | "post"; date?: string };
};

export type Out = { path: string; bytes: Uint8Array | string; type?: string; fixed?: boolean };

export type App = {
  id: string;
  dir: string;
  title: string;
  line: string;
  saver?: true;
  hidden?: true;
  deep?: true;
  source?: string;
  route?: string;
  page: string | null;
  icon: boolean;
};

type Post = { slug: string; route: string; title: string; date: string; lead: string; source: string };

export type Figure = { name: string; file: string };

export type Topic = {
  slug: string;
  title: string;
  date: string;
  lead: string;
  figure: string | null;
  figures: string[];
};

export type State = {
  rows: Row[];
  site: Record<string, unknown>;
  figures: Figure[];
  units: string[];
  apps: App[];
  posts: Post[];
  topics: Topic[];
  raw: string[];
  dropped: number;
};

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

const scanners = new Map<string, Bun.Transpiler>([
  ["js", jsx],
  ["jsx", jsx],
  ["ts", scanner],
  ["tsx", new Bun.Transpiler({ loader: "tsx" })],
]);

const scan = (file: string) => (scanners.get(ext(file)) ?? scanner).scan(read(file));

const UNITS = readdirSync(join(MRLY, "units")).sort();

const UNIT = /(?:@mrlyprod\/mrly\/|mrly_js_)([a-z]+)/g;

const unitWasm = (unit: string) => join(org, "wasm", unit, `mrly_js_${unit}_bg.wasm`);

export const folders = (dir: string) =>
  existsSync(dir)
    ? readdirSync(dir, { withFileTypes: true })
        .filter((item) => item.isDirectory() && !item.name.startsWith("."))
        .map((item) => item.name)
        .sort()
    : [];

const drawn = (dir: string) =>
  existsSync(dir)
    ? readdirSync(dir)
        .filter((name) => name.endsWith(".ts") && !/\.(test|d)\.ts$/.test(name))
        .sort()
    : [];

const places = () =>
  ROOTS.flatMap((root) => folders(root).map((id) => ({ id, dir: join(root, id) }))).sort((a, b) =>
    a.id.localeCompare(b.id),
  );

export const plates = (figures: Figure[]) =>
  figures.filter((one) => ROOTS.includes(dirname(dirname(one.file)))).map((one) => one.name);

export function roster(): Figure[] {
  const found = [
    ...places().map(({ id, dir }) => ({ name: id, file: under(dir, "icon") })),
    ...folders(BLOG).map((slug) => ({ name: `blog-${slug}`, file: under(join(BLOG, slug), "icon") })),
    ...folders(RESEARCH).flatMap((topic) =>
      drawn(join(RESEARCH, topic, "figures")).map((name) => ({
        name: name.slice(0, -3),
        file: join(RESEARCH, topic, "figures", name),
      })),
    ),
  ].filter((one) => existsSync(one.file));
  const seen = new Map<string, string>();
  for (const one of found) {
    if (!NAME.test(one.name))
      throw new Error(
        `site: ${rel(one.file)} draws the figure ${one.name}, and a figure name is lowercase words and digits joined by hyphens`,
      );
    if (seen.has(one.name))
      throw new Error(`site: two figures take the name ${one.name}: ${seen.get(one.name)} and ${rel(one.file)}`);
    seen.set(one.name, rel(one.file));
    if (!scan(one.file).exports.includes("default")) throw new Error(`site: ${rel(one.file)} exports no default draw`);
  }
  return found.sort((a, b) => a.name.localeCompare(b.name));
}

export const mrly: BunPlugin = {
  name: "mrly",
  setup(build) {
    build.onResolve({ filter: /^@mrlyprod\/mrly(?:\/|$)/ }, ({ path }) => {
      const door = EXPORTS[`.${path.slice(NPM.length)}`]?.default;
      return door ? { path: join(MRLY, door) } : undefined;
    });
    build.onResolve({ filter: /^#view(?:\/|$)/ }, ({ path }) => {
      const rest = path.slice("#view/".length);
      const to = rest ? IMPORTS["#view/*"]?.replace("*", rest) : IMPORTS["#view"];
      return to ? { path: join(org, to) } : undefined;
    });
  },
};

/* SOURCES */

const walk = (dir: string): string[] =>
  readdirSync(dir, { withFileTypes: true }).flatMap((item) =>
    item.name.startsWith(".") ? [] : item.isDirectory() ? walk(join(dir, item.name)) : [join(dir, item.name)],
  );

const SLUG = /^[a-z0-9]+(-[a-z0-9]+)*$/;

function posts(): Post[] {
  return folders(BLOG)
    .map((slug) => {
      const file = join(BLOG, slug, "index.md");
      if (!SLUG.test(slug))
        throw new Error(`site: src/pages/blog/${slug} is not a slug; use lowercase words and digits joined by hyphens`);
      if (!existsSync(file)) throw new Error(`site: src/pages/blog/${slug} has no index.md`);
      const { data } = front(read(file));
      for (const key of ["title", "date", "lead"])
        if (!data[key]) throw new Error(`site: src/pages/blog/${slug} names no ${key} in its front matter`);
      if (!/^\d{4}-\d{2}-\d{2}$/.test(data.date!))
        throw new Error(`site: src/pages/blog/${slug} dates itself ${data.date}; a date is YYYY-MM-DD`);
      return {
        slug,
        route: `/blog/${slug}/`,
        title: data.title!,
        date: data.date!,
        lead: lede(plain(data.lead!)),
        source: rel(file),
      };
    })
    .sort((a, b) => b.date.localeCompare(a.date) || a.slug.localeCompare(b.slug));
}

function topics(known: Set<string>): Topic[] {
  return folders(RESEARCH)
    .filter((slug) => slug !== "shared" && existsSync(join(RESEARCH, slug, "README.md")))
    .flatMap((slug) => {
      const { data } = front(read(join(RESEARCH, slug, "README.md")));
      if (!data.title) return [];
      if (!SLUG.test(slug))
        throw new Error(`site: research/${slug} is not a slug; use lowercase words and digits joined by hyphens`);
      if (data.slug && data.slug !== slug)
        throw new Error(`site: research/${slug}/README.md names the slug ${data.slug}, and the folder is ${slug}`);
      if (data.figure && !known.has(data.figure))
        throw new Error(`site: research/${slug}/README.md wears the figure ${data.figure}, which no roster file draws`);
      if (!/^\d{4}-\d{2}-\d{2}$/.test(data.date ?? ""))
        throw new Error(
          `site: research/${slug}/README.md dates itself ${data.date ?? "nowhere"}; its front matter needs a date, YYYY-MM-DD`,
        );
      const figures = drawn(join(RESEARCH, slug, "figures"))
        .map((name) => name.slice(0, -3))
        .filter((name) => known.has(name));
      return [
        {
          slug,
          title: data.title,
          date: data.date!,
          lead: data.lead ? lede(plain(data.lead)) : "",
          figure: data.figure || null,
          figures,
        },
      ];
    })
    .sort((a, b) => a.title.localeCompare(b.title) || a.slug.localeCompare(b.slug));
}

const FIELDS = new Set(["title", "line", "saver", "hidden", "deep", "source", "route"]);

const PARTS: Name[] = ["scene", "widget"];

const PAGER = under(join(org, "src", "lib", "app"), "page");

export function apps(found = places()): App[] {
  const list = found.map(({ id, dir }): App => {
    const at = (...parts: string[]) => join(dir, ...parts);
    if (!existsSync(at("app.json")))
      throw new Error(`site: ${rel(dir)}/ holds no app.json; every folder of src/pages/ and src/apps/ is a route`);
    const one = JSON.parse(read(at("app.json"))) as Omit<App, "id" | "dir" | "page" | "icon">;
    const say = (text: string) => new Error(`site: ${rel(dir)}/app.json ${text}`);
    if (!/^[a-z][a-z0-9]*$/.test(id))
      throw say("sits in a folder that is no root word; an id is lowercase letters and digits");
    const extra = Object.keys(one).find((key) => !FIELDS.has(key));
    if (extra) throw say(`sets ${extra}, and the fields are ${[...FIELDS].join(", ")}`);
    if (!one.title || !one.line) throw say("needs a title and a line");
    for (const key of ["saver", "hidden", "deep"] as const)
      if (one[key] !== undefined && one[key] !== true) throw say(`sets ${key} to ${one[key]}; it is true or absent`);
    if (one.route !== undefined && !one.route.startsWith("/")) throw say(`routes to ${one.route}, which is not rooted`);
    if (one.source !== undefined && !existsSync(join(repo, one.source)))
      throw say(`reads ${one.source}, which does not exist`);
    const parts = PARTS.every((name) => existsSync(at(NAMES[name])));
    const given = [
      ...[NAMES.index, "index.md"].filter((name) => existsSync(at(name))),
      ...(one.source ? ["source"] : []),
    ];
    const pages = given.length || !parts ? given : [NAMES.scene];
    if (pages.length !== 1)
      throw say(
        `needs exactly one page of ${NAMES.index}, index.md, a source, or ${NAMES.scene} with ${NAMES.widget}, and has ${pages.length ? pages.join(" and ") : "none"}`,
      );
    const lost = one.saver ? PARTS.find((name) => !existsSync(at(NAMES[name]))) : undefined;
    if (lost) throw say(`is a saver, and ${rel(dir)}/${NAMES[lost]} is missing`);
    const icon = existsSync(at(NAMES.icon));
    if (!icon && !one.hidden)
      throw say(`shows on the site, and ${rel(dir)}/${NAMES.icon} is missing; only a hidden app goes without an icon`);
    if (!one.hidden && !existsSync(at("icon.svg")))
      throw say(`shows on the site, and ${rel(at("icon.svg"))} is missing; every shown app wears an app icon`);
    return { id, dir, ...one, page: pages[0] === "source" ? null : pages[0]!, icon };
  });
  if (!list.some((one) => one.id === "home")) throw new Error("site: src/pages/home/ is missing, and home is /");
  return list.sort((a, b) => Number(b.id === "home") - Number(a.id === "home") || a.id.localeCompare(b.id));
}

/* SAVERS */

export type Saver = { id: string; app: string; name: string; title: string };

export type Host = { id: string; title: string; saver?: true; file: string };

export const SHELL: Host = { id: "mrly", title: "Mrly", file: shellFile("tiles") };

const record = (value: unknown): value is Record<string, unknown> =>
  typeof value === "object" && value !== null && [Object.prototype, null].includes(Object.getPrototypeOf(value));

export const hosts = (list: App[]): Host[] => [
  ...list
    .filter((one) => existsSync(under(one.dir, "scene")))
    .map((one) => ({ id: one.id, title: one.title, saver: one.saver, file: under(one.dir, "scene") })),
  SHELL,
];

function evaluate(file: string): Record<string, unknown> {
  if (!scan(file).exports.includes("SAVERS")) return {};
  try {
    return require(file);
  } catch (error) {
    throw new Error(`site: ${rel(file)} exports SAVERS, and Bun cannot load it: ${(error as Error).message}`);
  }
}

export function savers(
  list: Host[],
  load: (file: string) => Record<string, unknown>,
  desk: string = SITE.desk,
): { list: Saver[]; desk: string } {
  const lead = desk.split("/")[0];
  const rank = (one: Host) => (one.id === lead ? 0 : one.id === SHELL.id ? 2 : 1);
  const all = [...list]
    .sort((a, b) => rank(a) - rank(b) || a.title.localeCompare(b.title) || a.id.localeCompare(b.id))
    .flatMap((one): Saver[] => {
      const { SAVERS, SPEC, PAGE } = load(one.file);
      const say = (text: string) => new Error(`site: ${rel(one.file)} ${text}`);
      if (SAVERS !== undefined && one.saver)
        throw new Error(
          `site: ${rel(join(dirname(one.file), "app.json"))} sets saver, and its scene exports SAVERS, which replaces the flag`,
        );
      if (SAVERS === undefined && !one.saver) return [];
      const given: unknown = SAVERS ?? [{ id: one.id, title: one.title, pick: () => ({}) }];
      if (!Array.isArray(given) || given.length < 1 || given.length > 3)
        throw say("exports SAVERS, and SAVERS is a list of 1 to 3 savers");
      const defaults = record(PAGE) ? PAGE.defaults : null;
      const keys = new Set([
        ...(Array.isArray(SPEC) ? SPEC.map((row) => row?.key) : []),
        ...(record(defaults) ? Object.keys(defaults) : []),
      ]);
      const seen = new Set<string>();
      return given.map((saver: unknown) => {
        if (!record(saver)) throw say("lists a saver that is not { id, title, pick }");
        const { id, title, pick } = saver;
        if (typeof id !== "string" || !SLUG.test(id))
          throw say(`lists the saver ${id}, and a saver id is a slug: lowercase letters and digits joined by hyphens`);
        if (seen.has(id)) throw say(`lists the saver ${id} twice`);
        seen.add(id);
        if (typeof title !== "string" || !title.trim() || title.length > 32)
          throw say(`titles the saver ${id} ${JSON.stringify(title)}, and a title is 1 to 32 characters`);
        if (typeof pick !== "function") throw say(`gives the saver ${id} no pick function`);
        const value: unknown = pick(rng(1));
        if (!record(value)) throw say(`picks for the saver ${id} something that is not a plain object`);
        if (Object.hasOwn(value, "seed"))
          throw say(`picks a seed for the saver ${id}; the seed is rolled, never picked`);
        const extra = Object.keys(value).find((key) => !keys.has(key));
        if (extra) throw say(`picks ${extra} for the saver ${id}, which is no key of SPEC or PAGE.defaults`);
        return { id: `${one.id}/${id}`, app: one.id, name: one.title, title };
      });
    });
  const found = desk.includes("/") ? all.find((one) => one.id === desk) : all.find((one) => one.app === desk);
  if (!found) throw new Error(`site: site.json names the desk ${desk}, which is no saver`);
  return { list: all, desk: found.id };
}

/* LAUNCHPAD */

export const launchable = (list: App[]) =>
  list.filter((one) => !one.hidden && one.id !== "home" && `/${one.id}/` !== SITE.menu).map((one) => one.id);

export type Pad = { sections: Shelf[]; dock: string[][] };

export function launchpad(list: App[], file = LAUNCHPAD): Pad {
  const say = (text: string) => new Error(`site: launchpad.json ${text}`);
  const data = JSON.parse(read(file)) as { sections?: unknown; dock?: unknown };
  if (
    !Array.isArray(data.sections) ||
    !data.sections.length ||
    !Array.isArray(data.dock) ||
    Object.keys(data).some((key) => key !== "sections" && key !== "dock")
  )
    throw say("holds two fields: sections, a list of { name, apps }, and dock, a list of groups of ids");
  const known = new Set(launchable(list));
  const names = new Set<string>();
  const placed = new Set<string>();
  const sections = data.sections.map((one: Shelf) => {
    if (
      !one ||
      typeof one.name !== "string" ||
      !one.name.trim() ||
      !Array.isArray(one.apps) ||
      Object.keys(one).some((key) => key !== "name" && key !== "apps")
    )
      throw say("lists a section that is not { name, apps }");
    if (names.has(one.name.toLowerCase())) throw say(`names the section ${one.name} twice`);
    names.add(one.name.toLowerCase());
    for (const id of one.apps) {
      if (!known.has(id)) throw say(`lists ${id} under ${one.name}, and it is no route the Launchpad shows`);
      if (placed.has(id)) throw say(`lists ${id} twice`);
      placed.add(id);
    }
    return { name: one.name, apps: [...one.apps] };
  });
  const lost = [...known].filter((id) => !placed.has(id));
  if (lost.length) throw say(`leaves out ${lost.join(", ")}; every shown route sits in one section`);
  const docked = new Set<string>();
  const dock = data.dock.map((group: unknown) => {
    if (!Array.isArray(group)) throw say("docks a group that is not a list of ids");
    for (const id of group) {
      if (!known.has(id)) throw say(`docks ${id}, and it is no route the Launchpad shows`);
      if (docked.has(id)) throw say(`docks ${id} twice`);
      docked.add(id);
    }
    return [...group] as string[];
  });
  return { sections, dock };
}

/* ROWS */

function rows(list: App[], blog: Post[]): Row[] {
  const out: Row[] = [];
  for (const app of list) {
    const route = app.route ?? (app.id === "home" ? "/" : `/${app.id}/`);
    const source = app.page === "index.md" ? rel(join(app.dir, "index.md")) : (app.source ?? null);
    const meta: Row["meta"] = {
      id: app.id,
      ...(spells(app.page, "scene") ? { scene: true } : {}),
      ...(app.deep ? { deep: true } : {}),
      ...(source ? { prose: app.source ? "source" : "page" } : {}),
    };
    out.push({
      route,
      kind: spells(app.page, "index") || spells(app.page, "scene") ? "app" : "prose",
      title: app.title,
      source,
      date: null,
      lead: lede(plain(app.line)),
      figure: app.icon ? app.id : null,
      hidden: Boolean(app.hidden),
      meta,
    });
    if (app.id === "blog")
      for (const post of blog)
        out.push({
          route: post.route,
          kind: "prose",
          title: post.title,
          source: post.source,
          date: post.date,
          lead: post.lead,
          figure: `blog-${post.slug}`,
          hidden: false,
          meta: { prose: "post", date: post.date },
        });
  }
  return out;
}

/* LIVE */

export function live(figures: Figure[] = roster()): string[] {
  const code = [
    ...figures.map((one) => one.file),
    ...walk(join(org, "src")).filter((file) => /\.[jt]sx?$/.test(file) && !/\.test\./.test(file)),
  ];
  const used = new Set(code.flatMap((file) => [...read(file).matchAll(UNIT)].map((hit) => hit[1]!)));
  return UNITS.filter((unit) => used.has(unit));
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
    if (row.figure && !known.has(row.figure))
      throw new Error(`site: ${row.route} wears the figure ${row.figure}, which no roster file draws`);
    if (!row.source) continue;
    if (!existsSync(join(repo, row.source)))
      throw new Error(`site: ${row.route} reads ${row.source}, which does not exist`);
    for (const name of named(read(join(repo, row.source))))
      if (!known.has(name)) throw new Error(`site: ${row.source} shows the figure ${name}, which no roster file draws`);
  }
  if (!all.some((row) => row.meta.id === "missing"))
    throw new Error("site: src/pages/missing/ is missing, and its row draws every path that is no route");
  const units = live(figures);
  for (const unit of units)
    if (!existsSync(unitWasm(unit)))
      throw new Error(
        `site: a live figure or app imports @mrlyprod/mrly/${unit}, and site/wasm/${unit}/ does not hold it; bun run wasm copies the live units there`,
      );
  const moves = JSON.parse(read(join(org, "redirects.json"))) as Record<string, { to: string; since: string }>;
  const kept = Object.fromEntries(Object.entries(moves).filter(([, row]) => seen.has(row.to.split("?")[0]!)));
  const raw = tree(git.root);
  const shown = savers(hosts(list), evaluate);
  const site = {
    ...Object.fromEntries(
      ["title", "company", "tagline", "since", "prefix", "tint", "menu", "socials", "contact", "tree"].map((key) => [
        key,
        SITE[key as keyof typeof SITE],
      ]),
    ),
    root,
    version: MANIFEST.version,
    units,
    savers: shown.list,
    desk: shown.desk,
    licences: credits(raw),
    redirects: kept,
    launchpad: launchpad(list),
    git: { slug: git.slug, branch: git.branch },
  };
  held = {
    rows: all,
    site,
    figures,
    units,
    apps: list,
    posts: blog,
    topics: research,
    raw,
    dropped: Object.keys(moves).length - Object.keys(kept).length,
  };
  return held;
}

/* PLUGIN */

const thunks = (list: [string, string][]) =>
  `{ ${list.map(([key, file]) => `${JSON.stringify(key)}: () => import(${JSON.stringify(file)})`).join(", ")} }`;

const appFiles = (list: App[]): [string, string][] =>
  list.flatMap((one): [string, string][] =>
    spells(one.page, "index")
      ? [[one.id, join(one.dir, one.page)]]
      : spells(one.page, "scene")
        ? [[one.id, `app:${one.id}`]]
        : [],
  );

const scenes = (ready: State): [string, string][] => {
  const files = new Map(hosts(ready.apps).map((one) => [one.id, one.file]));
  return [...new Set((ready.site.savers as Saver[]).map((one) => one.app))].map((app) => [app, files.get(app)!]);
};

export const entry = (dir: string) =>
  [
    `import { page } from ${JSON.stringify(PAGER)};`,
    `import { PAGE } from ${JSON.stringify(under(dir, "scene"))};`,
    `import { Widget } from ${JSON.stringify(under(dir, "widget"))};`,
    "export const { mount, unmount } = page(Widget, { seed: 0, ...PAGE.defaults }, PAGE);",
  ].join("\n");

export const own = (list: App[]) =>
  list
    .filter((one) => spells(one.page, "index") && scan(join(one.dir, one.page)).exports.includes("render"))
    .map((one) => one.id);

export function icons(list: App[]): string {
  const made = list
    .filter((one) => !one.hidden)
    .map((one) => [one.id, appicon(one.id, read(join(one.dir, "icon.svg")))] as const);
  return `export const defs = ${JSON.stringify(made.map(([, one]) => one.defs).join(""))};\nexport const arts = ${JSON.stringify(Object.fromEntries(made.map(([id, one]) => [id, one.art])))};`;
}

export function modules(ready: Ready): Record<string, string> {
  return {
    "site:routes": `export const site = ${JSON.stringify(ready.site)};\nexport const rows = ${JSON.stringify(ready.rows)};`,
    "site:figures": [
      ...ready.units.map((unit) => `import ${unit} from ${JSON.stringify(unitWasm(unit))};`),
      `export const units = { ${ready.units.join(", ")} };`,
      `export const figures = ${thunks(ready.figures.map((one) => [one.name, one.file]))};`,
      `export const plates = ${JSON.stringify(plates(ready.figures))};`,
    ].join("\n"),
    "site:apps": `export const apps = ${thunks(appFiles(ready.apps))};\nexport const own = new Set(${JSON.stringify(own(ready.apps))});\nexport const lock = () => import(${JSON.stringify(LOCK)});`,
    "site:css": `export const css = ${JSON.stringify(ready.css)};`,
    "site:icons": icons(ready.apps),
    "apps:scenes": `export const scenes = ${thunks(scenes(ready))};`,
    ...Object.fromEntries(
      ready.apps.filter((one) => spells(one.page, "scene")).map((one) => [`app:${one.id}`, entry(one.dir)]),
    ),
  };
}

export const plugin = (get: () => Ready): BunPlugin => ({
  name: "site",
  setup(build) {
    mrly.setup(build);
    build.onResolve({ filter: /(?:^|\/)wasm\/[a-z]+\/mrly_js_[a-z]+(?:_bg\.wasm|\.js)$/ }, ({ path }) => ({
      path: join(org, "wasm", ...path.split("/").slice(-2)),
    }));
    build.onResolve({ filter: /^\// }, ({ path }) => (existsSync(path) ? undefined : { path, external: true }));
    build.onResolve({ filter: /^(site|apps?):/ }, ({ path }) => ({ path, namespace: "site" }));
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
      if (!seen.has(file))
        seen.set(file, `/fonts/${basename(file, ".woff2")}-${sha(readFileSync(file)).slice(0, 8)}.woff2`);
      return { path: seen.get(file)!, external: true };
    });
  },
});

/* PRIME */

async function outs(list: BuildArtifact[]): Promise<Out[]> {
  return Promise.all(
    list.map(async (one) => ({ path: one.path.replace(/^\.\//, ""), bytes: new Uint8Array(await one.arrayBuffer()) })),
  );
}

const LICENSES = [
  ...readdirSync(join(org, "src", "styles", "fonts"))
    .filter((name) => name.startsWith("LICENSE-"))
    .map((name) => join(org, "src", "styles", "fonts", name)),
  join(org, "src", "lib", "git", "seti", "LICENSE-seti.txt"),
];

const faces = (seen: Map<string, string>): Out[] =>
  [...seen].map(([file, url]) => ({ path: url.slice(1), bytes: readFileSync(file) }));

const licenses = (): Out[] => LICENSES.map((file) => ({ path: `fonts/${basename(file)}`, bytes: readFileSync(file) }));

const BY: Record<string, string> = { icons: "Material Symbols", seti: "Seti UI" };

function credits(raw: string[]) {
  const name = (file: string) => {
    const stem = basename(file, ".txt").slice("LICENSE-".length);
    const found = SITE.fonts.faces.find((one) => one.file === stem || one.licence === stem)?.family ?? BY[stem];
    if (!found) throw new Error(`site: ${rel(file)} ships under /fonts/, and no font of site.json names it`);
    return { name: found, href: `/fonts/${basename(file)}` };
  };
  return [
    ...LICENSES.map(name).sort((a, b) => a.name.localeCompare(b.name)),
    ...(raw.includes("LICENSE") ? [{ name: SITE.title, href: "/raw/LICENSE" }] : []),
  ];
}

async function prime(seen: Map<string, string>) {
  const boot = shellFile("boot");
  if (scanner.scanImports(read(boot)).length)
    throw new Error(`site: ${rel(boot)} imports nothing; it runs alone as the blocking classic script of the shell`);
  const sheets = { git: join(org, "src", "styles", "git.css"), mrly: join(org, "src", "lib", "app", "mrly.css") };
  const built = await Bun.build({
    entrypoints: [boot, ...Object.values(sheets)],
    minify: true,
    publicPath: "/",
    naming: NAMING,
    plugins: [fonts(seen)],
  });
  if (!built.success)
    throw new Error(`site: the boot script and the sheets failed to bundle\n${built.logs.join("\n")}`);
  const made = await outs(built.outputs);
  const find = (stem: string, ext: string) =>
    made.find((one) => one.path.startsWith(`${stem}-`) && one.path.endsWith(ext))!.path;
  return {
    boot: `/${find("boot", ".js")}`,
    css: Object.fromEntries(Object.keys(sheets).map((name) => [name, `/${find(name, ".css")}`])),
    assets: made,
  };
}

let primed: Promise<Ready> | null = null;

let done: Ready | null = null;

export function ready(): Promise<Ready> {
  if (primed) return primed;
  const seen = new Map<string, string>();
  primed = prime(seen).then((made) => {
    done = { ...state(), ...made, assets: [...made.assets, ...faces(seen), ...licenses()] };
    return done;
  });
  return primed;
}

export default plugin(() => {
  if (!done)
    throw new Error(
      "site: await ready() before a bundle asks for a site: module; a Bun.build inside a plugin never returns",
    );
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
  if ((count.get("pages") ?? 0) > 1)
    throw new Error("site: /pages/ holds more than one url, and sitemap-pages.xml is the lone pages' map");
  const groups = new Map<string, Url[]>();
  for (const one of urls) {
    const name = count.get(segment(one.route))! > 1 ? segment(one.route) : "pages";
    groups.set(name, [...(groups.get(name) ?? []), one]);
  }
  const last = (list: Url[]) =>
    list
      .map((one) => one.date ?? "")
      .sort()
      .at(-1) ?? "";
  const mod = (date: string | null) => (date ? `<lastmod>${date}</lastmod>` : "");
  const names = [...groups.keys()].sort();
  const maps = names.map((name) => ({
    path: `sitemap-${name}.xml`,
    bytes: `${XML}<urlset xmlns="${SCHEMA}">\n${groups
      .get(name)!
      .map((one) => `<url><loc>${esc(root + encodeURI(one.route))}</loc>${mod(one.date)}</url>`)
      .join("\n")}\n</urlset>\n`,
  }));
  const index = names.map(
    (name) =>
      `<sitemap><loc>${esc(`${root}/sitemap-${name}.xml`)}</loc>${mod(last(groups.get(name)!) || null)}</sitemap>`,
  );
  return [
    { path: "sitemap.xml", bytes: `${XML}<sitemapindex xmlns="${SCHEMA}">\n${index.join("\n")}\n</sitemapindex>\n` },
    ...maps,
  ];
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
    {
      path: "llms.txt",
      bytes: llms(
        SITE.title,
        root,
        SITE.llms,
        ready.rows,
        new Set(ready.raw),
        (ready.site.launchpad as Pad).sections,
        ready.topics,
      ),
    },
    { path: "research.json", bytes: `${JSON.stringify({ topics: ready.topics })}\n` },
    ...sitemaps([
      ...shown.map((row) => ({ route: row.route, date: row.date })),
      ...ready.raw.map((path) => ({ route: `/raw/${path}`, date: null })),
    ]),
    { path: "robots.txt", bytes: robots() },
    { path: TREE, bytes: JSON.stringify(forest(git, ready.raw)), type: "application/json" },
    { path: "redirects.json", bytes: `${JSON.stringify(ready.site.redirects, null, 2)}\n` },
  ];
}

/* COPIES */

export const typed = (path: string, bytes: Uint8Array) =>
  rawKind(path, bytes.length <= HUGE && !IMAGE.has(ext(path)) && ext(path) !== "pdf" && reads(bytes) !== null);

function copies(ready: State): Out[] {
  const raw = ready.raw.map((path) => {
    const bytes = readFileSync(join(repo, path));
    return { path: `raw/${path}`, bytes, type: typed(path, bytes) };
  });
  const pub = walk(join(org, "public")).map((file) => ({
    path: relative(join(org, "public"), file),
    bytes: readFileSync(file),
  }));
  return [...raw, ...pub];
}

/* BUILD */

export type Built = {
  out: string;
  rows: Row[];
  files: { path: string; hash: string; type?: string; fixed?: boolean }[];
  raw: number;
  dropped: number;
  units: string[];
};

export function withTint(html: string) {
  if (!html.includes(TINT))
    throw new Error(`site: src/shell/index.html must hold ${TINT}, which the build fills with the tint rules`);
  return html.replace(TINT, `<style id="tint">\n${tintCss(SITE.tint)}</style>`);
}

export async function build(out = DIST): Promise<Built> {
  const seen = new Map<string, string>();
  const made = await prime(seen);
  const ready: Ready = { ...state(), ...made };
  const entries = [
    join(org, "src", "shell", "index.html"),
    ...SHELLS,
    under(join(org, "src", "pages", "git"), "git"),
    ...ready.figures.map((one) => one.file),
    ...appFiles(ready.apps).flatMap(([, file]) => (file.startsWith("/") ? [file] : [])),
    LOCK,
  ];
  const built = await Bun.build({
    entrypoints: entries,
    splitting: true,
    minify: true,
    publicPath: "/",
    naming: NAMING,
    define: { "process.env.NODE_ENV": '"production"' },
    plugins: [plugin(() => ready), fonts(seen)],
  });
  if (!built.success) throw new Error(`site: the bundle failed\n${built.logs.join("\n")}`);
  const bundle = await outs(built.outputs);
  const page = bundle.find((one) => one.path.endsWith(".html"))!;
  const shell = new TextDecoder().decode(page.bytes as Uint8Array);
  if (!shell.includes('<script src="/boot.js"></script>'))
    throw new Error("site: src/shell/index.html must load /boot.js as its first, classic script");
  page.path = "index.html";
  page.bytes = withTint(shell.replace('<script src="/boot.js"></script>', `<script src="${made.boot}"></script>`));
  const loose = [...bundle, ...made.assets].filter((one) => one !== page && !HASHED.test(one.path));
  if (loose.length) throw new Error(`site: the bundle named ${loose.map((one) => one.path).join(", ")} without a hash`);
  const hashed = [...faces(seen), ...made.assets, ...bundle.filter((one) => one !== page)].map((one) => ({
    ...one,
    fixed: true,
  }));
  const all: Out[] = [
    ...copies(ready),
    ...globals(ready),
    ...licenses(),
    ...hashed,
    page,
    { path: "404.html", bytes: page.bytes },
  ];
  rmSync(out, { recursive: true, force: true });
  const files = new Map<string, Built["files"][number]>();
  for (const one of all) {
    const file = join(out, one.path);
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, one.bytes);
    files.set(one.path, { path: one.path, hash: sha(one.bytes).slice(0, 16), type: one.type, fixed: one.fixed });
  }
  return {
    out,
    rows: ready.rows,
    files: [...files.values()],
    raw: ready.raw.length,
    dropped: ready.dropped,
    units: ready.units,
  };
}

/* MAIN */

if (import.meta.main) {
  const done = await build();
  const size = done.files.reduce((sum, one) => sum + statSync(join(done.out, one.path)).size, 0);
  console.log(
    `site: ${done.rows.length} rows, ${done.raw} raw files, ${done.files.length} files (${(size / 1e6).toFixed(1)} MB) in ${done.out.startsWith(`${org}/`) ? relative(org, done.out) : done.out}, wasm ${done.units.join(" ")}, ${done.dropped} redirects dropped for a target that is no route`,
  );
}
