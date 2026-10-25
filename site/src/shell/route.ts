import type { Row } from "../../scripts/build.ts";

/* TYPES */

export type Place = Pick<URL, "origin" | "pathname" | "search" | "href">;

export type Titled = { title: string };

export type Sorted = "" | "inner" | "mark" | "page";

export type Routed = Pick<Row, "route" | "meta">;

export type Wired = { route: string; doors?: undefined };

/* MATCH */

export const deeps = (rows: Routed[]) => rows.filter((row) => row.meta?.deep).map((row) => row.route);

export const walled = (path: string, deep: string[]) =>
  deep.filter((wall) => path.startsWith(wall)).sort((a, b) => b.length - a.length)[0] ?? "";

export function match<R extends Routed>(rows: R[], path: string): R {
  const want = path === "/index.html" ? "/" : path;
  const exact = rows.find((row) => row.route === want);
  if (exact) return exact;
  const deep = rows
    .filter((row) => row.meta?.deep && want.startsWith(row.route))
    .sort((a, b) => b.route.length - a.route.length)[0];
  return deep ?? rows.find((row) => row.meta?.id === "missing")!;
}

/* SORT */

export function sort(to: Place, from: Place, deep: string[]): Sorted {
  if (to.origin !== from.origin) return "";
  const wall = walled(to.pathname, deep);
  if (wall && wall === walled(from.pathname, deep)) return "inner";
  if (!wall && !to.pathname.endsWith("/")) return "";
  if (to.pathname === from.pathname && to.search === from.search && to.href.includes("#")) return "mark";
  return "page";
}

/* HEAD */

export const named = (row: Titled, site: Titled) =>
  row.title === site.title ? row.title : `${row.title} · ${site.title}`;
