import { useEffect, useRef, useState } from "react";
import { createRoot } from "react-dom/client";
import { Frame, usePanes } from "./frame.jsx";
import { publish } from "./menus.js";

export const layout = { wide: true, bare: true, sheet: "mrly", window: "app" };

const COPIED = 1500;

/* PAGE */

export function own(draw) {
  const live = new Map();
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

export const rule = (...parts) =>
  parts.filter((part) => part.length).flatMap((part, n) => (n ? [null, ...part] : part));

export const library = (panes) => ({
  label: "Show Library",
  act: "pane:left",
  keys: ["mod", "["],
  on: panes.open.left,
});

export const COPY = { label: "Copy Link", act: "copy" };

export function useCopy(link = () => location.href) {
  const [copied, setCopied] = useState(false);
  const hush = useRef(0);
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

export function Reader({ title, menus, run, panes: given, children }) {
  const mine = usePanes();
  const panes = given ?? mine;
  const act = useRef(run);
  act.current = run;
  const sign = JSON.stringify(menus);
  useEffect(() => {
    publish({ title, menus, run: (one) => (one.startsWith("pane:") ? panes.flip(one.slice(5)) : act.current?.(one)) });
  }, [title, sign]);
  useEffect(() => () => publish(null), []);
  return (
    <Frame label={title} panes={panes}>
      {children}
    </Frame>
  );
}
