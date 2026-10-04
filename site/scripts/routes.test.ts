import { expect, test } from "bun:test";
import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { globals, render, scan, type Output } from "../kit/ssg/build.ts";
import { crumbs } from "../ui/crumbs.js";
import { spec } from "./site.ts";

const home = resolve(import.meta.dir, "..");
const site = await scan(spec);

const text = (out: Output[], path: string) => {
  const hit = out.find((one) => one.path === path)!;
  return typeof hit.bytes === "string" ? hit.bytes : new TextDecoder().decode(hit.bytes);
};

const drawn = async (route: string) => text(await render(site, site.routes.find((one) => one.route === route)!, spec), `${route.slice(1)}index.html`);

const main = (html: string) => html.match(/<main[\s\S]*<\/main>/)![0];

const sub = (html: string) => html.match(/<div class="subheader">([\s\S]*?)<\/div><div class="panes">/)![1]!;

const slot = (html: string) => sub(html).match(/<div class="actions">([\s\S]*)<\/div>$/)![1]!;

const CRUMB = /<li><a href="([^"]+)"( aria-current="page")?>([^<]*)<\/a><\/li>/g;

const CHROME = ["/menu/", "/", "/cart/"];

const hrefs = (html: string) => [...html.matchAll(/<a class="tile" href="([^"]+)"/g)].map((found) => found[1]!);

const ICON = /<a class="tile" href="([^"]+)"><picture><source data-dark srcset="\/figures\/([a-z0-9-]+)-dark\.webp"/g;

const OG = /<meta property="og:image" content="[^"]*\/(?:figures\/([a-z0-9-]+)-dark\.png|og\.png)">/;

const SETTING = /data-(?:theme-toggle|font-pick|tint-pick|saver-pick)/g;

const HEAD = /<h([23]) id="[^"]+">.*?<\/h\1>/g;

const RIGHT = /<aside class="pane right"[^>]*>([\s\S]*?)<\/aside>/;

const LIST = /^<nav class="contents" aria-label="Contents"><details><summary>Contents<\/summary><ol>(?:<li class="h[23]"><a href="#[^"]+">.*?<\/a><\/li>)+<\/ol><\/details><\/nav>$/;

const pages = readdirSync(join(home, "pages")).filter((name) => name.endsWith(".md")).map((name) => `/${name.slice(0, -3)}/`).sort();

const apps = (JSON.parse(readFileSync(join(home, "apps", "apps.json"), "utf8")) as { id: string }[]).map((one) => `/${one.id}/`).sort();

type Door = { name: string; href?: string; nodes?: Door[] };

const tree = site.config.tree as Door[];

const filled: Record<string, string[]> = { Pages: pages, Apps: apps, Elsewhere: [...(site.config.socials as Door[]).map((one) => one.href!), `mailto:${site.config.contact}`] };

