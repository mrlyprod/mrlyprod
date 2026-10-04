import { halves } from '../kit/ssg/pic.ts';
import { letters } from '../kit/font/font.js';
import { FEW, fills } from './bar.js';
import { conf, HUES } from './config.js';
import { Contents } from './contents.jsx';
import { crumbs } from './crumbs.js';
import { SAVERS } from './savers/index.js';

/* GLYPHS */

function Cells({ rows, cols, grid, className, label, fill, children }) {
  const cells = [];
  grid.forEach((row, y) =>
    row.forEach((on, x) => {
      if (!on && !fill) return;
      const i = y * cols + x;
      cells.push(<rect key={i} className={on ? undefined : 'ink'} style={on ? undefined : { '--i': i }} x={x} y={y} width={1} height={1} />);
    }),
  );
  return (
    <svg className={className ? `glyphs ${className}` : 'glyphs'} viewBox={`0 0 ${cols} ${rows}`} role={label ? 'img' : undefined} aria-label={label} aria-hidden={label ? undefined : true}>
      {cells}
      {children}
    </svg>
  );
}

export function Glyph({ text, className, label }) {
  return <Cells {...letters(text)} className={className} label={label} />;
}

const pixels = (rows) => rows.map((row) => [...row].map((c) => (c === '1' ? 1 : 0)));

const PANEL = {
  left: ['11111', '11001', '11001', '11001', '11111'],
  right: ['11111', '10011', '10011', '10011', '11111'],
};

function Panel({ side }) {
  return <Cells rows={5} cols={5} grid={pixels(PANEL[side])} fill />;
}

function Ring() {
  const dots = [];
  for (let y = 1; y <= 3; y++) for (let x = 1; x <= 3; x++) dots.push(<rect key={`${x}.${y}`} className="dot" x={x} y={y} width={1} height={1} />);
  return <Cells {...letters('O')}>{dots}</Cells>;
}

function Wordmark({ className }) {
  return <Glyph text={conf().title.toUpperCase()} className={className} />;
}

/* HEADER */

function Header({ dialog }) {
  const site = conf();
  return (
    <header className="top">
      <a className="glyph" href={site.menu} aria-label="Menu" data-router={dialog}>
        <Glyph text="+" />
      </a>
      <a className="mark" href="/" aria-label={`${site.title} home`}>
        <Wordmark />
      </a>
      <a className="glyph" href={site.cart} data-cart aria-label="Cart">
        <Ring />
      </a>
    </header>
  );
}

/* SUBHEADER */

function Opener({ side, label, late }) {
  return (
    <button type="button" className="glyph" data-pane={side} aria-controls={side} aria-expanded="false" aria-label={label} hidden={late || undefined} data-few={late ? FEW : undefined}>
      <Panel side={side} />
    </button>
  );
}

function Subheader({ route = '/', left = false, bar = false, late = false }) {
  const trail = route.endsWith('/') ? crumbs(route) : [];
  return (
    <div className="subheader">
      {trail.length > 0 && (
        <nav className="crumbs" aria-label="Breadcrumb">
          <ol>
            {trail.map((crumb, n) => (
              <li key={crumb.href}>
                <a href={crumb.href} aria-current={n === trail.length - 1 ? 'page' : undefined}>{crumb.name}</a>
              </li>
            ))}
          </ol>
        </nav>
      )}
      <div className="actions">
        {left && <Opener side="left" label="Files" />}
        {(bar || late) && <Opener side="right" label="Page tools" late={!bar} />}
      </div>
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
      <details open={holds(node, current) || undefined} data-lazy={lazy ? node.lazy : undefined}>
        <summary>{node.href ? <Leaf node={node} here={here} /> : node.name}</summary>
        <ul>{(node.nodes ?? []).map((sub) => <Node key={sub.name} node={sub} current={current} />)}</ul>
      </details>
    </li>
  );
}

const EXPLORER = '/git.json';

const hasLazy = (nodes) => nodes.some((node) => node.lazy !== undefined || hasLazy(node.nodes ?? []));

