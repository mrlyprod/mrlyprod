import { mount as start, unmount as stop } from '../../lib/git.js';

const TREE = '/git.json';

function tree(site) {
  const list = document.createElement('ul');
  list.className = 'tree';
  list.dataset.source = TREE;
  const item = document.createElement('li');
  const fold = document.createElement('details');
  fold.dataset.lazy = '';
  const head = document.createElement('summary');
  const a = document.createElement('a');
  a.href = '/git/';
  a.textContent = site.git.slug.split('/').pop();
  head.append(a);
  fold.append(head, document.createElement('ul'));
  item.append(fold);
  list.append(item);
  return list;
}

export function render(row, site, host, { left }) {
  host.innerHTML = `<noscript><p>The code viewer draws in the browser and needs JavaScript. The same files are on <a href="https://github.com/${site.git.slug}">GitHub</a>.</p></noscript>`;
  left.append(tree(site));
}

export function mount(host) {
  const fold = document.querySelector('#left .tree details[data-lazy=""]');
  if (fold) fold.open = true;
  start(host);
}

export const unmount = stop;
