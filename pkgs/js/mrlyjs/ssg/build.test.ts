import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { globals, walk, type Output, type Site, type Spec } from "./build.ts";

/* SITE */

const site = () =>
  ({
    config: {
      title: "Demo",
      root: "https://demo.test",
      llms: {
        about: "A demo tree.",
        links: [
          { href: "/raw/README.md", name: "README", note: "the readme" },
          { href: "/git/", name: "Code", note: "the tree" },
          { href: "/nowhere/", name: "Nowhere" },
        ],
      },
    },
    routes: [
      { route: "/", name: "Home", at: "2026-01-01" },
      { route: "/404.html", kind: "missing", hidden: true },
      { route: "/git/", kind: "gitdir", name: "demo", sitemap: true, at: "2026-02-02" },
      {
        route: "/git/README.md",
        kind: "gitfile",
        name: "README.md",
        hidden: true,
        sitemap: true,
        at: "2026-03-03",
        urls: [{ route: "/git/README.md" }, { route: "/raw/README.md" }],
      },
    ],
  }) as unknown as Site;

const made = async () => {
  const out = await globals(site(), {} as unknown as Spec);
  const find = (path: string) => (out.find((one: Output) => one.path === path)!.bytes as string);
  return { sitemap: find("sitemap.xml"), robots: find("robots.txt"), llms: find("llms.txt") };
};

/* SITEMAP */

test("the sitemap carries every git route and every raw path with its own date", async () => {
  const { sitemap } = await made();
  expect(sitemap).toContain("<loc>https://demo.test/git/</loc><lastmod>2026-02-02</lastmod>");
  expect(sitemap).toContain("<loc>https://demo.test/git/README.md</loc><lastmod>2026-03-03</lastmod>");
  expect(sitemap).toContain("<loc>https://demo.test/raw/README.md</loc><lastmod>2026-03-03</lastmod>");
  expect(sitemap).toContain("<loc>https://demo.test/</loc><lastmod>2026-01-01</lastmod>");
  expect(sitemap).not.toContain("404.html");
  expect(sitemap.match(/<url>/g)!.length).toBe(4);
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
  expect(llms).not.toContain("Nowhere");
});

/* CSS */

const selectors = (css: string) => {
  const out: string[] = [];
  let depth = 0;
  let buf = "";
  for (const c of css.replace(/\/\*[\s\S]*?\*\//g, "")) {
    if (c === "{") {
      if (depth === 0) out.push(buf.trim().replace(/\s+/g, " "));
      depth++;
      buf = "";
    } else if (c === "}") {
      depth--;
      buf = "";
    } else if (depth === 0) buf += c;
  }
  return out;
};

test("the kit css never repeats a top-level selector with another rule between", () => {
  const home = join(import.meta.dir, "..", "ui");
  for (const file of walk(home, false).filter((f) => f.endsWith(".css"))) {
    const list = selectors(readFileSync(file, "utf8"));
    const last = new Map<string, number>();
    list.forEach((sel, n) => {
      const was = last.get(sel);
      if (was !== undefined && list.slice(was + 1, n).some((other) => other !== sel)) throw new Error(`${file}: '${sel}' at ${was} and ${n}`);
      last.set(sel, n);
    });
  }
});
