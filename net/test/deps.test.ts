import { expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";

const REPO = resolve(import.meta.dir, "..", "..");

const CORE = ["net/spec", "net/kernel", "net/sdk", "net/shell", "net/system", "net/doors"];

const OWN = /^(react|react-dom)(\/|$)|^(site|app|apps):|^@mrlyprod\/|^node:|^bun(:|$)/;

const BARE = /(?:\bfrom|\bimport)\s*\(?\s*(['"])([^'".][^'"\n]*)\1/g;

const APP = /^net\/system\/([^/]+)\//;

type Manifest = { dependencies?: Record<string, string>; devDependencies?: Record<string, string> };

const read = (file: string): Manifest => (existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : {});

const ROOT = read(join(REPO, "package.json"));

const files = CORE.flatMap((top) =>
  readdirSync(join(REPO, top), { encoding: "utf8", recursive: true })
    .filter((path) => /\.(ts|tsx)$/.test(path) && !/\.test\./.test(path) && !path.includes("node_modules"))
    .map((path) => join(REPO, top, path)),
);

const named = (spec: string) => (spec.startsWith("@") ? spec.split("/").slice(0, 2).join("/") : spec.split("/")[0]!);

const pulled = [
  ...new Set(
    files.flatMap((file) =>
      [...readFileSync(file, "utf8").matchAll(BARE)]
        .map(([, , spec]) => spec)
        .filter((spec) => !OWN.test(spec))
        .map((spec) => `${relative(REPO, file)} -> ${named(spec)}`),
    ),
  ),
].sort();

const apps = readdirSync(join(REPO, "net", "system"), { withFileTypes: true })
  .filter((one) => one.isDirectory() && existsSync(join(REPO, "net", "system", one.name, "app.json")))
  .map((one) => one.name)
  .sort();

const manifest = (app: string) => read(join(REPO, "net", "system", app, "package.json"));

const declared = (app: string) => {
  const one = manifest(app);
  return [...Object.keys(one.dependencies ?? {}), ...Object.keys(one.devDependencies ?? {})].map((name) =>
    name.replace(/^@types\//, ""),
  );
};

const pulls = (app: string) =>
  [...new Set(pulled.filter((one) => one.startsWith(`net/system/${app}/`)).map((one) => one.split(" -> ")[1]!))].sort();

const owned = (one: string) => {
  const app = one.match(APP)?.[1];
  return app !== undefined && declared(app).includes(one.split(" -> ")[1]!);
};

test("MrlyNet's code pulls in React and nothing else; a system app pulls only what its own package.json declares", () => {
  expect(pulled.filter((one) => !owned(one))).toEqual([]);
});

test("a system app's package.json declares exactly the packages its folder imports, a type package as @types/<name>", () => {
  expect(
    Object.fromEntries(apps.filter((app) => declared(app).length).map((app) => [app, declared(app).sort()])),
  ).toEqual(Object.fromEntries(apps.filter((app) => pulls(app).length).map((app) => [app, pulls(app)])));
});

test("the root package.json installs every runtime dependency a system app declares, at the same range", () => {
  const rows = apps.flatMap((app) =>
    Object.entries(manifest(app).dependencies ?? {}).map(([name, range]) => ({ app, name, range })),
  );
  expect(
    rows
      .filter((row) => ROOT.dependencies?.[row.name] !== row.range)
      .map((row) => `${row.app}: ${row.name}@${row.range}`),
  ).toEqual([]);
});
