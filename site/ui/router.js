import { css } from 'site:css';
import { pages } from 'site:pages';
import { rows, site } from 'site:routes';
import { LAYOUT } from './pages/index.js';
import { deeps, match, named, sort, trail, walled } from './route.js';

const FRESH = 30000;
const STILL = '(prefers-reduced-motion: reduce)';
const SIDES = ['left', 'right'];
const ONCE = `${site.prefix}reload`;

/* STATE */

const html = document.documentElement;
const main = document.getElementById('main');
const panes = { left: document.getElementById('left'), right: document.getElementById('right') };
const DEEP = deeps(rows);
const place = (at) => walled(at.pathname, DEEP) || at.pathname;
const key = (at) => walled(at.pathname, DEEP) || at.pathname + at.search;
const state = () => (history.state && typeof history.state === 'object' ? history.state : {});
const frame = () => new Promise((done) => requestAnimationFrame(done));
const still = () => matchMedia(STILL).matches;
let here = key(location);
let shown = place(location);
let turn = 0;
let current = null;
let art = null;
let drawn = null;

/* LOAD */

const texts = new Map();

function source(row) {
  if (!row.source) return Promise.resolve('');
  const url = `/raw/${row.source}`;
  const hit = texts.get(url);
  if (hit && performance.now() - hit.born < FRESH) return hit.text;
  const text = fetch(url).then((reply) => (reply.ok ? reply.text() : Promise.reject(new Error(`router: ${url} answered ${reply.status}`))));
  text.catch(() => texts.get(url)?.text === text && texts.delete(url));
  texts.set(url, { born: performance.now(), text });
  return text;
}

function page(kind) {
  const thunk = pages[kind];
  return thunk ? thunk() : Promise.reject(new Error(`router: no page draws the kind ${kind}`));
}

const sheets = new Map();

function wear(name) {
  const url = name ? css[name] : null;
  if (!url) return null;
  if (!sheets.has(url)) {
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.media = 'not all';
    link.href = url;
    sheets.set(url, { link, ready: new Promise((ok, no) => Object.assign(link, { onload: ok, onerror: no })) });
    document.head.append(link);
  }
  return sheets.get(url);
}

function figures() {
  art ??= Promise.all([import('../lib/figure.js'), import('site:figures')]).then(([one, table]) => {
    one.start(table);
    drawn = one;
    return one;
  });
  return art;
}

/* RENDER */

async function prepare(row, url) {
  const layout = LAYOUT[row.kind] ?? {};
  const [kind, text] = await Promise.all([page(row.kind), source(row)]);
  const sheet = wear(layout.sheet);
  const host = document.createElement(layout.bare ? 'div' : 'article');
  if (!layout.bare) host.className = 'prose';
  const side = { left: document.createElement('div'), right: document.createElement('div') };
  await kind.render(row, site, host, { text, rows, url, ...side });
  await sheet?.ready;
  return { row, layout, kind, host, sheet, side };
}

/* HEAD */

function meta(selector, make) {
  let node = document.head.querySelector(selector);
  if (!node && make) {
    node = make();
    document.head.append(node);
  }
  return node;
}

function head(row, url) {
  document.title = named(row, site);
  meta('meta[name="description"]')?.setAttribute('content', row.lead || site.tagline);
  const canonical = meta('link[rel="canonical"]', () => Object.assign(document.createElement('link'), { rel: 'canonical' }));
  canonical.href = site.root + (row.meta?.deep ? url.pathname : row.route);
  const robots = meta('meta[name="robots"]');
  if (row.kind === 'missing' && !robots) document.head.append(Object.assign(document.createElement('meta'), { name: 'robots', content: 'noindex' }));
  if (row.kind !== 'missing') robots?.remove();
}

