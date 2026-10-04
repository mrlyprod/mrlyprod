import { expect, test } from "bun:test";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { shell as gitShell } from "../git/git.ts";
import { build, forget, globals, guard, jsonScript, render, walk, type Output, type Site, type Spec } from "./build.ts";

/* SITE */

const site = (config: Site["config"] = {}) =>
  ({
    config: {
      title: "Demo",
      root: "https://demo.test",
      llms: {
        about: "A demo tree.",
        links: [
          { href: "/raw/README.md", name: "README", note: "the readme" },
          { href: "/git/", name: "Code", note: "the tree" },
          { href: "/faq/", name: "FAQ" },
          { href: "/nowhere/", name: "Nowhere" },
        ],
      },
      ...config,
    },
    routes: [
      { route: "/", name: "Home", at: "2026-01-01" },
      { route: "/404.html", kind: "missing", hidden: true },
      { route: "/cart/", kind: "cart", hidden: true },
      { route: "/faq/", kind: "page", name: "FAQ", at: "2026-04-04" },
      { route: "/git/", kind: "git", name: "demo", sitemap: true, at: "2026-02-02" },
      { route: "/raw/README.md", kind: "raw", name: "README.md", hidden: true, sitemap: true, at: "2026-03-03", urls: [{ route: "/raw/README.md" }] },
      { route: "/raw/src/a.rs", kind: "raw", name: "a.rs", hidden: true, sitemap: true, at: "2026-03-05", urls: [{ route: "/raw/src/a.rs" }] },
    ],
    copies: [],
    made: new Set<string>(),
  }) as unknown as Site;

const made = async (one = site(), spec = {} as unknown as Spec) => {
  const out = await globals(one, spec);
  const find = (path: string) => out.find((item: Output) => item.path === path)?.bytes as string | undefined;
  const children = out.filter((item) => item.path.startsWith("sitemap-"));
  return { out, index: find("sitemap.xml")!, children, sitemap: children.map((item) => item.bytes).join(""), robots: find("robots.txt")!, llms: find("llms.txt") };
};

/* SITEMAP */

test("the sitemap carries every shown route, the code viewer once and every raw path, and no hidden one", async () => {
  const { sitemap } = await made();
  expect(sitemap).toContain("<loc>https://demo.test/</loc><lastmod>2026-01-01</lastmod>");
  expect(sitemap).toContain("<loc>https://demo.test/faq/</loc><lastmod>2026-04-04</lastmod>");
  expect(sitemap).toContain("<loc>https://demo.test/git/</loc><lastmod>2026-02-02</lastmod>");
  expect(sitemap).not.toContain("/git/README.md");
  expect(sitemap).toContain("<loc>https://demo.test/raw/README.md</loc><lastmod>2026-03-03</lastmod>");
  expect(sitemap).not.toContain("404.html");
  expect(sitemap).not.toContain("/cart/");
  expect(sitemap.match(/<url>/g)!.length).toBe(5);
});

test("the sitemap indexes one child per first segment, dated by its newest url, the lone pages sharing pages.xml", async () => {
  const { index, children } = await made();
  expect(index).toBe(
    '<?xml version="1.0" encoding="UTF-8"?>\n<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n' +
      "<sitemap><loc>https://demo.test/sitemap-pages.xml</loc><lastmod>2026-04-04</lastmod></sitemap>\n" +
      "<sitemap><loc>https://demo.test/sitemap-raw.xml</loc><lastmod>2026-03-05</lastmod></sitemap>\n</sitemapindex>\n",
  );
  const locs = (path: string) => [...(children.find((item) => item.path === path)!.bytes as string).matchAll(/<loc>([^<]+)<\/loc>/g)].map((m) => m[1]);
  expect(locs("sitemap-raw.xml")).toEqual(["https://demo.test/raw/README.md", "https://demo.test/raw/src/a.rs"]);
  expect(locs("sitemap-pages.xml")).toEqual(["https://demo.test/", "https://demo.test/faq/", "https://demo.test/git/"]);
  const crowded = site();
  crowded.routes.push({ route: "/pages/", name: "Pages" }, { route: "/pages/one/", name: "One" });
  await expect(made(crowded)).rejects.toThrow("sitemap-pages.xml");
});

test("a route or a file that claims a sitemap's path stops the build", async () => {
  const one = site();
  one.made.add("sitemap-raw.xml");
  await expect(made(one)).rejects.toThrow("sitemap-raw.xml is the sitemap's");
});

/* ROBOTS */

