import { afterAll, expect, mock, spyOn, test } from "bun:test";
import type { Row } from "../../scripts/build.ts";

type Fired = {
  target: unknown;
  button: number;
  defaultPrevented: boolean;
  preventDefault: () => unknown;
  metaKey?: boolean;
  ctrlKey?: boolean;
  shiftKey?: boolean;
  altKey?: boolean;
};

type Heard = (e: Fired) => void;

type Link = HTMLAnchorElement & {
  href: string;
  origin: string;
  pathname: string;
  search: string;
  hash: string;
  target: string;
  hasAttribute: () => boolean;
  closest: () => Link | null;
};

type Drawn = Node & { row: Row; text: string; form: string | undefined; innerHTML: string };

const ROOT = "https://mrly.net";

const SITE = {
  title: "mrly",
  root: ROOT,
  tagline: "designs",
  prefix: "mrly-",
  company: "MrlyProd",
  since: 2026,
  doors: [],
};

const ROWS = [
  { route: "/", kind: "app", title: "mrly", source: null, lead: "home", meta: { id: "home" } },
  { route: "/about/", kind: "prose", title: "About", source: "site/README.md", lead: "who", meta: { id: "about" } },
  { route: "/git/", kind: "app", title: "Code", source: null, lead: "code", meta: { id: "git", deep: true } },
  { route: "/life/", kind: "app", title: "Life", source: null, lead: "cells", meta: { id: "life", scene: true } },
  { route: "/404.html", kind: "app", title: "Nothing here", source: null, lead: "", meta: { id: "missing" } },
];

const KEPT = [
  "document",
  "window",
  "location",
  "history",
  "matchMedia",
  "sessionStorage",
  "requestAnimationFrame",
  "scrollTo",
  "scrollY",
  "fetch",
  "addEventListener",
  "Element",
  "HTMLAnchorElement",
  "MutationObserver",
];

const kept = Object.fromEntries(KEPT.map((name) => [name, Object.getOwnPropertyDescriptor(globalThis, name)]));

afterAll(() => {
  for (const [name, was] of Object.entries(kept)) {
    if (was) Object.defineProperty(globalThis, name, was);
    else Reflect.deleteProperty(globalThis, name);
  }
});

class Element {}

class HTMLAnchorElement extends Element {}

const matches = (one: Node, selector: string) => {
  const [, tag, key, value] = /^(\w+)\[(\w+)="([^"]+)"\]$/.exec(selector) ?? [];
  return one.tag === tag && (Reflect.get(one, String(key)) ?? one.attrs[String(key)]) === value;
};

class Node extends Element {
  tag: string;
  kids: Node[] = [];
  attrs: Record<string, string> = {};
  dataset: Record<string, string> = {};
  parent: Node | null = null;
  className = "";
  hidden = false;
  scrollTop = 0;
  declare href?: string;
  declare rel?: string;
  declare name?: string;
  declare content?: string | null;

  constructor(tag: string) {
    super();
    this.tag = tag;
  }

  get childNodes() {
    return this.kids;
  }

  get isConnected() {
    return this.parent !== null;
  }

  setAttribute(key: string, value: string) {
    this.attrs[key] = String(value);
  }

  getAttribute(key: string) {
    return this.attrs[key] ?? null;
  }

  removeAttribute(key: string) {
    delete this.attrs[key];
  }

  append(...more: Node[]) {
    for (const kid of more) {
      kid.remove();
      kid.parent = this;
      this.kids.push(kid);
    }
  }

  replaceChildren(...more: Node[]) {
    for (const kid of this.kids) kid.parent = null;
    this.kids = [];
    this.append(...more);
  }

  remove() {
    this.parent?.kids.splice(this.parent.kids.indexOf(this), 1);
    this.parent = null;
  }

  after(kid: Node) {
    this.parent?.append(kid);
  }

  querySelector(selector: string) {
    return this.kids.find((kid) => matches(kid, selector)) ?? null;
  }

  focus() {}

  scrollTo({ top }: { top: number }) {
    this.scrollTop = top;
  }

  addEventListener() {}
}

const el = (tag: string) => new Node(tag);

const heard = {
  click: [] as Heard[],
  wire: [] as string[],
  gone: [] as string[],
  unmounts: 0,
  form: null as (() => Promise<void>) | null,
};
const body = el("body");
const main = el("main");
const head = el("head");
const describe = Object.assign(el("meta"), { name: "description" });
head.append(describe);
body.append(main, el("aside"));
const panes: Record<string, Node> = { main, right: body.kids[1] };
const texts: Record<string, string> = { "/raw/site/README.md": "# About" };
const breaks = new Set<string>();
const store = new Map<string, string>();

let at = new URL(`${ROOT}/`);

