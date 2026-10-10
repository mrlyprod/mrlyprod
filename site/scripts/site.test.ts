import { afterAll, beforeAll, expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { figures as named } from "../kit/md/md.ts";
import { build, roster, state, type Built } from "./site.ts";

const out = join(tmpdir(), `site-${process.pid}`);

const repo = join(import.meta.dir, "..", "..");

const files = (dir: string, at = ""): string[] =>
  readdirSync(join(dir, at), { withFileTypes: true }).flatMap((one) => (one.isDirectory() ? files(dir, `${at}${one.name}/`) : [`${at}${one.name}`]));

let made: Built;

beforeAll(async () => {
  made = await build(out);
}, 60_000);

afterAll(() => rmSync(out, { recursive: true, force: true }));

test("every row has a kind and a title, and a markdown row's source is under dist/raw", () => {
  const { rows } = JSON.parse(readFileSync(join(out, "routes.json"), "utf8")) as { rows: Built["rows"] };
  expect(rows.length).toBe(made.rows.length);
  expect(rows.filter((row) => !row.kind || !row.title)).toEqual([]);
  expect(rows.filter((row) => row.source && !existsSync(join(out, "raw", row.source))).map((row) => row.route)).toEqual([]);
});

test("every lone image in the markdown names a live figure", () => {
  const live = new Set(roster().map((one) => one.name));
  const sources = state().rows.flatMap((row) => (row.source ? [row.source] : []));
  expect(sources.flatMap((source) => named(readFileSync(join(repo, source), "utf8"))).filter((name) => !live.has(name))).toEqual([]);
});

test("nothing from research/ reaches /raw/, git.json or llms.txt", () => {
  expect(existsSync(join(out, "raw", "research"))).toBe(false);
  expect(JSON.parse(readFileSync(join(out, "git.json"), "utf8")).c.some((one: { n: string }) => one.n === "research")).toBe(false);
  expect(readFileSync(join(out, "llms.txt"), "utf8")).not.toContain("/raw/research/");
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
