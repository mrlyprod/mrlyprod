import { describe, expect, test } from "bun:test";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { assets, cache, changes, IMMUTABLE, kind, mine, REVALIDATE, rules, seal, spread, staged, sweep, typing, type Lister } from "./push.ts";
import { build, forget, globals, type Manifest, type Spec } from "./ssg/build.ts";

const SHOP = rules(["(^|/)lib-[^/]+\\.js$", "(^|/)lib-[^/]+\\.css$", "(^|/)js/[^/]+\\.js$", "\\.wasm$", "-[0-9a-f]{8}\\.[^./]+$"]);

const NET = rules(["(^|/)lib-[^/]+\\.js$", "(^|/)lib-[^/]+\\.css$", "\\.wasm$", "-[0-9a-f]{8}\\.[^./]+$"]);

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

const fake: Lister = { list: async ({ prefix }) => TREE[prefix] ?? page([], []) };

describe("push", () => {
  test("a hashed name is immutable for a year, every other name revalidates", () => {
    expect(cache("js/main.js", SHOP)).toBe(IMMUTABLE);
    expect(cache("ui/tokens-8a0bcf6d.css", SHOP)).toBe(IMMUTABLE);
    expect(cache("index.html", SHOP)).toBe(REVALIDATE);
    expect(cache("js/main.js", NET)).toBe(REVALIDATE);
  });

  test("a type declared in the manifest wins over the extension", () => {
    expect(typing(MANIFEST).get("props.json")).toBe("application/json");
    expect(typing(MANIFEST).has("index.html")).toBe(false);
    expect(kind("index.html")).toBe("text/html; charset=utf-8");
  });

  test("every output points back at the record that wrote it", () => {
    expect([...spread(MANIFEST)]).toEqual([
      ["index.html", "/"],
      ["props.json", "/"],
      ["shop/index.html", "/shop/"],
      ["ui/tokens-8a0bcf6d.css", "@ui/tokens-8a0bcf6d.css"],
    ]);
  });

  test("a re-rendered output uploads only when its bytes moved", () => {
    const old: Manifest = { "/": { hash: "a", at: "2026-09-21", outputs: ["index.html", "props.json"], sums: { "index.html": "s1", "props.json": "s2" } } };
    const next: Manifest = { "/": { hash: "b", at: "2026-09-21", outputs: ["index.html", "props.json"] } };
    expect(changes(old, next, [], (path) => (path === "index.html" ? "s1" : "s3"))).toEqual(["props.json"]);
    expect(next["/"]!.sums).toEqual({ "index.html": "s1", "props.json": "s3" });
  });

  test("a forced push reseals every output, so bytes the fingerprint missed still upload", () => {
    const old: Manifest = { "/": { hash: "a", at: "2026-09-21", outputs: ["index.html", "props.json"], sums: { "index.html": "s1", "props.json": "s2" } } };
    const sealed = (path: string) => (path === "index.html" ? "s1" : "s3");
    const same = (): Manifest => ({ "/": { ...old["/"]! } });
    expect(changes(old, same(), [], sealed)).toEqual([]);
    expect(changes(old, same(), [], sealed, true)).toEqual(["props.json"]);
  });

  test("hashed files upload first, then the other files, then every page, each group in its own order and its own batches", () => {
    const type = (path: string) => (path === "git/shell" ? "text/html; charset=utf-8" : kind(path));
    const order = staged(["index.html", "props.json", "git/shell", "demos/x/index.js", "shop/index.html", "lib-a1.js", "ui/router-8a0bcf6d.js", "search.json"], type, (path) => cache(path, NET));
    expect(order).toEqual([["lib-a1.js", "ui/router-8a0bcf6d.js"], ["props.json", "demos/x/index.js", "search.json"], ["index.html", "git/shell", "shop/index.html"]]);
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

/* SPA */

test("an edit only one spa route's entry sees uploads its changed files and deletes its old chunk, whatever the other route's hash", async () => {
  const home = join(tmpdir(), `kitpush-spa-${process.pid}`);
  rmSync(home, { recursive: true, force: true });
  mkdirSync(join(home, "app"), { recursive: true });
  const file = (name: string) => join(home, "app", name);
  writeFileSync(file("only.js"), 'export const mark = "first-cut";\n');
  writeFileSync(file("a.js"), 'const { mark } = await import("./only.js");\ndocument.title = mark;\n');
  writeFileSync(file("b.js"), 'document.title = "b";\n');
  writeFileSync(join(home, "site.json"), "{}");
  const spec: Spec = {
    root: home,
    out: join(home, "dist"),
    collect: () => ({ routes: [{ route: "/a/", mode: "spa", entry: file("a.js"), inputs: [file("a.js"), file("only.js")] }, { route: "/b/", mode: "spa", entry: file("b.js"), inputs: [file("b.js")] }] }),
    render: () => [],
    spa: { entries: () => [file("a.js"), file("b.js")], page: (_site, _route, shell) => `<script type="module" src="${shell.script}"></script>` },
  };
  const round = async (old: Manifest) => {
    forget();
    writeFileSync(join(home, "manifest.json"), JSON.stringify(old));
    const done = await build(spec, { manifest: "manifest.json", verify: false });
    const next: Manifest = { ...done.manifest, ...assets(await globals(done.site, spec), old) };
    const upload = changes(old, next, [], (path) => seal(readFileSync(join(home, "dist", path)), "", ""));
    const want = spread(next);
    return { next, upload, remove: [...spread(old).keys()].filter((path) => !want.has(path)), rendered: done.rendered };
  };
  const first = await round({});
  const chunk = first.upload.find((path) => /^lib-.+\.js$/.test(path))!;
  expect(first.upload.sort()).toEqual(["a/index.html", "app/a.js", "app/b.js", "b/index.html", chunk, "robots.txt", "sitemap-pages.xml", "sitemap.xml"].sort());
  expect((await round(first.next)).upload).toEqual([]);
  writeFileSync(file("only.js"), 'export const mark = "second-cut";\n');
  const second = await round(first.next);
  const fresh = second.upload.find((path) => /^lib-.+\.js$/.test(path))!;
  expect(fresh).not.toBe(chunk);
  expect(second.upload.sort()).toEqual(["app/a.js", fresh].sort());
  expect(second.remove).toEqual([chunk]);
  expect(readFileSync(join(home, "dist", "app/a.js"), "utf8")).toContain(fresh);
  rmSync(home, { recursive: true, force: true });
});
