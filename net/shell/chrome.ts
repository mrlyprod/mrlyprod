import "./mount.ts";
import { handle } from "../sdk/handle.ts";
import { state } from "../sdk/history.ts";
import { px } from "../sdk/px.ts";
import { seek, track, type } from "./catalog.ts";
import { canvas, desk, greeting, listen, meet, route, visit } from "./desk.ts";
import { measure } from "./measure.ts";
import { menus } from "./menus.ts";
import {
  calm,
  icons,
  KEY,
  magnify,
  PREFIX,
  read,
  root,
  screen,
  shade,
  shine,
  theme,
  tint,
  turn,
  widths,
  write,
} from "./prefs.ts";
import type { Wired } from "./route.ts";
import { expand, turns } from "./tree.ts";

/* TYPES */

type Width = [string, string, number];

type Piece = { start: () => void };

/* SCROLL */

const main = () => document.getElementById("main");

const spot = () => `${PREFIX}spot:${location.href}`;

function keep() {
  if (root().getAttribute("aria-busy") === "true") return;
  try {
    sessionStorage.setItem(spot(), String(main()?.scrollTop ?? 0));
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
  const kind = (performance.getEntriesByType("navigation") as PerformanceNavigationTiming[])[0]?.type;
  return kind === "back_forward" || kind === "reload";
}

function land() {
  const at = saved() || state().y || 0;
  if (at > 0 && resumed() && !location.hash) main()?.scrollTo({ top: at, behavior: "instant" });
}

/* DRAWERS */

const rem = () => (Number.parseFloat(getComputedStyle(root()).fontSize) || 16) / 16;
const docked = () => root().hasAttribute("data-docked");
const isOpen = (side: string) => root().dataset[side] === "open";

function set(side: string, open: boolean) {
  root().dataset[side] = open ? "open" : "shut";
  sync();
}

function stow() {
  set("right", false);
}

function dock() {
  const box = document.querySelector(".window");
  const on =
    root().dataset.window === "wide" && Boolean(box) && box!.getBoundingClientRect().width >= measure.dock * rem();
  if (on === docked()) return;
  root().toggleAttribute("data-docked", on);
  stow();
}

function watch() {
  dock();
  new MutationObserver(dock).observe(root(), { attributeFilter: ["data-window", "data-text"] });
  const box = document.querySelector(".window");
  if (box && typeof ResizeObserver === "function") new ResizeObserver(dock).observe(box);
}

function sync() {
  for (const button of document.querySelectorAll<HTMLElement>("[data-pane]"))
    button.setAttribute("aria-expanded", String(isOpen(button.dataset.pane!)));
}

function toggle(side: string) {
  const open = !isOpen(side);
  set(side, open);
  if (open) document.getElementById(side)?.querySelector<HTMLElement>("a, button, input, select, textarea")?.focus();
}

function shut() {
  if (docked()) return false;
  if (!isOpen("right")) return false;
  set("right", false);
  document.querySelector<HTMLElement>('[data-pane="right"]')?.focus();
  return true;
}

/* CONTENTS */

const eyes = new Map<Element, IntersectionObserver>();

function unread() {
  for (const [nav, eye] of eyes) {
    if (nav.isConnected) continue;
    eye.disconnect();
    eyes.delete(nav);
  }
}

function contents(nav: Element) {
  nav.querySelector("details")?.setAttribute("open", "");
  const links = [...nav.querySelectorAll<HTMLAnchorElement>('a[href^="#"]')];
  const targets = links
    .map((a) => document.getElementById(decodeURIComponent(a.hash.slice(1))))
    .filter((t) => t !== null);
  if (!targets.length) return;
  const seen = new Map<Element, boolean>();
  const eye = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) seen.set(entry.target, entry.isIntersecting);
      const hit = targets.find((t) => seen.get(t));
      if (!hit) return;
      for (const a of links) {
        if (a.hash.slice(1) === hit.id) a.setAttribute("aria-current", "location");
        else a.removeAttribute("aria-current");
      }
    },
    { root: main(), rootMargin: "0% 0% -60% 0%" },
  );
  for (const t of targets) eye.observe(t);
  eyes.set(nav, eye);
}

/* WINDOW */

const WIDTHS: Record<"page" | "wide", Width> & Partial<Record<string, Width>> = {
  page: ["--win", KEY.page, measure.page],
  wide: ["--wide", KEY.wide, measure.wide],
};
const step = (n: number) => Math.round(n / measure.step) * measure.step;

function edges() {
  const box = document.querySelector<HTMLElement>(".window");
  if (!box) return;
  const kind = () => WIDTHS[root().dataset.window ?? ""] ?? WIDTHS.page;
  const room = () => innerWidth - 2 * measure.gutter * rem();
  const get = () => box.getBoundingClientRect().width;
  const put = (n: number) => root().style.setProperty(kind()[0], px(step(n)));
  for (const grip of document.querySelectorAll<HTMLElement>("[data-edge]")) {
    const side = grip.dataset.edge;
    handle(grip, {
      get,
      set: put,
      done: (n) => write(kind()[1], n >= room() - 1 ? "" : String(step(n))),
      reset: () => kind()[2],
      min: measure.least,
      max: room,
      rate: side === "left" ? -2 : 2,
      label: "Window width",
      busy: (on) => box.classList.toggle("drag", on),
    });
  }
}