Object.assign(globalThis, {
  Element,
  HTMLAnchorElement,
  document: {
    documentElement: el("html"),
    head,
    body,
    createElement: el,
    getElementById: (id: string) => panes[id] ?? null,
    querySelector: () => null,
    addEventListener: (type: string, fn: Heard) => type === "click" && heard.click.push(fn),
  },
  window: {
    dispatchEvent: (event: CustomEvent<{ route: string }>) =>
      event.type === "wire" && heard.wire.push(event.detail.route),
    addEventListener: () => {},
  },
  history: {
    state: null,
    pushState: (_state: unknown, _title: string, href: string) => {
      at = new URL(href, at);
    },
    replaceState: (_state: unknown, _title: string, href?: string) => {
      if (href) at = new URL(href, at);
    },
  },
  matchMedia: () => ({ matches: true }),
  sessionStorage: {
    getItem: (key: string) => store.get(key) ?? null,
    setItem: (key: string, value: string) => store.set(key, value),
    removeItem: (key: string) => store.delete(key),
  },
  requestAnimationFrame: (fn: () => void) => setTimeout(fn, 0),
  scrollTo: () => {},
  scrollY: 0,
  addEventListener: () => {},
  MutationObserver: class {
    constructor(fn: () => Promise<void>) {
      heard.form = fn;
    }

    observe() {}
  },
  fetch: async (url: string) => ({ ok: url in texts, status: url in texts ? 200 : 404, text: async () => texts[url] }),
});

Object.defineProperty(globalThis, "location", {
  configurable: true,
  get: () =>
    Object.assign(at, { reload: () => heard.gone.push(at.href), assign: (href: string) => heard.gone.push(href) }),
});

const shell = () => ({
  page: (row: Row) => ({
    layout: {},
    render: (one: Row, _site: unknown, host: Node, { text }: { text: string }) => {
      if (breaks.has(row.route)) throw new Error("no chunk");
      Object.assign(host, { row: one, text, form: document.documentElement.dataset.form });
    },
    unmount: () => heard.unmounts++,
  }),
});

const app = () => ({ ...shell(), gated: (row: Row) => Boolean(row.meta.scene) });

mock.module("site:css", () => ({ css: {} }));
mock.module("site:routes", () => ({ rows: ROWS, site: SITE }));
mock.module("./app.tsx", app);
mock.module("./prose.tsx", shell);

const { go } = await import("./router.ts");

const visit = (path: string) => go(new URL(path, ROOT));

const shown = () => main.kids[0] as Drawn;

const canonical = () => head.querySelector('link[rel="canonical"]')?.href;

const robots = () => head.querySelector('meta[name="robots"]')?.content ?? null;

test("a known route renders its row with its raw source, names the page, pushes the url and wires the route", async () => {
  await visit("/about/");
  expect([
    shown().row.route,
    shown().text,
    document.title,
    describe.getAttribute("content"),
    canonical(),
    location.pathname,
    robots(),
    heard.wire.at(-1),
  ]).toEqual(["/about/", "# About", "About · mrly", "who", `${ROOT}/about/`, "/about/", null, "/about/"]);
});

test("/index.html renders home, the title alone when it is the site title", async () => {
  await visit("/index.html");
  expect([shown().row.route, document.title, canonical(), heard.wire.at(-1)]).toEqual(["/", "mrly", `${ROOT}/`, "/"]);
});

test("a path under git renders the git row with the path itself as canonical", async () => {
  await visit("/git/site/src/shell/router.ts");
  expect([shown().row.route, canonical(), location.pathname, heard.wire.at(-1)]).toEqual([
    "/git/",
    `${ROOT}/git/site/src/shell/router.ts`,
    "/git/site/src/shell/router.ts",
    "/git/",
  ]);
});

test("an unknown path renders the missing row with noindex, and the next known page drops it", async () => {
  await visit("/gone/");
  const lost = [shown().row.meta.id, robots(), canonical(), location.pathname];
  await visit("/about/");
  expect([...lost, robots()]).toEqual(["missing", "noindex", `${ROOT}/404.html`, "/gone/", null]);
});

test("a click takes a page link and leaves a raw file, another origin and a modified click to the browser", () => {
  const link = (href: string): Link => {
    const url = new URL(href, at);
    return Object.assign(new HTMLAnchorElement(), {
      href: url.href,
      origin: url.origin,
      pathname: url.pathname,
      search: url.search,
      hash: url.hash,
      target: "",
      hasAttribute: () => false,
      closest: (): Link | null => null,
    });
  };
  const click = (href: string, more: Partial<Fired> = {}) => {
    const a = link(href);
    a.closest = () => a;
    let taken = false;
    for (const fn of heard.click)
      fn({ target: a, button: 0, defaultPrevented: false, preventDefault: () => (taken = true), ...more });
    return taken;
  };
  expect([
    click("/git/"),
    click("/raw/site/src/shell/router.ts"),
    click("https://oeis.org/A000001"),
    click("/about/", { metaKey: true }),
  ]).toEqual([true, false, false, false]);
});

test("a page that fails to load reloads once, then shows the error page", async () => {
  const quiet = spyOn(console, "error").mockImplementation(() => {});
  breaks.add("/about/");
  heard.gone.length = 0;
  await visit("/about/");
  await visit("/about/");
  breaks.clear();
  quiet.mockRestore();
  expect([heard.gone, shown().className, shown().innerHTML.includes("did not load")]).toEqual([
    [`${ROOT}/about/`],
    "lede",
    true,
  ]);
});

test("a gated app redraws when the form turns, and any other page stays", async () => {
  const { dataset } = document.documentElement;
  dataset.form = "desk";
  await visit("/about/");
  dataset.form = "phone";
  await heard.form!();
  const about = shown().form;
  await visit("/life/");
  const before = heard.unmounts;
  dataset.form = "desk";
  await heard.form!();
  delete dataset.form;
  expect([about, shown().row.route, shown().form, heard.unmounts - before, location.pathname]).toEqual([
    "desk",
    "/life/",
    "desk",
    1,
    "/life/",
  ]);
});
