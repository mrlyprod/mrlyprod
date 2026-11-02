import type { Site } from "site:routes";
import type { CSSProperties, ReactNode } from "react";
import { flushSync } from "react-dom";
import type { Root } from "react-dom/client";
import { createRoot } from "react-dom/client";
import { Figure } from "../sdk/host.tsx";
import type { Row } from "../sdk/rows.ts";
import { parts } from "./active.ts";
import type { Props } from "./design.ts";
import { WORD } from "./welcome.ts";

export type { FigureProps } from "../sdk/host.tsx";
export { Figure };

/* TYPES */

export type Vars = CSSProperties & Record<`--${string}`, string | number>;

export type LedeProps = { id?: string; title: string; lead?: string; children?: ReactNode };

export type CardNode = { href: string; name: string; figure?: string | null; text?: string; dates?: string[] };

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

/* DESK */

const bare = () => ({});

export function ground(host: Element, home: boolean) {
  if (!parts.desk) return null;
  show(host, <parts.desk canvas={bare} home={home} />);
  return host.querySelector("canvas");
}

/* ENDING */

const finite = (one: Animation) => one.effect?.getComputedTiming().endTime !== Infinity;

export function ending(host: Element, change: () => void): Promise<unknown> | null {
  const before = new Set(host.getAnimations?.({ subtree: true }) ?? []);
  change();
  const fresh = (host.getAnimations?.({ subtree: true }) ?? []).filter((one) => !before.has(one) && finite(one));
  return fresh.length ? Promise.all(fresh.map((one) => one.finished.catch(() => null))) : null;
}

/* LOCK */

export const locks = () => typeof parts.lock === "function";

export function seal(host: Element, state: Props["lock"]["state"]) {
  const Lock = parts.lock;
  if (Lock) show(host, <Lock word={WORD} state={state} bind={bare} />);
}

/* SAVER */

export const saves = () => typeof parts.saver === "function";

export function veil(host: Element, props: Props["saver"]) {
  const Saver = parts.saver;
  if (Saver) show(host, <Saver {...props} />);
}

/* LEDE */

export function Lede({ id, title, lead, children }: LedeProps) {
  return (
    <parts.prose kind="hub" id={id} title={title} lead={lead} contents={[]}>
      {children}
    </parts.prose>
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

/* LOST */

export function Lost({ site, rows, row = rows.find((one) => one.meta?.id === "missing")! }: LostProps) {
  const lead = (
    <>
      {row.lead} The <a href={site.menu}>Launchpad</a> holds every door of this site. A page that moved is on{" "}
      <a href="/redirects/">Redirects</a>.
    </>
  );
  return <parts.prose kind="lost" id="lost" title={row.title} lead={lead} contents={[]} />;
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
