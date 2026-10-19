import { beforeAll, expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { folders, state, type Row } from "../scripts/build.ts";
import { dead } from "../scripts/links.ts";
import { dist, out } from "./dist.ts";

/* TREE */

const repo = resolve(import.meta.dir, "../..");
const at = (...parts: string[]) => join(repo, ...parts);
const there = (...parts: string[]) => existsSync(at(...parts));

type Doc = { name: string; lines: string[] };

const read = (name: string): Doc => ({ name, lines: readFileSync(at(name), "utf8").split("\n") });

const plain = (doc: Doc) => {
  const lines: [number, string][] = [];
  let fence = false;
  doc.lines.forEach((line, i) => {
    if (/^\s*(```|~~~)/.test(line)) {
      fence = !fence;
      return;
    }
    if (!fence) lines.push([i + 1, line]);
  });
  return lines;
};

const LINK = /\[[^\]\n]*\]\(([^)\s]+)\)/g;
const SKIP = new Set(["node_modules", "target"]);

const places = ["site/src/pages", "site/src/apps"].flatMap((top) => folders(at(top)).map((id) => `${top}/${id}`)).filter((dir) => there(`${dir}/app.json`));
const pages = places.filter((dir) => there(`${dir}/index.md`)).map((dir) => read(`${dir}/index.md`));
const posts = folders(at("site/src/pages/blog")).filter((slug) => there(`site/src/pages/blog/${slug}/index.md`)).map((slug) => read(`site/src/pages/blog/${slug}/index.md`));
const sources = places.flatMap((dir) => (JSON.parse(readFileSync(at(`${dir}/app.json`), "utf8")).source ?? []) as string[]).map((name) => read(name));
const notes = there("research")
  ? (readdirSync(at("research"), { recursive: true }) as string[])
      .filter((name) => name.endsWith(".md") && !name.split("/").some((part) => part.startsWith(".") || SKIP.has(part)))
      .sort()
      .map((name) => read(`research/${name}`))
  : [];

const routes = new Set(state().rows.map((row) => row.route));

/* PROSE */

test("every markdown link in a page, a post or a source lands, a rooted one on a route", () => {
  const bad: string[] = [];
  for (const doc of [...pages, ...posts, ...sources]) {
    const home = dirname(at(doc.name));
    for (const [n, line] of plain(doc))
      for (const hit of line.matchAll(LINK)) {
        const target = hit[1]!.split("#")[0]!;
        if (target === "" || /^(https?:|mailto:)/.test(target)) continue;
        if (target.startsWith("/")) {
          if (!routes.has(target)) bad.push(`${doc.name}:${n} ${target} is no route`);
        } else if (!existsSync(resolve(home, target))) bad.push(`${doc.name}:${n} ${target}`);
      }
  }
  expect(bad).toEqual([]);
});

test("every link in a research note lands, a rooted page link on a route", () => {
  const bad: string[] = [];
  for (const doc of notes) {
    const home = doc.name === "research/shared/ledger/src/prose.md" ? at("research/shared") : dirname(at(doc.name));
    for (const [n, line] of plain(doc))
      for (const hit of line.replace(/`[^`]*`/g, "").replace(/!\[.*?\]\([^)\s]*\)/g, "").matchAll(LINK)) {
        const target = hit[1]!.split(/[#?]/)[0]!;
        if (target === "" || /^[a-z]+:/.test(target)) continue;
        if (target.startsWith("/")) {
          if (/^\/[\w-]+\/$/.test(target) && !routes.has(target)) bad.push(`${doc.name}:${n} ${target} is no route`);
          continue;
        }
        let path = target;
        try {
          path = decodeURIComponent(target);
        } catch {}
        if (!existsSync(resolve(home, path))) bad.push(`${doc.name}:${n} ${target}`);
      }
  }
  expect(bad).toEqual([]);
});

test("pages, posts and notes keep the house rules: no two blank lines, no em or en dash, a blank line after a heading", () => {
  const bad: string[] = [];
  for (const doc of [...pages, ...posts, ...notes]) {
    doc.lines.forEach((line, i) => {
      if (i > 0 && line === "" && doc.lines[i - 1] === "") bad.push(`${doc.name}:${i + 1} two blank lines`);
    });
    for (const [n, line] of plain(doc)) {
      if (/[–—]/.test(line)) bad.push(`${doc.name}:${n} em-dash or en-dash`);
      if (line.startsWith("#") && n < doc.lines.length && doc.lines[n] !== "") bad.push(`${doc.name}:${n} heading without a blank line`);
    }
  }
  expect(bad).toEqual([]);
});

/* DIST */

let shipped: Row[] = [];

beforeAll(async () => {
  shipped = (await dist()).rows;
}, 60_000);

test("every link on a built page lands on a route, a file of dist or a node of the tree", () => {
  const site = JSON.parse(readFileSync(at("site/site.json"), "utf8")) as { root: string; push: { guard: string[] } };
  const tree = JSON.parse(readFileSync(join(out, "git.json"), "utf8"));
  const found = dead(out, site.root, site.push.guard, tree, new Set(shipped.map((row) => row.route)));
  expect(found.links).toBeGreaterThan(0);
  expect(found.dead.map((one) => `${one.page} ${one.url}`)).toEqual([]);
});

test("every kept redirect goes from a gone path to a route, never through another redirect", () => {
  const known = new Set(shipped.map((row) => row.route));
  const moves = JSON.parse(readFileSync(join(out, "redirects.json"), "utf8")) as Record<string, { to: string }>;
  const stray = Object.entries(moves).flatMap(([from, { to }]) => [
    ...(known.has(from) ? [`${from} is still a route`] : []),
    ...(known.has(to.split("?")[0]!) ? [] : [`${from} -> ${to}, no such route`]),
    ...(to.split("?")[0]! in moves ? [`${from} -> ${to} chains`] : []),
  ]);
  expect(stray).toEqual([]);
});