test("every written page and every app is a route at the root, and none sits under /pages/, /apps/ or /tools/", () => {
  for (const route of pages) expect([route, site.routes.find((one) => one.route === route)?.kind]).toEqual([route, "page"]);
  for (const route of apps) expect([route, site.routes.some((one) => one.route === route)]).toEqual([route, true]);
  expect(site.routes.filter((one) => /^\/(pages|apps|tools)\//.test(one.route))).toEqual([]);
});

test("the menu is two levels: the tree's doors and one tile per folder, then each folder a section of the same page with no url of its own", async () => {
  const html = main(await drawn("/menu/"));
  const [top, ...rest] = html.slice(html.indexOf('<div class="menu">')).split("<section ");
  const folders = tree.filter((node) => !node.href);
  expect(hrefs(top!)).toEqual(tree.map((node) => node.href ?? `#${node.name.toLowerCase()}`));
  expect(rest).toHaveLength(folders.length);
  folders.forEach((folder, n) => {
    expect(rest[n]!.startsWith(`id="${folder.name.toLowerCase()}" aria-label="${folder.name}"><h2>${folder.name}</h2>`)).toBe(true);
    expect(hrefs(rest[n]!).sort()).toEqual([...(folder.nodes?.map((node) => node.href!) ?? filled[folder.name]!)].sort());
  });
  expect([top, ...rest].join("")).not.toContain("<p>");
});

test("what the menu is fingerprinted by names only the routes of the tree, the pages folder and the app list, so a new leaf page leaves it alone", () => {
  const route = site.routes.find((one) => one.route === "/menu/")!;
  const kept = JSON.stringify([route.data, route.inputs ?? [], route.source ?? ""]);
  const doors = new Set([...tree.flatMap((node) => [node, ...(node.nodes ?? [])]).flatMap((node) => node.href ?? []), ...pages, ...apps]);
  const named = site.routes.map((one) => one.route).filter((path) => kept.includes(`"${path}"`));
  expect(named.length).toBeGreaterThan(10);
  expect(named.filter((path) => !doors.has(path))).toEqual([]);
  expect(site.routes.filter((one) => one.route.endsWith("/")).length - named.length).toBeGreaterThan(200);
});

test("a menu entry's icon is the figure its own page ships, site-page when the page ships none, and a folder's is its door's or its own", async () => {
  const icons = new Map([...main(await drawn("/menu/")).matchAll(ICON)].map((found) => [found[1]!, found[2]!]));
  const mine = [...icons.keys()].filter((href) => site.routes.some((one) => one.route === href));
  expect(mine.length).toBeGreaterThan(15);
  for (const href of mine) {
    const page = text(await render(site, site.routes.find((one) => one.route === href)!, spec), href.endsWith("/") ? `${href.slice(1)}index.html` : href.slice(1));
    expect([href, icons.get(href)]).toEqual([href, page.match(OG)![1] ?? "site-page"]);
  }
  for (const href of [...icons.keys()].filter((one) => !mine.includes(one) && !one.startsWith("#"))) expect([href, icons.get(href)]).toEqual([href, "site-page"]);
  expect(icons.get("#research")).toBe(icons.get("/research/")!);
  expect(icons.get("#apps")).toBe("site-apps");
});

test("every file the menu's Root folder links is one the build writes at the root", async () => {
  const written = new Set([...(await globals(site, spec, true)).map((one) => one.path), ...site.routes.map((one) => one.route.slice(1))]);
  const files = tree.find((node) => node.name === "Root")!.nodes!.map((node) => node.href!.slice(1));
  expect(files.length).toBeGreaterThan(3);
  expect(files.filter((path) => path.includes("/") || !written.has(path))).toEqual([]);
});

test("the four settings sit on the settings page alone, beside a line for a reader without scripts", async () => {
  const page = main(await drawn("/settings/"));
  expect([...new Set(page.match(SETTING))].sort()).toEqual(["data-font-pick", "data-saver-pick", "data-theme-toggle", "data-tint-pick"]);
  expect(page.match(/<noscript>/g)).toHaveLength(1);
  for (const route of ["/", "/about/", "/research/wiki/farey-sequence/", "/demos/sponge/"]) expect((await drawn(route)).match(SETTING)).toBeNull();
});

test("the stats page carries one line for a reader without scripts, above its two mounts", async () => {
  const page = main(await drawn("/stats/"));
  expect(page.match(/<noscript><p>[^<]+<\/p><\/noscript>/g)).toHaveLength(1);
  expect(page.indexOf("<noscript>")).toBeLessThan(page.indexOf("data-stats"));
});

test("a page gets the right pane, its button and the scrim from three headings up, the contents one closed details", async () => {
  const seen = new Set<boolean>();
  for (const route of ["/about/", "/settings/", "/research/wiki/farey-sequence/", "/research/notes/apollonian/", "/math/"]) {
    const html = await drawn(route);
    const heads = main(html).match(HEAD)?.length ?? 0;
    const pane = html.match(RIGHT)?.[1];
    seen.add(heads >= 3);
    expect([route, pane !== undefined]).toEqual([route, heads >= 3]);
    expect(slot(html).includes('data-pane="right"')).toBe(heads >= 3);
    expect(html.match(/data-pane=/g)?.length ?? 0).toBe(heads >= 3 ? 1 : 0);
    expect(html.includes('<div class="scrim">')).toBe(heads >= 3);
    expect(html).not.toContain('class="pane left"');
    if (pane === undefined) continue;
    expect(pane).toMatch(LIST);
    expect(pane.match(/<li /g)).toHaveLength(heads);
  }
  expect(seen.size).toBe(2);
});

test("the code viewer's shell holds the explorer, no right pane, and the right button hidden with the rule the viewer shows it by", async () => {
  const html = await drawn("/git/");
  expect(html).toContain('<nav class="pane left" id="left"');
  expect(html).not.toContain('class="pane right"');
  expect(slot(html)).toMatch(/^<button[^>]*data-pane="left"[^>]*aria-label="Files">.*<\/button><button[^>]*data-pane="right"[^>]* hidden="" data-few="3">.*<\/button>$/);
  expect(html).toContain('<div class="scrim">');
});

test("a page's crumbs are the folders of its own path, the last one the page; home and the 404 page have none", async () => {
  for (const route of ["/about/", "/research/", "/research/wiki/farey-sequence/", "/demos/sponge/", "/git/"]) {
    const found = [...sub(await drawn(route)).matchAll(CRUMB)];
    const parts = route.split("/").filter(Boolean);
    expect(found.map((one) => one[1]!)).toEqual(parts.map((_, n) => `/${parts.slice(0, n + 1).join("/")}/`));
    expect(found.map((one) => one[3]!)).toEqual(parts);
    expect(found.map((one) => Boolean(one[2]))).toEqual(parts.map((_, n) => n === parts.length - 1));
  }
  expect(sub(await drawn("/"))).not.toContain("crumbs");
  const lost = text(await render(site, site.routes.find((one) => one.route === "/404.html")!, spec), "404.html");
  expect(sub(lost)).toBe('<div class="actions"></div>');
});

test("every folder above a page is a page of its own, so every crumb resolves", () => {
  const known = new Set(site.routes.flatMap((one) => [one.route, ...(one.urls ?? []).map((url) => url.route)]));
  const pages = [...known].filter((route) => route.endsWith("/"));
  expect(pages.length).toBeGreaterThan(200);
  expect(pages.flatMap((route) => crumbs(route).map((crumb) => crumb.href).filter((href) => !known.has(href)))).toEqual([]);
});

test("a file path ends on the file itself, so the code viewer's crumbs follow the address bar", () => {
  expect(crumbs("/git/site/ui/chrome.js").map((one) => one.href)).toEqual(["/git/", "/git/site/", "/git/site/ui/", "/git/site/ui/chrome.js"]);
  expect(crumbs("/git/a%20b/").map((one) => one.name)).toEqual(["git", "a b"]);
});

test("the header is three plain links, to the menu, home and the cart, and the subheader repeats none of them", async () => {
  for (const route of ["/", "/about/", "/research/wiki/farey-sequence/", "/demos/sponge/", "/git/"]) {
    const html = await drawn(route);
    const head = html.match(/<header class="top">([\s\S]*?)<\/header>/)![1]!;
    expect([...head.matchAll(/<a\b[^>]*href="([^"]+)"/g)].map((one) => one[1]!)).toEqual(CHROME);
    expect(head.replace(/<svg[\s\S]*?<\/svg>/g, "").replace(/<a\b[^>]*>|<\/a>/g, "")).toBe("");
    expect([...sub(html).matchAll(/href="([^"]+)"/g)].map((one) => one[1]!).filter((href) => CHROME.includes(href))).toEqual([]);
  }
});
