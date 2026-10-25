import type { Site } from "site:routes";
import type { CSSProperties, ReactNode } from "react";
import { flushSync } from "react-dom";
import type { Root } from "react-dom/client";
import { createRoot } from "react-dom/client";
import type { Row } from "../../scripts/build.ts";

/* TYPES */

export type Vars = CSSProperties & Record<`--${string}`, string | number>;

export type FigureProps = { name: string; label?: string; className?: string; caption?: ReactNode };

export type LedeProps = { id?: string; title: string; lead?: string; children?: ReactNode };

export type CardNode = { href: string; name: string; figure?: string | null; text?: string; dates?: string[] };

export type Art = keyof typeof ART;

export type GlyphProps = { art: Art; className?: string };

export type LostProps = { site: Pick<Site, "menu">; rows: Row[]; row?: Row };

export type Heading = { id: string; level?: number; text: string | null };

const FEW = 3;

const fills = (items: Heading[]) => items.length >= FEW;

/* ROOTS */

const roots = new WeakMap<Element, Root>();

export function show(host: Element, node: ReactNode) {
  const root = roots.get(host) ?? createRoot(host);
  roots.set(host, root);
  flushSync(() => root.render(node));
}

export function hide(host: Element) {
  roots.get(host)?.unmount();
  roots.delete(host);
}

/* FIGURE */

export function Figure({ name, label = "", className, caption }: FigureProps) {
  return (
    <figure className={className} data-figure={name}>
      <canvas role={label ? "img" : undefined} aria-label={label || undefined} aria-hidden={label ? undefined : true} />
      {caption !== undefined && <figcaption>{caption}</figcaption>}
    </figure>
  );
}

/* LEDE */

export function Lede({ id, title, lead, children }: LedeProps) {
  return (
    <div className="lede">
      <h1 id={id}>{title}</h1>
      {lead && <p className="lead">{lead}</p>}
      {children}
    </div>
  );
}

/* GRID */

function Dates({ dates }: { dates?: string[] }) {
  if (!dates?.length) return null;
  return (
    <p className="dates">
      {dates.map((date) => (
        <span key={date}>{date}</span>
      ))}
    </p>
  );
}

export function Card({ node }: { node: CardNode }) {
  return (
    <a className={node.figure ? "tile" : "tile plain"} href={node.href}>
      {node.figure && <Figure name={node.figure} />}
      <h2>{node.name}</h2>
      {node.text && <p>{node.text}</p>}
      <Dates dates={node.dates} />
    </a>
  );
}

export function Grid({ nodes = [] }: { nodes?: CardNode[] }) {
  if (!nodes.length) return null;
  return (
    <div className="gallery grid">
      {nodes.map((node) => (
        <Card key={node.href} node={node} />
      ))}
    </div>
  );
}

/* GLYPH */

export const ART = {
  plus: "..#.. ..#.. ##### ..#.. ..#..",
  find: "####. #..#. #..#. ####. ....#",
  apps: "##.## ##.## ..... ##.## ##.##",
  minus: "..... ..... ##### ..... .....",
};

export function Glyph({ art, className = "glyphs" }: GlyphProps) {
  const rows = ART[art].split(" ");
  const cells = rows.flatMap((line, y) => [...line].flatMap((c, x) => (c === "#" ? [{ x, y }] : [])));
  return (
    <svg
      className={className}
      viewBox={`0 0 ${rows[0].length} ${rows.length}`}
      style={{ "--rows": rows.length, "--cols": rows[0].length } as Vars}
      aria-hidden="true"
    >
      {cells.map(({ x, y }) => (
        <rect key={`${x}:${y}`} x={x} y={y} width="1" height="1" />
      ))}
    </svg>
  );
}

/* LOST */

export function Lost({ site, rows, row = rows.find((one) => one.meta?.id === "missing")! }: LostProps) {
  return (
    <div className="lede">
      <h1 id="lost">{row.title}</h1>
      <p className="lead">
        {row.lead} The <a href={site.menu}>Launchpad</a> holds every door of this site. A page that moved is on{" "}
        <a href="/redirects/">Redirects</a>.
      </p>
    </div>
  );
}

/* SETTINGS */

export { PANES } from "./panes.ts";

/* CONTENTS */

export function Contents({ items = [] }: { items?: Heading[] }) {
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

export const heads = (root: ParentNode): Heading[] =>
  [...root.querySelectorAll("h2[id], h3[id]")].map((one) => ({
    id: one.id,
    level: Number(one.tagName[1]),
    text: one.textContent,
  }));
