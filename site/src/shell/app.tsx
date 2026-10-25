import { apps, own } from "site:apps";
import { useState } from "react";
import type { Row } from "../../scripts/build.ts";
import type { Island } from "../lib/app/island.tsx";
import { esc } from "../lib/md/text.ts";
import { AppIcon } from "./appicon.tsx";
import { Figure, hide, show } from "./parts.tsx";
import type { Layout, Mounting, Own } from "./router.ts";

/* TYPES */

export type GateProps = { row: Row };

const APP: Layout = { wide: true, bare: true, sheet: "mrly", window: "app" };

const PAGE: Layout = { ...APP, window: "wide" };

const DEV = new Set<string | undefined>(["figures", "kit", "screensavers"]);

const FAR = "Link copied. Open it on a computer.";

const COPIED = "Link copied.";

const live = new WeakMap<HTMLElement, () => void>();

const chunk = (id: string) => (apps[id] ? apps[id]() : Promise.reject(new Error(`app: no chunk for ${id}`)));

const phone = () => globalThis.document?.documentElement.dataset.form === "phone";

export const gated = (row: Row) => Boolean(row.meta.scene) || DEV.has(row.meta.id);

/* STILL */

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

/* GATE */

export function Gate({ row }: GateProps) {
  const [said, say] = useState("");
  const copy = (done: string) => () => {
    const url = location.href;
    const fail = () => say(`Copy this link: ${url}`);
    if (!navigator.clipboard) return fail();
    navigator.clipboard.writeText(url).then(() => say(done), fail);
  };
  return (
    <div className="gate">
      <AppIcon id={row.meta.id ?? ""} />
      <h1>{row.title}</h1>
      {row.lead && <p className="lead">{row.lead}</p>}
      {row.figure && <Figure className="opener" name={row.figure} label={row.title} />}
      <p className="need">Needs a bigger screen</p>
      <p className="buttons">
        <button type="button" className="primary" onClick={copy(FAR)}>
          Open on a computer
        </button>
        <button type="button" onClick={copy(COPIED)}>
          Copy link
        </button>
      </p>
      <p className="said" role="status">
        {said}
      </p>
    </div>
  );
}

const gate: Own = {
  layout: { bare: true, window: "page" },
  render(row, _site, host) {
    show(host, <Gate row={row} />);
  },
  unmount: hide,
};

export function page(row: Row): Own | Promise<Own> {
  if (gated(row) && phone()) return gate;
  const id = row.meta.id ?? "";
  return own.has(id) ? (chunk(id) as Promise<Own>) : { ...still, layout: row.meta.scene ? APP : PAGE };
}
