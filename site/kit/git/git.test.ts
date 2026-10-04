import { afterAll, expect, test } from "bun:test";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { collect, explorer, forest, mime, render, shell } from "./git.ts";
import { seti } from "../code/seti/seti.ts";
import { paint } from "./code.ts";
import { block, decode, dirRoute, draw, fileRoute, href, lang, link, owner, rawPath, type Tools, type Wood } from "./view.ts";
import type { Route, Site, Spec } from "../ssg/build.ts";
import type { Shell } from "../ssg/modes.ts";

const shiki = await import("@shikijs/core").then(
  () => true,
  () => false,
);

/* TREE */

const home = join(tmpdir(), `mrlygit-${process.pid}`);

mkdirSync(join(home, "src"), { recursive: true });
writeFileSync(join(home, "README.md"), "# demo\n\nA tree.\n");
writeFileSync(join(home, "LICENSE"), "MIT\n");
writeFileSync(join(home, ".gitignore"), "dist\n");
writeFileSync(join(home, "src", "a.rs"), "fn main() {}\n");

afterAll(() => rmSync(home, { recursive: true, force: true }));

const modes = { "/git/": { mode: "spa" as const, deep: true } };

const site = (git: unknown, root = home, ships = new Map<string, string>()) => ({ root, config: git ? { git, modes } : {}, ships }) as unknown as Site;

const repo = { root: ".", slug: "mrlyprod/mrlyprod" };

/* RULES */

test("a file routes to its own path, with or without an extension", () => {
  expect(fileRoute("crates/a/src/lib.rs")).toBe("/git/crates/a/src/lib.rs");
  expect(fileRoute("LICENSE")).toBe("/git/LICENSE");
  expect(rawPath("LICENSE")).toBe("raw/LICENSE");
  expect(rawPath("x/y.png")).toBe("raw/x/y.png");
});

test("a directory route ends in a slash", () => {
  expect(dirRoute("")).toBe("/git/");
  expect(dirRoute("crates/a")).toBe("/git/crates/a/");
});

test("a raw path names its own file route", () => {
  expect(owner("/raw/src/a.rs")).toBe("/git/src/a.rs");
  expect(owner("/git/src/a.rs")).toBe(null);
});

test("an href percent-encodes a path and html-escapes the rest", () => {
  expect(href(fileRoute("notes/a b#c&d.md"))).toBe("/git/notes/a%20b%23c&amp;d.md");
  expect(href(dirRoute("notes/a b"))).toBe("/git/notes/a%20b/");
});

test("a raw svg is served as text, a raw png keeps its image type", () => {
  expect(mime("files/figures/logo.svg", false)).toBe("text/plain; charset=utf-8");
  expect(mime("files/figures/logo.png", false)).toBe("image/png");
});

test("a language comes from the extension", () => {
  expect(lang("src/a.rs")).toBe("rust");
  expect(lang("LICENSE")).toBe("text");
});

test("a file named after an object's own machinery is plain text with a plain icon", () => {
  expect(lang("x.constructor")).toBe("text");
  expect(seti("x.constructor")).toBe("si");
  expect(seti("constructor")).toBe("si");
  expect(mime("x.constructor", false)).toBe("application/octet-stream");
});

test("a path that is not valid percent-encoding is kept as written", () => {
  expect(decode("a%20b")).toBe("a b");
  expect(decode("100%")).toBe("100%");
  expect(decode("%E0%A4%A")).toBe("%E0%A4%A");
});

test("a repo-relative link lands in the viewer, an image or a pdf on its raw bytes, a script url nowhere", () => {
  expect(link("crates/a", "src/lib.rs")).toBe("/git/crates/a/src/lib.rs");
  expect(link("crates/a", "../b/README.md#top")).toBe("/git/crates/b/README.md#top");
  expect(link("docs", "shot.png")).toBe("/raw/docs/shot.png");
  expect(link("docs", "paper.pdf")).toBe("/raw/docs/paper.pdf");
  expect(link("docs", "https://mrly.net")).toBe("https://mrly.net");
  expect(link("docs", "java\tscript:alert(1)")).toBe("#");
});

/* COLLECT */

