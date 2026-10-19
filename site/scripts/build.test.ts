import { beforeAll, expect, test } from "bun:test";
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { figures as named } from "../src/lib/md/md.ts";
import { launchable, launchpad, plates, roster, savers, SHELL, state, type Built, type Host, type Pad } from "./build.ts";
import SITE from "../site.json";
import { tintCss } from "../src/lib/theme/tint.js";
import { dist, out } from "../test/dist.ts";

const repo = join(import.meta.dir, "..", "..");

const files = (dir: string, at = ""): string[] =>
  readdirSync(join(dir, at), { withFileTypes: true }).flatMap((one) => (one.isDirectory() ? files(dir, `${at}${one.name}/`) : [`${at}${one.name}`]));

const host = (id: string, title: string, saver?: true): Host => ({ id, title, saver, file: join(repo, "site", "src", "apps", id, "scene.js") });

const scene = (SAVERS?: unknown) => () => ({ SPEC: [{ key: "hue" }], PAGE: { defaults: { code: "7" } }, SAVERS });

const TWO = [{ id: "one", title: "One", pick: () => ({ hue: 1 }) }, { id: "two", title: "Two", pick: () => ({ code: "9" }) }];

let made: Built;

beforeAll(async () => {
  made = await dist();
}, 60_000);

test("every row has a kind and a title, and a markdown row's source is under dist/raw", () => {
  const { rows } = JSON.parse(readFileSync(join(out, "routes.json"), "utf8")) as { rows: Built["rows"] };
  expect(rows.length).toBe(made.rows.length);
  expect(rows.filter((row) => !row.kind || !row.title)).toEqual([]);
  expect(rows.filter((row) => row.source && !existsSync(join(out, "raw", row.source))).map((row) => row.route)).toEqual([]);
});

