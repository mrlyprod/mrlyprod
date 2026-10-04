import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { law } from './check/lock.ts';

const started = performance.now();
const root = resolve(import.meta.dir, '..');
const at = (...parts: string[]) => join(root, ...parts);
const there = (...parts: string[]) => existsSync(at(...parts));
const checks: [string, unknown, unknown][] = [];
const report = (label: string, tally: string, bad: string[]) =>
  checks.push([label, bad.length ? `${bad.length} bad: ${bad.slice(0, 3).join('; ')}` : tally, tally]);

// TREE

type Doc = { name: string; lines: string[]; front: Map<string, string>; body: number };

const desks = new Map<string, Doc>();

const read = (name: string): Doc => {
  const held = desks.get(name);
  if (held) return held;
  const lines = readFileSync(at(name), 'utf8').split('\n');
  const front = new Map<string, string>();
  let body = 0;
  if (lines[0] === '---') {
    const close = lines.indexOf('---', 1);
    if (close > 0) {
      for (const line of lines.slice(1, close)) {
        const cut = line.indexOf(':');
        if (cut > 0) front.set(line.slice(0, cut).trim(), line.slice(cut + 1).trim());
      }
      body = close + 1;
    }
  }
  const doc = { name, lines, front, body };
  desks.set(name, doc);
  return doc;
};

const sheets = (folder: string) =>
  there(folder) ? readdirSync(at(folder)).filter((n) => n.endsWith('.md') && n !== 'README.md').sort().map((n) => `${folder}/${n}`) : [];

const under = (folder: string, suffix: string) => {
  const out: string[] = [];
  const walk = (here: string) => {
    for (const entry of readdirSync(at(here), { withFileTypes: true })) {
      if (entry.isDirectory()) walk(`${here}/${entry.name}`);
      else if (entry.name.endsWith(suffix)) out.push(`${here}/${entry.name}`);
    }
  };
  if (there(folder)) walk(folder);
  return out;
};

const stem = (name: string) => name.slice(name.lastIndexOf('/') + 1).replace(/\.[a-z]+$/, '');

