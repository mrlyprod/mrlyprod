import { word } from './word.js';

const DOCK = '(min-width: 74rem)';
const PREFIX = (typeof document !== 'undefined' && document.documentElement.dataset.prefix) || 'mrly-';
const KEY = { theme: `${PREFIX}theme`, font: `${PREFIX}font`, tint: `${PREFIX}tint`, saver: `${PREFIX}saver`, cart: `${PREFIX}cart`, welcome: `${PREFIX}welcome` };
const SIDES = ['left', 'right'];
const SHADE = 'screen and (prefers-color-scheme: dark)';

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
const sub = () => document.querySelector('.subheader');
const ready = (fn) => (document.readyState === 'complete' ? fn() : addEventListener('load', fn, { once: true }));

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

/* HALVES */

function halves() {
  const set = root().dataset.theme;
  const media = set === 'dark' ? 'screen' : set === 'light' ? 'not all' : SHADE;
  for (const source of document.querySelectorAll('picture source[data-dark]')) source.media = media;
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

/* SAVER */

function screen(next) {
  write(KEY.saver, next);
  const head = next.split('?')[0];
  for (const pick of document.querySelectorAll('[data-saver-pick]')) pick.value = [...pick.options].some((one) => one.value === head) ? head : 'random';
}

/* NAME */

const SVG = 'http://www.w3.org/2000/svg';
const STILL = '(prefers-reduced-motion: reduce)';

function paint(svg, rows, cols, on) {
  svg.setAttribute('viewBox', `0 0 ${cols} ${rows}`);
  svg.style.setProperty('--rows', rows);
  svg.style.setProperty('--cols', cols);
  svg.replaceChildren(
    ...on.map((i) => {
      const cell = document.createElementNS(SVG, 'rect');
      for (const [k, v] of [['x', i % cols], ['y', Math.floor(i / cols)], ['width', 1], ['height', 1]]) cell.setAttribute(k, v);
      return cell;
    }),
  );
}

function folds(mark) {
  let anim = null;
  let at = 0;
  let way = 0;
  let timer = 0;
  const halt = () => {
    clearInterval(timer);
    timer = 0;
  };
  const tick = () => {
    const next = at + way;
    if (next < 0 || next >= anim.frames.length) return halt();
    at = next;
    paint(mark.querySelector('svg'), anim.rows, anim.cols, anim.frames[at]);
  };
  const run = async (dir) => {
    const text = mark.dataset.word;
    if (!text || (dir > 0 && matchMedia(STILL).matches)) return;
    if (anim?.text !== text) {
      const { fold } = await import('../kit/font/font.js');
      if (mark.dataset.word !== text) return;
      anim = { text, ...fold(text) };
      at = 0;
    }
    way = dir;
    if (!timer) timer = setInterval(tick, 1000 / anim.fps);
  };
  mark.addEventListener('pointerenter', (e) => e.pointerType === 'mouse' && run(1));
  mark.addEventListener('pointerleave', (e) => e.pointerType === 'mouse' && run(-1));
  mark.refold = () => {
    halt();
    anim = null;
    at = 0;
  };
}

async function rename() {
  const mark = document.querySelector('.top .mark');
  if (!mark) return;
  const here = location.pathname;
  if (mark.seen === undefined || mark.seen === here) {
    mark.seen = here;
    return;
  }
  mark.seen = here;
  const next = word(here, JSON.parse(mark.dataset.doors || '[]'));
  if (next === (mark.dataset.word ?? '')) return;
  mark.dataset.word = next;
  const title = mark.getAttribute('aria-label').replace(/^.*, /, '');
  mark.setAttribute('aria-label', next ? `${next}, ${title}` : title);
  mark.refold?.();
  if (here === '/') return arrive(mark);
  depart(mark);
  const { letters } = await import('../kit/font/font.js');
  if (mark.dataset.word !== next) return;
  const { rows, cols, grid } = letters(next || 'X');
  paint(mark.querySelector('svg'), rows, cols, grid.flat().flatMap((on, i) => (on ? [i] : [])));
}

/* WELCOME */

const PLAY = 2000;
const FADE = 250;
const QUIET = 200;
const PAGING = ['ArrowDown', 'PageDown', ' ', 'End'];
const LIVE = 'main a[href], main button, main input, main select, main textarea, main summary';

let playing = false;
let asked = false;
let quiet = 0;
let watch = null;
let owed = false;
let tapped = 0;

const hall = () => document.querySelector('.welcome');
const greeting = () => root().dataset.welcome;
const jump = (top) => scrollTo({ top, behavior: 'instant' });
const rolls = () => CSS.supports('animation-timeline', 'scroll()');

function glide(path, time, done) {
  playing = true;
  const span = matchMedia(STILL).matches ? 0 : time;
  const start = performance.now();
  const step = (now) => {
    const t = span ? Math.min(1, Math.max(0, (now - start) / span)) : 1;
    jump(path(t, hall()?.offsetHeight ?? 0));
    if (t < 1) return requestAnimationFrame(step);
    playing = false;
    done();
  };
  requestAnimationFrame(step);
}

function calm() {
  if (greeting() === 'open') return;
  if (performance.now() < quiet) return setTimeout(calm, QUIET);
  unlisten();
}

function settle() {
  unlock();
  const top = Math.max(0, scrollY - (hall()?.offsetHeight ?? 0));
  write(KEY.welcome, '1');
  root().dataset.welcome = 'shut';
  jump(top);
  quiet = performance.now() + QUIET;
  setTimeout(calm, QUIET);
}

function enter() {
  if (greeting() !== 'open') return;
  asked = playing;
  if (playing) return;
  const mine = shown;
  const go = () => {
    if (shown !== mine || playing || greeting() !== 'open') return;
    clearTimeout(nap);
    delete root().dataset.lock;
    const h = hall()?.offsetHeight ?? 0;
    if (!h || scrollY >= h) return settle();
    const from = scrollY / h;
    glide((t, height) => height * (from + (1 - from) * t), PLAY * (1 - from), settle);
  };
  if (mine?.stop?.leave && root().dataset.lock) return mine.stop.leave(go);
  go();
}

function greet() {
  if (playing || greeting() !== 'shut' || !hall()) return;
  const at = scrollY;
  playing = true;
  asked = false;
  root().dataset.welcome = 'open';
  listen();
  jump(at + hall().offsetHeight);
  setTimeout(() => {
    jump(hall().offsetHeight);
    glide((t, height) => height * (1 - t), PLAY, () => (asked ? enter() : lid()));
  }, matchMedia(STILL).matches ? 0 : FADE);
}

function hold(e) {
  const open = greeting() === 'open';
  if (!open && !playing) {
    if (e.type === 'wheel' && performance.now() < quiet) {
      e.preventDefault();
      quiet = performance.now() + QUIET;
    }
    return;
  }
  const target = e.target instanceof Element ? e.target : null;
  if (e.type === 'wheel') {
    if (Math.abs(e.deltaX) > Math.abs(e.deltaY)) return;
    e.preventDefault();
    return enter();
  }
  if (e.type === 'touchmove') {
    if (e.touches.length > 1) return;
    e.preventDefault();
    return enter();
  }
  if (e.type === 'keydown') {
    if (e.metaKey || e.ctrlKey || e.altKey || !PAGING.includes(e.key)) return;
    e.preventDefault();
    return enter();
  }
  if (target?.closest(LIVE)) {
    if (open && !playing) settle();
    return;
  }
  if (e.type === 'pointerdown') return enter();
  e.preventDefault();
}

function drift() {
  if (greeting() === 'open' && !playing && scrollY > 0) enter();
}

function listen() {
  if (watch) return;
  watch = new AbortController();
  const options = { passive: false, capture: true, signal: watch.signal };
  for (const type of ['wheel', 'touchmove', 'keydown', 'pointerdown', 'click']) addEventListener(type, hold, options);
  addEventListener('scroll', drift, { passive: true, signal: watch.signal });
  addEventListener('pointermove', stir, { passive: true, signal: watch.signal });
}

function unlisten() {
  watch?.abort();
  watch = null;
}

function knock(mark) {
  mark.addEventListener('click', (e) => {
    if (e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
    const now = performance.now();
    const twice = now - tapped < TWICE;
    tapped = twice ? 0 : now;
    if (location.pathname === '/' && greeting() === 'shut' && hall()) {
      e.preventDefault();
      return greet();
    }
    if (!twice) return;
    e.preventDefault();
    if (!greeting()) owed = true;
  });
}

async function arrive(mark) {
  const { markup, welcome } = await import('./welcome.js');
  if (location.pathname !== '/') return;
  const model = welcome();
  const svg = mark.querySelector('svg');
  svg.setAttribute('class', 'glyphs fold');
  svg.setAttribute('viewBox', `0 0 ${model.cols} 5`);
  svg.style.setProperty('--rows', 5);
  svg.style.setProperty('--cols', model.cols);
  svg.innerHTML = markup(model);
  mark.style.setProperty('--cols', model.cols);
  const due = owed;
  owed = false;
  if (!rolls()) return;
  root().dataset.welcome = read(KEY.welcome) || matchMedia(STILL).matches ? 'shut' : 'open';
  if (greeting() === 'open') listen();
  if (due) greet();
}

function depart(mark) {
  owed = false;
  unlock();
  delete root().dataset.welcome;
  unlisten();
  mark.style.removeProperty('--cols');
  mark.querySelector('svg')?.setAttribute('class', 'glyphs');
}

/* LOCK */

const IDLE = 3000;
const TWICE = 400;

let shown = null;
let nap = 0;

async function lid() {
  const pick = read(KEY.saver);
  if (!pick || shown || greeting() !== 'open') return;
  const box = document.createElement('div');
  box.className = 'lock';
  box.setAttribute('aria-hidden', 'true');
  box.append(document.createElement('canvas'));
  const mine = { box, stop: null };
  shown = mine;
  document.body.append(box);
  const stop = await import('/lock.js').then(({ lock }) => lock(box.firstChild, pick)).catch(() => null);
  if (stop && stop.pick !== pick) screen(stop.pick);
  if (stop && shown === mine && !playing && greeting() === 'open') {
    mine.stop = stop;
    root().dataset.lock = 'rest';
    return;
  }
  stop?.();
  if (shown === mine) unlock();
}

function stir(e) {
  if (!root().dataset.lock || e.pointerType === 'touch') return;
  root().dataset.lock = 'wake';
  clearTimeout(nap);
  nap = setTimeout(() => {
    if (root().dataset.lock) root().dataset.lock = 'rest';
  }, IDLE);
}

function unlock() {
  clearTimeout(nap);
  delete root().dataset.lock;
  shown?.stop?.();
  shown?.box.remove();
  shown = null;
}

/* EXPLORER */

let forest = null;

const fetchTree = (url) => (forest ??= fetch(url).then((reply) => (reply.ok ? reply.json() : null)).catch(() => null));

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
    a.href = `${base}${path}`;
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

function wire() {
  sync();
  unread();
  theme(root().dataset.theme ?? '');
  face(root().dataset.font ?? '');
  tint(root().dataset.tint ?? '');
  screen(read(KEY.saver));
  cart();
  once('.contents', contents);
  once('.top .mark', (mark) => {
    folds(mark);
    knock(mark);
  });
  if (greeting() === 'open') listen();
  rename();
}

function boot() {
  root().classList.add('js');
  if ('scrollRestoration' in history) history.scrollRestoration = 'manual';
  ready(land);
  window.addEventListener('theme', halves);
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
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', wire);
  else wire();
}

if (typeof document !== 'undefined') boot();
