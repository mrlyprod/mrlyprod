import { start } from '../kit/git/client.ts';

/* MARKDOWN */

async function md(text, link) {
  const [{ front, render }, katex] = await Promise.all([import('../kit/ssg/md.ts'), text.includes('$') ? import('katex') : null]);
  const math = katex ? (tex, display) => katex.default.renderToString(tex, { output: 'mathml', throwOnError: false, displayMode: display }) : undefined;
  return render(front(text).body, { math, link });
}

/* CHROME */

const SVG = 'http://www.w3.org/2000/svg';

async function word(path) {
  const slot = document.querySelector('.dock .route');
  if (!slot) return;
  const name = path.split('/').filter(Boolean).pop() ?? 'git';
  const { letters } = await import('../kit/font/font.js');
  const { rows, cols, grid } = letters(name.toUpperCase());
  const svg = document.createElementNS(SVG, 'svg');
  svg.setAttribute('class', 'glyphs');
  svg.setAttribute('viewBox', `0 0 ${cols} ${rows}`);
  svg.setAttribute('role', 'img');
  svg.setAttribute('aria-label', name);
  grid.forEach((row, y) =>
    row.forEach((on, x) => {
      if (!on) return;
      const rect = document.createElementNS(SVG, 'rect');
      for (const [key, value] of [['x', x], ['y', y], ['width', 1], ['height', 1]]) rect.setAttribute(key, value);
      svg.append(rect);
    }),
  );
  slot.replaceChildren(svg);
}

function contents(mount) {
  const pane = document.querySelector('.pane.right');
  if (!pane) return;
  pane.querySelector('.contents')?.remove();
  const heads = [...mount.querySelectorAll('.readme h2[id], .readme h3[id]')];
  if (!heads.length) return;
  const nav = document.createElement('nav');
  nav.className = 'contents';
  nav.setAttribute('aria-label', 'Contents');
  const title = document.createElement('h2');
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
  nav.append(title, list);
  pane.prepend(nav);
}

/* START */

const MOUNT = '#main article';

start({
  tree: '/git.json',
  mount: MOUNT,
  md,
  paint: () => import('../kit/git/code.ts').then((one) => one.paint),
  after: (view) => {
    word(view.path);
    contents(document.querySelector(MOUNT));
  },
});
