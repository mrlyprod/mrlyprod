import { expect, setSystemTime, test } from "bun:test";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { shell as gitShell } from "../git/git.ts";
import { build, bytes, forget, globals, graph, guard, jsonScript, probe, render, walk, type Output, type Site, type Spec } from "./build.ts";
import { resolve as link } from "./links.ts";

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

test("icons come through the spec: a glyph grid draws six files and no grid draws none", async () => {
  const rows = ["10101", "01010", "10101", "01010", "10101"];
  const { out } = await made(site(), { icons: { rows, svg: "<svg/>" } } as unknown as Spec);
  const paths = out.map((one) => one.path);
  for (const path of ["favicon.svg", "favicon.png", "favicon.ico", "apple-touch-icon.png", "icon-192.png", "icon-512.png"]) expect(paths).toContain(path);
  expect(out.find((one) => one.path === "favicon.svg")!.bytes).toBe("<svg/>");
  const png = out.find((one) => one.path === "favicon.png")!.bytes as Uint8Array;
  expect([...png.subarray(0, 8)]).toEqual([137, 80, 78, 71, 13, 10, 26, 10]);
  const ico = out.find((one) => one.path === "favicon.ico")!.bytes as Uint8Array;
  expect([...ico.subarray(0, 8)]).toEqual([0, 0, 1, 0, 1, 0, 32, 32]);
  expect([...ico.subarray(22, 30)]).toEqual([137, 80, 78, 71, 13, 10, 26, 10]);
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

const fresh = (name: string, config: Site["config"] = {}) => {
  const home = join(tmpdir(), `kitssg-${name}-${process.pid}`);
  rmSync(home, { recursive: true, force: true });
  mkdirSync(home, { recursive: true });
  writeFileSync(join(home, "site.json"), JSON.stringify(config));
  return home;
};

const CODE = { git: { root: "." }, modes: { "/git/": { mode: "spa", deep: true } } } as Site["config"];

test("a second build drops a dead route's outputs, prunes their empty folders and sweeps a stale asset", async () => {
  const home = fresh("sweep", { assets: [{ path: "ui", out: "ui", hash: true }] });
  const out = join(home, "dist");
  mkdirSync(join(home, "ui"), { recursive: true });
  writeFileSync(join(home, "ui", "a.css"), "b{}");
  const spec = (routes: string[]): Spec => ({
    root: home,
    out,
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
  const home = fresh("gitoff", CODE);
  const out = join(home, "dist");
  writeFileSync(join(home, "README.md"), "# off\n");
  writeFileSync(join(home, "view.js"), "document.title = 1;\n");
  const done = await build({
    root: home,
    out,
    collect: () => ({ routes: [] }),
    render: () => [],
    git: { entry: "view.js" },
    spa: { entries: () => [join(home, "view.js")], page: () => "" },
  });
  expect(done.site.routes.map((one) => one.route).sort()).toEqual(["/git/", "/raw/README.md", "/raw/site.json", "/raw/view.js"]);
  expect(Object.keys(done.manifest)).toEqual(["@spa"]);
  expect(await render(done.site, done.site.routes.find((one) => one.route === "/git/")!, { git: { entry: "view.js" } } as Spec)).toEqual([]);
  expect(existsSync(join(out, "git/index.html"))).toBe(false);
  expect(existsSync(join(out, "raw/README.md"))).toBe(false);
  rmSync(home, { recursive: true, force: true });
});

/* GIT ON */

test("a git block builds one shell under /git/, the tree data beside it and the raw bytes", async () => {
  const home = fresh("giton", CODE);
  const out = join(home, "dist");
  writeFileSync(join(home, "Makefile"), "all:\n");
  writeFileSync(join(home, "view.js"), "document.title = 1;\n");
  const one: Spec = {
    root: home,
    out,
    collect: () => ({ routes: [] }),
    render: () => [],
    git: { entry: "view.js", page: (_site, leaf) => `<main>${leaf.body}</main><script type="module" src="${leaf.island}"></script>` },
    spa: { entries: () => [join(home, "view.js")], page: (site, route, shell) => gitShell(site, route, shell, one) },
  };
  await build(one, { manifest: "manifest.json" });
  expect(walk(join(out, "git")).map((file) => file.slice(out.length + 1))).toEqual(["git/index.html"]);
  expect(readFileSync(join(out, "git/index.html"), "utf8")).toMatch(/^<main><noscript>.+<\/noscript><\/main><script type="module" src="\/view.js"><\/script>$/);
  expect(JSON.parse(readFileSync(join(out, "git.json"), "utf8")).c.map((kid: { n: string }) => kid.n)).toEqual(["Makefile", "site.json", "view.js"]);
  expect(readFileSync(join(out, "raw/Makefile"), "utf8")).toBe("all:\n");
  expect(JSON.parse(readFileSync(join(home, "manifest.json"), "utf8"))["/raw/Makefile"].types).toEqual({ "raw/Makefile": "text/plain; charset=utf-8" });
  rmSync(home, { recursive: true, force: true });
});

/* SHEETS */

test("a sheet is its members' placed bytes in order under one hashed name, and the members ship no more", async () => {
  const home = fresh("sheets", { assets: [{ path: "ui", out: "ui", hash: true, files: ["a.css", "b.css", "f.woff2"] }], sheets: [{ name: "page.css", files: ["b.css", "a.css"] }] });
  mkdirSync(join(home, "ui"));
  writeFileSync(join(home, "ui", "a.css"), "p { color: red; }\n");
  writeFileSync(join(home, "ui", "b.css"), '@font-face { src: url("f.woff2"); }\np { color: blue; }\n');
  writeFileSync(join(home, "ui", "f.woff2"), "font");
  const done = await build({ root: home, out: join(home, "dist"), collect: () => ({ routes: [] }), render: () => [] });
  const face = done.site.asset("f.woff2");
  expect(readFileSync(join(home, "dist", done.site.asset("page.css")), "utf8")).toBe(`@font-face { src: url("${face}"); }\np { color: blue; }\np { color: red; }\n`);
  expect(walk(join(home, "dist/ui")).length).toBe(2);
  expect(() => done.site.asset("a.css")).toThrow(/no asset named a.css/);
  rmSync(home, { recursive: true, force: true });
});

test("a later member of a sheet may carry no @import and no @charset", async () => {
  const home = fresh("sheetsimport", { assets: [{ path: "ui", out: "ui", hash: true, files: ["a.css", "b.css"] }], sheets: [{ name: "page.css", files: ["a.css", "b.css"] }] });
  mkdirSync(join(home, "ui"));
  writeFileSync(join(home, "ui", "a.css"), '@charset "utf-8";\np { color: red; }\n');
  writeFileSync(join(home, "ui", "b.css"), '@import "x.css";\n');
  await expect(build({ root: home, out: join(home, "dist"), collect: () => ({ routes: [] }), render: () => [] })).rejects.toThrow(/b.css carries an @import/);
  rmSync(home, { recursive: true, force: true });
});

/* READS */

test("a file a render ships or looks for outside its inputs repaints the route when it changes or appears", async () => {
  const home = fresh("reads");
  mkdirSync(join(home, "figures"));
  writeFileSync(join(home, "figures", "a.webp"), "one");
  const drawn: string[] = [];
  const spec: Spec = {
    root: home,
    out: join(home, "dist"),
    collect: () => ({ routes: [{ route: "/a/" }, { route: "/b/" }] }),
    render: (_site, route) => {
      drawn.push(route.route);
      const page = { path: `${route.route.slice(1)}index.html`, bytes: "<p>x</p>" };
      if (route.route === "/b/") return [page];
      const late = join(home, "figures", "late.webp");
      return [page, { path: "figures/a.webp", bytes: bytes(join(home, "figures", "a.webp")) }, ...(probe(late) ? [{ path: "figures/late.webp", bytes: bytes(late) }] : [])];
    },
  };
  const round = async () => {
    drawn.length = 0;
    forget();
    const done = await build(spec, { manifest: "manifest.json" });
    return `${drawn.join(" ")}|${done.manifest["/a/"]!.reads}`;
  };
  expect(await round()).toBe("/a/ /b/|figures/a.webp,figures/late.webp");
  expect(await round()).toBe("|figures/a.webp,figures/late.webp");
  writeFileSync(join(home, "figures", "a.webp"), "two");
  expect(await round()).toBe("/a/|figures/a.webp,figures/late.webp");
  expect(readFileSync(join(home, "dist/figures/a.webp"), "utf8")).toBe("two");
  writeFileSync(join(home, "figures", "late.webp"), "late");
  expect(await round()).toBe("/a/|figures/a.webp,figures/late.webp");
  expect(readFileSync(join(home, "dist/figures/late.webp"), "utf8")).toBe("late");
  rmSync(home, { recursive: true, force: true });
});

/* SHAPES */

test("an output that turns from a file into a folder, or back, is written and the old shape is gone", async () => {
  const home = fresh("shapes");
  const spec = (paths: string[]): Spec => ({ root: home, out: join(home, "dist"), collect: () => ({ routes: [{ route: "/a/", data: paths }] }), render: () => paths.map((path) => ({ path, bytes: path })) });
  const files = async (paths: string[]) => (forget(), await build(spec(paths), { manifest: "manifest.json" }), walk(join(home, "dist", "raw")).map((file) => file.slice(home.length + 6)));
  expect(await files(["raw/x"])).toEqual(["raw/x"]);
  expect(await files(["raw/x/y"])).toEqual(["raw/x/y"]);
  expect(await files(["raw/x"])).toEqual(["raw/x"]);
  rmSync(home, { recursive: true, force: true });
});

/* STAMP */

test("the stamp follows the imports of the modules that draw, and no file beside them", async () => {
  const home = fresh("graph");
  writeFileSync(join(home, "draw.ts"), 'import "./part.js";\nimport data from "./data.json";\nimport { test } from "bun:test";\nexport const late = () => import("./late.ts");\n');
  writeFileSync(join(home, "part.js"), "export const part = 1;\n");
  writeFileSync(join(home, "late.ts"), "export const late = 1;\n");
  writeFileSync(join(home, "data.json"), "{}");
  writeFileSync(join(home, "draw.test.ts"), 'import "./draw.ts";\n');
  writeFileSync(join(home, "README.md"), "# kit\n");
  expect(graph([join(home, "draw.ts")]).map((file) => file.slice(home.length + 1))).toEqual(["data.json", "draw.ts", "late.ts", "part.js"]);
  const spec: Spec = { root: home, out: join(home, "dist"), templates: ["draw.ts"], collect: () => ({ routes: [{ route: "/a/" }] }), render: () => [{ path: "a/index.html", bytes: "<p>a</p>" }] };
  const count = async () => (forget(), (await build(spec, { manifest: "manifest.json" })).rendered);
  expect(await count()).toBe(1);
  writeFileSync(join(home, "README.md"), "# kit, edited\n");
  writeFileSync(join(home, "draw.test.ts"), 'import "./draw.ts";\nexport {};\n');
  expect(await count()).toBe(0);
  writeFileSync(join(home, "late.ts"), "export const late = 2;\n");
  expect(await count()).toBe(1);
  rmSync(home, { recursive: true, force: true });
});

test("the stamp holds the calendar year, so a page that prints it repaints after a new year", async () => {
  const home = fresh("year");
  const spec: Spec = { root: home, out: join(home, "dist"), collect: () => ({ routes: [{ route: "/a/" }] }), render: () => [{ path: "a/index.html", bytes: `<p>${new Date().getFullYear()}</p>` }] };
  const round = async (day: string) => (setSystemTime(new Date(day)), forget(), (await build(spec, { manifest: "manifest.json" })).rendered);
  expect(await round("2026-12-30T12:00:00")).toBe(1);
  expect(await round("2026-12-31T12:00:00")).toBe(0);
  expect(await round("2027-01-01T12:00:00")).toBe(1);
  expect(readFileSync(join(home, "dist/a/index.html"), "utf8")).toBe("<p>2027</p>");
  setSystemTime();
  rmSync(home, { recursive: true, force: true });
});

test("a route that appears or goes repaints only itself and the page whose link it answers", async () => {
  const home = fresh("routes");
  const names = ["a", "b"];
  for (const name of [...names, "c"]) writeFileSync(join(home, `${name}.md`), name);
  const drawn: string[] = [];
  const spec: Spec = {
    root: home,
    out: join(home, "dist"),
    collect: () => ({ routes: names.map((name) => ({ route: `/${name}/`, source: join(home, `${name}.md`) })) }),
    render: (site, route) => {
      drawn.push(route.route);
      return [{ path: `${route.route.slice(1)}index.html`, bytes: route.route === "/a/" ? `<a href="${link(site, "a.md", "c.md")}">c</a>` : "<p>x</p>" }];
    },
  };
  const round = async () => {
    drawn.length = 0;
    forget();
    await build(spec, { manifest: "manifest.json" });
    return `${drawn.join(" ")}|${readFileSync(join(home, "dist/a/index.html"), "utf8")}`;
  };
  expect(await round()).toBe('/a/ /b/|<a href="c.md">c</a>');
  names.push("c");
  expect(await round()).toBe('/a/ /c/|<a href="/c/">c</a>');
  expect(await round()).toBe('|<a href="/c/">c</a>');
  names.pop();
  expect(await round()).toBe('/a/|<a href="c.md">c</a>');
  rmSync(home, { recursive: true, force: true });
});

test("a new repo file repaints only the page whose link it answers", async () => {
  const home = fresh("asks", CODE);
  writeFileSync(join(home, "view.js"), "document.title = 1;\n");
  const drawn: string[] = [];
  const spec: Spec = {
    root: home,
    out: join(home, "dist"),
    collect: () => ({ routes: [{ route: "/a/" }, { route: "/b/" }] }),
    render: (site, route) => {
      drawn.push(route.route);
      return [{ path: `${route.route.slice(1)}index.html`, bytes: route.route === "/a/" ? `<a href="${link(site, "page.md", "notes.txt")}">a</a>` : "<p>b</p>" }];
    },
    git: { entry: "view.js", page: () => "" },
    spa: { entries: () => [join(home, "view.js")], page: () => "" },
  };
  const round = async () => {
    drawn.length = 0;
    forget();
    await build(spec, { manifest: "manifest.json" });
    return drawn.join(" ");
  };
  expect(await round()).toBe("/a/ /b/");
  expect(readFileSync(join(home, "dist/a/index.html"), "utf8")).toBe('<a href="notes.txt">a</a>');
  mkdirSync(join(home, "deep"));
  writeFileSync(join(home, "deep/other.txt"), "x\n");
  expect(await round()).toBe("");
  writeFileSync(join(home, "notes.txt"), "x\n");
  expect(await round()).toBe("/a/");
  expect(readFileSync(join(home, "dist/a/index.html"), "utf8")).toBe('<a href="/git/notes.txt">a</a>');
  rmSync(home, { recursive: true, force: true });
});
