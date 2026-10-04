import { load, mount, unmount } from './islands.js';

const FRESH = 30000;
const SHEET = 'link[rel="stylesheet"]';
const HINT = 'link[rel="modulepreload"]';
const META = 'meta[name]:not([name="viewport"]), meta[property], link[rel="canonical"], script[type="application/ld+json"]';
const STILL = '(prefers-reduced-motion: reduce)';
const SIDES = ['left', 'right'];

/* RULE */

export const walled = (path, deep) => deep.find((wall) => path.startsWith(wall)) ?? '';

export function sort(to, from, deep) {
  if (to.origin !== from.origin) return '';
  const wall = walled(to.pathname, deep);
  if (wall && wall === walled(from.pathname, deep)) return 'inner';
  if (!wall && !to.pathname.endsWith('/')) return '';
  if (to.pathname === from.pathname && to.search === from.search && to.href.includes('#')) return 'mark';
  return 'page';
}

/* STATE */

const html = typeof document === 'undefined' ? null : document.documentElement;
const DEEP = (html?.dataset.deep ?? '').split(' ').filter(Boolean);
const first = html ? location.href : '';
const pages = new Map();
const loads = new WeakMap();
const place = (at) => walled(at.pathname, DEEP) || at.pathname;
const key = (at) => walled(at.pathname, DEEP) || at.pathname + at.search;
const state = () => (history.state && typeof history.state === 'object' ? history.state : {});
const frame = () => new Promise((done) => requestAnimationFrame(done));
const still = () => matchMedia(STILL).matches;
let here = html ? key(location) : '';
let shown = html ? place(location) : '';
let turn = 0;

/* FETCH */

export function grab(url, old = false) {
  const at = url.origin + key(url);
  const hit = pages.get(at);
  if (hit && (old || performance.now() - hit.born < FRESH)) return hit.text;
  const text = fetch(at, { headers: { accept: 'text/html' } }).then((reply) => {
    if (!reply.ok || reply.redirected || !(reply.headers.get('content-type') ?? '').startsWith('text/html')) throw new Error(`router: ${at} is no page`);
    return reply.text();
  });
  text.catch(() => pages.get(at)?.text === text && pages.delete(at));
  pages.set(at, { born: performance.now(), text });
  return text;
}

export const read = (text) => new DOMParser().parseFromString(text, 'text/html');

/* HEAD */

const absolute = (node, base) => new URL(node.getAttribute('href'), base).href;

const scripts = (doc) => [...doc.head.querySelectorAll('script[src^="/"]:not([src^="//"])')].map((one) => one.getAttribute('src')).join(' ');

function carry(node, base) {
  const copy = document.importNode(node, true);
  if (copy.hasAttribute('href')) copy.setAttribute('href', absolute(node, base));
  return copy;
}

function wear(doc, url) {
  const have = new Map([...document.head.querySelectorAll(SHEET)].map((link) => [absolute(link, first), link]));
  let last = [...have.values()].at(-1);
  return [...doc.head.querySelectorAll(SHEET)].map((one) => {
    const at = absolute(one, url);
    if (have.has(at)) return have.get(at);
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.media = 'not all';
    link.href = at;
    loads.set(link, new Promise((ok, no) => {
      link.onload = ok;
      link.onerror = no;
    }));
    if (last) last.after(link);
    else document.head.append(link);
    have.set(at, link);
    last = link;
    return link;
  });
}

/* BODY */

function span(doc) {
  const top = doc.querySelector('body > header');
  const base = doc.querySelector('body > footer');
  if (!top || !base) throw new Error('router: the page has no frame');
  const out = [];
  for (let node = top.nextSibling; node && node !== base; node = node.nextSibling) out.push(node);
  return out;
}

function target(hash) {
  try {
    return hash ? document.getElementById(decodeURIComponent(hash.slice(1))) : null;
  } catch {
    return null;
  }
}

/* SWAP */

async function dress(doc, url) {
  if (scripts(doc) !== scripts(document)) throw new Error('router: the page runs other scripts');
  const body = span(doc);
  const sheets = wear(doc, url);
  const hints = [...doc.head.querySelectorAll(HINT)].map((one) => carry(one, url));
  document.head.append(...hints);
  await Promise.all([...sheets.map((link) => loads.get(link)), load(doc)]);
  return { doc, body, sheets, hints };
}

function pin() {
  if (place(location) === shown) history.replaceState({ ...state(), y: Math.round(scrollY) }, '');
}

