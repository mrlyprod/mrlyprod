import { useEffect } from 'react';
import { letters } from './font.js';
import { wire } from './chrome.js';
import { conf } from './config.js';

/* GLYPHS */

function Glyph({ text, className, label }) {
  const { rows, cols, grid } = letters(text);
  const cells = [];
  grid.forEach((row, y) => row.forEach((on, x) => on && cells.push(<rect key={`${x}.${y}`} x={x} y={y} width={1} height={1} />)));
  return (
    <svg className={className ? `glyphs ${className}` : 'glyphs'} viewBox={`0 0 ${cols} ${rows}`} role={label ? 'img' : undefined} aria-label={label} aria-hidden={label ? undefined : true}>
      {cells}
    </svg>
  );
}

function Ring() {
  const dots = [];
  for (let y = 1; y <= 3; y++) for (let x = 1; x <= 3; x++) dots.push(<rect key={`${x}.${y}`} className="dot" x={x} y={y} width={1} height={1} />);
  return (
    <svg className="glyphs shut" viewBox="0 0 5 5" aria-hidden="true">
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
      <a className="glyph" href={site.menu} data-pane="left" aria-controls="left" aria-expanded="false" aria-label="Menu">
        <Glyph text="+" className="shut" />
        <Glyph text="×" className="open" />
      </a>
      {brand ?? (
        <a className="mark" href="/" aria-label={`${site.title} home`}>
          <Wordmark />
        </a>
      )}
      <a className="glyph" href={site.cart} data-pane="right" data-cart aria-controls="right" aria-expanded="false" aria-label="Cart">
        <Ring />
        <Glyph text="×" className="open" />
      </a>
    </header>
  );
}

/* TREE */

const holds = (node, current) => node.href === current || (node.nodes ?? []).some((sub) => holds(sub, current));

function Node({ node, current }) {
  const here = node.href === current ? 'page' : undefined;
  const lazy = node.lazy !== undefined;
  if (!node.nodes && !lazy) return <li><a href={node.href} aria-current={here}>{node.name}</a></li>;
  return (
    <li>
      <details open={node.open || holds(node, current) || undefined} data-lazy={lazy ? node.lazy : undefined}>
        <summary>{node.href ? <a href={node.href} aria-current={here}>{node.name}</a> : node.name}</summary>
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

function Branch({ nodes }) {
  return (
    <ul>
      {nodes.map((node) => (
        <li key={node.name}>
          {node.href ? <a href={node.href} className={node.nodes ? 'group' : undefined}>{node.name}</a> : <span className="group">{node.name}</span>}
          {node.nodes && node.nodes.length > 0 && <Branch nodes={node.nodes} />}
        </li>
      ))}
    </ul>
  );
}

export function Menu({ tree = [] }) {
  const groups = tree.filter((node) => node.nodes && node.nodes.length);
  const pages = tree.filter((node) => !(node.nodes && node.nodes.length) && node.href);
  return (
    <div className="menu">
      {groups.map((group) => (
        <section key={group.name} aria-label={group.name}>
          <h2>{group.href ? <a href={group.href}>{group.name}</a> : group.name}</h2>
          <Branch nodes={group.nodes} />
        </section>
      ))}
      {pages.length > 0 && (
        <section aria-label="Pages">
          <h2>Pages</h2>
          <Branch nodes={pages} />
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
      <div className="foot">
        <div className="who">
          <a href="/" aria-label={`${site.title} home`}><Mark /></a>
          {site.tagline && <p className="tag fine">{site.tagline}</p>}
        </div>
        {site.footer.map((section) => (
          <nav key={section.name} aria-label={section.name}>
            <h2>{section.name}</h2>
            <ul>{section.links.map((link) => <li key={link.href}><a href={link.href}>{link.name}</a></li>)}</ul>
          </nav>
        ))}
        {site.socials.length > 0 && (
          <nav className="social" aria-label="Social">
            <h2>Social</h2>
            <ul>{site.socials.map((social) => <li key={social.href}><a href={social.href} rel="me noopener">{social.name}</a></li>)}</ul>
          </nav>
        )}
      </div>
      <p className="legal fine">
        <span>Copyright {site.company || site.title} {span}. All rights reserved.</span>
        {site.contact && <a href={`mailto:${site.contact}`}>{site.contact}</a>}
      </p>
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
          <a className="cartrow" href={site.cart}>Cart <b data-cart-count>0</b></a>
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
