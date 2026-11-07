import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { viewFaults } from "@mrlyprod/os/contract/index.ts";
import { rawKind } from "@mrlyprod/os/hosts/cloud/mime.ts";
import type { Config, App as Made } from "@mrlyprod/os/kernel/index.ts";
import { Fault, kernel } from "@mrlyprod/os/kernel/index.ts";
import type { BuildArtifact, BunPlugin } from "bun";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { appicon } from "../../kit/appicon.ts";
import { ext, HUGE, IMAGE, reads } from "../../kit/git/view.ts";
import { NAME, figures as named, plain, summary } from "../../kit/md/md.ts";
import { esc } from "../../kit/md/text.ts";
import { rng } from "../../kit/rand.ts";
import type { App, Host, Pad, Row, Saver } from "../../kit/rows.ts";
import { tintCss } from "../../kit/theme/tint.ts";
import { apply } from "../../shell/active.ts";
import type { Design } from "../../shell/design.ts";
import { draw, LISTED, sheets, source } from "./design.ts";
import { forest, config as gitConfig, TREE, tree } from "./git.ts";
import { closed, HOME, home, REPO, SITE, UNITS, unit } from "./home.ts";
import { llms, type Shelf } from "./llms.ts";
import { packages, where } from "./packages.ts";

export const NAMES = {
  index: "index.tsx",
  scene: "scene.ts",
  widget: "widget.tsx",
  icon: "icon.ts",
  desk: "desk.tsx",
  page: "page.tsx",
  lock: "lock.ts",
  tiles: "tiles.ts",
  boot: "boot.ts",
  app: "app.tsx",
  prose: "prose.tsx",
  git: "git.ts",
  world: "app.ts",
  view: "view.tsx",
} as const;

type Name = keyof typeof NAMES;

const under = (dir: string, name: Name) => join(dir, NAMES[name]);

export const spells = <N extends Name>(file: string | null, name: N): file is (typeof NAMES)[N] => file === NAMES[name];

const repo = REPO;
const DIST = process.env.MRLY_DIST ? resolve(process.env.MRLY_DIST) : join(HOME, "dist");
const root = (process.env.MRLY_SITE ?? SITE.root).replace(/\/$/, "");
const ROOTS = [join(repo, "shell", "routes"), ...SITE.roots.map((one) => resolve(HOME, one))];
const GIT = gitConfig(HOME, SITE.git)!;
const LAUNCHPAD = join(HOME, "launchpad.json");
const WORLD = join(HOME, "world.json");
const shellFile = (name: Name) => under(join(repo, "shell"), name);
const SHELLS = [shellFile("app"), shellFile("prose")];
const LOCK = shellFile("lock");
const TINT = '<style id="tint"></style>';
const MARK = '<link rel="icon" type="image/svg+xml">';
const HASHED = /-[0-9a-z]{8}\.[^./]+$/;
const NAMING = { entry: "[name]-[hash].[ext]", chunk: "lib-[hash].[ext]", asset: "[name]-[hash].[ext]" };

const read = (file: string) => readFileSync(file, "utf8");
const sha = (data: string | Uint8Array) => createHash("sha256").update(data).digest("hex");
const rel = (file: string) => relative(GIT.root, file);
export const rawFile = (path: string) => join(GIT.root, path);

export type { App, Host, Pad, Row, Saver };

export type Out = { path: string; bytes: Uint8Array | string; type?: string; fixed?: boolean };

export type Figure = { name: string; file: string };

export type World = { config: Config; files: [string, string][] };

export type State = {
  world: World;
  rows: Row[];
  site: Record<string, unknown>;
  figures: Figure[];
  units: string[];
  apps: App[];
  raw: string[];
  widgets: string[];
  dropped: number;
};

export type Look = {
  name: string;
  title: string;
  skin: Design["skin"];
  bar: Design["bar"];
  frame: string;
};

export type Dressed = { designs: Look[] };

export type Primed = { boot: string; css: Record<string, string>; pages: Record<string, string>; assets: Out[] };

export type Ready = State & Dressed & Primed;

const lede = (text: string, max = 200) => {
  const all = summary(text, Infinity);
  if (all.length <= max) return all;
  const cut = all.slice(0, max + 1).lastIndexOf(". ");
  return cut > 0 ? all.slice(0, cut + 1) : summary(text, max);
};