function Tree({ nodes = [], current = '' }) {
  const source = hasLazy(nodes) ? EXPLORER : undefined;
  return <ul className="tree" data-source={source}>{nodes.map((node) => <Node key={node.name} node={node} current={current} />)}</ul>;
}

/* MENU */

function Dates({ dates }) {
  if (!dates?.length) return null;
  return <p className="dates">{dates.map((date) => <span key={date}>{date}</span>)}</p>;
}

function Card({ node }) {
  if (!node.figure) return <a className="tile plain" href={node.href}><h2>{node.name}</h2>{node.text && <p>{node.text}</p>}<Dates dates={node.dates} /></a>;
  return (
    <a className="tile" href={node.href}>
      <picture dangerouslySetInnerHTML={{ __html: halves(node.figure, '', '', ' loading="lazy" decoding="async"') }} />
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

const anchor = (name) => name.toLowerCase();

export function Menu({ tree = [], island, search }) {
  return (
    <div className="menu" data-island={island} data-search={search}>
      <Grid nodes={tree.map(({ nodes, ...node }) => (nodes ? { ...node, href: `#${anchor(node.name)}` } : node))} />
      {tree.filter((node) => node.nodes).map((folder) => (
        <section key={folder.name} id={anchor(folder.name)} aria-label={folder.name}>
          <h2>{folder.name}</h2>
          <Grid nodes={folder.nodes} />
        </section>
      ))}
    </div>
  );
}

/* BAR */

function Controls({ children }) {
  return <section className="controls" aria-label="Controls">{children}</section>;
}

const FACES = [
  ['', 'System'],
  ['sans', 'Noto Sans'],
  ['serif', 'Noto Serif'],
  ['mono', 'Noto Sans Mono'],
  ['mrly', 'MrlyFont'],
];

const TINTS = [['', 'Auto'], ...HUES.map((hue) => [hue, hue[0].toUpperCase() + hue.slice(1)])];

const SCREENS = [['', 'Wordmark'], ...SAVERS.map((name) => [name, name[0].toUpperCase() + name.slice(1)])];

function Pick({ label, name, options }) {
  return (
    <label className="pick">
      {label}
      <select {...{ [name]: true }} defaultValue="">
        {options.map(([value, text]) => <option key={value} value={value}>{text}</option>)}
      </select>
    </label>
  );
}

export function Settings() {
  return (
    <section className="settings" aria-label="Settings">
      <div className="row">
        <button type="button" className="theme" data-theme-toggle>Theme <b>auto</b></button>
        <Pick label="Font" name="data-font-pick" options={FACES} />
        <Pick label="Tint" name="data-tint-pick" options={TINTS} />
        <Pick label="Saver" name="data-saver-pick" options={SCREENS} />
      </div>
    </section>
  );
}

/* FOOTER */

function Mark() {
  const site = conf();
  const text = site.title.toUpperCase();
  const { rows, cols } = letters(text);
  return (
    <>
      <canvas className="mark" width={cols + 2} height={rows + 2} data-text={text} role="img" aria-label={site.title}></canvas>
      <Wordmark className="still" />
    </>
  );
}

function Footer() {
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

export function Shell({ route = '/', tree = [], current = route, contents = [], controls, late = false, wide = false, dialog, children }) {
  const left = tree.length > 0;
  const bar = Boolean(controls) || fills(contents);
  return (
    <>
      <a className="skip" href="#main">Skip to content</a>
      <Header dialog={dialog} />
      <Subheader route={route} left={left} bar={bar} late={late} />
      <div className="panes">
        {left && (
          <nav className="pane left" id="left" aria-label="Files">
            <Tree nodes={tree} current={current} />
          </nav>
        )}
        <main id="main" tabIndex={-1} className={wide ? 'wide' : undefined}>
          {children}
        </main>
        {bar && (
          <aside className="pane right" id="right" aria-label="Page tools">
            {controls && <Controls>{controls}</Controls>}
            <Contents items={contents} />
          </aside>
        )}
        {(left || bar || late) && <div className="scrim"></div>}
      </div>
      <Footer />
    </>
  );
}
