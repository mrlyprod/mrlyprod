import { dirname, isAbsolute, relative, resolve as under } from "node:path";
import { HOME, config as gitConfig, isGit, type Hooks } from "../git/git.ts";
import { dirRoute, link as gitLink, owner } from "../git/view.ts";
import { label, type Site } from "./build.ts";

/* TYPES */

export type Index = { base: string; slug: string; branch: string; map: Map<string, string>; git: Set<string>; served: (path: string) => string | null };

/* INDEX */

export function index(site: Site, hooks?: Hooks): Index {
  const git = gitConfig(site);
  const roots = new Set(Object.values(site.inputs ?? {}).map((one) => one.path));
  const map = new Map<string, string>();
  const put = (file: string, route: string) => {
    if (!file) return;
    const name = label(site, file);
    map.set(file, route);
    if (name.includes("/") || roots.has(file)) map.set(name, route);
  };
  for (const route of site.routes) {
    if (isGit(route)) continue;
    if (route.source) put(route.source, route.route);
    for (const one of route.urls ?? []) if (one.source) put(one.source, one.route);
  }
  const code = new Set<string>();
  for (const route of site.routes) {
    const page = isGit(route) ? owner(route.route) : null;
    if (!page) continue;
    code.add(page);
    let dir = page.slice(0, page.lastIndexOf("/") + 1);
    while (dir.length >= HOME.length && !code.has(dir)) {
      code.add(dir);
      dir = dir.slice(0, dir.lastIndexOf("/", dir.length - 2) + 1);
    }
  }
  return {
    base: git?.root ?? site.root,
    slug: git?.slug ?? "",
    branch: git?.branch ?? "main",
    map,
    git: code,
    served: (path) => site.ships?.get(under(git?.root ?? site.root, path)) ?? hooks?.served?.(site, path) ?? null,
  };
}

/* RESOLVE */

const OUT = /^(https?:|mailto:|tel:|#|\/)/;

const DENY = /^(?:javascript|data|vbscript):/i;

const denied = (url: string) => DENY.test(url.replace(/[\u0000-\u0020]/g, ""));

const keys = (path: string) => [path, `${path}.md`, path.replace(/\.md$/, ""), `${path}/README.md`];

const known = (idx: Index, path: string) => {
  for (const key of keys(path)) {
    const hit = idx.map.get(key);
    if (hit) return hit;
  }
  return "";
};

const routed = (idx: Index, path: string) => known(idx, under(idx.base, path)) || known(idx, path);

export const answer = (idx: Index, ask: string) => (ask.startsWith("/") ? String(idx.git.has(ask)) : routed(idx, ask));

export function resolve(site: Site, from: string, url: string): string {
  const idx = site.index;
  if (denied(url)) return "#";
  if (!idx || OUT.test(url)) return url;
  const cut = url.search(/[#?]/);
  const tail = cut < 0 ? "" : url.slice(cut);
  const head = cut < 0 ? url : url.slice(0, cut);
  if (!head) return url;
  const file = under(idx.base, from);
  const target = under(dirname(file), head);
  const path = relative(idx.base, target);
  if (path) site.asks?.add(path);
  const hit = routed(idx, path);
  if (hit) return hit + tail;
  if (!path || path.startsWith("..") || isAbsolute(path)) return url;
  if (idx.git.size) {
    const where = gitLink(relative(idx.base, dirname(file)), head);
    const page = where.startsWith("/raw/") ? (owner(where) ?? "") : where;
    const tracked = (one: string) => {
      site.asks?.add(one);
      return idx.git.has(one);
    };
    if (tracked(page)) return ((where.startsWith("/raw/") && idx.served(path)) || where) + tail;
    return tracked(dirRoute(path)) ? dirRoute(path) + tail : url;
  }
  return idx.slug ? `https://github.com/${idx.slug}/blob/${idx.branch}/${path}${tail}` : url;
}