function zoom(button: Element) {
  const box = document.querySelector(".window");
  if (!box) return;
  const on = !box.classList.contains("zoomed");
  box.classList.toggle("zoomed", on);
  button.setAttribute("aria-pressed", String(on));
}

function close(e: MouseEvent) {
  if (e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
  if (!((state().depth ?? 0) > 0)) return;
  e.preventDefault();
  history.back();
}

/* TYPE */

let typing = false;

function ahead(e: KeyboardEvent) {
  if (
    route !== "/" ||
    greeting() === "open" ||
    e.metaKey ||
    e.ctrlKey ||
    e.altKey ||
    e.key.length !== 1 ||
    !/\S/.test(e.key)
  )
    return;
  const at = e.target instanceof Element ? e.target : null;
  if (
    at?.closest("input, textarea, select, [contenteditable]") ||
    document.querySelector("dialog[open], :popover-open") ||
    root().hasAttribute("data-editing")
  )
    return;
  e.preventDefault();
  type(e.key);
  if (typing) return;
  typing = true;
  dispatchEvent(new CustomEvent("go", { detail: "/menu/" }));
}

/* PIECES */

const pieces: Record<"bar" | "dock" | "widgets", Promise<void> | null> = { bar: null, dock: null, widgets: null };

function lay(name: keyof typeof pieces, load: () => Promise<Piece>) {
  pieces[name] ??= load()
    .then((one) => one.start())
    .catch((error) => {
      pieces[name] = null;
      console.error(error);
    });
}

/* WIRE */

const wired = new WeakSet<Element>();

const once = (selector: string, fn: (el: HTMLElement) => void) => {
  for (const el of document.querySelectorAll<HTMLElement>(selector)) {
    if (wired.has(el)) continue;
    wired.add(el);
    fn(el);
  }
};

let landed = false;
let lit = false;

function wire(event?: CustomEvent<Wired>) {
  if (event?.detail?.route) visit(event.detail);
  typing = false;
  if (!landed && route) {
    landed = true;
    land();
  }
  sync();
  unread();
  theme(root().dataset.theme ?? "");
  tint(root().dataset.tint ?? "");
  shine();
  icons(read(KEY.icons), read(KEY.always) === "1");
  screen(read(KEY.saver));
  magnify(read(KEY.magnify) === "1");
  once(".contents", contents);
  once(".tree summary", turns);
  if (greeting() === "open") listen();
  if (route === "/") root().dataset.desk = "";
  else delete root().dataset.desk;
  desk();
  meet();
  lay("bar", () => import("./bar.tsx"));
  lay("dock", () => import("./dock.tsx"));
  if (route === "/") lay("widgets", () => import("./widgets.tsx"));
  if (!lit) {
    lit = true;
    import("./active.ts")
      .then(({ design }) => design.start?.({ root: root(), desk: canvas, calm }))
      .catch((error) => console.error(error));
  }
}

function boot() {
  root().classList.add("js");
  if ("scrollRestoration" in history) history.scrollRestoration = "manual";
  theme(read(KEY.theme));
  tint(read(KEY.tint));
  shine();
  icons(read(KEY.icons), read(KEY.always) === "1");
  screen(read(KEY.saver));
  magnify(read(KEY.magnify) === "1");
  widths();
  matchMedia("(prefers-color-scheme: dark)").addEventListener("change", shade);
  track();
  menus();
  stow();
  edges();
  watch();
  document.addEventListener("click", (e) => {
    const target = e.target instanceof Element ? e.target : null;
    if (!target) return;
    const button = target.closest<HTMLElement>("[data-pane]");
    if (button) return button.closest("#right") ? shut() : toggle(button.dataset.pane!);
    if (target.closest("[data-theme-toggle]")) return turn();
    if (target.closest("[data-close]")) return close(e);
    const zoomer = target.closest("[data-zoom]");
    if (zoomer) return zoom(zoomer);
    if (target.closest("[data-scrim]") || target.closest("#right a[href]")) shut();
  });
  document.addEventListener("toggle", expand, true);
  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape" && !document.querySelector("dialog[open]") && shut()) return e.preventDefault();
    if ((e.metaKey || e.ctrlKey) && !e.altKey && !e.shiftKey && e.key.toLowerCase() === "k") {
      e.preventDefault();
      seek();
      return;
    }
    ahead(e);
  });
  window.addEventListener("wire", wire);
  window.addEventListener("pagehide", keep);
  import("./world.ts").then(({ start }) => start()).catch((error) => console.error(error));
}

if (typeof document !== "undefined") boot();
