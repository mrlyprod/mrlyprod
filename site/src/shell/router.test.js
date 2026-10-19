import { afterAll, expect, mock, spyOn, test } from 'bun:test';

const ROOT = 'https://mrly.net';

const SITE = { title: 'mrly', root: ROOT, tagline: 'designs', prefix: 'mrly-', company: 'MrlyProd', since: 2026, doors: [] };

const ROWS = [
  { route: '/', kind: 'app', title: 'mrly', source: null, lead: 'home', meta: { id: 'home' } },
  { route: '/about/', kind: 'prose', title: 'About', source: 'site/README.md', lead: 'who', meta: { id: 'about' } },
  { route: '/git/', kind: 'app', title: 'Code', source: null, lead: 'code', meta: { id: 'git', deep: true } },
  { route: '/404.html', kind: 'app', title: 'Nothing here', source: null, lead: '', meta: { id: 'missing' } },
];

const KEPT = ['document', 'window', 'location', 'history', 'matchMedia', 'sessionStorage', 'requestAnimationFrame', 'scrollTo', 'scrollY', 'fetch', 'addEventListener', 'Element', 'HTMLAnchorElement'];

const kept = Object.fromEntries(KEPT.map((name) => [name, Object.getOwnPropertyDescriptor(globalThis, name)]));

afterAll(() => {
  for (const [name, was] of Object.entries(kept)) {
    if (was) Object.defineProperty(globalThis, name, was);
    else delete globalThis[name];
  }
});

class Element {}

class HTMLAnchorElement extends Element {}

const matches = (one, selector) => {
  const [, tag, key, value] = /^(\w+)\[(\w+)="([^"]+)"\]$/.exec(selector) ?? [];
  return one.tag === tag && (one[key] ?? one.attrs[key]) === value;
};

class Node extends Element {
  constructor(tag) {
    super();
    Object.assign(this, { tag, kids: [], attrs: {}, dataset: {}, parent: null, className: '', hidden: false, scrollTop: 0 });
  }

  get childNodes() {
    return this.kids;
  }

  get isConnected() {
    return this.parent !== null;
  }

  setAttribute(key, value) {
    this.attrs[key] = String(value);
  }

  getAttribute(key) {
    return this.attrs[key] ?? null;
  }

  removeAttribute(key) {
    delete this.attrs[key];
  }

  append(...more) {
    for (const kid of more) {
      kid.remove();
      kid.parent = this;
      this.kids.push(kid);
    }
  }

  replaceChildren(...more) {
    for (const kid of this.kids) kid.parent = null;
    this.kids = [];
    this.append(...more);
  }

  remove() {
    this.parent?.kids.splice(this.parent.kids.indexOf(this), 1);
    this.parent = null;
  }

  before(kid) {
    this.parent?.append(kid);
  }

  after(kid) {
    this.parent?.append(kid);
  }

  querySelector(selector) {
    return this.kids.find((kid) => matches(kid, selector)) ?? null;
  }

  focus() {}

  scrollTo({ top }) {
    this.scrollTop = top;
  }

  addEventListener() {}
}

const el = (tag) => new Node(tag);

const heard = { click: [], wire: [], gone: [] };
const body = el('body');
const main = el('main');
const head = el('head');
const describe = Object.assign(el('meta'), { name: 'description' });
head.append(describe);
body.append(el('aside'), main, el('aside'));
const panes = { main, left: body.kids[0], right: body.kids[2] };
const texts = { '/raw/site/README.md': '# About' };
const breaks = new Set();
const store = new Map();

let at = new URL(`${ROOT}/`);

Object.assign(globalThis, {
  Element,
  HTMLAnchorElement,
  document: { documentElement: el('html'), head, body, createElement: el, getElementById: (id) => panes[id] ?? null, querySelector: () => null, addEventListener: (type, fn) => type === 'click' && heard.click.push(fn) },
  window: { dispatchEvent: (event) => event.type === 'wire' && heard.wire.push(event.detail.route) },
  history: { state: null, pushState: (state, title, href) => (at = new URL(href, at)), replaceState: (state, title, href) => href && (at = new URL(href, at)) },
  matchMedia: () => ({ matches: true }),
  sessionStorage: { getItem: (key) => store.get(key) ?? null, setItem: (key, value) => store.set(key, value), removeItem: (key) => store.delete(key) },
  requestAnimationFrame: (fn) => setTimeout(fn, 0),
  scrollTo: () => {},
  scrollY: 0,
  addEventListener: () => {},
  fetch: async (url) => ({ ok: url in texts, status: url in texts ? 200 : 404, text: async () => texts[url] }),
});

