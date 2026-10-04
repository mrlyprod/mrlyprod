import { existsSync, readdirSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { seti } from "../code/seti/seti.ts";
import { bytes, digest, type Bytes, type Node, type Output, type Route, type Site, type Spec } from "../ssg/build.ts";
import { deeps, type Shell } from "../ssg/modes.ts";
import { HUGE, IMAGE, dirRoute, ext, fileRoute, home, rawPath, reads, stem, type Twig, type Wood } from "./view.ts";

/* TYPES */

export type Git = { root: string; slug: string; branch: string; name: string };

export type Kind = "dir" | "file";

export type Child = [string, number, Kind];

export type File = { path: string; size: number };

export type Leaf = {
  route: string;
  name: string;
  description: string;
  body: string;
  type?: string;
  code?: boolean;
  tree?: Node[];
  scripts?: string[];
};

export type Hooks = {
  page?: (site: Site, leaf: Leaf) => Bytes;
  entry?: string;
  served?: (site: Site, path: string) => string | null;
};

/* CONFIG */

export const HOME = "/git/";

export const TREE = "git.json";

export function config(site: Site): Git | null {
  const decl = site.config.git as { root?: string; slug?: string; branch?: string } | undefined;
  if (!decl) return null;
  const slug = decl.slug ?? "";
  return {
    root: resolve(site.root, decl.root ?? "."),
    slug,
    branch: decl.branch ?? "main",
    name: slug.split("/").pop() || "code",
  };
}

/* TREE */

const SKIP = new Set([".git", ".cache", ".venv", "__pycache__", "node_modules", "dist", "target", "data", "pkg"]);

function crawl(root: string, at: string, out: string[]) {
  for (const item of readdirSync(at ? join(root, at) : root, { withFileTypes: true })) {
    if (SKIP.has(item.name)) continue;
    const path = at ? `${at}/${item.name}` : item.name;
    if (item.isDirectory()) crawl(root, path, out);
    else if (item.isFile()) out.push(path);
  }
}

function tree(git: Git): string[] {
  if (existsSync(join(git.root, ".git"))) {
    const run = Bun.spawnSync(["git", "ls-files", "-z"], { cwd: git.root, stderr: "ignore" });
    const list = run.success ? run.stdout.toString().split("\0").filter((path) => path && existsSync(join(git.root, path))) : [];
    if (list.length) return list.sort();
  }
  const out: string[] = [];
  crawl(git.root, "", out);
  return out.sort();
}

/* ROUTES */

export const isGit = (route: Route) => route.kind === "git" || route.kind === "raw";

/* COLLECT */

const KIDS = new WeakMap<Site, Map<string, Child[]>>();

export function collect(site: Site, hooks?: Hooks): { routes: Route[]; node: Node | null } {
  const git = config(site);
  if (!git) return { routes: [], node: null };
  const paths = tree(git);
  if (!paths.length) return { routes: [], node: null };
  if (!deeps(site.config).includes(HOME)) throw new Error(`git: site.json declares git, and its modes must make ${HOME} a deep spa route`);
  const kids = new Map<string, Child[]>([["", []]]);
  KIDS.set(site, kids);
  for (const path of paths) {
    let at = "";
    for (const part of path.split("/").slice(0, -1)) {
      at = at ? `${at}/${part}` : part;
      if (!kids.has(at)) kids.set(at, []);
    }
  }
  const sizes = new Map<string, number>();
  for (const path of paths) {
    const size = statSync(join(git.root, path)).size;
    sizes.set(path, size);
    kids.get(home(path))!.push([stem(path), size, "file"]);
  }
  for (const dir of kids.keys()) if (dir) kids.get(home(dir))!.push([stem(dir), 0, "dir"]);
  for (const [dir, list] of kids) for (const one of list) if (one[2] === "dir") one[1] = kids.get(dir ? `${dir}/${one[0]}` : one[0])!.length;
  for (const list of kids.values()) {
    list.sort((a, b) => (a[2] !== b[2] ? (a[2] === "dir" ? -1 : 1) : a[0].localeCompare(b[0])));
  }
  const routes: Route[] = hooks?.entry ? [{ route: HOME, kind: "git", name: git.name, entry: resolve(site.root, hooks.entry), sitemap: true }] : [];
  for (const path of paths) {
    const source = join(git.root, path);
    routes.push({
      route: `/${rawPath(path)}`,
      kind: "raw",
      name: stem(path),
      data: { path, size: sizes.get(path)! } satisfies File,
      inputs: [source],
      urls: [{ route: `/${rawPath(path)}`, name: stem(path) }],
      hidden: true,
      sitemap: true,
    });
  }
  return { routes, node: { name: "Code", href: HOME } };
}

/* EXPLORER */

const kindOf = (name: string) => seti(name).replace(/^si( si-)?/, "");

export function explorer(site: Site): Node[] {
  const git = config(site);
  const kids = KIDS.get(site);
  if (!git || !kids) return site.nav;
  const nodes = (kids.get("") ?? []).map(([name, , kind]): Node => (kind === "file" ? { name, href: fileRoute(name), icon: seti(name) } : { name, href: dirRoute(name), lazy: name, nodes: [] }));
  return [{ name: git.name, href: HOME, lazy: "", nodes }];
}

function twigs(site: Site, git: Git, kids: Map<string, Child[]>, dir: string, hooks?: Hooks): Twig[] {
  return (kids.get(dir) ?? []).map(([name, weight, kind]) => {
    const path = dir ? `${dir}/${name}` : name;
    if (kind === "dir") return { n: name, k: "d", c: twigs(site, git, kids, path, hooks) };
    const twig: Twig = { n: name, k: "f" };
    const i = kindOf(name);
    if (i) twig.i = i;
    twig.s = weight;
    const u = shelved(site, git, path, weight, hooks);
    if (u) twig.u = u;
    return twig;
  });
}

export function forest(site: Site, hooks?: Hooks): Wood | null {
  const git = config(site);
  const kids = KIDS.get(site);
  return git && kids ? { base: HOME, name: git.name, slug: git.slug, branch: git.branch, c: twigs(site, git, kids, "", hooks) } : null;
}

/* FINGERPRINT */

export function print(site: Site, route: Route, hooks?: Hooks): string {
  const parts: Bytes[] = ["git", route.route, route.kind ?? "", JSON.stringify(route.data ?? null), site.stamp];
  if (route.kind === "raw") parts.push(mirror(site, route, hooks) ?? "");
  else parts.push(JSON.stringify(explorer(site)));
  for (const file of route.inputs ?? []) {
    parts.push(stem(file));
    parts.push(existsSync(file) ? bytes(file) : "gone");
  }
  return digest(parts).slice(0, 16);
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

/* SERVED */

function shelved(site: Site, git: Git, path: string, weight: number, hooks?: Hooks): string | null {
  const ship = site.ships.get(join(git.root, path));
  if (ship) return ship;
  const url = hooks?.served?.(site, path);
  if (!url) return null;
  const kind = ext(path);
  if (weight > HUGE || IMAGE.has(kind) || kind === "pdf") return url;
  return reads(bytes(join(git.root, path))) === null ? url : null;
}

export function mirror(site: Site, route: Route, hooks?: Hooks): string | null {
  if (route.kind !== "raw") return null;
  const git = config(site);
  if (!git) return null;
  const { path, size } = route.data as File;
  return shelved(site, git, path, size, hooks);
}

/* RENDER */

export function render(site: Site, route: Route, spec: Spec): Output[] {
  const git = config(site);
  if (!git) throw new Error(`git: ${route.route} has no git block in site.json`);
  const { path, size } = route.data as File;
  if (shelved(site, git, path, size, spec.git)) return [];
  const body = bytes(join(git.root, path));
  const text = size <= HUGE && !IMAGE.has(ext(path)) && ext(path) !== "pdf" && reads(body) !== null;
  return [{ path: rawPath(path), bytes: body, type: mime(path, text) }];
}

export function shell(site: Site, route: Route, shell: Shell, spec: Spec): Bytes {
  const git = config(site);
  const page = spec.git?.page;
  if (!git || !page) throw new Error("git: the viewer's shell needs a git block in site.json and the spec's git.page");
  const description = `The source of ${git.name}: every tracked file, browsable, with its raw bytes under /raw/.`;
  return page(site, { route: route.route, name: git.name, description, body: "", type: "website", code: true, tree: explorer(site), scripts: [shell.script] });
}
