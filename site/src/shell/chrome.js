import { handle } from "../lib/handle.js";
import { track, type } from "./catalog.js";
import { desk, greeting, knock, listen, mark, route, visit } from "./desk.js";
import { menus } from "./menus.js";
import {
  face,
  glass,
  icons,
  KEY,
  magnify,
  PREFIX,
  read,
  root,
  screen,
  shade,
  stamp,
  subscribe,
  theme,
  tint,
  turn,
  write,
} from "./prefs.js";
import { expand, turns } from "./tree.js";

const EDGES = ["left", "right"];

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
  const kind = performance.getEntriesByType("navigation")[0]?.type;
  return kind === "back_forward" || kind === "reload";
}

function land() {
  const at = saved() || Number(history.state?.y) || 0;
  if (at > 0 && resumed() && !location.hash) main()?.scrollTo({ top: at, behavior: "instant" });
}

/* DRAWERS */

const docked = () => {
  const pane = document.querySelector(".panes > .pane");
  return Boolean(pane) && getComputedStyle(pane).position === "static";
};
const isOpen = (side) => root().dataset[side] === "open";

function set(side, open) {
  root().dataset[side] = open ? "open" : "shut";
  sync();
}

function stow() {
  set("right", false);
}

function watch() {
  const box = document.querySelector(".window");
  if (!box || typeof ResizeObserver !== "function") return;
  let was = docked();
  new ResizeObserver(() => {
    const now = docked();
    if (now !== was) stow();
    was = now;
  }).observe(box);
}

function sync() {
  for (const button of document.querySelectorAll("[data-pane]"))
    button.setAttribute("aria-expanded", String(isOpen(button.dataset.pane)));
}

function toggle(side) {
  const open = !isOpen(side);
  set(side, open);
  if (open) document.getElementById(side)?.querySelector("a, button, input, select, textarea")?.focus();
}

function shut() {
  if (docked()) return false;
  if (!isOpen("right")) return false;
  set("right", false);
  document.querySelector('[data-pane="right"]')?.focus();
  return true;
}

/* CART */

function count() {
  try {
    const items = JSON.parse(read(KEY.cart) || "[]");
    return Array.isArray(items) ? items.reduce((sum, item) => sum + (Number(item.qty) || 1), 0) : 0;
  } catch {
    return 0;
  }
}

function cart() {
  const n = count();
  for (const link of document.querySelectorAll("[data-cart]")) {
    link.setAttribute("aria-label", n ? `Cart, ${n} item${n === 1 ? "" : "s"}` : "Cart");
    link.querySelectorAll(".dot").forEach((dot, i) => {
      dot.classList.toggle("on", i < n);
    });
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
  nav.querySelector("details")?.setAttribute("open", "");
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
        if (a.hash.slice(1) === hit.id) a.setAttribute("aria-current", "location");
        else a.removeAttribute("aria-current");
      }
    },
    { root: main(), rootMargin: "0px 0px -60% 0px" },
  );
  for (const t of targets) eye.observe(t);
  eyes.set(nav, eye);
}

/* CLOCK */

let ticking = 0;

const two = (n) => String(n).padStart(2, "0");

function tick() {
  const now = new Date();
  const date = document.querySelector(".tray .date");
  const clock = document.querySelector(".tray .clock");
  if (date) {
    date.textContent = now.toLocaleDateString(undefined, { weekday: "short", day: "numeric", month: "short" });
    date.dateTime = `${now.getFullYear()}-${two(now.getMonth() + 1)}-${two(now.getDate())}`;
  }
  if (clock) {
    clock.textContent = stamp(now, read(KEY.hours));
    clock.dateTime = `${two(now.getHours())}:${two(now.getMinutes())}`;
  }
  clearTimeout(ticking);
  ticking = setTimeout(tick, 60000 - (now.getSeconds() * 1000 + now.getMilliseconds()) + 50);
}

/* WINDOW */

const WIDTHS = { page: ["--win", KEY.page, 720], wide: ["--wide", KEY.wide, 1280] };
const LEAST = 352;

