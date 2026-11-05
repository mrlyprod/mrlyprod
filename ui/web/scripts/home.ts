import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

export type Face = { family: string; file: string; axes?: string; weight: string; licence: string };

export type Site = {
  title: string;
  wordmark?: string;
  cli?: string;
  root: string;
  since: number;
  prefix: string;
  mark: string;
  marks?: Record<string, string>;
  tint: string;
  menu: string;
  desk: string;
  tiles?: { id: string; title: string };
  wasm?: string[];
  designs?: string[];
  seams?: { apps?: string };
  lock?: { realm: string };
  research?: string;
  widgets?: string[];
  cart?: boolean;
  roots: string[];
  screen: number[];
  min: number[];
  company: string;
  tagline: string;
  tree: { name: string; nodes: { name: string; href: string }[] }[];
  socials: { name: string; href: string }[];
  contact: string;
  shots: { routes: string[] };
  push?: { prefix: string; guard: string[]; bucket: string; store: { bucket: string; prefix: string } };
  git: { root?: string; slug?: string; branch?: string; folder?: string };
  llms: { about: string; legend?: string };
  fonts: { out?: string; keep?: string; faces: Face[] };
  icons?: { names: string[]; out: string };
};

export const top = (dir: string): string =>
  existsSync(join(dir, ".git")) || dirname(dir) === dir ? dir : top(dirname(dir));

export const REPO = resolve(import.meta.dir, "..", "..");
export const SIBLINGS = dirname(REPO);

const named = process.env.MRLY_ROOT;
if (!named) throw new Error("site: MRLY_ROOT names no instance; run bun run build from the instance's own folder");

export const HOME = resolve(named);
export const SITE = JSON.parse(readFileSync(join(HOME, "site.json"), "utf8")) as Site;
export const UNITS = SITE.wasm ?? [];
export const unit = (name: string) => join(SIBLINGS, "packages", name, "js");
export const wasm = (name: string) => join(unit(name), "pkg", `mrly_${name}_js_bg.wasm`);