const scanner = new Bun.Transpiler({ loader: "ts" });

const jsx = new Bun.Transpiler({ loader: "jsx" });

const scanners = new Map<string, Bun.Transpiler>([
  ["js", jsx],
  ["jsx", jsx],
  ["ts", scanner],
  ["tsx", new Bun.Transpiler({ loader: "tsx" })],
]);

const scan = (file: string) => (scanners.get(ext(file)) ?? scanner).scan(read(file));

export const folders = (dir: string) =>
  existsSync(dir)
    ? readdirSync(dir, { withFileTypes: true })
        .filter((item) => item.isDirectory() && !item.name.startsWith("."))
        .map((item) => item.name)
        .sort()
    : [];

const version = () => {
  const held = UNITS.map((name) => join(unit(name), "package.json")).find((file) => existsSync(file));
  return held ? (JSON.parse(read(held)) as { version: string }).version : null;
};

const UNIT = /@mrlyprod\/([a-z]+)/g;

const CALL = /\b(?:wasm|useUnit)\(([^)]*)\)/g;

const LOADERS = join(HOME, "wasm");

const TABLE = join(repo, "kit", "units", "load.ts");

const LEAF = new RegExp(`^@mrlyprod/(${UNITS.join("|") || "(?!)"})$`);

const unitWasm = (unit: string) => join(HOME, "wasm", "blobs", `${home(unit)}.wasm`);

const SHELF = new Set(["shared", "data", "node_modules"]);

const SHELVES = [join(repo, "..", "apps"), join(repo, "examples")];

const shelved = (id: string) => {
  const shelf = SHELVES.find((one) => existsSync(join(one, id, "app.json")));
  if (!shelf) throw new Error(`site: site.json names the system app ${id}, and no such app exists`);
  return join(shelf, id);
};

const places = () =>
  [
    ...new Map([
      ...folders(ROOTS[0]!).map((id): [string, { id: string; dir: string }] => [id, { id, dir: join(ROOTS[0]!, id) }]),
      ...(SITE.apps ?? []).map((id): [string, { id: string; dir: string }] => [id, { id, dir: shelved(id) }]),
      ...ROOTS.slice(1).flatMap((root) =>
        folders(root)
          .filter((id) => !SHELF.has(id))
          .map((id): [string, { id: string; dir: string }] => [id, { id, dir: join(root, id) }]),
      ),
    ]).values(),
  ].sort((a, b) => a.id.localeCompare(b.id));

export const plates = (figures: Figure[]) =>
  figures.filter((one) => [...ROOTS, ...SHELVES].includes(dirname(dirname(one.file)))).map((one) => one.name);

export function roster(): Figure[] {
  const found = [...places().map(({ id, dir }) => ({ name: id, file: under(dir, "icon") }))].filter((one) =>
    existsSync(one.file),
  );
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
    build.onResolve({ filter: LEAF }, ({ path }) => ({
      path: join(unit(path.slice("@mrlyprod/".length)), "index.js"),
    }));
    packages.setup(build);
  },
};

const walk = (dir: string): string[] =>
  readdirSync(dir, { withFileTypes: true }).flatMap((item) =>
    item.name.startsWith(".") ? [] : item.isDirectory() ? walk(join(dir, item.name)) : [join(dir, item.name)],
  );

const SLUG = /^[a-z0-9]+(-[a-z0-9]+)*$/;

const FIELDS = new Set(["title", "line", "aliases", "saver", "hidden", "deep", "frame", "source", "route"]);

const ALIASES = { list: 12, each: 24 };

const PARTS: Name[] = ["scene", "widget"];

const PAGER = under(join(repo, "kit"), "page");

const WORDS: Record<string, () => string> = {
  title: () => SITE.title,
  domain: () => new URL(root).host,
  repo: () => GIT.name,
};

const words = (text: string) =>
  text.replace(/\{(title|domain|repo)\}/g, (_, key: string) => JSON.stringify(WORDS[key]!()).slice(1, -1));

