import { lock as saver } from "site:apps";
import { source } from "../lib/glass/feed.js";
import { calm, KEY, read, root, screen, write } from "./prefs.js";

/* MARK */

export let route = "";
let homed = null;

export function visit(detail) {
  ({ route } = detail);
}

export function mark() {
  const home = route === "/";
  if (!route || home === homed) return;
  homed = home;
  const one = document.querySelector(".bar .mark");
  if (!one) return;
  if (home) return arrive(one);
  depart();
}

/* WELCOME */

const PLAY = 2000;
const FADE = 250;
const QUIET = 200;
const PAGING = ["ArrowDown", "PageDown", " ", "End"];
const LIVE = "main a[href], main button, main input, main select, main textarea, main summary";

let playing = false;
let asked = false;
let quiet = 0;
let watch = null;
let owed = false;
let tapped = 0;

const hall = () => document.querySelector(".welcome");
export const greeting = () => root().dataset.welcome;
const jump = (top) => scrollTo({ top, behavior: "instant" });
const rolls = () => CSS.supports("animation-timeline", "scroll()");

function glide(path, time, done) {
  playing = true;
  const span = calm() ? 0 : time;
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

function hush() {
  if (greeting() === "open") return;
  if (performance.now() < quiet) return setTimeout(hush, QUIET);
  unlisten();
}

function settle() {
  unlock();
  shown?.stop?.coast?.(SETTLE);
  const top = Math.max(0, scrollY - (hall()?.offsetHeight ?? 0));
  write(KEY.welcome, "1");
  root().dataset.welcome = "shut";
  jump(top);
  quiet = performance.now() + QUIET;
  setTimeout(hush, QUIET);
}

function enter() {
  if (greeting() !== "open") return;
  asked = playing;
  if (playing) return;
  const mine = shown;
  const go = () => {
    if (shown !== mine || playing || greeting() !== "open") return;
    unlock();
    mine?.stop?.coast?.(SETTLE);
    const h = hall()?.offsetHeight ?? 0;
    if (!h || scrollY >= h) return settle();
    const from = scrollY / h;
    glide((t, height) => height * (from + (1 - from) * t), PLAY * (1 - from), settle);
  };
  if (mine?.stop?.leave && root().dataset.lock) return mine.stop.leave(go);
  go();
}

function greet() {
  if (playing || greeting() !== "shut" || !hall()) return;
  const at = scrollY;
  playing = true;
  asked = false;
  clearTimeout(later);
  shown?.stop?.play?.();
  root().dataset.welcome = "open";
  listen();
  jump(at + hall().offsetHeight);
  setTimeout(
    () => {
      jump(hall().offsetHeight);
      glide(
        (t, height) => height * (1 - t),
        PLAY,
        () => (asked ? enter() : lid()),
      );
    },
    calm() ? 0 : FADE,
  );
}

function hold(e) {
  const open = greeting() === "open";
  if (!open && !playing) {
    if (e.type === "wheel" && performance.now() < quiet) {
      e.preventDefault();
      quiet = performance.now() + QUIET;
    }
    return;
  }
  const target = e.target instanceof Element ? e.target : null;
  if (e.type === "wheel") {
    if (Math.abs(e.deltaX) > Math.abs(e.deltaY)) return;
    e.preventDefault();
    return enter();
  }
  if (e.type === "touchmove") {
    if (e.touches.length > 1) return;
    e.preventDefault();
    return enter();
  }
  if (e.type === "keydown") {
    if (e.metaKey || e.ctrlKey || e.altKey || !PAGING.includes(e.key)) return;
    e.preventDefault();
    return enter();
  }
  if (target?.closest(LIVE)) {
    if (open && !playing) settle();
    return;
  }
  if (e.type === "pointerdown") return enter();
  e.preventDefault();
}

function drift() {
  if (greeting() === "open" && !playing && scrollY > 0) enter();
}

export function listen() {
  if (watch) return;
  watch = new AbortController();
  const options = { passive: false, capture: true, signal: watch.signal };
  for (const type of ["wheel", "touchmove", "keydown", "pointerdown", "click"]) addEventListener(type, hold, options);
  addEventListener("scroll", drift, { passive: true, signal: watch.signal });
  addEventListener("pointermove", stir, { passive: true, signal: watch.signal });
}

function unlisten() {
  watch?.abort();
  watch = null;
}

export function knock(mark) {
  mark.addEventListener("click", (e) => {
    if (e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
    const now = performance.now();
    const twice = now - tapped < TWICE;
    tapped = twice ? 0 : now;
    if (route === "/" && greeting() === "shut" && hall()) {
      e.preventDefault();
      return greet();
    }
    if (!twice) return;
    e.preventDefault();
    if (!greeting()) owed = true;
  });
}

async function arrive(mark) {
  const { markup, welcome } = await import("./welcome.js");
  if (!homed) return;
  const model = welcome();
  const svg = mark.querySelector("svg");
  svg.setAttribute("class", "glyphs fold");
  svg.setAttribute("viewBox", `0 0 ${model.cols} 5`);
  svg.style.setProperty("--rows", 5);
  svg.style.setProperty("--cols", model.cols);
  svg.innerHTML = markup(model);
  const due = owed;
  owed = false;
  if (!rolls()) return;
  root().dataset.welcome = read(KEY.welcome) || calm() ? "shut" : "open";
  if (greeting() === "open") {
    clearTimeout(later);
    shown?.stop?.play?.();
    listen();
    lid();
  }
  if (due) greet();
}

function depart() {
  owed = false;
  unlock();
  delete root().dataset.welcome;
  unlisten();
}

/* DESK */

const IDLE = 3000;
const TWICE = 400;
const HOLD = 3000;
const SETTLE = 3000;

let shown = null;
let pinned = null;
let nap = 0;
let later = 0;
let idle = false;

export async function desk() {
  if (!idle) {
    idle = true;
    import("./idle.js").then((one) => one.watch()).catch(() => {});
  }
  if (shown?.want === read(KEY.saver)) return;
  if (shown) close();
  const box = document.createElement("div");
  box.className = "desk";
  box.setAttribute("aria-hidden", "true");
  box.append(document.createElement("canvas"));
  const mine = { box, stop: null, want: read(KEY.saver) };
  shown = mine;
  document.body.append(box);
  const want = read(KEY.saver);
  const use = pinned?.pick === want ? pinned.as : want;
  const stop = await saver()
    .then(({ lock }) => lock(box.firstChild, use))
    .catch(() => null);
  if (shown !== mine) return stop?.();
  if (!stop) return close();
  mine.drop = source(box.firstChild);
  mine.stop = stop;
  mine.want = use === want && stop.pick !== want ? stop.pick : want;
  pinned = { pick: mine.want, as: stop.as };
  if (mine.want !== want) screen(mine.want);
  box.classList.add("on");
  if (greeting() === "open") return lid();
  later = setTimeout(() => stop.coast?.(SETTLE), HOLD);
}

function lid() {
  if (!shown?.stop || playing || greeting() !== "open") return;
  root().dataset.lock = "wake";
  doze();
}

function doze() {
  clearTimeout(nap);
  nap = setTimeout(() => {
    if (root().dataset.lock) root().dataset.lock = "rest";
  }, IDLE);
}

function stir(e) {
  if (!root().dataset.lock || e.pointerType === "touch") return;
  root().dataset.lock = "wake";
  doze();
}

function unlock() {
  clearTimeout(nap);
  delete root().dataset.lock;
}

function close() {
  unlock();
  clearTimeout(later);
  shown?.stop?.();
  shown?.drop?.();
  shown?.box.remove();
  shown = null;
}
