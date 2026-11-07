import { existsSync, readdirSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { home, stem, type Twig, type Wood } from "../../kit/git/view.ts";

export type Git = { root: string; slug: string; branch: string; folder: string; name: string };

export const HOME = "/git/";

export const TREE = "git.json";

export function config(
  at: string,
  decl?: { root?: string; slug?: string; branch?: string; folder?: string },
): Git | null {
  if (!decl) return null;
  const slug = decl.slug ?? "";
  return {
    root: resolve(at, decl.root ?? "."),
    slug,
    branch: decl.branch ?? "main",
    folder: decl.folder ?? "",
    name: slug.split("/").pop() || "code",
  };
}

const SKIP = new Set([
  ".git",
  ".cache",
  ".venv",
  "__pycache__",
  "node_modules",
  "dist",
  "target",
  "data",
  "pkg",
  "wasm",
  ".cargo",
  ".ruff_cache",
  ".pytest_cache",
]);

function crawl(root: string, at: string, out: string[]) {
  for (const item of readdirSync(at ? join(root, at) : root, { withFileTypes: true })) {
    if (SKIP.has(item.name)) continue;
    const path = at ? `${at}/${item.name}` : item.name;
    if (item.isDirectory()) crawl(root, path, out);
    else if (item.isFile()) out.push(path);
  }
}

export function tree(root: string): string[] {
  {
    const run = Bun.spawnSync(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], {
      cwd: root,
      stderr: "ignore",
    });
    const list = run.success
      ? run.stdout
          .toString()
          .split("\0")
          .filter((path) => path && existsSync(join(root, path)))
      : [];
    if (list.length) return list.sort();
  }
  const out: string[] = [];
  crawl(root, "", out);
  return out.sort();
}

export function forest(git: Git, paths: string[], file = (path: string) => join(git.root, path)): Wood {
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
    dir(home(path)).push({ n: stem(path), k: "f", s: statSync(file(path)).size });
  }
  for (const list of kids.values())
    list.sort((a, b) => (a.k !== b.k ? (a.k === "d" ? -1 : 1) : a.n.localeCompare(b.n)));
  return { base: HOME, name: git.name, slug: git.slug, branch: git.branch, folder: git.folder, c: kids.get("")! };
}