test("robots names the crawlers it welcomes and points at the sitemap", async () => {
  const { robots } = await made();
  for (const agent of ["GPTBot", "ClaudeBot", "Claude-Web", "CCBot", "Google-Extended", "anthropic-ai", "PerplexityBot"]) {
    expect(robots).toContain(`User-agent: ${agent}\nAllow: /`);
  }
  expect(robots).toContain("User-agent: *\nAllow: /");
  expect(robots).toContain("Sitemap: https://demo.test/sitemap.xml");
});

/* LLMS */

test("llms.txt says what the site is and links only what the site publishes", async () => {
  const { llms } = await made();
  expect(llms).toContain("# Demo");
  expect(llms).toContain("A demo tree.");
  expect(llms).toContain("- [README](https://demo.test/raw/README.md): the readme");
  expect(llms).toContain("- [Code](https://demo.test/git/): the tree");
  expect(llms).toContain("- [FAQ](https://demo.test/faq/)");
  expect(llms).not.toContain("Nowhere");
});

test("the spec's llms hook lays its sections under the header and its legend", async () => {
  const one = site({ llms: { about: "A demo tree.", legend: "Links are sources." } });
  const rows = [{ href: "/raw/README.md#top", name: "README", note: "the readme" }, { href: "/faq/" }, { href: "/nowhere/", name: "Nowhere" }];
  const { llms } = await made(one, { llms: () => [{ name: "Read", rows }, { name: "Gone", rows: rows.slice(2) }] } as unknown as Spec);
  expect(llms).toBe("# Demo\n\n> https://demo.test\n\nA demo tree.\n\nLinks are sources.\n\n## Read\n\n- [README](https://demo.test/raw/README.md#top): the readme\n- [/faq/](https://demo.test/faq/)\n");
});

test("no llms block in site.json means no llms.txt", async () => {
  const { llms } = await made(site({ llms: undefined }));
  expect(llms).toBeUndefined();
});

/* JSON-LD */

test("a headline that closes a script tag cannot close the ld+json block", () => {
  const out = jsonScript({ headline: "</script><img src=x onerror=alert(1)>" });
  expect(out).toBe('<script type="application/ld+json">{"headline":"\\u003c/script>\\u003cimg src=x onerror=alert(1)>"}</script>');
});

/* ICONS */

test("icons come through the spec: a glyph grid draws five files and no grid draws none", async () => {
  const rows = ["10101", "01010", "10101", "01010", "10101"];
  const { out } = await made(site(), { icons: { rows, svg: "<svg/>" } } as unknown as Spec);
  const paths = out.map((one) => one.path);
  for (const path of ["favicon.svg", "favicon.png", "apple-touch-icon.png", "icon-192.png", "icon-512.png"]) expect(paths).toContain(path);
  expect(out.find((one) => one.path === "favicon.svg")!.bytes).toBe("<svg/>");
  const png = out.find((one) => one.path === "favicon.png")!.bytes as Uint8Array;
  expect([...png.subarray(0, 8)]).toEqual([137, 80, 78, 71, 13, 10, 26, 10]);
  expect((await made()).out.map((one) => one.path)).not.toContain("favicon.png");
});

/* GUARD */

test("the guard passes a listed inline script, json data and a raw mirror, and throws on an unlisted one", () => {
  const known = new Set(["const a=1"]);
  expect(() => guard("index.html", "<script data-boot>const a=1</script>", known)).not.toThrow();
  expect(() => guard("index.html", '<script type="application/ld+json">{"a":1}</script>', known)).not.toThrow();
  expect(() => guard("index.html", '<script id="props" type="application/json">{"a":1}</script>', known)).not.toThrow();
  expect(() => guard("raw/site/demos/index.html", "<script>const b=2</script>", known)).not.toThrow();
  expect(() => guard("about/index.html", "<script>const b=2</script>", known)).toThrow(/boot list/);
});

/* BUILD */

const fresh = (name: string) => {
  const home = join(tmpdir(), `kitssg-${name}-${process.pid}`);
  rmSync(home, { recursive: true, force: true });
  mkdirSync(home, { recursive: true });
  return home;
};

test("prepare runs before the scan reads a bundle", async () => {
  const home = fresh("prepare");
  const out = join(home, "dist");
  const done = await build({
    root: home,
    out,
    config: { assets: [{ path: "ui", out: "ui", hash: true, files: ["a.css"] }] },
    prepare: () => {
      mkdirSync(join(home, "ui"), { recursive: true });
      writeFileSync(join(home, "ui", "a.css"), "b{}");
    },
    collect: () => ({ routes: [] }),
    render: () => [],
  });
  expect(done.site.styles).toEqual([done.site.asset("a.css")]);
  expect(existsSync(join(out, done.site.asset("a.css")))).toBe(true);
  rmSync(home, { recursive: true, force: true });
});

