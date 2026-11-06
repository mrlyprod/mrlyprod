import { css } from "site:css";
import type { Site } from "site:routes";
import { useCallback, useEffect, useRef, useState } from "react";
import { Btn, Icon } from "../kit/controls.tsx";
import { Bar, usePanes } from "../kit/frame.tsx";
import { view as start } from "../kit/git/page.ts";
import type { View } from "../kit/git/view.ts";
import { away } from "../kit/git/view.ts";
import { Library } from "../kit/library.tsx";
import { COPY, library, Reader, rule, useCopy } from "../kit/reader.tsx";
import { turns } from "./tree.ts";

type Props = { site: Site };

const TREE = "/git.json";

let linked: HTMLLinkElement | null = null;

function link() {
  if (linked?.isConnected || !css.git) return;
  linked = Object.assign(document.createElement("link"), { rel: "stylesheet", href: css.git });
  document.head.append(linked);
}

function grown(site: Site) {
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

const open = (href: string) => href && window.open(href, "_blank", "noopener");

export function Code({ site }: Props) {
  const panes = usePanes();
  const box = useRef<HTMLDivElement>(null);
  const [view, setView] = useState<View | null>(null);
  const [copied, copy] = useCopy();
  useEffect(() => {
    link();
    return start(box.current!, TREE, setView);
  }, []);
  const grow = useCallback(
    (at: HTMLDivElement | null) => {
      if (!at || at.firstChild) return;
      at.append(grown(site));
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
          <Icon name="share" />
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
