import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { folders, state, type Row } from './build.ts';
import { dead as gone } from './links.ts';

const started = performance.now();
const root = resolve(import.meta.dir, '../..');
const at = (...parts: string[]) => join(root, ...parts);
const there = (...parts: string[]) => existsSync(at(...parts));
const checks: [string, unknown, unknown][] = [];
const report = (label: string, tally: string, bad: string[]) =>
  checks.push([label, bad.length ? `${bad.length} bad: ${bad.slice(0, 3).join('; ')}` : tally, tally]);

/* TREE */

type Doc = { name: string; lines: string[] };

const read = (name: string): Doc => ({ name, lines: readFileSync(at(name), 'utf8').split('\n') });

const plain = (doc: Doc) => {
  const out: [number, string][] = [];
  let fence = false;
  doc.lines.forEach((line, i) => {
    if (/^\s*(```|~~~)/.test(line)) { fence = !fence; return; }
    if (!fence) out.push([i + 1, line]);
  });
  return out;
};

const places = ['site/src/pages', 'site/src/apps'].flatMap((root) => folders(at(root)).map((id) => `${root}/${id}`)).filter((dir) => there(`${dir}/app.json`));
const pages = places.filter((dir) => there(`${dir}/index.md`)).map((dir) => read(`${dir}/index.md`));
const posts = folders(at('site/src/pages/blog')).filter((slug) => there(`site/src/pages/blog/${slug}/index.md`)).map((slug) => read(`site/src/pages/blog/${slug}/index.md`));
const sources = places.flatMap((dir) => JSON.parse(readFileSync(at(`${dir}/app.json`), 'utf8')).source ?? []).map((name: string) => read(name));
const prose = [...pages, ...posts, ...sources];
const SKIP = new Set(['node_modules', 'target']);
const notes = existsSync(at('research'))
  ? (readdirSync(at('research'), { recursive: true }) as string[])
      .filter((name) => name.endsWith('.md') && !name.split('/').some((part) => part.startsWith('.') || SKIP.has(part)))
      .sort()
      .map((name) => read(`research/${name}`))
  : [];

/* ROWS */

let rows: Row[] = [];
try {
  rows = state().rows;
  report('rows', `${rows.length} rows`, []);
} catch (error) {
  report('rows', 'the build collects', [error instanceof Error ? error.message : String(error)]);
}
const routes = new Set(rows.map((row) => row.route));

/* LINKS */

const dead: string[] = [];
let aimed = 0;
for (const doc of prose) {
  const home = dirname(at(doc.name));
  for (const [n, line] of plain(doc))
    for (const hit of line.matchAll(/\[[^\]\n]*\]\(([^)\s]+)\)/g)) {
      const target = hit[1].split('#')[0];
      if (target === '' || /^(https?:|mailto:)/.test(target)) continue;
      aimed += 1;
      if (target.startsWith('/')) {
        if (!routes.has(target)) dead.push(`${doc.name}:${n} ${target} is no route`);
      } else if (!existsSync(resolve(home, target))) dead.push(`${doc.name}:${n} ${target}`);
    }
}
report('links', `${aimed} links`, dead);

/* RESEARCH */

const lost: string[] = [];
let tied = 0;
for (const doc of notes) {
  const home = doc.name === 'research/shared/ledger/src/prose.md' ? at('research/shared') : dirname(at(doc.name));
  for (const [n, line] of plain(doc))
    for (const hit of line.replace(/`[^`]*`/g, '').replace(/!\[.*?\]\([^)\s]*\)/g, '').matchAll(/\[[^\]\n]*\]\(([^)\s]+)\)/g)) {
      const target = hit[1].split(/[#?]/)[0];
      if (target === '' || /^[a-z]+:/.test(target)) continue;
      tied += 1;
      if (target.startsWith('/')) {
        if (/^\/[\w-]+\/$/.test(target) && !routes.has(target)) lost.push(`${doc.name}:${n} ${target} is no route`);
        continue;
      }
      let path = target;
      try {
        path = decodeURIComponent(target);
      } catch {}
      if (!existsSync(resolve(home, path))) lost.push(`${doc.name}:${n} ${target}`);
    }
}
report('research links', `${tied} links in ${notes.length} files`, lost);

/* HOUSE */

const house: string[] = [];
let ruled = 0;
for (const doc of [...pages, ...posts, ...notes]) {
  ruled += doc.lines.length;
  doc.lines.forEach((line, i) => {
    if (i > 0 && line === '' && doc.lines[i - 1] === '') house.push(`${doc.name}:${i + 1} two blank lines`);
  });
  for (const [n, line] of plain(doc)) {
    if (/[–—]/.test(line)) house.push(`${doc.name}:${n} em-dash or en-dash`);
    if (line.startsWith('#') && n < doc.lines.length && doc.lines[n] !== '') house.push(`${doc.name}:${n} heading without a blank line`);
  }
}
report('house', `${pages.length + posts.length} files, ${ruled} lines`, house);

/* DIST */

const dist = process.env.MRLY_DIST ? resolve(process.env.MRLY_DIST) : at('site/dist');
if (existsSync(join(dist, 'routes.json'))) {
  const site = JSON.parse(readFileSync(at('site/site.json'), 'utf8')) as { root: string; push: { guard: string[] } };
  const tree = JSON.parse(readFileSync(join(dist, 'git.json'), 'utf8'));
  const shipped = JSON.parse(readFileSync(join(dist, 'routes.json'), 'utf8')) as { rows: Row[] };
  const built = gone(dist, site.root, site.push.guard, tree, new Set(shipped.rows.map((row) => row.route)));
  report('dist links', `${built.links} links on ${built.pages} pages`, built.dead.map((one) => `${one.page} ${one.url}`));
  const known = new Set(shipped.rows.map((row) => row.route));
  const moves = JSON.parse(readFileSync(join(dist, 'redirects.json'), 'utf8')) as Record<string, { to: string }>;
  const stray = Object.entries(moves).flatMap(([from, { to }]) => [
    ...(known.has(from) ? [`${from} is still a route`] : []),
    ...(known.has(to) ? [] : [`${from} -> ${to}, no such route`]),
    ...(to in moves ? [`${from} -> ${to} chains`] : []),
  ]);
  report('redirects', `${Object.keys(moves).length} moves, each from a gone path to a route`, stray);
} else report('dist', 'no dist/routes.json, build first', []);

let failed = 0;
for (const [label, got, want] of checks) {
  const ok = got === want;
  if (!ok) failed += 1;
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${String(label).padEnd(26)} ${String(got)}${ok ? '' : ` (want ${String(want)})`}`);
}
console.log(failed ? `${failed} failed` : `${checks.length} checks green`);
console.log(`${(performance.now() - started).toFixed(0)} ms`);
process.exit(failed ? 1 : 0);
