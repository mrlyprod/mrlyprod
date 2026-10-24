import { css } from "site:css";
import { rows, site } from "site:routes";
import { drawn, figures } from "./figures.js";
import { calm } from "./prefs.js";
import { deeps, match, named, sort, walled } from "./route.js";

const FRESH = 30000;
const ONCE = `${site.prefix}reload`;
const SHELLS = { app: () => import("./app.jsx"), prose: () => import("./prose.jsx") };

/* STATE */

const html = document.documentElement;
const main = document.getElementById("main");
const pane = () => document.querySelector(".window");
const bar = () => document.querySelector(".window > .title");
const drawer = document.getElementById("right");
const DEEP = deeps(rows);
const place = (at) => walled(at.pathname, DEEP) || at.pathname;
const key = (at) => walled(at.pathname, DEEP) || at.pathname + at.search;
const state = () => (history.state && typeof history.state === "object" ? history.state : {});
const frame = () => new Promise((done) => requestAnimationFrame(done));
const roll = (top) => main.scrollTo({ top, behavior: "instant" });
let here = key(location);
let shown = place(location);
let turn = 0;
let current = null;

/* LOAD */

const texts = new Map();

function source(row) {
  if (!row.source) return Promise.resolve("");
  const url = `/raw/${row.source}`;
  const hit = texts.get(url);
  if (hit && performance.now() - hit.born < FRESH) return hit.text;
  const text = fetch(url).then((reply) =>
    reply.ok ? reply.text() : Promise.reject(new Error(`router: ${url} answered ${reply.status}`)),
  );
  text.catch(() => texts.get(url)?.text === text && texts.delete(url));
  texts.set(url, { born: performance.now(), text });
  return text;
}

function page(row) {
  const shell = SHELLS[row.kind];
  return shell
    ? shell().then((one) => one.page(row))
    : Promise.reject(new Error(`router: no shell draws the kind ${row.kind}`));
}

const sheets = new Map();

function wear(name) {
  const url = name ? css[name] : null;
  if (!url) return null;
  if (!sheets.has(url)) {
    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.media = "not all";
    link.href = url;
    sheets.set(url, { link, ready: new Promise((ok, no) => Object.assign(link, { onload: ok, onerror: no })) });
    document.head.append(link);
  }
  return sheets.get(url);
}

/* RENDER */

async function prepare(row, url) {
  const form = html.dataset.form;
  const [kind, text] = await Promise.all([page(row), source(row)]);
  const layout = kind.layout ?? {};
  const sheet = wear(layout.sheet);
  const host = document.createElement(layout.bare ? "div" : "article");
  if (!layout.bare) host.className = "prose";
  const side = document.createElement("div");
  await kind.render(row, site, host, { text, rows, url, right: side });
  await sheet?.ready;
  return { row, layout, kind, host, sheet, side, form };
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
  meta('meta[name="description"]')?.setAttribute("content", row.lead || site.tagline);
  const canonical = meta('link[rel="canonical"]', () =>
    Object.assign(document.createElement("link"), { rel: "canonical" }),
  );
  canonical.href = site.root + (row.meta?.deep ? url.pathname : row.route);
  const robots = meta('meta[name="robots"]');
  const lost = row.meta?.id === "missing";
  if (lost && !robots)
    document.head.append(Object.assign(document.createElement("meta"), { name: "robots", content: "noindex" }));
  if (!lost) robots?.remove();
}

/* WINDOW */

const kind = (layout) => layout.window ?? (layout.wide ? "wide" : "page");

function title(row) {
  const name = bar()?.querySelector(".name");
  if (name) name.textContent = row.title;
}

function dress(next) {
  const { layout, side, row } = next;
  const shape = kind(layout);
  const box = pane();
  html.dataset.window = shape;
  if (box) {
    box.setAttribute("aria-label", row.title);
    if (shape === "page" || shape === "wide") box.dataset.lens = "window";
    else delete box.dataset.lens;
  }
  title(row);
  const app = document.querySelector(".trio .app");
  if (app) app.textContent = shape === "app" ? row.title : "";
  main.className = layout.wide ? "wide" : "";
  const fill = side.childNodes.length > 0;
  drawer.replaceChildren(...(fill ? [side] : []));
  drawer.hidden = false;
  if (fill && !drawer.isConnected) main.after(drawer);
  if (!fill) drawer.remove();
  const button = document.querySelector('.title .actions [data-pane="right"]');
  if (button) button.hidden = !fill;
  html.dataset.right = "shut";
  for (const { link } of sheets.values()) link.media = link === next.sheet?.link ? "" : "not all";
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
  if (place(location) === shown) history.replaceState({ ...state(), y: Math.round(main.scrollTop) }, "");
}

function swap(next, url, mode, y) {
  if (mode === "push") pin();
  leave();
  const depth = state().depth ?? 0;
  if (mode === "push") history.pushState({ depth: depth + 1 }, "", url.href);
  if (mode === "replace") history.replaceState({ depth }, "", url.href);
  head(next.row, url);
  dress(next);
  main.replaceChildren(next.host);
  current = next;
  here = key(location);
  shown = place(location);
  const spot = mode === "pop" ? null : target(url.hash);
  if (mode === "boot") return spot?.scrollIntoView();
  if (spot) spot.scrollIntoView();
  else roll(y);
  main.focus({ preventScroll: true });
}