function crumbs(row, url) {
  const list = document.querySelector('.subheader .crumbs ol');
  const steps = trail(row, url.pathname);
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

/* FRAME */

function dress(next) {
  const { layout, side } = next;
  main.className = layout.wide ? 'wide' : '';
  const late = layout.right === 'late';
  const fill = { left: Boolean(layout.left), right: late || side.right.childNodes.length > 0 };
  for (const name of SIDES) {
    const pane = panes[name];
    pane.replaceChildren(...(side[name].childNodes.length ? [side[name]] : []));
    pane.hidden = name === 'right' && late;
    if (fill[name] && !pane.isConnected) (name === 'left' ? main.before(pane) : main.after(pane));
    if (!fill[name]) pane.remove();
    const button = document.querySelector(`.subheader .actions [data-pane="${name}"]`);
    if (button) button.hidden = !fill[name] || (name === 'right' && late);
    html.dataset[name] = 'shut';
  }
  for (const { link } of sheets.values()) link.media = link === next.sheet?.link ? '' : 'not all';
}

function leave() {
  drawn?.drop();
  if (!current) return main.replaceChildren();
  current.kind.unmount(current.host);
  current = null;
}

function target(hash) {
  try {
    return hash ? document.getElementById(decodeURIComponent(hash.slice(1))) : null;
  } catch {
    return null;
  }
}

function pin() {
  if (place(location) === shown) history.replaceState({ ...state(), y: Math.round(scrollY) }, '');
}

function swap(next, url, mode, y) {
  if (mode === 'push') pin();
  leave();
  if (mode === 'push') history.pushState(null, '', url.href);
  if (mode === 'replace') history.replaceState(null, '', url.href);
  head(next.row, url);
  crumbs(next.row, url);
  dress(next);
  main.replaceChildren(next.host);
  current = next;
  here = key(location);
  shown = place(location);
  const spot = mode === 'pop' ? null : target(url.hash);
  if (mode === 'boot') return spot?.scrollIntoView();
  if (spot) spot.scrollIntoView();
  else scrollTo({ top: y, behavior: 'instant' });
  main.focus({ preventScroll: true });
}

async function settle(next, url, mine) {
  await next.kind.mount?.(next.host, { rows, site, url });
  if (mine !== turn) return;
  if (next.host.querySelector('figure[data-figure]')) {
    const one = await figures();
    if (mine === turn) one.scan(next.host, Boolean(next.host.querySelector('[data-eager]')));
  }
  window.dispatchEvent(new CustomEvent('wire', { detail: { route: next.row.route, doors: site.doors } }));
}

/* FAIL */

function fail(url, error) {
  console.error(error);
  let tried = '';
  try {
    tried = sessionStorage.getItem(ONCE) ?? '';
    if (tried !== url.href) sessionStorage.setItem(ONCE, url.href);
  } catch {}
  if (tried !== url.href) return url.href === location.href ? location.reload() : location.assign(url.href);
  try {
    sessionStorage.removeItem(ONCE);
  } catch {}
  leave();
  const lede = document.createElement('div');
  lede.className = 'lede';
  lede.innerHTML = '<h1 id="error">This page did not load</h1><p class="lead">The network or the site failed twice. <a href="">Reload</a> to try again, or go <a href="/">home</a>.</p>';
  main.replaceChildren(lede);
}

/* GO */

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
  const row = match(rows, url.pathname);
  try {
    const next = await prepare(row, url);
    if (mine !== turn) return next.kind.unmount(next.host);
    await morph(() => swap(next, url, mode, y), calm || mode === 'boot');
    try {
      sessionStorage.removeItem(ONCE);
    } catch {}
    await settle(next, url, mine);
    if (mode === 'pop' && y && mine === turn) {
      await frame();
      await frame();
      if (mine === turn) scrollTo({ top: y, behavior: 'instant' });
    }
  } catch (error) {
    if (mine === turn) fail(url, error);
  }
  if (mine === turn) html.removeAttribute('aria-busy');
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
  const kind = sort(a, location, DEEP);
  if (kind === 'mark') here = key(location);
  if (kind !== 'page') return;
  event.preventDefault();
  go(new URL(a.href), a.href === location.href ? 'replace' : 'push');
}

function hint(event) {
  const a = aim(event);
  if (!a || sort(a, location, DEEP) !== 'page') return;
  const row = match(rows, a.pathname);
  page(row.kind).catch(() => {});
  source(row).catch(() => {});
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
  if (html.getAttribute('aria-busy') === 'true' || place(location) !== shown) return;
  try {
    history.replaceState({ ...state(), y: Math.round(scrollY) }, '');
  } catch {}
}

/* BOOT */

function boot() {
  const year = new Date().getFullYear();
  const legal = document.querySelector('.base .legal');
  if (legal) legal.textContent = `Copyright © ${site.company || site.title} ${site.since < year ? `${site.since}-${year}` : year}. All rights reserved.`;
  for (const name of SIDES) panes[name].remove();
  document.addEventListener('click', click);
  for (const type of ['pointerenter', 'focus', 'pointerdown']) document.addEventListener(type, hint, true);
  addEventListener('popstate', pop);
  addEventListener('scrollend', note);
  addEventListener('pagehide', note);
  addEventListener('pageshow', back);
  document.addEventListener('visibilitychange', () => document.hidden && note());
  go(new URL(location.href), 'boot');
}

boot();