const plain = (doc: Doc) => {
  const out: [number, string][] = [];
  let fence = false;
  doc.lines.forEach((line, i) => {
    if (/^\s*(```|~~~)/.test(line)) { fence = !fence; return; }
    if (!fence) out.push([i + 1, line]);
  });
  return out;
};

const notes = sheets('research/notes').map(read);
const claims = sheets('research/claims').map(read);
const papers = sheets('research/papers').map(read);
const pages = sheets('site/pages').map(read);
const posts = sheets('site/blog').map(read);
const concepts = sheets('research/wiki').map(read);
const indexes = ['wiki', 'notes', 'claims', 'papers'].map((kind) => `research/${kind}/README.md`);
const stems = ['research/README.md', 'research/REFS.md', 'research/sequences.md', ...indexes].filter((name) => there(name)).map(read);
const fronted = [...notes, ...papers, ...pages, ...posts, ...concepts];
const prose = [...notes, ...claims, ...papers, ...stems, ...pages, ...posts, ...concepts];
const housed = [...under('research', '.md'), ...sheets('site/pages'), ...sheets('site/blog')].map(read);
const demos = new Set(
  there('site/demos/views')
    ? readdirSync(at('site/demos/views'), { withFileTypes: true })
        .filter((entry) => entry.isDirectory() && there(`site/demos/views/${entry.name}/index.html`))
        .map((entry) => entry.name)
    : [],
);
const RUNNERS = new Set(['press.ts', 'diff.ts']);
const made = new Set(
  there('figures')
    ? readdirSync(at('figures')).filter((name) => name.endsWith('.ts') && !/\.(test|d)\.ts$/.test(name) && !RUNNERS.has(name)).map((name) => name.slice(0, -3))
    : [],
);
const locked: Record<string, unknown> = there('site/figures.lock') ? JSON.parse(readFileSync(at('site/figures.lock'), 'utf8')) : {};
const pinned = new Set(Object.entries(locked).filter(([, row]) => typeof row !== 'string').map(([name]) => name));
const pairs = [...made].sort();
const pair = (name: string) => made.has(name);
const noted = new Set(notes.map((doc) => stem(doc.name)));

// CLAIMS

const CLAIM = /^- (\d{4})-(\d{2})-(\d{2}) \[(Proved|Verified|Conjecture|Refuted)\] \S/;

const real = (y: number, mo: number, d: number) => {
  const when = new Date(Date.UTC(y, mo - 1, d));
  return when.getUTCFullYear() === y && when.getUTCMonth() === mo - 1 && when.getUTCDate() === d;
};

const rows: [Doc, number, string][] = [];
const ledger: string[] = [];
for (const doc of claims) {
  if (!doc.lines[0].startsWith('# ')) ledger.push(`${doc.name}:1 no title`);
  let last = '';
  doc.lines.forEach((line, i) => {
    const where = `${doc.name}:${i + 1}`;
    if (i === 0 || line === '') return;
    const hit = CLAIM.exec(line);
    if (!hit) { ledger.push(`${where} ${line.startsWith('- ') ? 'untagged claim' : 'stray line'}`); return; }
    rows.push([doc, i + 1, line]);
    if (!real(Number(hit[1]), Number(hit[2]), Number(hit[3]))) ledger.push(`${where} not a date`);
    const date = `${hit[1]}-${hit[2]}-${hit[3]}`;
    if (last && date < last) ledger.push(`${where} ${date} under ${last}`);
    last = date;
  });
}
report('claims', `${claims.length} files, ${rows.length} claims`, ledger);

// TOP

const TOP = /^- \[[^\]]+\]\(([a-z0-9-]+\.md)\): "(.+)"$/;
const SOLID = /^- \d{4}-\d{2}-\d{2} \[(Proved|Verified)\] /;
const top: string[] = [];
let quoted = 0;
if (there('research/claims/README.md')) {
  const doc = read('research/claims/README.md');
  const from = doc.lines.indexOf('## TOP 10');
  const until = doc.lines.findIndex((line, i) => i > from && line.startsWith('## '));
  const seen = new Set<string>();
  if (from < 0) top.push(`${doc.name} has no TOP 10`);
  else doc.lines.slice(from + 1, until < 0 ? undefined : until).forEach((line, i) => {
    if (line === '') return;
    const where = `${doc.name}:${from + i + 2}`;
    const hit = TOP.exec(line);
    if (!hit) { top.push(`${where} is not a top-10 row`); return; }
    quoted += 1;
    if (seen.has(hit[1])) top.push(`${where} ${hit[1]} is quoted twice`);
    seen.add(hit[1]);
    const held = (claims.find((one) => one.name === `research/claims/${hit[1]}`)?.lines ?? []).filter((row) => row.includes(hit[2]));
    if (!held.length) top.push(`${where} quote is not in ${hit[1]}`);
    else if (!held.every((row) => SOLID.test(row))) top.push(`${where} quote is not Proved or Verified`);
  });
  if (quoted > 10) top.push(`${doc.name} holds ${quoted} rows`);
} else top.push('research/claims/README.md is missing');
report('top', `${quoted} quotes`, top);

// POINTERS

const NAME = /\b(?:fn|const|static|struct|enum|trait|type|mod|union)\s+([A-Za-z_]\w*)/g;
const declared = new Map<string, Set<string>>();
const lab = there('research/lab/rs')
  ? readdirSync(at('research/lab/rs'), { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => [entry.name, `research/lab/rs/${entry.name}/src`] as [string, string])
  : [];
for (const [crate, src] of [
  ['mrlyrs', 'pkgs/mrlyrs/src'],
  ['demos', 'site/demos/logic/src'],
  ...lab,
] as [string, string][]) {
  const names = new Set<string>();
  for (const file of under(src, '.rs')) {
    names.add(stem(file));
    for (const hit of readFileSync(at(file), 'utf8').matchAll(NAME)) names.add(hit[1]);
  }
  declared.set(crate, names);
}

const studies = new Set(
  ['rs', 'py'].flatMap((kind) =>
    there(`research/lab/${kind}`)
      ? readdirSync(at(`research/lab/${kind}`), { withFileTypes: true })
          .filter((entry) => entry.isDirectory())
          .map((entry) => `${kind}/${entry.name}`)
      : [],
  ),
);
const sequences = new Set(
  there('research/sequences.md')
    ? [...readFileSync(at('research/sequences.md'), 'utf8').matchAll(/\bsequence_[a-z0-9_=]+/g)].map((hit) => hit[0])
    : [],
);

const pointers: string[] = [];
let aimed = 0;
for (const [doc, n, line] of rows) {
  const blame = (what: string) => pointers.push(`${doc.name}:${n} ${what}`);
  for (const hit of line.matchAll(/\bmrly[a-z]+(?:::[A-Za-z_]\w*)+/g)) {
    aimed += 1;
    const parts = hit[0].split('::');
    const names = declared.get(parts[0]);
    if (!names) blame(`no crate ${parts[0]}`);
    else if (!names.has(parts[parts.length - 1])) blame(`${hit[0]} is undeclared`);
  }
  for (const hit of line.matchAll(/\blab\/([a-z0-9-]+)(?:\/([a-z0-9-]+))?/g)) {
    aimed += 1;
    if (hit[1] !== 'rs' && hit[1] !== 'py') blame(`lab/${hit[1]} names no kind`);
    else if (!hit[2]) blame(`lab/${hit[1]} names no study`);
    else if (!studies.has(`${hit[1]}/${hit[2]}`)) blame(`no study lab/${hit[1]}/${hit[2]}`);
  }
  for (const hit of line.matchAll(/\bsequence_[a-z0-9_=]+/g)) {
    aimed += 1;
    if (!sequences.has(hit[0])) blame(`${hit[0]} is off the ledger`);
  }
  for (const hit of line.matchAll(/\bA\d{4,8}\b/g)) {
    aimed += 1;
    if (hit[0].length !== 7) blame(`${hit[0]} is not an OEIS id`);
  }
  for (const hit of line.matchAll(/\b([a-z0-9-]+)\.md\b/g)) {
    aimed += 1;
    if (!['research/notes', 'research', 'research/claims', 'research/papers', 'research/wiki'].some((folder) => there(`${folder}/${hit[1]}.md`)))
      blame(`${hit[1]}.md names no page`);
  }
}
report('pointers', `${aimed} pointers`, pointers);

// FIGURES

const drawn: string[] = [];
for (const doc of fronted) {
  const figure = doc.front.get('figure');
  if (figure && !pair(figure)) drawn.push(`${doc.name} figure ${figure} has no pair`);
  for (const [n, line] of plain(doc))
    for (const hit of line.matchAll(/!\[[^\]\n]*\]\(([^)\s]+)\)/g)) {
      const target = hit[1];
      if (target.includes('/') || target.includes('.')) continue;
      if (!pinned.has(target) && !pair(target)) drawn.push(`${doc.name}:${n} image ${target} is not drawn`);
    }
}
for (const base of pairs) {
  if (base.startsWith('research-')) {
    const slug = base.slice(9);
    if (slug !== 'index' && !noted.has(slug) && !there(`research/${slug}.md`)) drawn.push(`${base} shades no note`);
  }
  if (base.startsWith('demo-') && !demos.has(base.slice(5))) drawn.push(`${base} shades no demo`);
  if (base.startsWith('wiki-') && !there(`research/wiki/${base.slice(5)}.md`)) drawn.push(`${base} shades no wiki page`);
}
report('figures', `${pairs.length} pairs`, drawn);

// LOCK

const pressed = resolve(root, '..', 'data/mrlyprod/figures/figures.lock');
report('lock', `${Object.keys(locked).length} rows`, law(pairs, locked, existsSync(pressed) ? JSON.parse(readFileSync(pressed, 'utf8')) : null));

// LINKS

const dead: string[] = [];
let aimedAt = 0;
for (const doc of prose) {
  const home = dirname(at(doc.name));
  for (const [n, line] of plain(doc))
    for (const hit of line.matchAll(/\[[^\]\n]*\]\(([^)\s]+)\)/g)) {
      const target = hit[1].split('#')[0];
      if (target === '' || /^(https?:|mailto:)/.test(target)) continue;
      const widget = target.match(/^demos\/([a-z0-9-]+)\/([a-z0-9-]+)$/);
      if (widget) {
        const file = `site/demos/views/${widget[1]}/widget.jsx`;
        if (!there(file)) dead.push(`${doc.name}:${n} ${target} has no widget.jsx`);
        else if (!new RegExp(`^export (?:function|const) ${widget[2]}\\b`, 'm').test(readFileSync(at(file), 'utf8'))) dead.push(`${doc.name}:${n} ${target} is not a view`);
        continue;
      }
      aimedAt += 1;
      if (target.startsWith('/')) {
        const [head, name] = target.split('/').filter((part) => part !== '');
        if (head !== 'demos') dead.push(`${doc.name}:${n} ${target} is a site URL, link the file`);
        else if (name && !demos.has(name)) dead.push(`${doc.name}:${n} ${target}`);
      } else if (target.includes('/') || target.includes('.')) {
        if (!existsSync(resolve(home, target))) dead.push(`${doc.name}:${n} ${target}`);
      }
    }
}
report('links', `${aimedAt} links`, dead);

// DEMOS

const shown = new Set<string>();
for (const doc of [...notes, ...papers, ...pages, ...posts, ...concepts, ...(there('README.md') ? [read('README.md')] : [])])
  for (const hit of doc.lines.join('\n').matchAll(/demos\/([a-z0-9-]+)\//g)) shown.add(hit[1]);
const orphans = [...demos].filter((name) => !shown.has(name)).sort();
report('demos', `${demos.size} demos`, orphans.map((name) => `${name} is linked nowhere`));

// NOTES

const written: string[] = [];
for (const doc of notes) {
  const slug = stem(doc.name);
  for (const key of ['title', 'lead', 'figure', 'slug']) if (!doc.front.has(key)) written.push(`${doc.name} has no ${key}`);
  if (doc.front.get('slug') !== slug) written.push(`${doc.name} slug is ${doc.front.get('slug')}`);
  doc.lines.slice(doc.body).forEach((line, i) => {
    const where = `${doc.name}:${doc.body + i + 1}`;
    if (line.startsWith('# ')) written.push(`${where} carries an H1`);
    if (/\b20\d\d-\d\d-\d\d\b/.test(line)) written.push(`${where} carries a date`);
  });
}
for (const doc of [...notes, ...claims]) if (!/^[a-z0-9-]+$/.test(stem(doc.name))) written.push(`${doc.name} is not a slug`);
report('notes', `${notes.length} notes`, written);

// WIKI

const taught: string[] = [];
const slugs = new Set(concepts.map((doc) => stem(doc.name)));
for (const doc of concepts) {
  for (const key of ['title', 'lead']) if (!doc.front.has(key)) taught.push(`${doc.name} has no ${key}`);
  for (const need of (doc.front.get('prerequisites') ?? '').split(',').map((s) => s.trim()).filter(Boolean))
    if (!slugs.has(need)) taught.push(`${doc.name} needs ${need}, which has no page`);
  if (!doc.lines.slice(doc.body).some((line) => line.startsWith('## In the tree'))) taught.push(`${doc.name} never says where it appears in the tree`);
  doc.lines.slice(doc.body).forEach((line, i) => {
    const where = `${doc.name}:${doc.body + i + 1}`;
    if (line.startsWith('# ')) taught.push(`${where} carries an H1`);
    if (/\b20\d\d-\d\d-\d\d\b/.test(line)) taught.push(`${where} carries a date`);
  });
}
report('wiki', `${concepts.length} pages`, taught);

// REFS

const HEADER = '| ref | title | url |';
const RULE = '|---|---|---|';
const refs: string[] = [];
let cited = 0;
if (there('research/REFS.md')) {
  const doc = read('research/REFS.md');
  let section: string | null = null;
  let state: string | null = null;
  const seen = new Map<string, number>();
  doc.lines.forEach((line, i) => {
    const where = `research/REFS.md:${i + 1}`;
    if (line.startsWith('## ')) { section = line.slice(3).trim(); state = null; return; }
    if (line.startsWith('|')) {
      if (section === null || section === 'UNRESOLVED') { refs.push(`${where} table row outside a table section`); return; }
      if (state === null) { if (line !== HEADER) refs.push(`${where} table opens on the wrong header`); state = 'header'; return; }
      if (state === 'header') { if (line !== RULE) refs.push(`${where} header rule is wrong`); state = 'rows'; return; }
      const cells = line.split('|');
      if (cells.length !== 5 || cells[0] !== '' || cells[4] !== '') { refs.push(`${where} ${cells.length - 1} pipes`); return; }
      const [ref, title, url] = cells.slice(1, 4).map((cell) => cell.trim());
      cited += 1;
      if (!ref || !title) refs.push(`${where} empty ref or title`);
      if (!/^https?:\/\/\S+$/.test(url)) refs.push(`${where} url cell is not one URL`);
      if (seen.has(ref)) refs.push(`${where} ref ${ref} is already at line ${seen.get(ref)}`);
      seen.set(ref, i + 1);
      return;
    }
    if (line.startsWith('- ') && section !== null && section !== 'UNRESOLVED') refs.push(`${where} bullet inside ${section}`);
    else if (line !== '' && !line.startsWith('#') && state === 'rows') refs.push(`${where} stray line after the rows of ${section}`);
  });
} else refs.push('research/REFS.md is missing');
report('refs', `${cited} refs`, refs);

// HOUSE

const house: string[] = [];
let ruled = 0;
for (const doc of housed) {
  ruled += doc.lines.length;
  doc.lines.forEach((line, i) => {
    if (i > 0 && line === '' && doc.lines[i - 1] === '') house.push(`${doc.name}:${i + 1} two blank lines`);
  });
  for (const [n, line] of plain(doc)) {
    if (/[–—]/.test(line)) house.push(`${doc.name}:${n} em-dash or en-dash`);
    if (line.startsWith('#') && n < doc.lines.length && doc.lines[n] !== '') house.push(`${doc.name}:${n} heading without a blank line`);
  }
}
report('house', `${housed.length} files, ${ruled} lines`, house);

// REGISTRIES

const registries: string[] = [];
if (there('site/pages.json')) registries.push('site/pages.json still exists');
report('registries', 'none', registries);

// WASM

if (process.argv.includes('--wasm')) checks.push(...(await import('./check/wasm.ts')).default);

let failed = 0;
for (const [label, got, want] of checks) {
  const ok = got === want;
  if (!ok) failed += 1;
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${String(label).padEnd(26)} ${String(got)}${ok ? '' : ` (want ${String(want)})`}`);
}
console.log(failed ? `${failed} failed` : `${checks.length} checks green`);
console.log(`${(performance.now() - started).toFixed(0)} ms`);
process.exit(failed ? 1 : 0);
