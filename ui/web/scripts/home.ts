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
  wasm?: Record<string, string[]>;
  designs?: string[];
  seams?: { apps?: string };
  research?: string;
  widgets?: string[];
  cart?: boolean;
  roots: string[];
  apps?: string[];
  screen: number[];
  min: number[];
  company: string;
  tree: { name: string; nodes: { name: string; href: string }[] }[];
  socials: { name: string; href: string }[];
  contact: string;
  shots: { routes: string[] };
  push?: { prefix: string; guard: string[]; bucket: string; store: { bucket: string; prefix: string } };
  git: { root?: string; slug?: string; branch?: string; folder?: string };
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
const JS = join(SIBLINGS, "data", "packages", "rs", "bindgen", "js");
const TABLE = join(JS, "blobs", "index.json");
type Crate = { home: string };
const CRATES: Record<string, Crate> =
  SITE.wasm && existsSync(TABLE)
    ? (JSON.parse(readFileSync(TABLE, "utf8")) as { crates: Record<string, Crate> }).crates
    : {};
export const UNITS = Object.keys(CRATES);
export const unit = (name: string) => join(JS, name);
export const blobs = join(JS, "blobs");
export const wasm = (_name: string) => join(blobs, "all.wasm");
export const home = (name: string) => CRATES[name]?.home ?? "all";
export const closed = (names: string[]): string[] => {
  const held = new Set(names.map(home));
  return UNITS.filter((name) => held.has(home(name)));
};
