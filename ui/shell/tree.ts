import type { Twig, Wood } from "../kit/git/view.ts";
import { kind } from "../kit/git/view.ts";
import { glyph } from "../kit/icon.tsx";

const SIZE = 16;

export const turns = (summary: Element) =>
  summary.querySelector(":scope > svg") ?? summary.prepend(glyph("down", SIZE));

let forest: Promise<Wood | null> | null = null;

const fetchTree = (url: string) =>
  (forest ??= fetch(url)
    .then((reply) => (reply.ok ? reply.json() : null))
    .catch(() => null));

function find(node: Wood, path: string | undefined): Pick<Twig, "c"> | null {
  let at: Pick<Twig, "c"> | undefined = node;
  for (const part of path ? path.split("/") : []) {
    at = (at.c ?? []).find((kid) => kid.n === part);
    if (!at) return null;
  }
  return at;
}

export function branch(base: string, kid: Twig, path: string) {
  const li = document.createElement("li");
  const a = document.createElement("a");
  if (kid.k !== "d") {
    const name = document.createElement("span");
    name.className = "name";
    name.textContent = kid.n;
    a.append(glyph(kind(kid.n), SIZE), name);
    a.href = `${base}${path}`;
    li.append(a);
    return li;
  }
  a.textContent = kid.n;
  a.href = `${base}${path}/`;
  const details = document.createElement("details");
  details.dataset.lazy = path;
  const summary = document.createElement("summary");
  summary.append(glyph("down", SIZE), a);
  details.append(summary, document.createElement("ul"));
  li.append(details);
  return li;
}

function fill(details: HTMLDetailsElement, data: Wood) {
  const list = details.querySelector(":scope > ul");
  if (!list || list.children.length) return;
  const path = details.dataset.lazy;
  const node = find(data, path);
  if (!node) return;
  for (const kid of node.c ?? []) list.append(branch(data.base ?? "/git/", kid, path ? `${path}/${kid.n}` : kid.n));
}

export function expand(event: Event) {
  const details = event.target;
  if (!(details instanceof HTMLDetailsElement) || !details.open || details.dataset.lazy === undefined) return;
  const url = details.closest<HTMLElement>(".tree")?.dataset.source;
  if (!url) return;
  fetchTree(url).then((data) => data && fill(details, data));
}