async function settle(next, url, mine) {
  await next.kind.mount?.(next.host, { rows, site, url });
  if (mine !== turn) return;
  if (next.host.querySelector("figure[data-figure]")) {
    const one = await figures();
    if (mine === turn) one.scan(next.host, Boolean(next.host.querySelector("[data-eager]")));
  }
  window.dispatchEvent(new CustomEvent("wire", { detail: { route: next.row.route, doors: site.doors } }));
}

/* FAIL */

function fail(url, error) {
  console.error(error);
  let tried = "";
  try {
    tried = sessionStorage.getItem(ONCE) ?? "";
    if (tried !== url.href) sessionStorage.setItem(ONCE, url.href);
  } catch {}
  if (tried !== url.href) return url.href === location.href ? location.reload() : location.assign(url.href);
  try {
    sessionStorage.removeItem(ONCE);
  } catch {}
  leave();
  html.dataset.window ??= "page";
  const lede = document.createElement("div");
  lede.className = "lede";
  lede.innerHTML =
    '<h1 id="error">This page did not load</h1><p class="lead">The network or the site failed twice. <a href="">Reload</a> to try again, or go <a href="/">home</a>.</p>';
  main.replaceChildren(lede);
}

/* GO */

function morph(run, still) {
  if (still || !document.startViewTransition) return run();
  const made = document.startViewTransition(run);
  made.ready.catch(() => {});
  made.finished.catch(() => {});
  return made.updateCallbackDone;
}

function moved(url) {
  const to = site.redirects?.[url.pathname]?.to;
  if (!to) return url;
  const next = new URL(to, url);
  for (const [key, value] of url.searchParams) next.searchParams.append(key, value);
  next.hash = url.hash;
  return next;
}

export async function go(asked, how = "push", y = 0, still = calm()) {
  const url = moved(asked);
  const mode = url !== asked && how === "boot" ? "replace" : how;
  const mine = ++turn;
  html.setAttribute("aria-busy", "true");
  const row = match(rows, url.pathname);
  try {
    const next = await prepare(row, url);
    if (mine !== turn) return next.kind.unmount(next.host);
    await morph(() => swap(next, url, mode, y), still || mode === "boot");
    try {
      sessionStorage.removeItem(ONCE);
    } catch {}
    await settle(next, url, mine);
    if (mode === "pop" && y && mine === turn) {
      await frame();
      await frame();
      if (mine === turn) roll(y);
    }
  } catch (error) {
    if (mine === turn) fail(url, error);
  }
  if (mine !== turn) return;
  html.removeAttribute("aria-busy");
  await reform();
}

/* FORM */

async function reform() {
  const row = current?.row;
  if (row?.kind !== "app" || current.form === html.dataset.form || html.getAttribute("aria-busy") === "true") return;
  const one = await SHELLS.app().catch(() => null);
  if (one?.gated(row) && current?.row === row && current.form !== html.dataset.form)
    await go(new URL(location.href), "replace", Math.round(main.scrollTop));
}

/* EVENTS */

function aim(event) {
  const a = event.target instanceof Element ? event.target.closest("a[href]") : null;
  return a instanceof HTMLAnchorElement && !a.target && !a.hasAttribute("download") ? a : null;
}

function click(event) {
  if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey)
    return;
  const a = aim(event);
  if (!a) return;
  const kind = sort(a, location, DEEP);
  if (kind === "mark") here = key(location);
  if (kind !== "page") return;
  event.preventDefault();
  go(new URL(a.href), a.href === location.href ? "replace" : "push");
}

function hint(event) {
  const a = aim(event);
  if (!a || sort(a, location, DEEP) !== "page") return;
  const row = match(rows, a.pathname);
  page(row).catch(() => {});
  source(row).catch(() => {});
}

function pop(event) {
  const y = typeof state().y === "number" ? state().y : null;
  if (key(location) !== here) return go(new URL(location.href), "pop", y ?? 0, event.hasUAVisualTransition || calm());
  turn++;
  html.removeAttribute("aria-busy");
  if (walled(location.pathname, DEEP)) return;
  const spot = y === null ? target(location.hash) : null;
  if (spot) spot.scrollIntoView();
  else roll(y ?? 0);
}

function back(event) {
  if (event.persisted) html.removeAttribute("aria-busy");
}

function note() {
  if (html.getAttribute("aria-busy") === "true" || place(location) !== shown) return;
  try {
    history.replaceState({ ...state(), y: Math.round(main.scrollTop) }, "");
  } catch {}
}

/* BOOT */

function boot() {
  drawer.remove();
  document.addEventListener("click", click);
  for (const type of ["pointerenter", "focus", "pointerdown"]) document.addEventListener(type, hint, true);
  addEventListener("popstate", pop);
  addEventListener("go", (e) => go(new URL(e.detail, location.href)));
  main.addEventListener("scrollend", note);
  addEventListener("pagehide", note);
  addEventListener("pageshow", back);
  new MutationObserver(reform).observe(html, { attributeFilter: ["data-form"] });
  document.addEventListener("visibilitychange", () => document.hidden && note());
  go(new URL(location.href), "boot");
}

boot();