export function apps(found = places()): App[] {
  const list = found.map(({ id, dir }): App => {
    const at = (...parts: string[]) => join(dir, ...parts);
    if (!existsSync(at("app.json")))
      throw new Error(`site: ${rel(dir)}/ holds no app.json; every folder of a route root is a route`);
    const one = JSON.parse(words(read(at("app.json")))) as Omit<App, "id" | "dir" | "page" | "icon">;
    const say = (text: string) => new Error(`site: ${rel(dir)}/app.json ${text}`);
    if (!/^[a-z][a-z0-9]*$/.test(id))
      throw say("sits in a folder that is no root word; an id is lowercase letters and digits");
    const extra = Object.keys(one).find((key) => !FIELDS.has(key));
    if (extra) throw say(`sets ${extra}, and the fields are ${[...FIELDS].join(", ")}`);
    if (!one.title || !one.line) throw say("needs a title and a line");
    if (one.aliases !== undefined) {
      const list = one.aliases;
      const bad =
        !Array.isArray(list) ||
        !list.length ||
        list.length > ALIASES.list ||
        new Set(list).size !== list.length ||
        list.some((alias) => typeof alias !== "string" || !alias.length || alias.length > ALIASES.each) ||
        list.some((alias) => alias !== alias.toLowerCase());
      if (bad)
        throw say(
          `sets aliases, and aliases are 1 to ${ALIASES.list} lowercase words of 1 to ${ALIASES.each} characters, none twice`,
        );
    }
    for (const key of ["saver", "hidden", "deep"] as const)
      if (one[key] !== undefined && one[key] !== true) throw say(`sets ${key} to ${one[key]}; it is true or absent`);
    if (one.route !== undefined && !one.route.startsWith("/")) throw say(`routes to ${one.route}, which is not rooted`);
    if (one.source !== undefined && !existsSync(rawFile(one.source)))
      throw say(`reads ${one.source}, which does not exist`);
    const parts = PARTS.every((name) => existsSync(at(NAMES[name])));
    const given = [
      ...[NAMES.index, "index.md"].filter((name) => existsSync(at(name))),
      ...(one.source ? ["source"] : []),
    ];
    const pages = given.length
      ? given
      : [...(parts ? [NAMES.scene] : []), ...(existsSync(at(NAMES.world)) ? [NAMES.world] : [])];
    if (pages.length !== 1)
      throw say(
        `needs exactly one page of ${NAMES.index}, index.md, a source, ${NAMES.scene} with ${NAMES.widget}, or ${NAMES.world}, and has ${pages.length ? pages.join(" and ") : "none"}`,
      );
    const lost = one.saver ? PARTS.find((name) => !existsSync(at(NAMES[name]))) : undefined;
    if (lost) throw say(`is a saver, and ${rel(dir)}/${NAMES[lost]} is missing`);
    const icon = existsSync(at(NAMES.icon));
    if (!icon && !one.hidden)
      throw say(`shows on the site, and ${rel(dir)}/${NAMES.icon} is missing; only a hidden app goes without an icon`);
    if (!one.hidden && !existsSync(at("icon.svg")))
      throw say(`shows on the site, and ${rel(at("icon.svg"))} is missing; every shown app needs an app icon`);
    return { id, dir, ...one, page: pages[0] === "source" ? null : pages[0]!, icon };
  });
  if (!list.some((one) => one.id === "home")) throw new Error("site: no route root holds home/, and home is /");
  return list.sort((a, b) => Number(b.id === "home") - Number(a.id === "home") || a.id.localeCompare(b.id));
}

const inWorld = (config: Config) => (config.apps ?? []).map((one) => (typeof one === "string" ? one : one.id));

export function world(list: App[], file = WORLD): World {
  const say = (text: string) => new Error(`site: ${rel(file)} ${text}`);
  const files = list
    .filter((one) => existsSync(under(one.dir, "world")))
    .map((one): [string, string] => [one.id, under(one.dir, "world")]);
  const config: Config = existsSync(file)
    ? (JSON.parse(read(file)) as Config)
    : { name: SITE.prefix.replace(/-$/, "") || "site", apps: files.map(([id]) => id) };
  const lost = files.map(([id]) => id).filter((id) => !inWorld(config).includes(id));
  if (lost.length) throw say(`leaves out ${lost.join(", ")}; every route with an ${NAMES.world} lives in the world`);
  const made = files.map(([id, path]) => {
    const one = (require(path) as { default?: Made }).default;
    if (!one || one.id !== id) throw new Error(`site: ${rel(path)} exports no default app() whose id is ${id}`);
    return one;
  });
  const faults = made.flatMap((one) => viewFaults(one, inWorld(config)));
  if (faults.length) throw say(faults.join("; "));
  try {
    kernel(made).boot(config);
  } catch (error) {
    throw say(`does not boot: ${error instanceof Fault ? error.note : (error as Error).message}`);
  }
  return { config, files };
}

