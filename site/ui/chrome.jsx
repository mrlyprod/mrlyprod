import { halves } from '../kit/ssg/pic.ts';
import { letters } from '../kit/font/font.js';
import { Opener } from '../lib/frame.jsx';
import { Btn, Group, Pick } from '../lib/knobs.jsx';
import { FEW, fills } from './bar.js';
import { conf } from './config.js';
import { Contents } from './contents.jsx';
import { crumbs } from './crumbs.js';
import { tints } from './hues.js';
import { markup, welcome } from './welcome.js';
import { word } from './word.js';

/* GLYPHS */

function Cells({ rows, cols, grid, className, label, children }) {
  const cells = [];
  grid.forEach((row, y) =>
    row.forEach((on, x) => {
      if (on) cells.push(<rect key={y * cols + x} x={x} y={y} width={1} height={1} />);
    }),
  );
  return (
    <svg className={className ? `glyphs ${className}` : 'glyphs'} viewBox={`0 0 ${cols} ${rows}`} style={{ '--rows': rows, '--cols': cols }} role={label ? 'img' : undefined} aria-label={label} aria-hidden={label ? undefined : true}>
      {cells}
      {children}
    </svg>
  );
}

function Glyph({ text, className, label }) {
  return <Cells {...letters(text)} className={className} label={label} />;
}

function Ring() {
  const dots = [];
  for (let y = 1; y <= 3; y++) for (let x = 1; x <= 3; x++) dots.push(<rect key={`${x}.${y}`} className="dot" x={x} y={y} width={1} height={1} />);
  return <Cells {...letters('O')}>{dots}</Cells>;
}

/* HEADER */

function Fold() {
  const model = welcome();
  return <svg className="glyphs fold" viewBox={`0 0 ${model.cols} 5`} style={{ '--rows': 5, '--cols': model.cols }} aria-hidden="true" dangerouslySetInnerHTML={{ __html: markup(model) }} />;
}

function Header({ route, doors }) {
  const site = conf();
  const name = word(route, doors);
  return (
    <header className="top">
      <a className="glyph" href={site.menu} aria-label="Menu">
        <Glyph text="+" />
      </a>
      <a className="mark" href="/" aria-label={name ? `${name}, ${site.title} home` : `${site.title} home`} data-word={name} data-doors={JSON.stringify(doors)} style={route === '/' ? { '--cols': welcome().cols } : undefined}>
        {route === '/' ? <Fold /> : <Glyph text={name || 'X'} />}
      </a>
      <a className="glyph" href={site.cart} data-cart aria-label="Cart">
        <Ring />
      </a>
    </header>
  );
}

/* SUBHEADER */

function Subheader({ route = '/', left = false, bar = false, late = false }) {
  const trail = route === '/404.html' ? [{ name: 'not found', href: route }] : route.endsWith('/') ? crumbs(route) : [];
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
        {left && <Opener side="left" label="Files" icon="paneLeft" />}
        {(bar || late) && <Opener side="right" label="Page tools" icon="paneRight" few={bar ? undefined : FEW} />}
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
      <Grid nodes={tree.filter((node) => node.href)} />
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
        <Pick label="Saver" options={[['', 'None'], ['random', 'Random'], ...savers.map((one) => [one.id, one.title])]} data-saver-pick defaultValue="" />
      </Group>
    </section>
  );
}

/* FOOTER */

function Footer() {
  const site = conf();
  const year = new Date().getFullYear();
  const span = site.since < year ? `${site.since}-${year}` : String(year);
  return (
    <footer className="base">
      <p className="legal fine">Copyright © {site.company || site.title} {span}. All rights reserved.</p>
    </footer>
  );
}

/* SHELL */

export function Shell({ route = '/', tree = [], doors = [], current = route, contents = [], controls, late = false, wide = false, children }) {
  const left = tree.length > 0;
  const bar = Boolean(controls) || fills(contents);
  return (
    <>
      <a className="skip" href="#main">Skip to content</a>
      <Header route={route} doors={doors} />
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
