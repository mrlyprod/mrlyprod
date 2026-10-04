import { expect, test } from "bun:test";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { graph, render, scan, type Output } from "../kit/ssg/build.ts";
import { spec } from "./site.ts";

const home = resolve(import.meta.dir, "..");
const views = join(home, "demos", "views");
const names = readdirSync(views).filter((name) => existsSync(join(views, name, "index.html"))).sort();
const site = await scan(spec);

const text = (out: Output[], path: string) => {
  const hit = out.find((one) => one.path === path)!;
  return typeof hit.bytes === "string" ? hit.bytes : new TextDecoder().decode(hit.bytes);
};

const drawn = async (route: string, path: string) => text(await render(site, site.routes.find((one) => one.route === route)!, spec), path);

const modules = (html: string) => [...html.matchAll(/<script type="module"[^>]*src="([^"]+)"/g)].map((found) => found[1]!);

test("no script the browser loads beside the chrome reaches the chrome, the site config or the tree", () => {
  const entries = [...names.map((name) => join(views, name, "index.jsx")), ...names.map((name) => join(views, name, "widget.jsx")).filter(existsSync), join(home, "demos", "live.js"), join(home, "lib", "git.js")];
  const banned = ["ui/chrome.js", "ui/chrome.jsx", "ui/config.js", "lib/site.js", "site.json"].map((name) => join(home, name));
  expect(graph(entries).filter((file) => banned.includes(file))).toEqual([]);
});

test("the gallery is drawn at build: every demo is a tile and the chrome is its only script", async () => {
  const html = await drawn("/demos/", "demos/index.html");
  expect([...html.matchAll(/<a class="tile" href="\/demos\/([a-z0-9-]+)\/">/g)].map((found) => found[1]!).sort()).toEqual(names);
  expect(modules(html)).toEqual([site.asset("chrome.js")]);
});

test("a demo page is the chrome around its title, its still and one line for a reader without scripts", async () => {
  const html = await drawn("/demos/sponge/", "demos/sponge/index.html");
  const main = html.match(/<main[\s\S]*<\/main>/)![0];
  expect(html).toContain('<header class="top">');
  expect(main).toContain('<div id="root"></div>');
  expect(main).toContain("<h1>The sponge</h1>");
  expect(main).toContain('srcset="/figures/demo-sponge-dark.webp"');
  expect(main.match(/<noscript>/g)).toHaveLength(1);
  expect(html).toContain('<section class="controls" aria-label="Controls"></section>');
  expect(modules(html)).toHaveLength(2);
  expect(modules(html)[0]).toBe(site.asset("chrome.js"));
  expect(html).not.toContain("application/json");
});

test("an embedded widget says what it needs to a reader without scripts", async () => {
  const html = await drawn("/research/wiki/farey-sequence/", "research/wiki/farey-sequence/index.html");
  expect(html).toMatch(/<figure class="widget"[^>]*><div class="mount"><\/div><noscript><p>[^<]+<\/p><\/noscript>/);
});

test("a demo's shell says none when its page hands the bar nothing, and that demo gets no pane and no button", async () => {
  for (const name of names) {
    const none = /<meta name="bar" content="none">/.test(readFileSync(join(views, name, "index.html"), "utf8"));
    const hands = /\b(?:controls|contents)=\{/.test(readFileSync(join(views, name, "index.jsx"), "utf8"));
    expect([name, none]).toEqual([name, !hands]);
  }
  const route = site.routes.find((one) => one.route === "/demos/sponge/")!;
  const bare = text(await render(site, { ...route, data: { ...(route.data as object), bar: false } }, spec), "demos/sponge/index.html");
  expect(bare).not.toContain('class="pane ');
  expect(bare).not.toContain("data-pane");
  expect(bare).not.toContain('class="scrim"');
});
