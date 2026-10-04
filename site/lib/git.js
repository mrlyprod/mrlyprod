import { start } from '../kit/git/client.ts';
import { crumbs } from '../ui/crumbs.js';

/* MARKDOWN */

async function md(text, link) {
  const [{ front, render }, katex] = await Promise.all([import('../kit/ssg/md.ts'), text.includes('$') ? import('katex') : null]);
  const math = katex ? (tex, display) => katex.default.renderToString(tex, { output: 'mathml', throwOnError: false, displayMode: display }) : undefined;
  return render(front(text).body, { math, link });
}

/* CHROME */

function trail() {
  const list = document.querySelector('.subheader .crumbs ol');
  if (!list) return;
  const steps = crumbs(location.pathname);
  list.replaceChildren(
    ...steps.map((step, n) => {
      const item = document.createElement('li');
      const a = document.createElement('a');
      a.href = step.href;
      a.textContent = step.name;
      if (n === steps.length - 1) a.setAttribute('aria-current', 'page');
      item.append(a);
      return item;
    }),
  );
}

const opener = () => document.querySelector('[data-pane="right"]');

function bar(nav) {
  const old = document.getElementById('right');
  const button = opener();
  if (button) button.hidden = !nav;
  if (!nav) {
    old?.remove();
    document.documentElement.dataset.right = 'shut';
    return;
  }
  const pane = old ?? document.createElement('aside');
  pane.className = 'pane right';
  pane.id = 'right';
  pane.setAttribute('aria-label', 'Page tools');
  pane.replaceChildren(nav);
  if (!old) document.getElementById('main')?.after(pane);
}

function contents(mount) {
  const heads = [...mount.querySelectorAll('.readme h2[id], .readme h3[id]')];
  const few = Number(opener()?.dataset.few) || Infinity;
  if (heads.length < few) return bar(null);
  const nav = document.createElement('nav');
  nav.className = 'contents';
  nav.setAttribute('aria-label', 'Contents');
  const fold = document.createElement('details');
  fold.open = true;
  const title = document.createElement('summary');
  title.textContent = 'Contents';
  const list = document.createElement('ol');
  for (const head of heads) {
    const item = document.createElement('li');
    item.className = head.tagName.toLowerCase();
    const a = document.createElement('a');
    a.href = `#${head.id}`;
    a.textContent = head.textContent;
    item.append(a);
    list.append(item);
  }
  fold.append(title, list);
  nav.append(fold);
  bar(nav);
}

/* ISLAND */

const live = new Map();

export function mount(host) {
  if (live.has(host)) return;
  live.set(
    host,
    start({
      tree: '/git.json',
      mount: host,
      md,
      paint: () => import('../kit/git/code.ts').then((one) => one.paint),
      after: () => {
        trail();
        contents(host);
      },
    }),
  );
}

export function unmount(host) {
  live.get(host)?.();
  live.delete(host);
}
