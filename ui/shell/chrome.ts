import "./mount.ts";
import { state } from "../kit/history.ts";
import { seek, track, type } from "./catalog.ts";
import { canvas, desk, greeting, listen, meet, route, visit } from "./desk.ts";
import { menus } from "./menus.ts";
import { calm, icons, KEY, magnify, PREFIX, read, root, screen, shade, shine, theme, tint } from "./prefs.ts";
import type { Wired } from "./route.ts";
import { expand, turns } from "./tree.ts";

type Piece = { start: () => void };

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

const pieces: Record<"bar" | "dock" | "widgets" | "live", Promise<void> | null> = {
  bar: null,
  dock: null,
  widgets: null,
  live: null,
};

function lay(name: keyof typeof pieces, load: () => Promise<Piece>) {
  pieces[name] ??= load()
    .then((one) => one.start())
    .catch((error) => {
      pieces[name] = null;
      console.error(error);
    });
}

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
  lay("live", () => import("./live.tsx"));
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
  matchMedia("(prefers-color-scheme: dark)").addEventListener("change", shade);
  track();
  menus();
  document.addEventListener("toggle", expand, true);
  document.addEventListener("keydown", (e) => {
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
