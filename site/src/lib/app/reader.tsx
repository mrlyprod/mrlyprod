import type { ReactNode } from "react";
import { useEffect, useRef, useState } from "react";
import { createRoot } from "react-dom/client";
import type { Panes, Side } from "./frame.tsx";
import { Frame, usePanes } from "./frame.tsx";
import type { Item, Menu } from "./menus.ts";
import { publish } from "./menus.ts";

/* TYPES */

export type Layout = { wide: boolean; bare: boolean; sheet: string; window: string };

export type Route = { title: string };

export type Draw<C> = (host: HTMLElement, context: C) => ReactNode;

export type Owned<C> = {
  layout: Layout;
  render: (row: Route, site: unknown, host: HTMLElement) => void;
  mount: (host: HTMLElement, context: C) => void;
  unmount: (host: HTMLElement) => void;
};

export type ReaderProps = {
  title: string;
  menus: Menu[];
  run?: (act: string) => unknown;
  panes?: Panes;
  children?: ReactNode;
};

export const layout: Layout = { wide: true, bare: true, sheet: "mrly", window: "app" };

const COPIED = 1500;

/* PAGE */

export function own<C = unknown>(draw: Draw<C>): Owned<C> {
  const live = new Map<HTMLElement, () => void>();
  return {
    layout,
    render(row, _site, host) {
      host.innerHTML = `<noscript><p>${row.title} draws in the browser and needs JavaScript.</p></noscript>`;
    },
    mount(host, context) {
      if (live.has(host)) return;
      host.replaceChildren();
      const root = createRoot(host);
      root.render(draw(host, context));
      live.set(host, () => root.unmount());
    },
    unmount(host) {
      live.get(host)?.();
      live.delete(host);
    },
  };
}

/* MENUS */

export const rule = (...parts: readonly (Item | null)[][]): (Item | null)[] =>
  parts.filter((part) => part.length).flatMap((part, n) => (n ? [null, ...part] : part));

export const library = (panes: { open: Pick<Panes["open"], "left"> }): Item => ({
  label: "Show Library",
  act: "pane:left",
  keys: ["mod", "["],
  on: panes.open.left,
});

export const COPY: Item = { label: "Copy Link", act: "copy" };

export function useCopy(link = () => location.href): [boolean, () => void] {
  const [copied, setCopied] = useState(false);
  const hush = useRef<ReturnType<typeof setTimeout> | 0>(0);
  useEffect(() => () => clearTimeout(hush.current), []);
  const copy = () => {
    navigator.clipboard?.writeText(link()).then(
      () => {
        clearTimeout(hush.current);
        setCopied(true);
        hush.current = setTimeout(() => setCopied(false), COPIED);
      },
      () => {},
    );
  };
  return [copied, copy];
}

/* READER */

export function Reader({ title, menus, run, panes: given, children }: ReaderProps) {
  const mine = usePanes();
  const panes = given ?? mine;
  const act = useRef(run);
  act.current = run;
  const sign = JSON.stringify(menus);
  useEffect(() => {
    publish({
      title,
      menus,
      run: (one) => (one.startsWith("pane:") ? panes.flip(one.slice(5) as Side) : act.current?.(one)),
    });
  }, [title, sign]);
  useEffect(() => () => publish(null), []);
  return (
    <Frame label={title} panes={panes}>
      {children}
    </Frame>
  );
}
