import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import type { BunPlugin } from "bun";
import { REPO } from "./home.ts";

/* PACKAGES */

type Manifest = { name: string; exports: Record<string, string> };

const HOMES = [
  "draw",
  "net/shell",
  "net/sdk",
  "net/system",
  ...readdirSync(join(REPO, "designs")).map((one) => `designs/${one}`),
].filter((one) => existsSync(join(REPO, one, "package.json")));

export const PACKAGES = new Map(
  HOMES.map((one) => {
    const dir = join(REPO, one);
    const { name, exports } = JSON.parse(readFileSync(join(dir, "package.json"), "utf8")) as Manifest;
    if (!name.startsWith("@mrlyprod/")) throw new Error(`site: ${one}/package.json names ${name}, outside @mrlyprod/`);
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
