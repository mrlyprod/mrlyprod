import site from './site.js';

const ROUTE = { demos: '/demos/' };
const ID = `${site.prefix}tree`;
const FILE = '/demos/tree.json';

let shelves = null;

function block() {
  const el = typeof document === 'undefined' ? null : document.getElementById(ID);
  return el ? JSON.parse(el.textContent) : null;
}

export function ready() {
  if (!shelves) shelves = block();
  return !!shelves;
}

export function demos() {
  return ready() ? shelves : [];
}

export async function load() {
  if (!ready()) shelves = await fetch(FILE).then((r) => r.json());
  return shelves;
}

export function tree(lists = {}) {
  const filled = { demos: demos(), ...lists };
  return site.tree.map(({ fill, ...node }) => {
    const key = fill ?? node.name.toLowerCase();
    const href = node.href ?? ROUTE[key];
    const nodes = filled[key];
    return nodes?.length ? { ...node, href, nodes } : { ...node, href };
  });
}
