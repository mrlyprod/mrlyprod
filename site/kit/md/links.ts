import { IMAGE, decode, dirRoute, ext, fileRoute, rawPath } from "../git/view.ts";

/* INDEX */

export type Index = { routes: Map<string, string>; blob: string };

export type Sourced = { route: string; source: string | null };

export function index(rows: Sourced[], blob: string): Index {
  const routes = new Map<string, string>();
  for (const row of rows) if (row.source) routes.set(row.source, row.route);
  return { routes, blob };
}

/* RESOLVE */

const DENY = /^(?:javascript|data|vbscript):/i;

const OUT = /^(?:https?:|mailto:|tel:|#|\/)/;

const ARCHIVE = "research";

const keys = (path: string) => [path, `${path}.md`, path.replace(/\.md$/, ""), `${path}/README.md`];

function walk(dir: string, head: string): string | null {
  const out = dir ? dir.split("/") : [];
  for (const part of head.split("/")) {
    if (part === "" || part === ".") continue;
    if (part !== "..") out.push(part);
    else if (out.pop() === undefined) return null;
  }
  return out.join("/");
}

export function resolve(source: string, href: string, idx: Index): string {
  if (DENY.test(href.replace(/[\u0000- ]/g, ""))) return "#";
  if (OUT.test(href)) return href;
  const cut = href.search(/[#?]/);
  const head = cut < 0 ? href : href.slice(0, cut);
  const tail = cut < 0 ? "" : href.slice(cut);
  const path = head ? walk(source.slice(0, Math.max(source.lastIndexOf("/"), 0)), head) : null;
  if (path === null) return href;
  const plain = decode(path);
  const hit = keys(plain).map((key) => idx.routes.get(key)).find(Boolean);
  if (hit) return hit + tail;
  if (plain === ARCHIVE || plain.startsWith(`${ARCHIVE}/`)) return idx.blob + path + tail;
  if (!path || head.endsWith("/")) return dirRoute(path) + tail;
  if (IMAGE.has(ext(path)) || ext(path) === "pdf") return `/${rawPath(path)}${tail}`;
  return fileRoute(path) + tail;
}
