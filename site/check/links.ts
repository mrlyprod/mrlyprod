import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { decode, find, type Wood } from '../kit/git/view.ts';

const ATTR = /\s(href|src|srcset)\s*=\s*(?:"([^"]*)"|'([^']*)')/gi;

const SCRIPT = /<script\b[^>]*>[\s\S]*?<\/script>/gi;

const ENTITY: Record<string, string> = { amp: '&', lt: '<', gt: '>', quot: '"', apos: "'", '#39': "'" };

const unescape = (text: string) => text.replace(/&(amp|lt|gt|quot|apos|#39);/g, (_, name: string) => ENTITY[name]!);

function urls(html: string): string[] {
  const out: string[] = [];
  const bare = html.replace(SCRIPT, (tag) => tag.slice(0, tag.indexOf('>') + 1));
  for (const hit of bare.matchAll(ATTR)) {
    const value = unescape(hit[2] ?? hit[3] ?? '').trim();
    if (hit[1]!.toLowerCase() === 'srcset') out.push(...value.split(',').map((one) => one.trim().split(/\s+/)[0]!).filter(Boolean));
    else if (value) out.push(value);
  }
  return out;
}

function files(dist: string, at = '', out: string[] = []): string[] {
  for (const entry of readdirSync(join(dist, at), { withFileTypes: true })) {
    const path = at ? `${at}/${entry.name}` : entry.name;
    if (entry.isDirectory()) files(dist, path, out);
    else if (entry.isFile()) out.push(path);
  }
  return out;
}

export function dead(dist: string, origin: string, guard: string[], tree: Wood | null, routes: Set<string> = new Set()): { links: number; pages: number; dead: { page: string; url: string }[] } {
  const all = new Set(files(dist));
  const held = (key: string) => {
    if (routes.has(`/${key}`)) return true;
    if (!key || key.endsWith('/')) return all.has(`${key}index.html`);
    return all.has(key) || (!key.slice(key.lastIndexOf('/') + 1).includes('.') && all.has(`${key}/index.html`));
  };
  const pages = [...all].filter((path) => path.endsWith('.html') && !path.startsWith('raw/')).sort();
  const out: { page: string; url: string }[] = [];
  let links = 0;
  for (const page of pages) {
    for (const url of urls(readFileSync(join(dist, page), 'utf8'))) {
      if (url.startsWith('#')) continue;
      const where = URL.parse(url, `${origin}/${page}`);
      if (where && where.origin !== origin) continue;
      links += 1;
      const path = where ? decode(where.pathname) : null;
      if (path !== null && guard.some((one) => path.slice(1).startsWith(one))) continue;
      const alive = path !== null && (tree && path.startsWith(tree.base) ? find(tree, path.slice(tree.base.length).replace(/\/$/, '')) !== null : held(path.slice(1)));
      if (!alive) out.push({ page, url });
    }
  }
  return { links, pages: pages.length, dead: out };
}