Object.defineProperty(globalThis, 'location', { configurable: true, get: () => Object.assign(at, { reload: () => heard.gone.push(at.href), assign: (href) => heard.gone.push(href) }) });

const shell = () => ({
  page: (row) => ({
    layout: {},
    render: (one, site, host, { text }) => {
      if (breaks.has(row.route)) throw new Error('no chunk');
      Object.assign(host, { row: one, text });
    },
    unmount: () => {},
  }),
});

mock.module('site:css', () => ({ css: {} }));
mock.module('site:routes', () => ({ rows: ROWS, site: SITE }));
mock.module('./app.jsx', shell);
mock.module('./prose.jsx', shell);

const { go } = await import('./router.js');

const visit = (path) => go(new URL(path, ROOT));

const shown = () => main.kids[0];

const canonical = () => head.querySelector('link[rel="canonical"]')?.href;

const robots = () => head.querySelector('meta[name="robots"]')?.content ?? null;

test('a known route renders its row with its raw source, names the page, pushes the url and wires the route', async () => {
  await visit('/about/');
  expect([shown().row.route, shown().text, document.title, describe.getAttribute('content'), canonical(), location.pathname, robots(), heard.wire.at(-1)]).toEqual(['/about/', '# About', 'About · mrly', 'who', `${ROOT}/about/`, '/about/', null, '/about/']);
});

test('/index.html renders home, the title alone when it is the site title', async () => {
  await visit('/index.html');
  expect([shown().row.route, document.title, canonical(), heard.wire.at(-1)]).toEqual(['/', 'mrly', `${ROOT}/`, '/']);
});

test('a path under git renders the git row with the path itself as canonical', async () => {
  await visit('/git/site/src/shell/router.js');
  expect([shown().row.route, canonical(), location.pathname, heard.wire.at(-1)]).toEqual(['/git/', `${ROOT}/git/site/src/shell/router.js`, '/git/site/src/shell/router.js', '/git/']);
});

test('an unknown path renders the missing row with noindex, and the next known page drops it', async () => {
  await visit('/gone/');
  const lost = [shown().row.meta.id, robots(), canonical(), location.pathname];
  await visit('/about/');
  expect([...lost, robots()]).toEqual(['missing', 'noindex', `${ROOT}/404.html`, '/gone/', null]);
});

test('a click takes a page link and leaves a raw file, another origin and a modified click to the browser', () => {
  const link = (href) => {
    const url = new URL(href, at);
    return Object.assign(new HTMLAnchorElement(), { href: url.href, origin: url.origin, pathname: url.pathname, search: url.search, hash: url.hash, target: '', hasAttribute: () => false, closest: () => null });
  };
  const click = (href, more = {}) => {
    const a = link(href);
    a.closest = () => a;
    let taken = false;
    for (const fn of heard.click) fn({ target: a, button: 0, defaultPrevented: false, preventDefault: () => (taken = true), ...more });
    return taken;
  };
  expect([click('/git/'), click('/raw/site/src/shell/router.js'), click('https://oeis.org/A000001'), click('/about/', { metaKey: true })]).toEqual([true, false, false, false]);
});

test('a page that fails to load reloads once, then shows the error page', async () => {
  const quiet = spyOn(console, 'error').mockImplementation(() => {});
  breaks.add('/about/');
  heard.gone.length = 0;
  await visit('/about/');
  await visit('/about/');
  breaks.clear();
  quiet.mockRestore();
  expect([heard.gone, shown().className, shown().innerHTML.includes('did not load')]).toEqual([[`${ROOT}/about/`], 'lede', true]);
});
