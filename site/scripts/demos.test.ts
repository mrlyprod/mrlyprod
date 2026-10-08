import { expect, test } from "bun:test";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { graph, render, scan, type Output } from "../kit/ssg/build.ts";
import { spec } from "./site.ts";

const home = resolve(import.meta.dir, "..");
const views = join(home, "demos", "views");
const names = readdirSync(views).filter((name) => existsSync(join(views, name, "index.html"))).sort();
const rows = JSON.parse(readFileSync(join(home, "apps", "apps.json"), "utf8")) as { id: string; kind: string }[];
const apps = rows.filter((one) => one.kind !== "tool").map((one) => join(home, "apps", one.id, "index.jsx"));
const saver = rows.find((one) => one.kind === "saver")!.id;
const site = await scan(spec);

const text = (out: Output[], path: string) => {
  const hit = out.find((one) => one.path === path)!;
  return typeof hit.bytes === "string" ? hit.bytes : new TextDecoder().decode(hit.bytes);
};

const drawn = async (route: string, path: string) => text(await render(site, site.routes.find((one) => one.route === route)!, spec), path);

const modules = (html: string) => [...html.matchAll(/<script type="module"[^>]*src="([^"]+)"/g)].map((found) => found[1]!);

test("no script the browser loads beside the chrome reaches the chrome, the site config or the tree", () => {
  const entries = [...names.map((name) => join(views, name, "index.jsx")), ...names.map((name) => join(views, name, "widget.jsx")).filter(existsSync), join(home, "demos", "live.js"), join(home, "lib", "git.js"), join(home, "lib", "lock.js"), ...apps];
  const banned = ["ui/chrome.js", "ui/chrome.jsx", "ui/config.js", "lib/site.js", "site.json"].map((name) => join(home, name));
  expect(graph(entries).filter((file) => banned.includes(file))).toEqual([]);
});

test("the gallery is drawn at build: every demo is a tile and the chrome and the router are its only scripts", async () => {
  const html = await drawn("/demos/", "demos/index.html");
  expect([...html.matchAll(/<a class="tile" href="\/demos\/([a-z0-9-]+)\/">/g)].map((found) => found[1]!).sort()).toEqual(names);
  expect(modules(html)).toEqual([site.asset("chrome.js"), site.asset("router.js")]);
});

test("a demo page is the chrome around its title, its still and one line for a reader without scripts", async () => {
  const html = await drawn("/demos/sponge/", "demos/sponge/index.html");
  const main = html.match(/<main[\s\S]*<\/main>/)![0];
  expect(html).toContain('<header class="top">');
  expect(main).toContain('<div id="root" data-island="/demos/sponge/index.js"></div>');
  expect(main).toContain("<h1>The sponge</h1>");
  expect(main).toMatch(/srcset="\/figures\/demo-sponge-dark-[0-9a-f]{8}\.webp"/);
  expect(main.match(/<noscript>/g)).toHaveLength(1);
  expect(html).toContain('<section class="controls" aria-label="Controls"></section>');
  expect(modules(html)).toEqual([site.asset("chrome.js"), site.asset("router.js")]);
  expect(html).not.toContain("application/json");
});

test("an embedded widget says what it needs to a reader without scripts", async () => {
  const html = await drawn("/research/wiki/farey-sequence/", "research/wiki/farey-sequence/index.html");
  expect(html).toMatch(/<figure class="widget"[^>]*><div class="mount"><\/div><noscript><p>[^<]+<\/p><\/noscript>/);
});

test("every page loads the chrome and the router alone; a page preloads the chrome's one import, names each island's entry in its markup and preloads it and the loader", async () => {
  const pages = [
    ["/research/wiki/farey-sequence/", ["/demos/farey/widget.js"]],
    ["/research/wiki/sierpinski-carpet/", ["/live.js"]],
    ["/research/claims/automata/", [site.asset("claims.js")]],
    ["/stats/", [site.asset("stats.js")]],
    ["/menu/", [site.asset("search.js")]],
    ["/demos/sponge/", ["/demos/sponge/index.js"]],
    [`/${saver}/`, [`/${saver}/index.js`]],
    ["/about/", []],
  ] as const;
  for (const [route, entries] of pages) {
    const html = await drawn(route, `${route.slice(1)}index.html`);
    expect([route, [...new Set([...html.matchAll(/ data-island="([^"]+)"/g)].map((found) => found[1]!))]]).toEqual([route, [...entries]]);
    expect([route, modules(html)]).toEqual([route, [site.asset("chrome.js"), site.asset("router.js")]]);
    expect([route, [...html.matchAll(/<link rel="modulepreload" href="([^"]+)">/g)].map((found) => found[1]!)]).toEqual([route, [site.asset("word.js"), ...(entries.length ? [site.asset("islands.js"), ...entries] : [])]]);
    expect(html).not.toMatch(/<script>|onsubmit=/);
  }
});

test("every entry a page can name exports the contract's mount and unmount", () => {
  const entries = [...names.map((name) => join(views, name, "index.jsx")), ...readdirSync(views).map((name) => join(views, name, "widget.jsx")).filter(existsSync), join(home, "demos", "live.js"), join(home, "lib", "git.js"), join(home, "ui", "stats.js"), join(home, "ui", "claims.js"), join(home, "ui", "search.js"), ...apps];
  const scan = new Bun.Transpiler({ loader: "jsx" });
  for (const file of entries) expect([file, scan.scan(readFileSync(file, "utf8")).exports.filter((name) => name === "mount" || name === "unmount").sort()]).toEqual([file, ["mount", "unmount"]]);
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
