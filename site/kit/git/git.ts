import { existsSync, readdirSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { seti } from "../code/seti/seti.ts";
import { ext, home, stem, type Twig, type Wood } from "./view.ts";

/* CONFIG */

export type Git = { root: string; slug: string; branch: string; name: string };

export const HOME = "/git/";

export const TREE = "git.json";

export function config(at: string, decl?: { root?: string; slug?: string; branch?: string }): Git | null {
  if (!decl) return null;
  const slug = decl.slug ?? "";
  return { root: resolve(at, decl.root ?? "."), slug, branch: decl.branch ?? "main", name: slug.split("/").pop() || "code" };
}

/* FILES */

const SKIP = new Set([".git", ".cache", ".venv", "__pycache__", "node_modules", "dist", "target", "data", "pkg"]);

function crawl(root: string, at: string, out: string[]) {
  for (const item of readdirSync(at ? join(root, at) : root, { withFileTypes: true })) {
    if (SKIP.has(item.name)) continue;
    const path = at ? `${at}/${item.name}` : item.name;
    if (item.isDirectory()) crawl(root, path, out);
    else if (item.isFile()) out.push(path);
  }
}

export function tree(root: string): string[] {
  if (existsSync(join(root, ".git"))) {
    const run = Bun.spawnSync(["git", "ls-files", "-z"], { cwd: root, stderr: "ignore" });
    const list = run.success ? run.stdout.toString().split("\0").filter((path) => path && existsSync(join(root, path))) : [];
    if (list.length) return list.sort();
  }
  const out: string[] = [];
  crawl(root, "", out);
  return out.sort();
}

/* TREE */

const icon = (name: string) => seti(name).replace(/^si( si-)?/, "");

export function forest(git: Git, paths: string[]): Wood {
  const kids = new Map<string, Twig[]>([["", []]]);
  const dir = (at: string): Twig[] => {
    const held = kids.get(at);
    if (held) return held;
    const list: Twig[] = [];
    kids.set(at, list);
    dir(home(at)).push({ n: stem(at), k: "d", c: list });
    return list;
  };
  for (const path of paths) {
    const twig: Twig = { n: stem(path), k: "f" };
    const i = icon(twig.n);
    if (i) twig.i = i;
    twig.s = statSync(join(git.root, path)).size;
    dir(home(path)).push(twig);
  }
  for (const list of kids.values()) list.sort((a, b) => (a.k !== b.k ? (a.k === "d" ? -1 : 1) : a.n.localeCompare(b.n)));
  return { base: HOME, name: git.name, slug: git.slug, branch: git.branch, c: kids.get("")! };
}

/* TYPES BY EXTENSION */

const TEXT = "text/plain; charset=utf-8";

const MIME: Record<string, string> = {
  avif: "image/avif",
  gif: "image/gif",
  gz: "application/gzip",
  ico: "image/x-icon",
  jpeg: "image/jpeg",
  jpg: "image/jpeg",
  mp3: "audio/mpeg",
  mp4: "video/mp4",
  otf: "font/otf",
  pdf: "application/pdf",
  png: "image/png",
  svg: "image/svg+xml",
  ttf: "font/ttf",
  wasm: "application/wasm",
  wav: "audio/wav",
  webm: "video/webm",
  webp: "image/webp",
  woff: "font/woff",
  woff2: "font/woff2",
  zip: "application/zip",
};

const INERT = new Set(["svg"]);

export const mime = (path: string, text: boolean) =>
  text || INERT.has(ext(path)) ? TEXT : Object.hasOwn(MIME, ext(path)) ? MIME[ext(path)]! : "application/octet-stream";
