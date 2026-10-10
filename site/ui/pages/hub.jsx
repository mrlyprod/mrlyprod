import { mount as search, unmount as unsearch } from '../search.js';
import { filled, Grid, hide, Lede, Menu, show } from '../parts.jsx';

const PLAIN = 'site-page';

function dress(nodes, rows) {
  const figure = new Map(rows.map((one) => [one.route, one.figure]));
  const wear = (node) => ({ name: node.name, href: node.href, figure: node.href ? (figure.get(node.href) ?? PLAIN) : undefined, nodes: node.nodes?.map(wear) });
  return nodes.map(wear);
}

export function render(row, site, host, { rows }) {
  if (row.meta.was === 'menu') return show(host, <><Lede id="menu" title={row.title} lead={row.lead} /><Menu tree={dress(filled(site, rows), rows)} /></>);
  const posts = rows.filter((one) => one.meta?.was === 'post').map((one) => ({ name: one.title, href: one.route, figure: one.figure, text: one.lead, dates: [one.date] }));
  show(host, <><Lede id="blog" title={row.title} lead={row.lead} /><Grid nodes={posts} /></>);
}

export function mount(host, { rows }) {
  const menu = host.querySelector('.menu');
  if (menu) search(menu, rows);
}

export function unmount(host) {
  const menu = host.querySelector('.menu');
  if (menu) unsearch(menu);
  hide(host);
}
