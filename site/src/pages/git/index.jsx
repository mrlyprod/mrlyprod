import { css } from "site:css";
import { useCallback, useEffect, useRef, useState } from "react";
import { Btn, Icon } from "../../lib/app/controls.jsx";
import { Bar, usePanes } from "../../lib/app/frame.jsx";
import { Library } from "../../lib/app/library.jsx";
import { COPY, library, own, Reader, rule, useCopy } from "../../lib/app/reader.jsx";
import { away } from "../../lib/git/view.ts";
import { turns } from "../../shell/tree.js";
import { view as start } from "./git.js";

const TREE = "/git.json";

/* SHEET */

let linked = null;

function link() {
  if (linked?.isConnected || !css.git) return;
  linked = Object.assign(document.createElement("link"), { rel: "stylesheet", href: css.git });
  document.head.append(linked);
}

/* TREE */

function tree(site) {
  const list = document.createElement("ul");
  list.className = "tree";
  list.dataset.source = TREE;
  const item = document.createElement("li");
  const fold = document.createElement("details");
  fold.dataset.lazy = "";
  const head = document.createElement("summary");
  const a = document.createElement("a");
  a.href = "/git/";
  a.textContent = site.git.slug.split("/").pop();
  head.append(a);
  turns(head);
  fold.append(head, document.createElement("ul"));
  item.append(fold);
  list.append(item);
  return list;
}

/* WIDGET */

const open = (href) => href && window.open(href, "_blank", "noopener");

function Widget({ site }) {
  const panes = usePanes();
  const box = useRef(null);
  const [view, setView] = useState(null);
  const [copied, copy] = useCopy();
  useEffect(() => start(box.current, setView), []);
  const grow = useCallback(
    (at) => {
      if (!at || at.firstChild) return;
      at.append(tree(site));
      at.querySelector("details").open = true;
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
  const run = (act) => ({ copy, raw: () => open(raw), github: () => open(hub) })[act]?.();
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

export const { layout, mount, unmount } = own((_host, { site }) => <Widget site={site} />);

export function render(_row, site, host) {
  link();
  host.innerHTML = `<noscript><p>The code viewer draws in the browser and needs JavaScript. The same files are on <a href="https://github.com/${site.git.slug}">GitHub</a>.</p></noscript>`;
}
