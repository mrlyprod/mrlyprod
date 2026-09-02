import { useEffect } from 'react';
import { letters } from './font.js';
import { wire } from './chrome.js';
import site from './site.json';

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

export function Wordmark({ className }) {
  return <Glyph text={site.title.toUpperCase()} className={className} />;
}

/* HEADER */

export function Header() {
  return (
    <header className="top">
      <button type="button" className="glyph" data-pane="left" aria-controls="left" aria-expanded="false" aria-label="Site menu">
        <Glyph text="+" className="shut" />
        <Glyph text="×" className="open" />
      </button>
      <a className="mark" href="/" aria-label={`${site.title} home`}>
        <Wordmark />
      </a>
      <button type="button" className="glyph" data-pane="right" aria-controls="right" aria-expanded="false" aria-label="Page tools">
        <Glyph text="O" className="shut" />
        <Glyph text="×" className="open" />
      </button>
    </header>
  );
}

/* TREE */

const holds = (node, current) => node.href === current || (node.nodes ?? []).some((sub) => holds(sub, current));

function Node({ node, current }) {
  const here = node.href === current ? 'page' : undefined;
  if (!node.nodes) return <li><a href={node.href} aria-current={here}>{node.name}</a></li>;
  return (
    <li>
      <details open={node.open || holds(node, current) || undefined}>
        <summary>{node.href ? <a href={node.href} aria-current={here}>{node.name}</a> : node.name}</summary>
        <ul>{node.nodes.map((sub) => <Node key={sub.name} node={sub} current={current} />)}</ul>
      </details>
    </li>
  );
}

export function Tree({ nodes = [], current = '' }) {
  return <ul className="tree">{nodes.map((node) => <Node key={node.name} node={node} current={current} />)}</ul>;
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
  return (
    <section className="controls" aria-label="Controls">
      <h2>Controls</h2>
      {children}
    </section>
  );
}

function Settings() {
  return (
    <section className="settings" aria-label="Settings">
      <h2>Settings</h2>
      <button type="button" className="theme" data-theme-toggle>Theme <b>auto</b></button>
    </section>
  );
}

/* FOOTER */

export function Footer() {
  return (
    <footer className="base">
      <canvas className="mark" width={49} height={7} role="img" aria-label={site.title}></canvas>
      <Wordmark className="still" />
      <p className="fine">
        Copyright {site.title}, Inc. {site.since}-{new Date().getFullYear()}
        {site.socials.map((social) => <span key={social.name}> · <a href={social.href}>{social.name}</a></span>)}
      </p>
    </footer>
  );
}

/* SHELL */

export function Shell({ route = '/', title, lead, tree = [], current = route, contents = [], controls, wide = false, children }) {
  useEffect(() => {
    wire();
  }, []);
  return (
    <>
      <a className="skip" href="#main">Skip to content</a>
      <Header />
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
          <Settings />
        </aside>
        <div className="scrim"></div>
      </div>
      <Footer />
    </>
  );
}