function edges() {
  const box = document.querySelector(".window");
  if (!box || box.querySelector(":scope > .edge")) return;
  const kind = () => WIDTHS[root().dataset.window] ?? WIDTHS.page;
  const room = () => innerWidth - 2 * (Number.parseFloat(getComputedStyle(root()).fontSize) || 16) * 0.75;
  const get = () => box.getBoundingClientRect().width;
  const put = (px) => root().style.setProperty(kind()[0], `${Math.round(px)}px`);
  for (const side of EDGES) {
    const grip = document.createElement("div");
    grip.className = "edge";
    grip.dataset.side = side;
    box.append(grip);
    handle(grip, {
      get,
      set: put,
      done: (px) => write(kind()[1], px >= room() - 1 ? "" : String(Math.round(px))),
      reset: () => kind()[2],
      min: LEAST,
      max: room,
      rate: side === "left" ? -2 : 2,
      label: "Window width",
      busy: (on) => box.classList.toggle("drag", on),
    });
  }
}

function zoom(button) {
  const box = button.closest(".window");
  const on = !box.classList.contains("zoomed");
  box.classList.toggle("zoomed", on);
  button.setAttribute("aria-pressed", String(on));
}

function close(e) {
  if (e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
  if (!(history.state?.depth > 0)) return;
  e.preventDefault();
  history.back();
}

/* TYPE */

let typing = false;

function ahead(e) {
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

const pieces = { dock: null, widgets: null };

function lay(name, load) {
  pieces[name] ??= load()
    .then((one) => one.start())
    .catch((error) => {
      pieces[name] = null;
      console.error(error);
    });
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
let lit = false;

function wire(event) {
  if (event?.detail?.route) visit(event.detail);
  typing = false;
  if (!landed && route) {
    landed = true;
    land();
  }
  sync();
  unread();
  theme(root().dataset.theme ?? "");
  face(root().dataset.font ?? "");
  tint(root().dataset.tint ?? "");
  glass(read(KEY.glass));
  icons(read(KEY.icons), read(KEY.always) === "1");
  screen(read(KEY.saver));
  magnify(read(KEY.magnify) === "1");
  cart();
  once(".contents", contents);
  once(".tree summary", turns);
  once(".bar .mark", knock);
  if (greeting() === "open") listen();
  if (route === "/") root().dataset.desk = "";
  else delete root().dataset.desk;
  desk();
  mark();
  lay("dock", () => import("./dock.jsx"));
  if (route === "/") lay("widgets", () => import("./widgets.jsx"));
  if (!lit) {
    lit = true;
    import("../lib/glass/index.js").then((one) => one.start()).catch((error) => console.error(error));
  }
}

function boot() {
  root().classList.add("js");
  if ("scrollRestoration" in history) history.scrollRestoration = "manual";
  theme(read(KEY.theme));
  face(read(KEY.font));
  tint(read(KEY.tint));
  glass(read(KEY.glass));
  icons(read(KEY.icons), read(KEY.always) === "1");
  screen(read(KEY.saver));
  magnify(read(KEY.magnify) === "1");
  matchMedia("(prefers-color-scheme: dark)").addEventListener("change", shade);
  track();
  menus();
  stow();
  tick();
  subscribe((key) => (key === null || key === KEY.hours) && tick());
  edges();
  watch();
  document.addEventListener("click", (e) => {
    const target = e.target instanceof Element ? e.target : null;
    if (!target) return;
    const button = target.closest("[data-pane]");
    if (button) return button.closest(".pane") ? shut() : toggle(button.dataset.pane);
    if (target.closest("[data-theme-toggle]")) return turn();
    if (target.closest(".window > .title .close")) return close(e);
    const zoomer = target.closest(".window > .title .zoom");
    if (zoomer) return zoom(zoomer);
    if (target.closest(".scrim") || target.closest(".pane a[href]")) shut();
  });
  document.addEventListener("toggle", expand, true);
  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape" && !document.querySelector("dialog[open]") && shut()) return e.preventDefault();
    if ((e.metaKey || e.ctrlKey) && !e.altKey && !e.shiftKey && e.key.toLowerCase() === "k") {
      e.preventDefault();
      const field = location.pathname === "/menu/" ? document.querySelector(".launchpad .find input") : null;
      if (field) {
        field.focus();
        field.select();
        return;
      }
      document.querySelector(".tray .find")?.click();
      return;
    }
    ahead(e);
  });
  document.addEventListener("visibilitychange", () => !document.hidden && tick());
  window.addEventListener("wire", wire);
  window.addEventListener("cart", cart);
  window.addEventListener("storage", cart);
  window.addEventListener("pageshow", cart);
  window.addEventListener("pagehide", keep);
  cart();
}

if (typeof document !== "undefined") boot();