test("no git block in site.json means no routes", () => {
  expect(collect(site(null))).toEqual({ routes: [] });
});

test("a git block with no deep /git/ rule stops the scan", () => {
  const bare = { root: home, config: { git: repo }, ships: new Map() } as unknown as Site;
  expect(() => collect(bare)).toThrow(/deep spa route/);
});

test("a git block routes one shell for the viewer and the raw bytes of every file, and no page per file", () => {
  const { routes } = collect(site(repo), { entry: "lib/git.js" });
  expect(routes.map((one) => one.route)).toEqual(["/git/", "/raw/.gitignore", "/raw/LICENSE", "/raw/README.md", "/raw/src/a.rs"]);
  expect(routes[0]).toEqual({ route: "/git/", kind: "git", name: "mrlyprod", entry: join(home, "lib/git.js"), sitemap: true });
  const one = routes.find((route) => route.route === "/raw/src/a.rs")!;
  expect(one.hidden).toBe(true);
  expect(one.sitemap).toBe(true);
  expect(one.urls).toEqual([{ route: "/raw/src/a.rs", name: "a.rs" }]);
});

test("the explorer draws the root and leaves every folder lazy", () => {
  const one = site(repo);
  collect(one);
  expect(explorer(one)).toEqual([
    {
      name: "mrlyprod",
      href: "/git/",
      lazy: "",
      nodes: [
        { name: "src", href: "/git/src/", lazy: "src", nodes: [] },
        { name: ".gitignore", href: "/git/.gitignore", icon: "si si-git" },
        { name: "LICENSE", href: "/git/LICENSE", icon: "si si-license" },
        { name: "README.md", href: "/git/README.md", icon: "si si-markdown" },
      ],
    },
  ]);
});

test("the tree data names the repo and gives every file its icon and its size", () => {
  const one = site(repo);
  collect(one);
  expect(forest(one)).toEqual({
    base: "/git/",
    name: "mrlyprod",
    slug: "mrlyprod/mrlyprod",
    branch: "main",
    c: [
      { n: "src", k: "d", c: [{ n: "a.rs", k: "f", i: "rust", s: 13 }] },
      { n: ".gitignore", k: "f", i: "git", s: 5 },
      { n: "LICENSE", k: "f", i: "license", s: 4 },
      { n: "README.md", k: "f", i: "markdown", s: 16 },
    ],
  });
});

/* RAW */

const raw = (routes: Route[], path: string) => routes.find((one) => one.route === `/raw/${path}`)!;

test("a text file with no extension is its own raw object, typed as text", () => {
  const one = site(repo);
  const { routes } = collect(one);
  const out = render(one, raw(routes, "LICENSE"), {} as Spec);
  expect(out.map((item) => [item.path, item.type])).toEqual([["raw/LICENSE", "text/plain; charset=utf-8"]]);
  expect(new TextDecoder().decode(out[0].bytes as Uint8Array)).toBe("MIT\n");
});

test("a binary the site already serves loses its raw copy and the tree data points at the served url", () => {
  const shelf = join(tmpdir(), `mrlyraw-${process.pid}`);
  mkdirSync(shelf, { recursive: true });
  writeFileSync(join(shelf, "logo.png"), Uint8Array.from([137, 80, 78, 71, 13, 10, 26, 10, 0, 1]));
  writeFileSync(join(shelf, "seal.bin"), Uint8Array.from([0, 1, 2, 3]));
  const one = site({ root: "." }, shelf);
  const { routes } = collect(one);
  const hooks = { served: (_s: Site, path: string) => (path === "logo.png" ? "/figures/logo.png" : null) };
  const spec = { git: hooks } as unknown as Spec;
  expect(render(one, raw(routes, "logo.png"), spec)).toEqual([]);
  expect(render(one, raw(routes, "seal.bin"), spec).map((item) => [item.path, item.type])).toEqual([["raw/seal.bin", "application/octet-stream"]]);
  expect(forest(one, hooks)!.c).toEqual([
    { n: "logo.png", k: "f", i: "image", s: 10, u: "/figures/logo.png" },
    { n: "seal.bin", k: "f", s: 4 },
  ]);
  rmSync(shelf, { recursive: true, force: true });
});

