import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';

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
  there(folder) ? readdirSync(at(folder)).filter((n) => n.endsWith('.md')).sort().map((n) => `${folder}/${n}`) : [];

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
const stems = ['research/README.md', 'research/REFS.md', 'research/sequences.md'].filter((name) => there(name)).map(read);
const fronted = [...notes, ...papers, ...pages, ...posts];
const prose = [...notes, ...claims, ...papers, ...stems, ...pages, ...posts];
const housed = [...under('research', '.md'), ...sheets('site/pages'), ...sheets('site/blog')].map(read);
const demos = new Set(
  there('site/demos')
    ? readdirSync(at('site/demos'), { withFileTypes: true })
        .filter((entry) => entry.isDirectory() && there(`site/demos/${entry.name}/index.html`))
        .map((entry) => entry.name)
    : [],
);
const figures = new Set(there('files/figures') ? readdirSync(at('files/figures')) : []);
const pairs = [...figures].filter((name) => name.endsWith('-dark.png')).map((name) => name.slice(0, -9));
const pair = (name: string) => figures.has(`${name}-dark.png`) && figures.has(`${name}-light.png`);
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

// POINTERS

const NAME = /\b(?:fn|const|static|struct|enum|trait|type|mod|union)\s+([A-Za-z_]\w*)/g;
const declared = new Map<string, Set<string>>();
for (const crate of there('crates')
  ? readdirSync(at('crates'), { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => entry.name)
  : []) {
  const names = new Set<string>();
  for (const file of under(`crates/${crate}/src`, '.rs')) {
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
    if (!['research/notes', 'research', 'research/claims', 'research/papers'].some((folder) => there(`${folder}/${hit[1]}.md`)))
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
      if (!figures.has(`${target}.png`) && !pair(target)) drawn.push(`${doc.name}:${n} image ${target} is not drawn`);
    }
}
for (const base of pairs) {
  if (base.startsWith('research-')) {
    const slug = base.slice(9);
    if (slug !== 'index' && !noted.has(slug) && !there(`research/${slug}.md`)) drawn.push(`${base} shades no note`);
  }
  if (base.startsWith('demo-') && !demos.has(base.slice(5))) drawn.push(`${base} shades no demo`);
}
report('figures', `${pairs.length} pairs`, drawn);

// LINKS

const site = (target: string) => {
  const parts = target.split('/').filter((part) => part !== '');
  if (parts.length === 0) return true;
  const [head, next] = parts;
  if (head === 'research') {
    if (!next) return true;
    if (next === 'discoveries') return there('research/claims');
    return there(`research/notes/${next}.md`) || there(`research/${next}.md`);
  }
  if (head === 'demos') return !next || demos.has(next);
  if (head === 'papers') return !next || there(`research/papers/${next}.md`);
  if (head === 'blog') return !next || there(`site/blog/${next}.md`);
  if (head === 'wiki') return !there('site/wiki') || there(`site/wiki/${next}.md`);
  if (head === 'tools' || head === 'math') return !next;
  if (head === 'method') return there('research/notes/method.md') || there('site/pages/method.md');
  return there(`site/pages/${head}.md`);
};

const dead: string[] = [];
let aimedAt = 0;
for (const doc of prose) {
  const home = dirname(at(doc.name));
  for (const [n, line] of plain(doc))
    for (const hit of line.matchAll(/\[[^\]\n]*\]\(([^)\s]+)\)/g)) {
      const target = hit[1].split('#')[0];
      if (target === '' || /^(https?:|mailto:)/.test(target)) continue;
      aimedAt += 1;
      if (target.startsWith('/')) {
        if (!site(target)) dead.push(`${doc.name}:${n} ${target}`);
      } else if (target.includes('/') || target.includes('.')) {
        if (!existsSync(resolve(home, target))) dead.push(`${doc.name}:${n} ${target}`);
      }
    }
}
report('links', `${aimedAt} links`, dead);

// DEMOS

const shown = new Set<string>();
for (const doc of [...notes, ...papers, ...pages, ...posts, ...(there('README.md') ? [read('README.md')] : [])])
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
if (there('crates/mrlyfig/Cargo.toml') && readFileSync(at('crates/mrlyfig/Cargo.toml'), 'utf8').includes('[[example]]'))
  registries.push('mrlyfig Cargo.toml carries an [[example]]');
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
