import { expect, test } from "bun:test";
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { build, type Config, type Route, type Site, type Spec } from "./build.ts";
import { deeps, mode, type Shell } from "./modes.ts";

/* SITE */

const site = (modes: Config["modes"]) => ({ config: { modes } }) as unknown as Site;

const fresh = (name: string) => {
  const home = join(tmpdir(), `kitmodes-${name}-${process.pid}`);
  rmSync(home, { recursive: true, force: true });
  mkdirSync(join(home, "app"), { recursive: true });
  writeFileSync(join(home, "app", "shared.js"), 'export const mark = "one-shared-module";\n');
  writeFileSync(join(home, "app", "a.js"), 'import { mark } from "./shared.js";\ndocument.title = "a" + mark;\n');
  writeFileSync(join(home, "app", "b.js"), 'import { mark } from "./shared.js";\ndocument.title = "b" + mark;\n');
  writeFileSync(join(home, "app", "c.html"), '<!doctype html>\n<html>\n<head>\n<title>c</title>\n</head>\n<body>\n<script type="module" src="./a.js"></script>\n</body>\n</html>\n');
  return home;
};

const seen: Shell[] = [];

const spec = (home: string, routes: Route[], config: Config = {}, entries = ["a.js", "b.js"]): Spec => {
  writeFileSync(join(home, "site.json"), JSON.stringify(config));
  return {
    root: home,
    out: join(home, "dist"),
    collect: () => ({ routes }),
    render: (_site, route) => [{ path: `${route.route.slice(1)}index.html`, bytes: "<p>built</p>" }],
    spa: {
      entries: () => entries.map((name) => join(home, "app", name)),
      page: (_site, _route, shell) => {
        seen.push(shell);
        return shell.html || `<html><head></head><body><script type="module" src="${shell.script}"></script></body></html>`;
      },
    },
  };
};

const read = (home: string, path: string) => readFileSync(join(home, "dist", path), "utf8");

/* RULES */

test("the longest prefix of modes wins, a route's own mode wins over it, and ssg is the default", () => {
  const one = site({ "/git/": { mode: "spa", deep: true }, "/git/raw/": "ssg", "/app/": "ssr" });
  expect(mode(one, { route: "/git/kit/serve.ts" })).toBe("spa");
  expect(mode(one, { route: "/git/raw/a.png" })).toBe("ssg");
  expect(mode(one, { route: "/app/" })).toBe("ssr");
  expect(mode(one, { route: "/about/" })).toBe("ssg");
  expect(mode(one, { route: "/git/", mode: "ssg" })).toBe("ssg");
  expect(deeps(one.config)).toEqual(["/git/"]);
});

test("modes refuses an unknown word, a key with no closing slash and a deep rule that is not spa", () => {
  expect(() => mode(site({ "/a/": "csr" as never }), { route: "/a/" })).toThrow(/ssg, spa or ssr/);
  expect(() => deeps({ modes: { "/a/": { mode: "ssg", deep: true } } })).toThrow(/deep/);
  expect(() => mode(site({ "/demo": "spa" }), { route: "/demos/" })).toThrow(/starts and ends with \//);
});

/* SPA */

test("two spa routes publish one shell each, and the one build that splits what they share once owns its files", async () => {
  const home = fresh("spa");
  const done = await build(spec(home, [{ route: "/a/", mode: "spa", entry: join(home, "app", "a.js") }, { route: "/b/", entry: join(home, "app", "b.js") }, { route: "/c/" }], { modes: { "/b/": "spa" } }));
  expect(done.manifest["/a/"].outputs).toEqual(["a/index.html"]);
  expect(done.manifest["/b/"].outputs).toEqual(["b/index.html"]);
  expect(done.manifest["@spa"].outputs.filter((path) => path.startsWith("app/"))).toEqual(["app/a.js", "app/b.js"]);
  expect(read(home, "a/index.html")).toContain('<script type="module" src="/app/a.js">');
  expect(read(home, "b/index.html")).toContain('<script type="module" src="/app/b.js">');
  expect(read(home, "c/index.html")).toBe("<p>built</p>");
  const chunks = readdirSync(join(home, "dist")).filter((name) => /^lib-.+\.js$/.test(name));
  expect(chunks.filter((name) => read(home, name).includes("one-shared-module")).length).toBe(1);
  expect(read(home, "app/a.js")).not.toContain("one-shared-module");
  expect(read(home, "app/a.js")).toContain(chunks[0]);
  rmSync(home, { recursive: true, force: true });
});

test("a shell names its script, and an html entry hands its bundled page over", async () => {
  const home = fresh("shell");
  seen.length = 0;
  const one = spec(home, [{ route: "/x/", mode: "spa", urls: [{ route: "/x/a/", entry: join(home, "app", "b.js") }, { route: "/x/c/", entry: join(home, "app", "c.html") }] }], {}, ["b.js", "c.html"]);
  one.spa!.place = (path) => path.replace(/^app\//, "x/");
  await build(one);
  const [a, c] = seen;
  expect(a.html).toBe("");
  expect(a.script).toBe("/x/b.js");
  expect(c.html).toContain("<title>c</title>");
  expect(c.script).toMatch(/^\/lib-.+\.js$/);
  expect(read(home, "x/c/index.html")).toBe(c.html);
  expect(existsSync(join(home, "dist", "x/c.html"))).toBe(false);
  rmSync(home, { recursive: true, force: true });
});

test("an spa route with no client entry stops the build", async () => {
  const home = fresh("bare");
  await expect(build(spec(home, [{ route: "/a/", mode: "spa" }]))).rejects.toThrow(/\/a\/ is spa and names no client entry/);
  await expect(build(spec(home, [{ route: "/a/", mode: "spa", entry: join(home, "app", "shared.js") }]))).rejects.toThrow(/spa entries do not list/);
  rmSync(home, { recursive: true, force: true });
});

/* SSR */

test("the build refuses an ssr route", async () => {
  const home = fresh("ssr");
  await expect(build(spec(home, [{ route: "/now/" }], { modes: { "/now/": "ssr" } }))).rejects.toThrow(/\/now\/ is ssr/);
  expect(existsSync(join(home, "dist"))).toBe(false);
  rmSync(home, { recursive: true, force: true });
});

/* DEEP */

test("the build refuses any other output under a deep shell", async () => {
  const home = fresh("deep");
  const deep = { modes: { "/a/": { mode: "spa" as const, deep: true } } };
  const shell = { route: "/a/", entry: join(home, "app", "a.js") };
  const alone = spec(home, [shell], deep);
  alone.spa!.place = (path) => path.replace(/^app\//, "js/");
  await build(alone);
  expect(read(home, "a/index.html")).toContain("/js/a.js");
  await expect(build(spec(home, [shell, { route: "/a/b/", mode: "ssg" }], deep))).rejects.toThrow(/a\/b\/index.html lands under the deep shell \/a\//);
  await expect(build(spec(home, [{ route: "/z/" }], deep))).rejects.toThrow(/no spa route sits there/);
  rmSync(home, { recursive: true, force: true });
});
