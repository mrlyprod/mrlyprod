const DOCK = '(min-width: 74rem)';
const PREFIX = (typeof document !== 'undefined' && document.documentElement.dataset.prefix) || 'mrly-';
const KEY = { theme: `${PREFIX}theme`, font: `${PREFIX}font`, tint: `${PREFIX}tint`, saver: `${PREFIX}saver`, cart: `${PREFIX}cart` };
const WORDMARK = 'wordmark';
const SCREENS = ['matrix', 'sleep', 'mandelbrot', 'julia'];
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

/* SUB */

const root = () => document.documentElement;
const docked = () => matchMedia(DOCK).matches;
const isOpen = (side) => root().dataset[side] === 'open';
const dock = () => document.querySelector('.dock');
const ready = (fn) => (document.readyState === 'complete' ? fn() : addEventListener('load', fn, { once: true }));

const spot = () => `${PREFIX}spot:${location.href}`;

function keep() {
  try {
    sessionStorage.setItem(spot(), String(scrollY));
  } catch {}
}

function saved() {
  try {
    const at = Number(sessionStorage.getItem(spot())) || 0;
    sessionStorage.removeItem(spot());
    return at;
  } catch {
    return 0;
  }
}

function resumed() {
  const kind = performance.getEntriesByType('navigation')[0]?.type;
  return kind === 'back_forward' || kind === 'reload';
}

function pin() {
  const top = dock()?.getBoundingClientRect().top ?? 0;
  if (top) scrollTo({ top: top + scrollY, behavior: 'instant' });
}

function land() {
  const at = saved();
  if (at > 0 && resumed() && !location.hash) scrollTo({ top: at, behavior: 'instant' });
}

/* DRAWERS */

function set(side, open) {
  root().dataset[side] = open ? 'open' : 'shut';
  sync();
}

function stow() {
  for (const side of SIDES) set(side, false);
}

function sync() {
  for (const button of document.querySelectorAll('[data-pane]')) button.setAttribute('aria-expanded', String(isOpen(button.dataset.pane)));
}

function toggle(side) {
  const open = !isOpen(side);
  if (open) pin();
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

/* TINT */

function tint(next) {
  if (next) root().dataset.tint = next;
  else delete root().dataset.tint;
  write(KEY.tint, next);
  for (const pick of document.querySelectorAll('[data-tint-pick]')) pick.value = next || '';
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
    { rootMargin: `-${Math.round(dock()?.getBoundingClientRect().height ?? 0)}px 0px -60% 0px` },
  );
  for (const t of targets) eye.observe(t);
}

/* MARK */

const marks = new Map();

const wanted = () => root().dataset.saver || WORDMARK;

function unmark() {
  for (const canvas of marks.keys()) {
    if (canvas.isConnected) continue;
    canvas.stop?.();
    marks.delete(canvas);
  }
}

async function footer(canvas) {
  canvas.stop?.();
  marks.delete(canvas);
  const next = canvas.cloneNode(false);
  const label = next.dataset.label ?? next.getAttribute('aria-label') ?? '';
  if (label) next.dataset.label = label;
  canvas.replaceWith(next);
  const name = wanted();
  marks.set(next, name);
  if (name === WORDMARK) {
    next.removeAttribute('aria-hidden');
    next.setAttribute('role', 'img');
    if (label) next.setAttribute('aria-label', label);
    const { cycle, mark } = await import('./font.js');
    next.stop = mark(next, cycle(next.dataset.text || 'MRLYPROD', 1));
  } else {
    next.setAttribute('aria-hidden', 'true');
    next.removeAttribute('role');
    next.removeAttribute('aria-label');
    const { saver } = await import('./savers/index.js');
    next.stop = saver(next, name);
  }
  if (!next.isConnected) next.stop();
}

function marked() {
  const name = wanted();
  for (const canvas of document.querySelectorAll('canvas.mark')) if (marks.get(canvas) !== name) footer(canvas);
}

function screen(next) {
  if (SCREENS.includes(next)) root().dataset.saver = next;
  else delete root().dataset.saver;
  const now = root().dataset.saver ?? '';
  write(KEY.saver, now);
  for (const pick of document.querySelectorAll('[data-saver-pick]')) pick.value = now;
}

function replay(name) {
  screen(name);
  for (const canvas of [...marks.keys()]) footer(canvas);
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
  if (kid.k !== 'd') {
    const icon = document.createElement('span');
    icon.className = kid.i ? `si si-${kid.i}` : 'si';
    icon.setAttribute('aria-hidden', 'true');
    const name = document.createElement('span');
    name.className = 'name';
    name.textContent = kid.n;
    a.append(icon, name);
    a.href = `${base}${named(path)}`;
    li.append(a);
    return li;
  }
  a.textContent = kid.n;
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
  unmark();
  theme(root().dataset.theme ?? '');
  face(root().dataset.font ?? '');
  tint(root().dataset.tint ?? '');
  screen(root().dataset.saver ?? '');
  cart();
  once('.contents', contents);
  marked();
}

function boot() {
  root().classList.add('js');
  if ('scrollRestoration' in history) history.scrollRestoration = 'manual';
  ready(land);
  theme(read(KEY.theme));
  face(read(KEY.font));
  tint(read(KEY.tint));
  screen(read(KEY.saver));
  stow();
  matchMedia(DOCK).addEventListener('change', stow);
  document.addEventListener('click', (e) => {
    const target = e.target instanceof Element ? e.target : null;
    if (!target) return;
    const button = target.closest('[data-pane]');
    if (button) return toggle(button.dataset.pane);
    if (target.closest('[data-theme-toggle]')) return turn();
    if (target.closest('.scrim') || target.closest('.pane a[href]')) shut();
  });
  document.addEventListener('change', (e) => {
    const pick = e.target instanceof Element ? e.target : null;
    if (!pick) return;
    if (pick.matches('[data-font-pick]')) return face(pick.value);
    if (pick.matches('[data-tint-pick]')) return tint(pick.value);
    if (pick.matches('[data-saver-pick]')) return replay(pick.value);
  });
  document.addEventListener('toggle', expand, true);
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && shut()) e.preventDefault();
  });
  window.addEventListener('cart', cart);
  window.addEventListener('storage', cart);
  window.addEventListener('pageshow', cart);
  window.addEventListener('pagehide', keep);
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', wire);
  else wire();
}

if (typeof document !== 'undefined') boot();