type Tiles = { id: string; title: string };

const TILES = (SITE as { tiles?: Tiles }).tiles;

export const SHELL: Host = { id: TILES?.id ?? "tiles", title: TILES?.title ?? "Tiles", file: shellFile("tiles") };

const record = (value: unknown): value is Record<string, unknown> =>
  typeof value === "object" && value !== null && [Object.prototype, null].includes(Object.getPrototypeOf(value));

export const hosts = (list: App[], tiles = Boolean(TILES)): Host[] => [
  ...list
    .filter((one) => existsSync(under(one.dir, "scene")))
    .map((one) => ({ id: one.id, title: one.title, saver: one.saver, file: under(one.dir, "scene") })),
  ...(tiles ? [SHELL] : []),
];

export function tiles(list: App[], given = TILES) {
  if (!given) return;
  const say = (text: string) => new Error(`site: site.json names the tiles saver ${JSON.stringify(given)}, ${text}`);
  if (!/^[a-z][a-z0-9]*$/.test(given.id ?? "")) throw say("and its id is no root word");
  if (typeof given.title !== "string" || !given.title.trim() || given.title.length > 32)
    throw say("and its title is not 1 to 32 characters");
  if (list.some((one) => one.id === given.id)) throw say("and a route takes its id");
}

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

export const DESK = ["clock", "continue"];

export function widgets(list: App[], given: unknown = (SITE as { widgets?: unknown }).widgets ?? DESK): string[] {
  const say = (text: string) => new Error(`site: site.json widgets ${text}`);
  if (!Array.isArray(given)) throw say("is a list of widget ids");
  const own = new Set(list.filter((one) => existsSync(under(one.dir, "desk"))).map((one) => one.id));
  const seen = new Set<string>();
  for (const id of given) {
    if (!DESK.includes(id) && !own.has(id))
      throw say(`names ${id}, which is neither a shell widget (${DESK.join(", ")}) nor a route holding ${NAMES.desk}`);
    if (seen.has(id)) throw say(`names ${id} twice`);
    seen.add(id);
  }
  return [...given];
}

export function gadgets(list: App[], order: string[]): string {
  const dirs = new Map(list.filter((one) => existsSync(under(one.dir, "desk"))).map((one) => [one.id, one.dir]));
  const routed = order.filter((id) => dirs.has(id));
  return [
    ...routed.map((id, i) => `import { widget as w${i} } from ${JSON.stringify(under(dirs.get(id)!, "desk"))};`),
    `export const order = ${JSON.stringify(order)};`,
    `export const widgets = { ${routed.map((id, i) => `${JSON.stringify(id)}: w${i}`).join(", ")} };`,
  ].join("\n");
}

export const launchable = (list: App[]) =>
  list.filter((one) => !one.hidden && one.id !== "home" && `/${one.id}/` !== SITE.menu).map((one) => one.id);

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

function rows(list: App[]): Row[] {
  const out: Row[] = [];
  for (const app of list) {
    const route = app.route ?? (app.id === "home" ? "/" : `/${app.id}/`);
    const source = app.page === "index.md" ? rel(join(app.dir, "index.md")) : (app.source ?? null);
    const meta: Row["meta"] = {
      id: app.id,
      ...(spells(app.page, "scene") ? { scene: true } : {}),
      ...(app.deep ? { deep: true } : {}),
      ...(app.frame ? { frame: app.frame } : {}),
      ...(source ? { prose: app.source ? "source" : "page" } : {}),
      ...(app.aliases ? { aliases: app.aliases } : {}),
    };
    out.push({
      route,
      kind: spells(app.page, "index") || spells(app.page, "scene") || spells(app.page, "world") ? "app" : "prose",
      title: app.title,
      source,
      date: null,
      lead: lede(plain(app.line)),
      figure: app.icon ? app.id : null,
      hidden: Boolean(app.hidden),
      meta,
    });
  }
  return out;
}

const code = (file: string) => /\.[jt]sx?$/.test(file) && !/\.test\./.test(file) && !file.includes("/node_modules/");

