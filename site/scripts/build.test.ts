import { beforeAll, expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { figures as named } from "../src/lib/md/md.ts";
import { plates, roster, state, type Built } from "./build.ts";
import SITE from "../site.json";
import { tintCss } from "../src/lib/theme/tint.js";
import { dist, out } from "../test/dist.ts";

const repo = join(import.meta.dir, "..", "..");

const files = (dir: string, at = ""): string[] =>
  readdirSync(join(dir, at), { withFileTypes: true }).flatMap((one) => (one.isDirectory() ? files(dir, `${at}${one.name}/`) : [`${at}${one.name}`]));

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
