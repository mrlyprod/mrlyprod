import { describe, expect, test } from "bun:test";
import { assets, changes, GONE, headers, IMMUTABLE, keep, mine, SHORT, spread, staged, sweep, typing, type Lister, type Manifest } from "./push.ts";

const GUARD = ["cdn/", "art/", "automator/automator.json", "stats/stats.json"];

const MANIFEST: Manifest = {
  "/": { hash: "a", at: "2026-09-21", outputs: ["index.html", "props.json"], types: { "props.json": "application/json" } },
  "/shop/": { hash: "b", at: "2026-09-21", outputs: ["shop/index.html"] },
  "@ui/tokens-8a0bcf6d.css": { hash: "c", at: "2026-09-21", outputs: ["ui/tokens-8a0bcf6d.css"] },
};

const page = (contents: string[], prefixes: string[]) => ({
  contents: contents.map((key) => ({ key })),
  commonPrefixes: prefixes.map((prefix) => ({ prefix })),
  isTruncated: false,
  nextContinuationToken: null,
});

const TREE: Record<string, ReturnType<typeof page>> = {
  "site/": page(["site/index.html"], ["site/art/", "site/cdn/", "site/shop/", "site/ui/"]),
  "site/shop/": page(["site/shop/index.html"], []),
  "site/ui/": page(["site/ui/tokens-8a0bcf6d.css"], []),
  "site/art/": page(["site/art/never.png"], []),
  "site/cdn/": page(["site/cdn/never.js"], []),
};

const cache = headers([
  { path: "lib-a1b2c3d4.js", hash: "", fixed: true },
  { path: "fonts/sans-afc7a910.woff2", hash: "", fixed: true },
  { path: "sitemap-research.xml", hash: "" },
  { path: "index.html", hash: "" },
]);

const fake: Lister = { list: async ({ prefix }) => TREE[prefix] ?? page([], []) };

describe("push", () => {
  test("a file the build hashed is immutable for a year, every other file revalidates, whatever its name", () => {
    expect([cache("lib-a1b2c3d4.js"), cache("fonts/sans-afc7a910.woff2")]).toEqual([IMMUTABLE, IMMUTABLE]);
    expect([cache("sitemap-research.xml"), cache("index.html"), cache("raw/site/a-12345678.ts")]).toEqual([SHORT, SHORT, SHORT]);
  });

  test("a file marked immutable with no hash in its name stops the push", () => {
    expect(() => headers([{ path: "boot.js", hash: "", fixed: true }])).toThrow(/boot\.js/);
  });

  test("a type declared in the manifest wins over the extension", () => {
    expect(typing(MANIFEST).get("props.json")).toBe("application/json");
    expect(typing(MANIFEST).has("index.html")).toBe(false);
  });

  test("every output points back at the record that wrote it", () => {
    expect([...spread(MANIFEST)]).toEqual([
      ["index.html", "/"],
      ["props.json", "/"],
      ["shop/index.html", "/shop/"],
      ["ui/tokens-8a0bcf6d.css", "@ui/tokens-8a0bcf6d.css"],
    ]);
  });

  test("a file whose bytes did not move keeps its record and its day", () => {
    const old = assets([{ path: "a.js", hash: "h1" }], {}, "2026-10-01");
    expect(assets([{ path: "a.js", hash: "h1", type: "text/javascript" }], old, "2026-10-10")).toEqual({ "@a.js": { hash: "h1", at: "2026-10-01", outputs: ["a.js"], types: { "a.js": "text/javascript" } } });
    expect(assets([{ path: "a.js", hash: "h2" }], old, "2026-10-10")["@a.js"]!.at).toBe("2026-10-10");
  });

  test("a re-rendered output uploads only when its bytes moved", () => {
    const old: Manifest = { "/": { hash: "a", at: "2026-09-21", outputs: ["index.html", "props.json"], sums: { "index.html": "s1", "props.json": "s2" } } };
    const next: Manifest = { "/": { hash: "b", at: "2026-09-21", outputs: ["index.html", "props.json"] } };
    expect(changes(old, next, [], (path) => (path === "index.html" ? "s1" : "s3"))).toEqual(["props.json"]);
    expect(next["/"]!.sums).toEqual({ "index.html": "s1", "props.json": "s3" });
  });

  test("a forced push reseals every output, so bytes the hash missed still upload", () => {
    const old: Manifest = { "/": { hash: "a", at: "2026-09-21", outputs: ["index.html", "props.json"], sums: { "index.html": "s1", "props.json": "s2" } } };
    const sealed = (path: string) => (path === "index.html" ? "s1" : "s3");
    const same = (): Manifest => ({ "/": { ...old["/"]! } });
    expect(changes(old, same(), [], sealed)).toEqual([]);
    expect(changes(old, same(), [], sealed, true)).toEqual(["props.json"]);
  });

  test("hashed files upload first, then the other files, then the shell and its copy", () => {
    expect(staged(["index.html", "routes.json", "lib-a1b2c3d4.js", "raw/LICENSE", "404.html", "fonts/sans-afc7a910.woff2"], cache)).toEqual([
      ["lib-a1b2c3d4.js", "fonts/sans-afc7a910.woff2"],
      ["routes.json", "raw/LICENSE"],
      ["index.html", "404.html"],
    ]);
  });

  test("every removed file, a page of the old route manifest too, is kept a week from the day it left", () => {
    const first = keep(MANIFEST, new Map(), "2026-10-08");
    expect(Object.keys(first).sort()).toEqual([GONE + "index.html", GONE + "props.json", GONE + "shop/index.html", GONE + "ui/tokens-8a0bcf6d.css"]);
    expect(first[GONE + "shop/index.html"]).toEqual({ hash: "b", at: "2026-10-08", outputs: ["shop/index.html"] });
    expect(keep(first, new Map(), "2026-10-14")).toEqual(first);
    expect(keep(first, new Map(), "2026-10-15")).toEqual({});
    expect(keep(first, spread(MANIFEST), "2026-10-09")).toEqual({});
  });

  test("the guard owns a prefix and a single key alike", () => {
    expect(mine("shop/index.html", GUARD)).toBe(true);
    expect(mine("cdn/reel.js", GUARD)).toBe(false);
    expect(mine("stats/stats.json", GUARD)).toBe(false);
    expect(mine("stats/index.html", GUARD)).toBe(true);
  });

  test("a sweep strips the prefix and never descends into a guarded folder", async () => {
    expect(await sweep(fake, "site/", GUARD)).toEqual(["index.html", "shop/index.html", "ui/tokens-8a0bcf6d.css"]);
  });

  test("a sweep at the bucket root walks from the empty prefix", async () => {
    const flat: Lister = { list: async ({ prefix }) => (prefix === "" ? page(["index.html"], ["cdn/"]) : page([], [])) };
    expect(await sweep(flat, "", ["cdn/"])).toEqual(["index.html"]);
  });
});
