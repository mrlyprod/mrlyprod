import { useEffect } from 'react';
import { letters } from './font.js';
import { wire } from './chrome.js';
import { conf } from './config.js';

/* GLYPHS */

function Cells({ rows, cols, grid, className, label }) {
  const cells = [];
  grid.forEach((row, y) => row.forEach((on, x) => on && cells.push(<rect key={`${x}.${y}`} x={x} y={y} width={1} height={1} />)));
  return (
    <svg className={className ? `glyphs ${className}` : 'glyphs'} viewBox={`0 0 ${cols} ${rows}`} role={label ? 'img' : undefined} aria-label={label} aria-hidden={label ? undefined : true}>
      {cells}
    </svg>
  );
}

function Glyph({ text, className, label }) {
  return <Cells {...letters(text)} className={className} label={label} />;
}

const PANEL = {
  left: ['11111', '11001', '11001', '11001', '11111'],
  right: ['11111', '10011', '10011', '10011', '11111'],
};

function Panel({ side, className }) {
  const grid = PANEL[side].map((row) => [...row].map((c) => (c === '1' ? 1 : 0)));
  return <Cells rows={5} cols={5} grid={grid} className={className} />;
}

function Ring() {
  const dots = [];
  for (let y = 1; y <= 3; y++) for (let x = 1; x <= 3; x++) dots.push(<rect key={`${x}.${y}`} className="dot" x={x} y={y} width={1} height={1} />);
  return (
    <svg className="glyphs" viewBox="0 0 5 5" aria-hidden="true">
      <path fillRule="evenodd" d="M0 0h5v5H0zM1 1v3h3V1z" />
      {dots}
    </svg>
  );
}

export function Wordmark({ className }) {
  const site = conf();
  if (!site.font) return <span className={className ? `word ${className}` : 'word'}>{site.title}</span>;
  return <Glyph text={site.title.toUpperCase()} className={className} />;
}

/* HEADER */

export function Header({ brand }) {
  const site = conf();
  return (
    <header className="top">
      <a className="glyph" href={site.menu} aria-label="Menu">
        <Glyph text="+" />
      </a>
      {brand ?? (
        <a className="mark" href="/" aria-label={`${site.title} home`}>
          <Wordmark />
        </a>
      )}
      <a className="glyph" href={site.cart} data-cart aria-label="Cart">
        <Ring />
      </a>
    </header>
  );
}

export function Dock({ route = '/' }) {
  const word = decodeURIComponent(route).split('/').filter(Boolean).pop() ?? 'home';
  return (
    <div className="dock">
      <button type="button" className="glyph" data-pane="left" aria-controls="left" aria-expanded="false" aria-label="Site tree">
        <Panel side="left" className="shut" />
        <Glyph text="×" className="open" />
      </button>
      <span className="route">
        <Glyph text={word.toUpperCase()} label={word} />
      </span>
      <button type="button" className="glyph" data-pane="right" aria-controls="right" aria-expanded="false" aria-label="Page tools">
        <Panel side="right" className="shut" />
        <Glyph text="×" className="open" />
      </button>
    </div>
  );
}

/* TREE */

const holds = (node, current) => node.href === current || (node.nodes ?? []).some((sub) => holds(sub, current));

function Leaf({ node, here }) {
  return (
    <a href={node.href} aria-current={here}>
      {node.icon && <span className={node.icon} aria-hidden="true"></span>}
      {node.icon ? <span className="name">{node.name}</span> : node.name}
    </a>
  );
}

function Node({ node, current }) {
  const here = node.href === current ? 'page' : undefined;
  const lazy = node.lazy !== undefined;
  if (!node.nodes && !lazy) return <li><Leaf node={node} here={here} /></li>;
  return (
    <li>
      <details open={node.open || holds(node, current) || undefined} data-lazy={lazy ? node.lazy : undefined}>
        <summary>{node.href ? <Leaf node={node} here={here} /> : node.name}</summary>
        <ul>{(node.nodes ?? []).map((sub) => <Node key={sub.name} node={sub} current={current} />)}</ul>
      </details>
    </li>
  );
}

const hasLazy = (nodes) => nodes.some((node) => node.lazy !== undefined || hasLazy(node.nodes ?? []));

export function Tree({ nodes = [], current = '' }) {
  const source = hasLazy(nodes) ? conf().explorer : undefined;
  return <ul className="tree" data-source={source}>{nodes.map((node) => <Node key={node.name} node={node} current={current} />)}</ul>;
}

/* MENU */