function swap(next, url, mode, y) {
  if (mode === 'push') pin();
  unmount();
  if (mode === 'push') history.pushState(null, '', url.href);
  if (mode === 'replace') history.replaceState(null, '', url.href);
  document.title = next.doc.title;
  const meta = [...next.doc.head.querySelectorAll(META)].map((one) => carry(one, url));
  for (const old of document.head.querySelectorAll(`${META}, ${HINT}`)) if (!next.hints.includes(old)) old.remove();
  document.head.querySelector('title').after(...meta, ...next.hints);
  for (const link of document.head.querySelectorAll(SHEET)) {
    if (next.sheets.includes(link)) link.removeAttribute('media');
    else link.remove();
  }
  for (const node of span(document)) node.remove();
  document.querySelector('body > footer').before(...next.body);
  for (const side of SIDES) html.dataset[side] = 'shut';
  window.dispatchEvent(new Event('wire'));
  const spot = mode === 'pop' ? null : target(url.hash);
  if (spot) spot.scrollIntoView();
  else scrollTo({ top: y, behavior: 'instant' });
  document.getElementById('main')?.focus({ preventScroll: true });
  here = key(location);
  shown = place(location);
  return mount();
}

/* GO */

const plain = (url) => (url.href === location.href ? location.reload() : location.assign(url.href));

function morph(run, calm) {
  if (calm || !document.startViewTransition) return run();
  const made = document.startViewTransition(run);
  made.ready.catch(() => {});
  made.finished.catch(() => {});
  return made.updateCallbackDone;
}

export async function go(url, mode = 'push', y = 0, calm = still()) {
  const mine = ++turn;
  html.setAttribute('aria-busy', 'true');
  try {
    const next = await dress(read(await grab(url, mode === 'pop')), url);
    let up = null;
    await morph(() => {
      if (mine === turn) up = swap(next, url, mode, y);
    }, calm);
    await up;
    if (mode === 'pop' && y) {
      await frame();
      await frame();
      if (mine === turn) scrollTo({ top: y, behavior: 'instant' });
    }
  } catch {
    if (mine === turn) return plain(url);
  }
  if (mine === turn) html.removeAttribute('aria-busy');
}

/* TAKERS */

const asks = (a) => Boolean(a.dataset.router) && key(new URL(a.href)) !== here && place(location) === shown;

const ask = (a, live) => import(a.dataset.router).then((taker) => taker.open(a, live));

function held() {
  const page = state().dialog;
  const a = page ? [...document.querySelectorAll('a[data-router]')].find((one) => one.getAttribute('href') === page) : null;
  if (a) ask(a, () => state().dialog === page).catch(() => {});
}

export function layer(keys) {
  pin();
  history.pushState({ ...state(), ...keys }, '');
  here = key(location);
}

/* EVENTS */

function aim(event) {
  const a = event.target instanceof Element ? event.target.closest('a[href]') : null;
  return a instanceof HTMLAnchorElement && !a.target && !a.hasAttribute('download') ? a : null;
}

function click(event) {
  if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
  const a = aim(event);
  if (!a) return;
  if (asks(a)) {
    event.preventDefault();
    const mine = ++turn;
    html.setAttribute('aria-busy', 'true');
    return ask(a, () => mine === turn).then(() => mine === turn && html.removeAttribute('aria-busy'), () => mine === turn && plain(new URL(a.href)));
  }
  const over = Boolean(state().dialog);
  const kind = sort(a, location, DEEP);
  if (over && kind === 'page' && key(new URL(a.href)) === here) {
    event.preventDefault();
    return history.back();
  }
  if (kind === 'mark') here = key(location);
  if (over ? !kind : kind !== 'page') return;
  event.preventDefault();
  go(new URL(a.href), over || a.href === location.href ? 'replace' : 'push');
}

function hint(event) {
  const a = aim(event);
  if (!a || sort(a, location, DEEP) !== 'page') return;
  grab(new URL(a.href), Boolean(a.dataset.router)).catch(() => {});
  if (asks(a)) import(a.dataset.router).catch(() => {});
}

function pop(event) {
  const y = typeof state().y === 'number' ? state().y : null;
  if (key(location) !== here) return go(new URL(location.href), 'pop', y ?? 0, event.hasUAVisualTransition || still());
  turn++;
  html.removeAttribute('aria-busy');
  if (walled(location.pathname, DEEP)) return;
  const spot = y === null ? target(location.hash) : null;
  if (spot) spot.scrollIntoView();
  else scrollTo({ top: y ?? 0, behavior: 'instant' });
}

function back(event) {
  if (event.persisted) html.removeAttribute('aria-busy');
}

function note() {
  if (document.querySelector('[aria-busy="true"]')) return;
  try {
    history.replaceState({ ...state(), y: Math.round(scrollY) }, '');
  } catch {}
}

/* BOOT */

if (html) {
  mount().catch(() => {});
  document.addEventListener('click', click);
  document.addEventListener('pointerenter', hint, true);
  document.addEventListener('focus', hint, true);
  addEventListener('popstate', pop);
  addEventListener('popstate', held);
  addEventListener('scrollend', note);
  addEventListener('pageshow', back);
  held();
}