test("launchpad.json sets every shown route in exactly one named section and docks each id once, and the build stops on any other", () => {
  const { apps, site } = state();
  expect((site.launchpad as Pad).sections.flatMap((one) => one.apps).sort()).toEqual(launchable(apps).sort());
  const dir = mkdtempSync(join(tmpdir(), "mrly-pad-"));
  const file = join(dir, "launchpad.json");
  const ids = launchable(apps);
  const fails = (sections: unknown, dock: unknown = [[ids[0]]]) => {
    writeFileSync(file, JSON.stringify({ sections, dock }));
    return (() => launchpad(apps, file)) as () => unknown;
  };
  try {
    expect(fails([{ name: "All", apps: ids }])()).toEqual({ sections: [{ name: "All", apps: ids }], dock: [[ids[0]]] });
    expect(fails([{ name: "All", apps: ids.slice(1) }])).toThrow("leaves out");
    expect(fails([{ name: "All", apps: [...ids, ids[0]] }])).toThrow("twice");
    expect(fails([{ name: "All", apps: [...ids, "home"] }])).toThrow("no route");
    expect(fails([{ name: "A", apps: ids }, { name: "a", apps: [] }])).toThrow("twice");
    expect(fails([{ name: "All", apps: ids }], [[ids[0]], [ids[0]]])).toThrow("docks");
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("the built shell fills its tint style with the rules of the site tint", () => {
  expect(readFileSync(join(out, "index.html"), "utf8")).toContain(`<style id="tint">\n${tintCss(SITE.tint)}</style>`);
});

test("every lone image in the markdown, the rows' sources and the research notes, names a live figure", () => {
  const live = new Set(roster().map((one) => one.name));
  const notes = (readdirSync(join(repo, "research"), { recursive: true }) as string[]).filter((path) => path.endsWith(".md") && !/(^|\/)(\.|node_modules\/|target\/)/.test(path)).map((path) => `research/${path}`);
  const sources = [...state().rows.flatMap((row) => (row.source ? [row.source] : [])), ...notes];
  expect(sources.flatMap((source) => named(readFileSync(join(repo, source), "utf8"))).filter((name) => !live.has(name))).toEqual([]);
});

test("the roster is every page and app icon by its id, every post icon as blog-<slug> and every research figure by its stem, each name once", () => {
  const src = join(import.meta.dir, "..", "src");
  const folders = (dir: string) => readdirSync(dir, { withFileTypes: true }).filter((one) => one.isDirectory()).map((one) => one.name);
  const research = folders(join(repo, "research")).flatMap((topic) => (existsSync(join(repo, "research", topic, "figures")) ? readdirSync(join(repo, "research", topic, "figures")).filter((name) => /^[^.]+\.ts$/.test(name)).map((name) => name.slice(0, -3)) : []));
  const routes = ["pages", "apps"].flatMap((root) => folders(join(src, root)).filter((id) => existsSync(join(src, root, id, "icon.js"))));
  const want = [...routes, ...folders(join(src, "pages", "blog")).filter((slug) => existsSync(join(src, "pages", "blog", slug, "icon.js"))).map((slug) => `blog-${slug}`), ...research].sort();
  const names = roster().map((one) => one.name);
  expect(new Set(names).size).toBe(names.length);
  expect([...names].sort()).toEqual(want);
});

test("an app icon is a plate on a clear ground; a post or research figure keeps its ground", () => {
  const all = roster();
  const flat = plates(all);
  expect(flat).toContain("life");
  expect(flat.some((name) => name.startsWith("blog-"))).toBe(false);
  expect(flat.length).toBe(all.filter((one) => one.file.endsWith("icon.js") && !one.name.startsWith("blog-")).length);
});

test("an app row wears its own id, a post blog-<slug>, a hidden app may wear none and a shown one always does", () => {
  const rows = state().rows;
  expect(rows.filter((row) => row.meta.id && row.figure !== null && row.figure !== row.meta.id).map((row) => row.route)).toEqual([]);
  expect(rows.filter((row) => row.meta.prose === "post" && row.figure !== `blog-${row.route.split("/")[2]}`).map((row) => row.route)).toEqual([]);
  expect(rows.filter((row) => !row.hidden && !row.figure).map((row) => row.route)).toEqual([]);
});

test("research ships under /raw/ and git.json, and each topic is a card in research.json with every figure of its folder and a README line in llms.txt", () => {
  const { topics } = JSON.parse(readFileSync(join(out, "research.json"), "utf8")) as { topics: { slug: string; figures: string[] }[] };
  const drawn = (slug: string) => (existsSync(join(repo, "research", slug, "figures")) ? readdirSync(join(repo, "research", slug, "figures")).filter((name) => /^[^.]+\.ts$/.test(name)).map((name) => name.slice(0, -3)).sort() : []);
  expect(topics.filter((one) => one.figures.join() !== drawn(one.slug).join()).map((one) => one.slug)).toEqual([]);
  const llms = readFileSync(join(out, "llms.txt"), "utf8");
  expect(existsSync(join(out, "raw", "research", "README.md"))).toBe(true);
  expect(JSON.parse(readFileSync(join(out, "git.json"), "utf8")).c.some((one: { n: string }) => one.n === "research")).toBe(true);
  expect(topics.map((one) => one.slug)).not.toContain("shared");
  expect(topics.filter((one) => !existsSync(join(out, "raw", "research", one.slug, "README.md")) || !llms.includes(`/raw/research/${one.slug}/README.md`)).map((one) => one.slug)).toEqual([]);
  expect(llms.includes("/raw/research/README.md)")).toBe(topics.length > 0);
});

test("no html outside raw/ but the shell and its 404 copy", () => {
  expect(files(out).filter((path) => path.endsWith(".html") && !path.startsWith("raw/")).sort()).toEqual(["404.html", "index.html"]);
  expect(readFileSync(join(out, "404.html"), "utf8")).toBe(readFileSync(join(out, "index.html"), "utf8"));
});

test("the shell loads its boot script first as a classic script, and every url it names is a route or a file in dist", () => {
  const shell = readFileSync(join(out, "index.html"), "utf8");
  const urls = [...shell.matchAll(/(?:src|href)="(\/[^"]*)"/g)].map((hit) => hit[1]!);
  const routes = new Set(made.rows.map((row) => row.route));
  expect(shell.match(/<script[^>]*>/)![0]).toMatch(/^<script src="\/boot-[0-9a-z]{8}\.js">$/);
  expect(urls.filter((url) => !routes.has(url) && !existsSync(join(out, url)))).toEqual([]);
  expect(urls.filter((url) => /\.(js|css)$/.test(url) && !/-[0-9a-z]{8}\.(js|css)$/.test(url))).toEqual([]);
});

test("a scene's SAVERS is 1 to 3 savers, each a slug once, a title of at most 32 characters and a pick of spec or default keys without the seed, never beside the saver flag", () => {
  const run = (list: unknown, saver?: true) => () => savers([host("a", "A", saver)], scene(list), "a");
  const one = (more: object) => [{ ...TWO[0]!, ...more }];
  expect(run(TWO)().list).toEqual([{ id: "a/one", app: "a", name: "A", title: "One" }, { id: "a/two", app: "a", name: "A", title: "Two" }]);
  expect(run([])).toThrow("1 to 3");
  expect(run([...TWO, ...one({ id: "three" }), ...one({ id: "four" })])).toThrow("1 to 3");
  expect(run(one({ id: "One" }))).toThrow("slug");
  expect(run([...TWO, ...one({})])).toThrow("twice");
  expect(run(one({ title: "" }))).toThrow("1 to 32");
  expect(run(one({ title: "x".repeat(33) }))).toThrow("1 to 32");
  expect(run(one({ pick: {} }))).toThrow("no pick");
  expect(run(one({ pick: () => [] }))).toThrow("plain object");
  expect(run(one({ pick: () => ({ seed: 1 }) }))).toThrow("seed");
  expect(run(one({ pick: () => ({ size: 1 }) }))).toThrow("picks size");
  expect(run(TWO, true)).toThrow("replaces the flag");
});

test("an app with the saver flag and no SAVERS is one saver named after it, and an app with neither has none", () => {
  expect(savers([host("a", "Arcs", true), host("b", "Bees")], scene(), "a").list).toEqual([{ id: "a/a", app: "a", name: "Arcs", title: "Arcs" }]);
});

test("the desk of site.json is an app, meaning its first saver, or one saver in full, and a desk that is no saver stops the build", () => {
  const list = [host("a", "A"), host("b", "B")];
  const load = (file: string) => scene(file.includes("/b/") ? TWO : undefined)();
  expect(savers(list, load, "b").desk).toBe("b/one");
  expect(savers(list, load, "b/two").desk).toBe("b/two");
  expect(() => savers(list, load, "a")).toThrow("no saver");
  expect(() => savers(list, load, "b/three")).toThrow("no saver");
});

test("the savers run the desk app's first, then the apps by name, then the shell's", () => {
  const list = [SHELL, host("c", "Cuts"), host("z", "Zeta"), host("a", "Arcs")];
  expect(savers(list, scene(TWO), "z/two").list.map((one) => one.id)).toEqual(["z/one", "z/two", "a/one", "a/two", "c/one", "c/two", "mrly/one", "mrly/two"]);
});

test("routes.json names the package version, the live units, a shipped file for every licence and the savers, the desk among them", () => {
  const { site } = JSON.parse(readFileSync(join(out, "routes.json"), "utf8")) as { site: { version: string; units: string[]; licences: { name: string; href: string }[]; savers: { id: string }[]; desk: string } };
  expect(site.version).toBe(JSON.parse(readFileSync(join(repo, "bindings", "js", "package.json"), "utf8")).version);
  expect(site.units).toEqual(made.units);
  expect(site.licences.filter((one) => !one.name || !existsSync(join(out, one.href))).map((one) => one.href)).toEqual([]);
  expect(site.licences.some((one) => one.href === "/raw/LICENSE")).toBe(existsSync(join(out, "raw", "LICENSE")));
  expect(site.savers.map((one) => one.id)).toContain(site.desk);
});
