import { apps, own } from "site:apps";
import type { Island } from "../kit/island.tsx";
import { esc } from "../kit/md/text.ts";
import type { Row } from "../kit/rows.ts";
import type { Layout, Mounting, Own } from "./router.ts";

const APP: Layout = { wide: true, bare: true, sheet: "mrly" };

const live = new WeakMap<HTMLElement, () => void>();

const chunk = (id: string) => (apps[id] ? apps[id]() : Promise.reject(new Error(`app: no chunk for ${id}`)));

const still: Own = {
  render(row, _site, host) {
    const title = esc(row.title);
    const figure = row.figure
      ? `<figure class="opener" data-figure="${esc(row.figure)}"><canvas role="img" aria-label="${title}"></canvas></figure>`
      : "";
    host.innerHTML = `<div id="root" data-app="${esc(row.meta.id)}"></div><div><h1>${title}</h1>${figure}<noscript><p>This app draws in the browser and needs JavaScript.</p></noscript></div>`;
  },
  async mount(host, context) {
    const at = host.querySelector<HTMLElement>("#root")!;
    const entry = (await chunk(at.dataset.app!)) as Island<Mounting>;
    if (!at.isConnected || live.has(host)) return;
    entry.mount(at, context);
    live.set(host, () => entry.unmount(at));
  },
  unmount(host) {
    live.get(host)?.();
    live.delete(host);
  },
};

const layoutOf = (row: Row): Layout => (row.meta.frame === "page" ? { bare: true } : APP);

export function page(row: Row): Own | Promise<Own> {
  const id = row.meta.id ?? "";
  return own.has(id) ? (chunk(id) as Promise<Own>) : { ...still, layout: layoutOf(row) };
}