test("a file the build ships verbatim loses its raw copy whatever its type", () => {
  const shelf = join(tmpdir(), `mrlyship-${process.pid}`);
  mkdirSync(shelf, { recursive: true });
  writeFileSync(join(shelf, "make.py"), "print(1)\n");
  const one = site({ root: "." }, shelf, new Map([[join(shelf, "make.py"), "/blog/post/make.py"]]));
  const { routes } = collect(one);
  expect(render(one, raw(routes, "make.py"), {} as Spec)).toEqual([]);
  expect(forest(one)!.c[0].u).toBe("/blog/post/make.py");
  rmSync(shelf, { recursive: true, force: true });
});

/* SHELL */

test("the shell is the site's page around one noscript line and the viewer's one script", () => {
  const one = site(repo);
  const { routes } = collect(one, { entry: "lib/git.js" });
  const seen: unknown[] = [];
  const spec = { git: { page: (_s: Site, leaf: unknown) => (seen.push(leaf), "<html>") } } as unknown as Spec;
  expect(shell(one, routes[0], { script: "/lib/git.js" } as Shell, spec)).toBe("<html>");
  expect(seen[0]).toMatchObject({ route: "/git/", name: "mrlyprod", code: true, scripts: ["/lib/git.js"] });
  expect((seen[0] as { body: string }).body).toBe('<noscript><p>The code viewer draws in the browser and needs JavaScript. The same files are on <a href="https://github.com/mrlyprod/mrlyprod">GitHub</a>.</p></noscript>');
});

/* VIEW */

const wood: Wood = {
  base: "/git/",
  name: "demo",
  slug: "acme/demo",
  branch: "main",
  c: [
    { n: "docs", k: "d", c: [{ n: "paper.pdf", k: "f", s: 900 }, { n: "shot.png", k: "f", i: "image", s: 2048, u: "/figures/shot.png" }] },
    { n: "big.rs", k: "f", i: "rust", s: 2 << 20 },
    { n: "blob.bin", k: "f", s: 4 },
    { n: "Makefile", k: "f", s: 9 },
    { n: "README.md", k: "f", i: "markdown", s: 30 },
  ],
};

const held: Record<string, string | Uint8Array> = {
  "/raw/README.md": "# demo\n\nSee [the shot](docs/shot.png) and [make](Makefile).\n",
  "/raw/Makefile": "all:\n\techo\n",
  "/raw/blob.bin": Uint8Array.from([0, 1, 2, 3]),
};

const asked: string[] = [];

const tools: Tools = {
  load: async (url) => {
    asked.push(url);
    const hit = held[url];
    return hit === undefined ? null : typeof hit === "string" ? new TextEncoder().encode(hit) : hit;
  },
  md: (text, to) => text.replace(/\]\(([^)]+)\)/g, (_, url: string) => `](${to(url)})`),
};

test("a listing draws its folders and files with their sizes at once, and its README above them when it has loaded", async () => {
  const view = draw(wood, "");
  expect(view).toMatchObject({ path: "", name: "demo", found: true });
  expect(view.html).toContain('<p class="lead">1 directory and 4 files in demo.</p>');
  expect(view.html).toContain('<li class="dir"><span class="ico" aria-hidden="true"></span><a href="/git/docs/">docs/</a><span class="n">2 items</span></li>');
  expect(view.html).toContain('<a href="/git/Makefile">Makefile</a><span class="n">9 B</span>');
  expect(view.html).toContain('<a href="https://github.com/acme/demo/tree/main">GitHub</a>');
  expect(view.html).not.toContain("readme");
  const full = (await view.more!(tools))!;
  expect(full.html.indexOf('<div class="prose readme">')).toBeLessThan(full.html.indexOf('<ul class="files">'));
  expect(full.html).toContain("[the shot](/figures/shot.png) and [make](/git/Makefile)");
});

