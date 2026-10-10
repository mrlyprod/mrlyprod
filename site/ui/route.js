import { crumbs } from './crumbs.js';

/* MATCH */

export const deeps = (rows) => rows.filter((row) => row.meta?.deep).map((row) => row.route);

export const walled = (path, deep) => deep.filter((wall) => path.startsWith(wall)).sort((a, b) => b.length - a.length)[0] ?? '';

export function match(rows, path) {
  const want = path === '/index.html' ? '/' : path;
  const exact = rows.find((row) => row.route === want);
  if (exact) return exact;
  const deep = rows.filter((row) => row.meta?.deep && want.startsWith(row.route)).sort((a, b) => b.route.length - a.route.length)[0];
  return deep ?? rows.find((row) => row.kind === 'missing');
}

/* SORT */

export function sort(to, from, deep) {
  if (to.origin !== from.origin) return '';
  const wall = walled(to.pathname, deep);
  if (wall && wall === walled(from.pathname, deep)) return 'inner';
  if (!wall && !to.pathname.endsWith('/')) return '';
  if (to.pathname === from.pathname && to.search === from.search && to.href.includes('#')) return 'mark';
  return 'page';
}

/* HEAD */

export const named = (row, site) => (row.title === site.title ? row.title : `${row.title} · ${site.title}`);

export function trail(row, path) {
  if (row.kind === 'missing') return [{ name: 'not found', href: path }];
  return crumbs(row.meta?.deep ? path : row.route);
}