const follow = (path: string, from: string) => {
  const linked = where(path);
  if (linked) return linked;
  if (!/^\.{0,2}\//.test(path)) return null;
  try {
    return Bun.resolveSync(path.split("?")[0]!, dirname(from));
  } catch {
    return null;
  }
};

const imports = (file: string) => {
  try {
    return scan(file).imports.map((one) => one.path);
  } catch {
    return [];
  }
};

export function reach(seeds: string[]): string[] {
  const seen = new Set<string>();
  const queue = seeds.filter(code);
  while (queue.length) {
    const file = queue.pop()!;
    if (seen.has(file) || !existsSync(file)) continue;
    seen.add(file);
    for (const path of file === TABLE ? [] : imports(file)) {
      const next = follow(path, file);
      if (next && code(next) && !seen.has(next)) queue.push(next);
    }
  }
  return [...seen].sort();
}

export const shipped = (figures: Figure[], list: App[] = apps()) =>
  reach([
    ...[...ROOTS, ...(SITE.apps ?? []).map(shelved), join(repo, "shell"), ...LISTED].flatMap((dir) =>
      existsSync(dir) ? walk(dir) : [],
    ),
    join(repo, "kit", "git", "page.ts"),
    ...(list.some((one) => spells(one.page, "scene")) ? [PAGER] : []),
    ...figures.map((one) => one.file),
  ]);

const names = (text: string) => [
  ...[...text.matchAll(UNIT)].map((hit) => (hit[1] ?? hit[2])!),
  ...[...text.matchAll(CALL)].flatMap((hit) => [...hit[1]!.matchAll(/"([a-z]+)"/g)].map((one) => one[1]!)),
];

export function live(figures: Figure[] = roster(), list: App[] = apps()): string[] {
  const used = new Set(shipped(figures, list).flatMap((file) => names(read(file))));
  return closed(UNITS.filter((unit) => used.has(unit)));
}

let held: State | null = null;

export function state(): State {
  if (held) return held;
  const git = gitConfig(HOME, SITE.git)!;
  const figures = roster();
  const known = new Set(figures.map((one) => one.name));
  const list = apps();
  const all = rows(list);
  const seen = new Set<string>();
  for (const row of all) {
    if (seen.has(row.route)) throw new Error(`site: two rows claim ${row.route}`);
    seen.add(row.route);
    if (row.figure && !known.has(row.figure))
      throw new Error(`site: ${row.route} uses the figure ${row.figure}, which no roster file draws`);
    if (!row.source) continue;
    if (!existsSync(rawFile(row.source)))
      throw new Error(`site: ${row.route} reads ${row.source}, which does not exist`);
    for (const name of named(read(rawFile(row.source))))
      if (!known.has(name)) throw new Error(`site: ${row.source} shows the figure ${name}, which no roster file draws`);
  }
  if (!all.some((row) => row.meta.id === "missing"))
    throw new Error("site: no route root holds missing/, and its row draws every path that is no route");
  const units = live(figures, list);
  for (const unit of units)
    if (!existsSync(unitWasm(unit)))
      throw new Error(
        `site: a live figure or app imports @mrlyprod/${unit}, and the site wasm/blobs/ does not hold ${home(unit)}; bun run wasm copies the live blobs there`,
      );
  const moves = JSON.parse(read(join(HOME, "redirects.json"))) as Record<string, { to: string; since: string }>;
  const kept = Object.fromEntries(Object.entries(moves).filter(([, row]) => seen.has(row.to.split("?")[0]!)));
  const raw = tree(git.root).sort();
  tiles(list);
  const shown = savers(hosts(list), evaluate);
  const site = {
    ...Object.fromEntries(
      [
        "title",
        "wordmark",
        "company",
        "since",
        "prefix",
        "mark",
        "tint",
        "menu",
        "socials",
        "contact",
        "tree",
        "min",
      ].map((key) => [key, SITE[key as keyof typeof SITE]]),
    ),
    root,
    version: units.length ? version() : null,
    units,
    savers: shown.list,
    desk: shown.desk,
    licences: credits(raw),
    redirects: kept,
    launchpad: launchpad(list),
    git: { slug: git.slug, branch: git.branch, folder: git.folder },
  };
  held = {
    world: world(list),
    rows: all,
    site,
    figures,
    units,
    apps: list,
    raw,
    widgets: widgets(list),
    dropped: Object.keys(moves).length - Object.keys(kept).length,
  };
  return held;
}

const thunks = (list: [string, string][]) =>
  `{ ${list.map(([key, file]) => `${JSON.stringify(key)}: () => import(${JSON.stringify(file)})`).join(", ")} }`;

export const table = (units: string[]) =>
  `export const LOAD = ${thunks(
    units
      .map((unit): [string, string] => [unit, join(LOADERS, "blobs", `${home(unit)}.load.ts`)])
      .filter(([, file]) => existsSync(file)),
  )};`;

const appFiles = (list: App[]): [string, string][] =>
  list.flatMap((one): [string, string][] =>
    spells(one.page, "index")
      ? [[one.id, join(one.dir, one.page)]]
      : spells(one.page, "scene") || spells(one.page, "world")
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

const VIEWER = under(join(repo, "kit"), "view");

export const viewer = (id: string) =>
  [
    `import { view } from ${JSON.stringify(VIEWER)};`,
    `export const { mount, unmount } = view(${JSON.stringify(id)});`,
  ].join("\n");

export const worlds = ({ config, files }: World) =>
  [
    ...files.map(([, file], i) => `import app${i} from ${JSON.stringify(file)};`),
    `export const apps = [${files.map((_, i) => `app${i}`).join(", ")}];`,
    `export const config = ${JSON.stringify(config)};`,
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
    "site:widgets": gadgets(ready.apps, ready.widgets),
    "site:frame": `export const frames = ${JSON.stringify(Object.fromEntries(ready.designs.map((one) => [one.name, one.frame])))};`,
    "site:design": source(ready.designs.map((one) => one.name)),
    "apps:scenes": `export const scenes = ${thunks(scenes(ready))};`,
    "site:world": worlds(ready.world),
    ...Object.fromEntries(
      ready.apps.filter((one) => spells(one.page, "scene")).map((one) => [`app:${one.id}`, entry(one.dir)]),
    ),
    ...Object.fromEntries(
      ready.apps.filter((one) => spells(one.page, "world")).map((one) => [`app:${one.id}`, viewer(one.id)]),
    ),
  };
}

export const plugin = (get: () => Ready): BunPlugin => ({
  name: "site",
  setup(build) {
    build.onResolve({ filter: LEAF }, ({ path }) => {
      const copy = join(HOME, "wasm", path.slice("@mrlyprod/".length), "index.js");
      return existsSync(copy) ? { path: copy } : undefined;
    });
    mrly.setup(build);
    build.onResolve({ filter: /^\// }, ({ path }) => (existsSync(path) ? undefined : { path, external: true }));
    build.onLoad({ filter: /\/kit\/units\/load\.ts$/ }, () => ({
      contents: table(get().units),
      loader: "ts",
    }));
    build.onResolve({ filter: /^(site|apps?):/ }, ({ path }) => ({ path, namespace: "site" }));
    build.onLoad({ filter: /.*/, namespace: "site" }, ({ path }) => {
      const text = modules(get())[path];
      if (text === undefined) throw new Error(`site: no virtual module ${path}`);
      return { contents: text, loader: path.endsWith(".css") ? "css" : "js" };
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

async function outs(list: BuildArtifact[]): Promise<Out[]> {
  return Promise.all(
    list.map(async (one) => ({ path: one.path.replace(/^\.\//, ""), bytes: new Uint8Array(await one.arrayBuffer()) })),
  );
}

const faces = (seen: Map<string, string>): Out[] =>
  [...seen].map(([file, url]) => ({ path: url.slice(1), bytes: readFileSync(file) }));

const credits = (raw: string[]) => (raw.includes("LICENSE") ? [{ name: SITE.title, href: "/raw/LICENSE" }] : []);

const PAGE = join(repo, "shell", "page.css");

const styles = (one: Look): BunPlugin => ({
  name: "design",
  setup(build) {
    const held = sheets(one.name, one.skin);
    mrly.setup(build);
    build.onResolve({ filter: /^\// }, ({ path }) => (existsSync(path) ? undefined : { path, external: true }));
    build.onResolve({ filter: /^site:design/ }, ({ path }) => ({ path, namespace: "site" }));
    build.onLoad({ filter: /.*/, namespace: "site" }, ({ path }) => {
      if (held[path] === undefined) throw new Error(`site: no design sheet ${path}`);
      return { contents: held[path], loader: "css" };
    });
  },
});

export async function sheet(one: Look, seen: Map<string, string>): Promise<Out[]> {
  const built = await Bun.build({
    entrypoints: [PAGE],
    minify: true,
    publicPath: "/",
    naming: { ...NAMING, entry: `${one.name}-[hash].[ext]` },
    plugins: [styles(one), fonts(seen)],
  });
  if (!built.success)
    throw new Error(`site: the sheet of the design ${one.name} failed to bundle\n${built.logs.join("\n")}`);
  const made = await outs(built.outputs);
  const lead = made.find((each) => each.path.startsWith(`${one.name}-`) && each.path.endsWith(".css"));
  if (!lead) throw new Error(`site: the design ${one.name} bundled no sheet`);
  return [lead, ...made.filter((each) => each !== lead)];
}

async function prime(seen: Map<string, string>, looks: Look[]): Promise<Primed> {
  const boot = shellFile("boot");
  if (scanner.scanImports(read(boot)).length)
    throw new Error(`site: ${rel(boot)} imports nothing; it runs alone as the blocking classic script of the shell`);
  const sheets = { git: join(repo, "shared", "git.css"), mrly: join(repo, "kit", "mrly.css") };
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
  const pages = await Promise.all(looks.map((one) => sheet(one, seen)));
  return {
    boot: `/${find("boot", ".js")}`,
    css: Object.fromEntries(Object.keys(sheets).map((name) => [name, `/${find(name, ".css")}`])),
    pages: Object.fromEntries(looks.map((one, at) => [one.name, `/${pages[at]![0]!.path}`])),
    assets: [...made, ...pages.flat()],
  };
}

export async function dress(
  title: string,
  mark = SITE.mark,
  word = SITE.wordmark || title.toUpperCase(),
  cart = Boolean((SITE as { cart?: boolean }).cart),
): Promise<Dressed> {
  const drawn = await draw();
  const { Frame } = await import("../../shell/frame.tsx");
  return {
    designs: drawn.map(({ name, design }) => {
      apply(design);
      return {
        name,
        title: design.title,
        skin: design.skin,
        bar: design.bar,
        frame: renderToStaticMarkup(createElement(Frame, { design, title, mark, word, cart })),
      };
    }),
  };
}

const whole = (made: Primed, dressed: Dressed): Ready => {
  const held = state();
  const designs = dressed.designs.map((one) => ({ id: one.name, title: one.title }));
  return { ...held, ...made, ...dressed, site: { ...held.site, designs } };
};

let primed: Promise<Ready> | null = null;

let done: Ready | null = null;

export function ready(): Promise<Ready> {
  if (primed) return primed;
  const seen = new Map<string, string>();
  primed = dress(SITE.title).then(async (dressed) => {
    const made = await prime(seen, dressed.designs);
    done = whole({ ...made, assets: [...made.assets, ...faces(seen)] }, dressed);
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
  const git = gitConfig(HOME, SITE.git)!;
  const shown = ready.rows.filter((row) => !row.hidden);
  return [
    { path: "routes.json", bytes: `${JSON.stringify({ site: ready.site, rows: ready.rows })}\n` },
    {
      path: "llms.txt",
      bytes: llms(SITE.title, root, ready.rows, new Set(ready.raw), (ready.site.launchpad as Pad).sections),
    },
    ...sitemaps([
      ...shown.map((row) => ({ route: row.route, date: row.date })),
      ...ready.raw.map((path) => ({ route: `/raw/${path}`, date: null })),
    ]),
    { path: "robots.txt", bytes: robots() },
    { path: TREE, bytes: JSON.stringify(forest(git, ready.raw, rawFile)), type: "application/json" },
    { path: "redirects.json", bytes: `${JSON.stringify(ready.site.redirects, null, 2)}\n` },
  ];
}

export const typed = (path: string, bytes: Uint8Array) =>
  rawKind(path, bytes.length <= HUGE && !IMAGE.has(ext(path)) && ext(path) !== "pdf" && reads(bytes) !== null);

function copies(ready: State): Out[] {
  const raw = ready.raw.map((path) => {
    const bytes = readFileSync(rawFile(path));
    return { path: `raw/${path}`, bytes, type: typed(path, bytes) };
  });
  const pub = walk(join(HOME, "public")).map((file) => ({
    path: relative(join(HOME, "public"), file),
    bytes: readFileSync(file),
  }));
  return [...raw, ...pub];
}

export type Built = {
  out: string;
  designs: string[];
  rows: Row[];
  files: { path: string; hash: string; type?: string; fixed?: boolean }[];
  raw: number;
  dropped: number;
  units: string[];
};

export function withSite(html: string, { designs, pages }: Pick<Ready, "designs" | "pages">) {
  if (!html.includes(TINT))
    throw new Error(`site: shell/index.html must hold ${TINT}, which the build fills with the tint rules`);
  const fill = {
    title: SITE.title,
    prefix: SITE.prefix,
    root,
    domain: new URL(root).host,
    barLight: designs[0]!.bar.light,
    barDark: designs[0]!.bar.dark,
    designs: JSON.stringify(designs.map((one) => [one.name, pages[one.name], one.bar.light, one.bar.dark])),
  };
  const safe = (text: string) => text.replaceAll("&", "&amp;").replaceAll('"', "&quot;").replaceAll("<", "&lt;");
  let out = html.replace(TINT, `<style id="tint">\n${tintCss(SITE.tint)}</style>`);
  if (!out.includes(MARK))
    throw new Error(`site: shell/index.html must hold ${MARK}, which the build fills with site.mark`);
  out = out.replace(MARK, `<link rel="icon" href="${safe(SITE.mark)}" type="image/svg+xml">`);
  const marks: Record<string, string | undefined> = { ...SITE.marks };
  out = out.replace(/<link rel="([^"]+)" data-mark="(\w+)"([^>]*)>\n?/g, (_, rel: string, key: string, rest: string) =>
    marks[key] ? `<link rel="${rel}" href="${safe(marks[key])}"${rest}>\n` : "",
  );
  out = marks.og
    ? out.replaceAll("{og}", safe(marks.og))
    : out
        .split("\n")
        .filter((line) => !line.includes("{og}"))
        .join("\n");
  for (const [key, value] of Object.entries(fill)) {
    if (!out.includes(`{${key}}`))
      throw new Error(`site: shell/index.html must hold {${key}}, which the build fills from site.json`);
    out = out.replaceAll(`{${key}}`, safe(value));
  }
  return out;
}

export async function build(out = DIST): Promise<Built> {
  const seen = new Map<string, string>();
  const dressed = await dress(SITE.title);
  const made = await prime(seen, dressed.designs);
  const ready = whole(made, dressed);
  const entries = [
    join(repo, "shell", "index.html"),
    ...SHELLS,
    join(repo, "kit", "git", "page.ts"),
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
    throw new Error("site: shell/index.html must load /boot.js as its first, classic script");
  page.path = "index.html";
  page.bytes = withSite(
    shell.replace('<script src="/boot.js"></script>', `<script src="${made.boot}"></script>`),
    ready,
  );
  const loose = [...bundle, ...made.assets].filter((one) => one !== page && !HASHED.test(one.path));
  if (loose.length) throw new Error(`site: the bundle named ${loose.map((one) => one.path).join(", ")} without a hash`);
  const hashed = [...faces(seen), ...made.assets, ...bundle.filter((one) => one !== page)].map((one) => ({
    ...one,
    fixed: true,
  }));
  const all: Out[] = [...copies(ready), ...globals(ready), ...hashed, page, { path: "404.html", bytes: page.bytes }];
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
    designs: ready.designs.map((one) => one.name),
    rows: ready.rows,
    files: [...files.values()],
    raw: ready.raw.length,
    dropped: ready.dropped,
    units: ready.units,
  };
}

if (import.meta.main) {
  const done = await build();
  const size = done.files.reduce((sum, one) => sum + statSync(join(done.out, one.path)).size, 0);
  console.log(
    `site: designs ${done.designs.join(" ")}, ${done.rows.length} rows, ${done.raw} raw files, ${done.files.length} files (${(size / 1e6).toFixed(1)} MB) in ${done.out.startsWith(`${repo}/`) ? relative(repo, done.out) : done.out}, wasm ${done.units.join(" ") || "none"}, ${done.dropped} redirects dropped for a target that is no route`,
  );
}
