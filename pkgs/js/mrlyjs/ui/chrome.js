const DOCK = '(min-width: 74rem)';
const PREFIX = (typeof document !== 'undefined' && document.documentElement.dataset.prefix) || 'mrly-';
const KEY = { theme: `${PREFIX}theme`, font: `${PREFIX}font`, cart: `${PREFIX}cart` };
const SIDES = ['left', 'right'];

const read = (key) => {
  try {
    return localStorage.getItem(key) ?? '';
  } catch {
    return '';
  }
};

const write = (key, value) => {
  try {
    if (value) localStorage.setItem(key, value);
    else localStorage.removeItem(key);
  } catch {}
};

/* DRAWERS */

const root = () => document.documentElement;
const docked = () => matchMedia(DOCK).matches;
const isOpen = (side) => root().dataset[side] === 'open';

function set(side, open) {
  root().dataset[side] = open ? 'open' : 'shut';
  sync();
}

function place() {
  for (const side of SIDES) set(side, false);
}

function sync() {
  for (const button of document.querySelectorAll('[data-pane]')) button.setAttribute('aria-expanded', String(isOpen(button.dataset.pane)));
}

function toggle(side) {
  const open = !isOpen(side);
  set(side, open);
  if (!open) return;
  set(side === 'left' ? 'right' : 'left', false);
  document.getElementById(side)?.querySelector('a, button')?.focus();
}

function shut() {
  if (docked()) return false;
  const open = SIDES.filter(isOpen);
  for (const side of open) set(side, false);
  if (open.length) document.querySelector(`[data-pane="${open[0]}"]`)?.focus();
  return open.length > 0;
}

/* THEME */

function theme(next) {
  if (next) root().dataset.theme = next;
  else delete root().dataset.theme;
  write(KEY.theme, next);
  for (const label of document.querySelectorAll('[data-theme-toggle] b')) label.textContent = next || 'auto';
  window.dispatchEvent(new Event('theme'));
}

function turn() {
  const now = root().dataset.theme ?? '';
  theme(now === '' ? 'light' : now === 'light' ? 'dark' : '');
}

/* FONT */

function face(next) {
  if (next) root().dataset.font = next;
  else delete root().dataset.font;
  write(KEY.font, next);
  for (const pick of document.querySelectorAll('[data-font-pick]')) pick.value = next || '';
}

/* CART */

function count() {
  try {
    const items = JSON.parse(read(KEY.cart) || '[]');
    return Array.isArray(items) ? items.reduce((sum, item) => sum + (Number(item.qty) || 1), 0) : 0;
  } catch {
    return 0;
  }
}

function cart() {
  const n = count();
  for (const link of document.querySelectorAll('[data-cart]')) {
    link.dataset.count = String(n);
    link.setAttribute('aria-label', n ? `Cart, ${n} item${n === 1 ? '' : 's'}` : 'Cart');
    link.querySelectorAll('.dot').forEach((dot, i) => dot.classList.toggle('on', i < n));
  }
  for (const badge of document.querySelectorAll('[data-cart-count]')) badge.textContent = String(n);
}

/* CONTENTS */

function contents(nav) {
  const links = [...nav.querySelectorAll('a[href^="#"]')];
  const targets = links.map((a) => document.getElementById(decodeURIComponent(a.hash.slice(1)))).filter(Boolean);
  if (!targets.length) return;
  const seen = new Map();
  const eye = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) seen.set(entry.target, entry.isIntersecting);
      const hit = targets.find((t) => seen.get(t));
      if (!hit) return;
      for (const a of links) {
        if (a.hash.slice(1) === hit.id) a.setAttribute('aria-current', 'location');
        else a.removeAttribute('aria-current');
      }
    },
    { rootMargin: '-48px 0px -60% 0px' },
  );
  for (const t of targets) eye.observe(t);
}

/* MARK */

async function footer(canvas) {
  const { cycle, mark } = await import('./font.js');
  mark(canvas, cycle(canvas.dataset.text || 'MRLYPROD', 1, 40));
}

/* EXPLORER */

let forest = null;

const fetchTree = (url) => (forest ??= fetch(url).then((reply) => (reply.ok ? reply.json() : null)).catch(() => null));

const named = (path) => (path.slice(path.lastIndexOf('/') + 1).includes('.') ? path : `${path}.txt`);

function find(node, path) {
  let at = node;
  for (const part of path ? path.split('/') : []) {
    at = (at.c ?? []).find((kid) => kid.n === part);
    if (!at) return null;
  }
  return at;
}

function branch(base, kid, path) {
  const li = document.createElement('li');
  const a = document.createElement('a');
  a.textContent = kid.n;
  if (kid.k !== 'd') {
    a.href = `${base}${named(path)}`;
    li.append(a);
    return li;
  }
  a.href = `${base}${path}/`;
  const details = document.createElement('details');
  details.dataset.lazy = path;
  const summary = document.createElement('summary');
  summary.append(a);
  details.append(summary, document.createElement('ul'));
  li.append(details);
  return li;
}

function fill(details, data) {
  const list = details.querySelector(':scope > ul');
  if (!list || list.children.length) return;
  const path = details.dataset.lazy;
  const node = find(data, path);
  if (!node) return;
  for (const kid of node.c ?? []) list.append(branch(data.base ?? '/git/', kid, path ? `${path}/${kid.n}` : kid.n));
}

function expand(event) {
  const details = event.target;
  if (!(details instanceof HTMLDetailsElement) || !details.open || details.dataset.lazy === undefined) return;
  const url = details.closest('.tree')?.dataset.source;
  if (!url) return;
  fetchTree(url).then((data) => data && fill(details, data));
}

/* WIRE */

const wired = new WeakSet();

const once = (selector, fn) => {
  for (const el of document.querySelectorAll(selector)) {
    if (wired.has(el)) continue;
    wired.add(el);
    fn(el);
  }
};

export function wire() {
  sync();
  theme(root().dataset.theme ?? '');
  face(root().dataset.font ?? '');
  cart();
  once('.contents', contents);
  once('canvas.mark', footer);
}

function boot() {
  root().classList.add('js');
  theme(read(KEY.theme));
  face(read(KEY.font));
  place();
  matchMedia(DOCK).addEventListener('change', place);
  document.addEventListener('click', (e) => {
    const target = e.target instanceof Element ? e.target : null;
    if (!target) return;
    const button = target.closest('[data-pane]');
    if (button) {
      if (docked()) return;
      e.preventDefault();
      return toggle(button.dataset.pane);
    }
    if (target.closest('[data-theme-toggle]')) return turn();
    if (target.closest('.scrim') || target.closest('.pane a[href]')) shut();
  });
  document.addEventListener('change', (e) => {
    const pick = e.target instanceof Element ? e.target.closest('[data-font-pick]') : null;
    if (pick) face(pick.value);
  });
  document.addEventListener('toggle', expand, true);
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && shut()) e.preventDefault();
  });
  window.addEventListener('cart', cart);
  window.addEventListener('storage', cart);
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', wire);
  else wire();
}

if (typeof document !== 'undefined') boot();
