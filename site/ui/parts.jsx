import { flushSync } from 'react-dom';
import { createRoot } from 'react-dom/client';
import { Btn, Group, Pick } from '../lib/knobs.jsx';
import { tints } from './hues.js';

const FEW = 3;

const fills = (items) => items.length >= FEW;

/* ROOTS */

const roots = new WeakMap();

export function show(host, node) {
  const root = roots.get(host) ?? createRoot(host);
  roots.set(host, root);
  flushSync(() => root.render(node));
}

export function hide(host) {
  roots.get(host)?.unmount();
  roots.delete(host);
}

/* FIGURE */

export function Figure({ name, label = '', className, caption }) {
  return (
    <figure className={className} data-figure={name}>
      <canvas role={label ? 'img' : undefined} aria-label={label || undefined} aria-hidden={label ? undefined : true} />
      {caption !== undefined && <figcaption>{caption}</figcaption>}
    </figure>
  );
}

/* LEDE */

export function Lede({ id, title, lead, children }) {
  return (
    <div className="lede">
      <h1 id={id}>{title}</h1>
      {lead && <p className="lead">{lead}</p>}
      {children}
    </div>
  );
}

/* GRID */

function Dates({ dates }) {
  if (!dates?.length) return null;
  return <p className="dates">{dates.map((date) => <span key={date}>{date}</span>)}</p>;
}

export function Card({ node }) {
  return (
    <a className={node.figure ? 'tile' : 'tile plain'} href={node.href}>
      {node.figure && <Figure name={node.figure} />}
      <h2>{node.name}</h2>
      {node.text && <p>{node.text}</p>}
      <Dates dates={node.dates} />
    </a>
  );
}

export function Grid({ nodes = [] }) {
  if (!nodes.length) return null;
  return <div className="gallery grid">{nodes.map((node) => <Card key={node.href} node={node} />)}</div>;
}

/* MENU */

export const GROUPS = [
  ['play', 'Play'],
  ['read', 'Read'],
  ['tool', 'Tools'],
];

export function filled(site, rows) {
  const card = (one) => ({ name: one.title, href: one.route, figure: one.figure ?? undefined });
  const shown = rows.filter((one) => one.meta?.menu && !one.hidden).sort((a, b) => a.title.localeCompare(b.title));
  const groups = GROUPS.map(([key, name]) => ({ name, nodes: shown.filter((one) => one.meta.menu === key).map(card) }));
  const fills = { elsewhere: [...site.socials, { name: site.contact, href: `mailto:${site.contact}` }] };
  const tree = site.tree.map((node) => ({ ...node, nodes: node.nodes ?? fills[node.name.toLowerCase()] ?? [] }));
  return [...groups, ...tree].filter((node) => node.nodes.length);
}

const anchor = (name) => name.toLowerCase();

export function Menu({ tree = [] }) {
  return (
    <div className="menu">
      {tree.map((folder) => (
        <section key={folder.name} id={anchor(folder.name)} aria-label={folder.name}>
          <h2>{folder.name}</h2>
          <Grid nodes={folder.nodes} />
        </section>
      ))}
    </div>
  );
}

/* SETTINGS */

const FACES = [
  ['', 'Default'],
  ['sans', 'Noto Sans'],
  ['serif', 'Noto Serif'],
  ['mono', 'Noto Sans Mono'],
  ['mrly', 'MrlyFont'],
];

const TINTS = tints('Auto');

export function Settings({ savers }) {
  return (
    <section className="controls" aria-label="Settings">
      <Group name="Appearance">
        <Btn data-theme-toggle>Theme <b>auto</b></Btn>
        <Pick label="Font" options={FACES} data-font-pick defaultValue="" />
        <Pick label="Tint" options={TINTS} data-tint-pick defaultValue="" />
      </Group>
      <Group name="Screensaver">
        <Pick label="Saver" options={[['random', 'Random'], ...savers.map((one) => [one.id, one.title])]} data-saver-pick defaultValue={savers[0]?.id ?? 'random'} />
      </Group>
    </section>
  );
}

/* CONTENTS */

export function Contents({ items = [] }) {
  if (!fills(items)) return null;
  return (
    <nav className="contents" aria-label="Contents">
      <details>
        <summary>Contents</summary>
        <ol>
          {items.map((item) => (
            <li key={item.id} className={`h${item.level ?? 2}`}>
              <a href={`#${item.id}`}>{item.text}</a>
            </li>
          ))}
        </ol>
      </details>
    </nav>
  );
}

export const heads = (root) => [...root.querySelectorAll('h2[id], h3[id]')].map((one) => ({ id: one.id, level: Number(one.tagName[1]), text: one.textContent }));
