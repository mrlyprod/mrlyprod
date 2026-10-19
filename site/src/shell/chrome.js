import { KEY, PREFIX, face, read, root, screen, theme, tint, turn } from './prefs.js';
import { desk, folds, greeting, knock, listen, rename, route, visit } from './desk.js';
import { expand, turns } from './tree.js';

const DOCK = '(min-width: 74rem)';
const SIDES = ['left', 'right'];

/* SUB */

const docked = () => matchMedia(DOCK).matches;
const isOpen = (side) => root().dataset[side] === 'open';
const sub = () => document.querySelector('.subheader');

const spot = () => `${PREFIX}spot:${location.href}`;

function keep() {
  if (root().getAttribute('aria-busy') === 'true') return;
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
  const top = sub()?.getBoundingClientRect().top ?? 0;
  if (top) scrollTo({ top: top + scrollY, behavior: 'instant' });
}

function land() {
  const at = saved() || Number(history.state?.y) || 0;
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
  document.getElementById(side)?.querySelector('a, button, input, select, textarea')?.focus();
}

function shut() {
  if (docked()) return false;
  const open = SIDES.filter(isOpen);
  for (const side of open) set(side, false);
  if (open.length) document.querySelector(`[data-pane="${open[0]}"]`)?.focus();
  return open.length > 0;
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
    link.setAttribute('aria-label', n ? `Cart, ${n} item${n === 1 ? '' : 's'}` : 'Cart');
    link.querySelectorAll('.dot').forEach((dot, i) => dot.classList.toggle('on', i < n));
  }
}

/* CONTENTS */

const eyes = new Map();

function unread() {
  for (const [nav, eye] of eyes) {
    if (nav.isConnected) continue;
    eye.disconnect();
    eyes.delete(nav);
  }
}

function contents(nav) {
  nav.querySelector('details')?.setAttribute('open', '');
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
    { rootMargin: `-${Math.round(sub()?.getBoundingClientRect().height ?? 0)}px 0px -60% 0px` },
  );
  for (const t of targets) eye.observe(t);
  eyes.set(nav, eye);
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

let landed = false;

function wire(event) {
  if (event?.detail?.route) visit(event.detail);
  if (!landed && route) {
    landed = true;
    land();
  }
  sync();
  unread();
  theme(root().dataset.theme ?? '');
  face(root().dataset.font ?? '');
  tint(root().dataset.tint ?? '');
  screen(read(KEY.saver));
  cart();
  once('.contents', contents);
  once('.tree summary', turns);
  once('.top .mark', (mark) => {
    folds(mark);
    knock(mark);
  });
  if (greeting() === 'open') listen();
  if (route === '/') {
    root().dataset.desk = '';
    desk();
  } else delete root().dataset.desk;
  rename();
}

function boot() {
  root().classList.add('js');
  if ('scrollRestoration' in history) history.scrollRestoration = 'manual';
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
    if (button) return button.closest('.pane') ? shut() : toggle(button.dataset.pane);
    if (target.closest('[data-theme-toggle]')) return turn();
    if (target.closest('.scrim') || target.closest('.pane a[href]')) shut();
  });
  document.addEventListener('change', (e) => {
    const pick = e.target instanceof Element ? e.target : null;
    if (!pick) return;
    if (pick.matches('[data-font-pick]')) return face(pick.value);
    if (pick.matches('[data-tint-pick]')) return tint(pick.value);
    if (pick.matches('[data-saver-pick]')) return screen(pick.value);
  });
  document.addEventListener('toggle', expand, true);
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && !document.querySelector('dialog[open]') && shut()) e.preventDefault();
  });
  window.addEventListener('wire', wire);
  window.addEventListener('saver', (e) => screen(e.detail ?? ''));
  window.addEventListener('cart', cart);
  window.addEventListener('storage', cart);
  window.addEventListener('pageshow', cart);
  window.addEventListener('pagehide', keep);
  cart();
}

if (typeof document !== 'undefined') boot();
