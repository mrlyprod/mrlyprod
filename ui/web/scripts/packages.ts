import { existsSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import type { BunPlugin } from "bun";
import { REPO, SIBLINGS } from "./home.ts";

type Manifest = { name: string; exports: Record<string, string> };

const HOMES = [
  join(SIBLINGS, "packages", "draw"),
  REPO,
  join(SIBLINGS, "os"),
  join(SIBLINGS, "apps"),
  join(SIBLINGS, "designs"),
].filter((dir) => existsSync(join(dir, "package.json")));

export const PACKAGES = new Map(
  HOMES.map((dir) => {
    const { name, exports } = JSON.parse(readFileSync(join(dir, "package.json"), "utf8")) as Manifest;
    if (!name.startsWith("@mrlyprod/"))
      throw new Error(`site: ${relative(REPO, dir)}/package.json names ${name}, outside @mrlyprod/`);
    return [name, { dir, exports }] as const;
  }),
);

const NAME = /^(@mrlyprod\/[a-z-]+)(?:\/(.+))?$/;

export function where(spec: string): string | undefined {
  const [, name, rest] = spec.match(NAME) ?? [];
  const one = name ? PACKAGES.get(name) : undefined;
  if (!one) return undefined;
  const out = rest ? (one.exports[`./${rest}`] ?? one.exports["./*"]?.replace("*", rest)) : one.exports["."];
  if (!out) throw new Error(`site: ${name} exports no ${rest ? `./${rest}` : "."}`);
  return join(one.dir, out);
}

export const packages: BunPlugin = {
  name: "packages",
  setup(build) {
    build.onResolve({ filter: new RegExp(`^(?:${[...PACKAGES.keys()].join("|")})(?:/|$)`) }, ({ path }) => ({
      path: where(path)!,
    }));
  },
};