function Card({ node }) {
  if (!node.figure) return <a className="tile plain" href={node.href}><h2>{node.name}</h2>{node.text && <p>{node.text}</p>}</a>;
  return (
    <a className="tile" href={node.href}>
      <img className="dark" src={node.figure.dark} alt="" width="1024" height="1024" loading="lazy" decoding="async" />
      <img className="light" src={node.figure.light} alt="" width="1024" height="1024" loading="lazy" decoding="async" />
      <h2>{node.name}</h2>
      {node.text && <p>{node.text}</p>}
    </a>
  );
}

const leaves = (nodes) => nodes.filter((node) => node.href && !(node.nodes && node.nodes.length));

const groups = (nodes) => nodes.filter((node) => node.nodes && node.nodes.length);

function Grid({ nodes }) {
  const list = leaves(nodes);
  if (!list.length) return null;
  return <div className="gallery grid">{list.map((node) => <Card key={node.href} node={node} />)}</div>;
}

export function Menu({ tree = [] }) {
  const pages = leaves(tree);
  return (
    <div className="menu">
      {groups(tree).map((group) => (
        <section key={group.name} aria-label={group.name}>
          <h2>{group.href ? <a href={group.href}>{group.name}</a> : group.name}</h2>
          <Grid nodes={group.nodes} />
          {groups(group.nodes).map((shelf) => (
            <div key={shelf.name} className="shelf">
              <h3>{shelf.href ? <a href={shelf.href}>{shelf.name}</a> : shelf.name}</h3>
              <Grid nodes={shelf.nodes} />
            </div>
          ))}
        </section>
      ))}
      {pages.length > 0 && (
        <section aria-label="Pages">
          <h2>Pages</h2>
          <Grid nodes={pages} />
        </section>
      )}
    </div>
  );
}

/* CONTENTS */

export function Contents({ items = [], current = '' }) {
  return (
    <nav className="contents" aria-label="Contents">
      <h2>Contents</h2>
      <ol>
        {items.map((item) => (
          <li key={item.id} className={`h${item.level ?? 2}`}>
            <a href={`#${item.id}`} aria-current={item.id === current ? 'location' : undefined}>{item.text}</a>
          </li>
        ))}
      </ol>
    </nav>
  );
}

export function Controls({ children }) {
  return <section className="controls" aria-label="Controls">{children}</section>;
}

const FACES = [
  ['', 'System'],
  ['sans', 'Noto Sans'],
  ['serif', 'Noto Serif'],
  ['mono', 'Noto Sans Mono'],
  ['mrly', 'MrlyFont'],
];

export function Settings() {
  return (
    <section className="settings" aria-label="Settings">
      <h2>Settings</h2>
      <div className="row">
        <button type="button" className="theme" data-theme-toggle>Theme <b>auto</b></button>
        <label className="face">
          Font
          <select data-font-pick defaultValue="">
            {FACES.map(([value, name]) => <option key={value} value={value}>{name}</option>)}
          </select>
        </label>
      </div>
    </section>
  );
}

/* FOOTER */

function Mark() {
  const site = conf();
  if (!site.font) return <Wordmark className="still" />;
  const text = site.title.toUpperCase();
  const { rows, cols } = letters(text);
  return (
    <>
      <canvas className="mark" width={cols + 2} height={rows + 2} data-text={text} role="img" aria-label={site.title}></canvas>
      <Wordmark className="still" />
    </>
  );
}

export function Footer() {
  const site = conf();
  const year = new Date().getFullYear();
  const span = site.since < year ? `${site.since}-${year}` : String(year);
  return (
    <footer className="base">
      <a href="/" aria-label={`${site.title} home`}><Mark /></a>
      <p className="legal fine">Copyright © {site.company || site.title} {span}. All rights reserved.</p>
    </footer>
  );
}

/* SHELL */

export function Shell({ route = '/', title, lead, tree = [], current = route, contents = [], controls, wide = false, brand, children }) {
  const site = conf();
  useEffect(() => {
    wire();
  }, []);
  return (
    <>
      <a className="skip" href="#main">Skip to content</a>
      <Header brand={brand} />
      <Dock route={route} />
      <div className="panes">
        <nav className="pane left" id="left" aria-label="Site">
          <Tree nodes={tree} current={current} />
        </nav>
        <main id="main" tabIndex={-1} className={wide ? 'wide' : undefined}>
          {title && (
            <div className="lede">
              <h1>{title}</h1>
              {lead && <p className="lead">{lead}</p>}
            </div>
          )}
          {children}
        </main>
        <aside className="pane right" id="right" aria-label="Page tools">
          {controls && <Controls>{controls}</Controls>}
          {contents.length > 0 && <Contents items={contents} />}
          {site.settings && <Settings />}
        </aside>
        <div className="scrim"></div>
      </div>
      <Footer />
    </>
  );
}
