import { css } from "site:css";
import type { Site } from "site:routes";
import { useCallback, useEffect, useRef, useState } from "react";
import type { Row } from "../../../scripts/build.ts";
import { Btn, Icon } from "../../lib/app/controls.tsx";
import { Bar, usePanes } from "../../lib/app/frame.tsx";
import { Library } from "../../lib/app/library.tsx";
import { COPY, library, own, Reader, rule, useCopy } from "../../lib/app/reader.tsx";
import type { View } from "../../lib/git/view.ts";
import { away } from "../../lib/git/view.ts";
import type { Mounting } from "../../shell/router.ts";
import { turns } from "../../shell/tree.ts";
import { view as start } from "./git.ts";

/* TYPES */

type Props = { site: Site };

const TREE = "/git.json";

/* SHEET */

let linked: HTMLLinkElement | null = null;

function link() {
  if (linked?.isConnected || !css.git) return;
  linked = Object.assign(document.createElement("link"), { rel: "stylesheet", href: css.git });
  document.head.append(linked);
}

/* TREE */

function tree(site: Site) {
  const list = document.createElement("ul");
  list.className = "tree";
  list.dataset.source = TREE;
  const item = document.createElement("li");
  const fold = document.createElement("details");
  fold.dataset.lazy = "";
  const head = document.createElement("summary");
  const a = document.createElement("a");
  a.href = "/git/";
  a.textContent = site.git.slug.split("/").pop()!;
  head.append(a);
  turns(head);
  fold.append(head, document.createElement("ul"));
  item.append(fold);
  list.append(item);
  return list;
}

/* WIDGET */

const open = (href: string) => href && window.open(href, "_blank", "noopener");

function Widget({ site }: Props) {
  const panes = usePanes();
  const box = useRef<HTMLDivElement>(null);
  const [view, setView] = useState<View | null>(null);
  const [copied, copy] = useCopy();
  useEffect(() => start(box.current!, setView), []);
  const grow = useCallback(
    (at: HTMLDivElement | null) => {
      if (!at || at.firstChild) return;
      at.append(tree(site));
      at.querySelector("details")!.open = true;
    },
    [site],
  );
  const { raw, hub } = view ? away(site.git, view) : { raw: "", hub: "" };
  const menus = [
    {
      name: "File",
      items: rule(
        [COPY],
        [
          { label: "Open Raw", act: "raw", off: !raw },
          { label: "On GitHub", act: "github", off: !hub },
        ],
      ),
    },
    { name: "View", items: [library(panes)] },
  ];
  const run = (act: string) => {
    const acts: Partial<Record<string, () => unknown>> = { copy, raw: () => open(raw), github: () => open(hub) };
    return acts[act]?.();
  };
  const where = !view ? "" : !view.found ? "not found" : view.path || view.name;
  return (
    <Reader title="Code" menus={menus} run={run} panes={panes}>
      <div className="page source" ref={box} />
      <Library name="Files">
        <div ref={grow} />
      </Library>
      <Bar side="actions">
        <Btn className="share" onClick={copy}>
          <Icon name="ios_share" />
          Share
        </Btn>
        {raw && <Btn onClick={() => open(raw)}>Raw</Btn>}
        {hub && <Btn onClick={() => open(hub)}>GitHub</Btn>}
      </Bar>
      <Bar side="status">
        <span>{copied ? "link copied" : where}</span>
      </Bar>
    </Reader>
  );
}

/* PAGE */

export const { layout, mount, unmount } = own<Mounting>((_host, { site }) => <Widget site={site} />);

export function render(_row: Row, site: Site, host: HTMLElement) {
  link();
  host.innerHTML = `<noscript><p>The code viewer draws in the browser and needs JavaScript. The same files are on <a href="https://github.com/${site.git.slug}">GitHub</a>.</p></noscript>`;
}