test("a second build drops a dead route's outputs, prunes their empty folders and sweeps a stale asset", async () => {
  const home = fresh("sweep");
  const out = join(home, "dist");
  mkdirSync(join(home, "ui"), { recursive: true });
  writeFileSync(join(home, "ui", "a.css"), "b{}");
  const spec = (routes: string[]): Spec => ({
    root: home,
    out,
    config: { assets: [{ path: "ui", out: "ui", hash: true }] },
    collect: () => ({ routes: routes.map((route) => ({ route })) }),
    render: (_site, route) => [{ path: `${route.route.slice(1)}index.html`, bytes: "<p>x</p>" }],
  });
  const first = await build(spec(["/deep/page/"]), { manifest: "manifest.json" });
  const was = first.site.asset("a.css");
  expect(existsSync(join(out, "deep/page/index.html"))).toBe(true);
  writeFileSync(join(home, "ui", "a.css"), "c{}");
  forget();
  const second = await build(spec([]), { manifest: "manifest.json" });
  expect(existsSync(join(out, was))).toBe(false);
  expect(existsSync(join(out, second.site.asset("a.css")))).toBe(true);
  expect(existsSync(join(out, "deep"))).toBe(false);
  expect(second.removed).toBe(2);
  rmSync(home, { recursive: true, force: true });
});

/* GIT OFF */

test("a spec with no git page still collects the repo and renders none of it", async () => {
  const home = fresh("gitoff");
  const out = join(home, "dist");
  writeFileSync(join(home, "README.md"), "# off\n");
  writeFileSync(join(home, "view.js"), "document.title = 1;\n");
  const done = await build({
    root: home,
    out,
    config: { git: { root: "." }, modes: { "/git/": { mode: "spa", deep: true } } },
    collect: () => ({ routes: [] }),
    render: () => [],
    git: { entry: "view.js" },
    spa: { entries: () => [join(home, "view.js")], page: () => "" },
  });
  expect(done.site.routes.map((one) => one.route).sort()).toEqual(["/git/", "/raw/README.md", "/raw/view.js"]);
  expect(Object.keys(done.manifest)).toEqual(["@spa"]);
  expect(await render(done.site, done.site.routes.find((one) => one.route === "/git/")!, { git: { entry: "view.js" } } as Spec)).toEqual([]);
  expect(existsSync(join(out, "git/index.html"))).toBe(false);
  expect(existsSync(join(out, "raw/README.md"))).toBe(false);
  rmSync(home, { recursive: true, force: true });
});

/* GIT ON */

test("a git block builds one shell under /git/, the tree data beside it and the raw bytes", async () => {
  const home = fresh("giton");
  const out = join(home, "dist");
  writeFileSync(join(home, "Makefile"), "all:\n");
  writeFileSync(join(home, "view.js"), "document.title = 1;\n");
  const one: Spec = {
    root: home,
    out,
    config: { git: { root: "." }, modes: { "/git/": { mode: "spa", deep: true } } },
    collect: () => ({ routes: [] }),
    render: () => [],
    git: { entry: "view.js", page: (_site, leaf) => `<main>${leaf.body}</main><script type="module" src="${leaf.scripts![0]}"></script>` },
    spa: { entries: () => [join(home, "view.js")], page: (site, route, shell) => gitShell(site, route, shell, one) },
  };
  await build(one, { manifest: "manifest.json" });
  expect(walk(join(out, "git")).map((file) => file.slice(out.length + 1))).toEqual(["git/index.html"]);
  expect(readFileSync(join(out, "git/index.html"), "utf8")).toBe('<main></main><script type="module" src="/view.js"></script>');
  expect(JSON.parse(readFileSync(join(out, "git.json"), "utf8")).c.map((kid: { n: string }) => kid.n)).toEqual(["Makefile", "view.js"]);
  expect(readFileSync(join(out, "raw/Makefile"), "utf8")).toBe("all:\n");
  expect(JSON.parse(readFileSync(join(home, "manifest.json"), "utf8"))["/raw/Makefile"].types).toEqual({ "raw/Makefile": "text/plain; charset=utf-8" });
  rmSync(home, { recursive: true, force: true });
});
