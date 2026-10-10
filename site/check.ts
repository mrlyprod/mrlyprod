import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { dead as gone } from './check/links.ts';
import { figures as named } from './kit/md/md.ts';
import { roster, state, type Row } from './scripts/site.ts';

const started = performance.now();
const root = resolve(import.meta.dir, '..');
const at = (...parts: string[]) => join(root, ...parts);
const there = (...parts: string[]) => existsSync(at(...parts));
const checks: [string, unknown, unknown][] = [];
const report = (label: string, tally: string, bad: string[]) =>
  checks.push([label, bad.length ? `${bad.length} bad: ${bad.slice(0, 3).join('; ')}` : tally, tally]);

// TREE

type Doc = { name: string; lines: string[]; front: Map<string, string> };

const read = (name: string): Doc => {
  const lines = readFileSync(at(name), 'utf8').split('\n');
  const front = new Map<string, string>();
  const close = lines[0] === '---' ? lines.indexOf('---', 1) : -1;
  for (const line of lines.slice(1, Math.max(close, 1))) {
    const cut = line.indexOf(':');
    if (cut > 0) front.set(line.slice(0, cut).trim(), line.slice(cut + 1).trim());
  }
  return { name, lines, front };
};

const plain = (doc: Doc) => {
  const out: [number, string][] = [];
  let fence = false;
  doc.lines.forEach((line, i) => {
    if (/^\s*(```|~~~)/.test(line)) { fence = !fence; return; }
    if (!fence) out.push([i + 1, line]);
  });
  return out;
};

const pages = readdirSync(at('site/pages')).filter((name) => name.endsWith('.md')).sort().map((name) => read(`site/pages/${name}`));
const posts = readdirSync(at('site/blog'), { withFileTypes: true }).filter((one) => one.isDirectory() && there(`site/blog/${one.name}/index.md`)).map((one) => read(`site/blog/${one.name}/index.md`));
const prose = [...pages, ...posts, read('pkgs/mrlyrs/NAMES.md')];
const live = new Set(roster().map((one) => one.name));

// ROWS

let rows: Row[] = [];
try {
  rows = state().rows;
  report('rows', `${rows.length} rows`, []);
} catch (error) {
  report('rows', 'the build collects', [error instanceof Error ? error.message : String(error)]);
}
const routes = new Set(rows.map((row) => row.route));

// FIGURES

const drawn: string[] = [];
for (const doc of [...pages, ...posts]) {
  const figure = doc.front.get('figure');
  if (figure && !live.has(figure)) drawn.push(`${doc.name} figure ${figure} is no live figure`);
}
for (const doc of prose) for (const name of named(doc.lines.join('\n'))) if (!live.has(name)) drawn.push(`${doc.name} shows ${name}, no live figure`);
report('figures', `${live.size} live figures`, drawn);

// LINKS

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

// HOUSE

const house: string[] = [];
let ruled = 0;
for (const doc of [...pages, ...posts]) {
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

// DIST

const dist = process.env.MRLY_DIST ? resolve(process.env.MRLY_DIST) : at('site/dist');
if (existsSync(join(dist, 'routes.json'))) {
  const site = JSON.parse(readFileSync(at('site/site.json'), 'utf8')) as { root: string; push: { guard: string[] } };
  const tree = JSON.parse(readFileSync(join(dist, 'git.json'), 'utf8'));
  const shipped = JSON.parse(readFileSync(join(dist, 'routes.json'), 'utf8')) as { rows: Row[] };
  const built = gone(dist, site.root, site.push.guard, tree, new Set(shipped.rows.map((row) => row.route)));
  report('dist links', `${built.links} links on ${built.pages} pages`, built.dead.map((one) => `${one.page} ${one.url}`));
  const held = shipped.rows.flatMap((row) => [
    ...(row.kind && row.title ? [] : [`${row.route} has no kind or title`]),
    ...(row.source && !existsSync(join(dist, 'raw', row.source)) ? [`${row.route} reads ${row.source}, not under dist/raw`] : []),
  ]);
  report('dist rows', `${shipped.rows.length} rows, each source under raw/`, held);
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
