import { cycle, mark } from './font.js';

const DOCK = '(min-width: 74rem)';
const KEY = { left: 'mrly-left', right: 'mrly-right', theme: 'mrly-theme' };
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

/* PANES */

const root = () => document.documentElement;
const docked = () => matchMedia(DOCK).matches;
const isOpen = (side) => root().dataset[side] === 'open';

function set(side, open, remember = true) {
  root().dataset[side] = open ? 'open' : 'shut';
  if (remember && docked()) write(KEY[side], open ? 'open' : 'shut');
  sync();
}

function place() {
  for (const side of SIDES) set(side, docked() && read(KEY[side]) !== 'shut', false);
}

function sync() {
  for (const button of document.querySelectorAll('[data-pane]')) button.setAttribute('aria-expanded', String(isOpen(button.dataset.pane)));
}

function toggle(side) {
  const open = !isOpen(side);
  set(side, open);
  if (open && !docked()) {
    set(side === 'left' ? 'right' : 'left', false);
    document.getElementById(side)?.querySelector('a, button')?.focus();
  }
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
}

function turn() {
  const now = root().dataset.theme ?? '';
  theme(now === '' ? 'light' : now === 'light' ? 'dark' : '');
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
  if (globalThis.mrly?.font_cycle) return mark(canvas, cycle('MRLYPROD', 1, 40));
  const reply = await fetch('/ui/mark.json');
  if (reply.ok) mark(canvas, await reply.json());
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
  once('.contents', contents);
  once('canvas.mark', footer);
}

function boot() {
  root().classList.add('js');
  theme(read(KEY.theme));
  place();
  matchMedia(DOCK).addEventListener('change', place);
  document.addEventListener('click', (e) => {
    const target = e.target instanceof Element ? e.target : null;
    if (!target) return;
    const button = target.closest('[data-pane]');
    if (button) return toggle(button.dataset.pane);
    if (target.closest('[data-theme-toggle]')) return turn();
    if (target.closest('.scrim') || target.closest('.pane a[href]')) shut();
  });
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && shut()) e.preventDefault();
  });
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', wire);
  else wire();
}

if (typeof document !== 'undefined') boot();