test("a code file shows its bar first, then its numbered lines, and hands its text over for painting", async () => {
  const view = draw(wood, "Makefile");
  expect(view.html).toContain('<a href="/raw/Makefile">Raw</a> · <a href="https://github.com/acme/demo/blob/main/Makefile">GitHub</a>');
  expect(view.html).not.toContain('class="code');
  const full = (await view.more!(tools))!;
  expect(full.html).toContain('<p class="lead">9 B · text · 2 lines</p>');
  expect(full.html).toContain('<span class="line" id="L2"><a class="n" href="#L2">2</a><span class="t">\techo</span></span>');
  expect(full.code).toEqual({ text: "all:\n\techo\n", lang: "text" });
});

test("a markdown file is rendered by the site's pipeline with its links resolved in the viewer", async () => {
  const full = (await draw(wood, "README.md").more!(tools))!;
  expect(full.html).toContain('<div class="prose readme"># demo');
  expect(full.html).toContain("(/figures/shot.png)");
  expect(full.code).toBeUndefined();
});

test("a README link to an image the site serves elsewhere lands on that url, not on a raw object that is not there", () => {
  expect(link("", "docs/shot.png", wood)).toBe("/figures/shot.png");
  expect(link("docs", "paper.pdf#p2", wood)).toBe("/raw/docs/paper.pdf#p2");
});

test("a file whose bytes do not load says so and offers the raw link", async () => {
  const full = (await draw({ ...wood, c: [{ n: "gone.rs", k: "f", s: 5 }] }, "gone.rs").more!(tools))!;
  expect(full.html).toContain('gone.rs did not load. <a href="/raw/gone.rs">Open the raw file</a>');
});

test("an image and a pdf are shown from their bytes' url and fetch nothing", () => {
  asked.length = 0;
  const shot = draw(wood, "docs/shot.png");
  expect(shot.html).toContain('<figure class="shot"><img src="/figures/shot.png" alt="shot.png"></figure>');
  expect(shot.more).toBeUndefined();
  const paper = draw(wood, "docs/paper.pdf");
  expect(paper.html).toContain('<embed class="doc" src="/raw/docs/paper.pdf" type="application/pdf">');
  expect(paper.more).toBeUndefined();
  expect(asked).toEqual([]);
});

test("a file over 1 MB is a raw link only", () => {
  const view = draw(wood, "big.rs");
  expect(view.html).toContain('<a href="/raw/big.rs">Download big.rs</a> · 2.0 MB');
  expect(view.more).toBeUndefined();
});

test("a binary is a download link once its bytes say so", async () => {
  const full = (await draw(wood, "blob.bin").more!(tools))!;
  expect(full.html).toContain('<a href="/raw/blob.bin">Download blob.bin</a> · 4 B');
  expect(full.html).toContain('<p class="lead">4 B · bin</p>');
});

test("a folder asked for without its slash is its listing, and an unknown path is a not-found view", () => {
  expect(draw(wood, "docs")).toMatchObject({ path: "docs/", name: "docs", found: true });
  const lost = draw(wood, "nope/<x>.rs");
  expect(lost).toMatchObject({ name: "Not found", found: false });
  expect(lost.html).toContain("No nope/&lt;x&gt;.rs in demo.");
  expect(lost.more).toBeUndefined();
});

/* CODE */

test("a code block numbers its lines and a huge one stays plain", () => {
  const one = block("let a = 1;\nlet b = 2;\n", "typescript");
  expect(one).toContain('<span class="line" id="L2"><a class="n" href="#L2">2</a>');
  expect(one).not.toContain("L3");
  expect(one).toContain('<div class="code d2"');
  expect(block("x\n".repeat(120000), "text")).toContain('<div class="code plain"');
});

test("the gutter class counts the digits of the last line", () => {
  expect(block("x\n".repeat(400), "text")).toContain('<div class="code d3"');
});

/* PAINT */

test.skipIf(!shiki)("a rust snippet paints one string per line", async () => {
  const out = await paint("fn main() {\n    let a = 1;\n}\n", "rust");
  expect(out).not.toBeNull();
  expect(out!.length).toBe(3);
  expect(out![1]).toContain('<span class="tk-keyword">let</span>');
  expect(out!.join("")).not.toContain("style=");
});

test("an unknown extension paints nothing", async () => {
  expect(await paint("hello\n", lang("notes.bin"))).toBeNull();
  expect(await paint("hello\n", "text")).toBeNull();
});
