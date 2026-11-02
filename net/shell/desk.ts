import { lock as saver } from "site:apps";
import { source } from "../sdk/feed.ts";
import type { Lock } from "./lock.ts";
import { calm, KEY, read, root, screen, write } from "./prefs.ts";
import type { Wired } from "./route.ts";

/* TYPES */

type Shown = { box: HTMLDivElement; stop: Lock | null; want: string; drop?: () => void };

type Pinned = { pick: string; as: string };

type Path = (t: number, height: number) => number;

type Drawn = typeof import("./parts.tsx");

/* MARK */

export let route = "";
let homed: boolean | null = null;

export function visit(detail: Wired) {
  ({ route } = detail);
}

export function mark() {
  const home = route === "/";
  if (!route || home === homed) return;
  homed = home;
  if (!document.querySelector(".bar .mark")) return;
  if (home) return arrive();
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
let watch: AbortController | null = null;
let owed = false;
let tapped = 0;

const hall = () => document.querySelector<HTMLElement>(".welcome");
export const greeting = () => root().dataset.welcome;
const jump = (top: number) => scrollTo({ top, behavior: "instant" });
const rolls = () => CSS.supports("animation-timeline", "scroll()");

function glide(path: Path, time: number, done: () => void) {
  playing = true;
  const span = calm() ? 0 : time;
  const start = performance.now();
  const step = (now: number) => {
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
  const run = () => {
    const h = hall()?.offsetHeight ?? 0;
    if (!h || scrollY >= h) return settle();
    const from = scrollY / h;
    glide((t, height) => height * (from + (1 - from) * t), PLAY * (1 - from), settle);
  };
  const go = () => {
    if (shown !== mine || playing || greeting() !== "open") return;
    const left = leave();
    mine?.stop?.coast?.(SETTLE);
    if (!left) return run();
    playing = true;
    left.then(() => {
      playing = false;
      if (shown === mine && greeting() === "open") run();
    });
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
  jump(at + hall()!.offsetHeight);
  setTimeout(
    () => {
      jump(hall()!.offsetHeight);
      glide(
        (t, height) => height * (1 - t),
        PLAY,
        () => (asked ? enter() : lid()),
      );
    },
    calm() ? 0 : FADE,
  );
}

function hold(e: Event) {
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
    if (Math.abs((e as WheelEvent).deltaX) > Math.abs((e as WheelEvent).deltaY)) return;
    e.preventDefault();
    return enter();
  }
  if (e.type === "touchmove") {
    if ((e as TouchEvent).touches.length > 1) return;
    e.preventDefault();
    return enter();
  }
  if (e.type === "keydown") {
    const press = e as KeyboardEvent;
    if (press.metaKey || press.ctrlKey || press.altKey || !PAGING.includes(press.key)) return;
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

export function knock(mark: HTMLElement) {
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

async function arrive() {
  const one = await import("./parts.tsx").catch(() => null);
  if (!homed) return;
  drawn = one?.locks() ? one : null;
  const due = owed;
  owed = false;
  if (!drawn) return void delete root().dataset.welcome;
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

let shown: Shown | null = null;
let pinned: Pinned | null = null;
let nap: ReturnType<typeof setTimeout> | 0 = 0;
let later: ReturnType<typeof setTimeout> | 0 = 0;
let idle = false;
let drawn: Drawn | null = null;

export const canvas = () => shown?.box.querySelector("canvas") ?? null;

const ground = (box: HTMLElement) =>
  import("./parts.tsx").then((one) => one.ground(box, route === "/")).catch(() => null);

export async function desk() {
  if (!idle) {
    idle = true;
    import("./idle.ts").then((one) => one.watch()).catch(() => {});
  }
  if (shown?.want === read(KEY.saver)) return void ground(shown.box);
  if (shown) close();
  const box = document.createElement("div");
  box.className = "desk";
  box.setAttribute("aria-hidden", "true");
  const mine: Shown = { box, stop: null, want: read(KEY.saver) };
  shown = mine;
  document.body.append(box);
  const surface = await ground(box);
  if (shown !== mine || !surface) return;
  const want = read(KEY.saver);
  const use = pinned?.pick === want ? pinned.as : want;
  const stop = await saver()
    .then(({ lock }) => lock(surface, use))
    .catch(() => null);
  if (shown !== mine) return stop?.();
  if (!stop) return close();
  mine.drop = source(surface);
  mine.stop = stop;
  mine.want = use === want && stop.pick !== want ? stop.pick : want;
  pinned = { pick: mine.want, as: stop.as };
  if (mine.want !== want) screen(mine.want);
  box.classList.add("on");
  if (greeting() === "open") return lid();
  later = setTimeout(() => stop.coast?.(SETTLE), HOLD);
}

function face(state: "wake" | "rest") {
  root().dataset.lock = state;
  const host = hall();
  if (host) drawn?.seal(host, state);
}

function lid() {
  if (!drawn || !shown?.stop || playing || greeting() !== "open") return;
  face("wake");
  doze();
}

function doze() {
  clearTimeout(nap);
  nap = setTimeout(() => {
    if (root().dataset.lock) face("rest");
  }, IDLE);
}

function stir(e: Event & Partial<Pick<PointerEvent, "pointerType">>) {
  if (!root().dataset.lock || e.pointerType === "touch") return;
  face("wake");
  doze();
}

function leave(): Promise<unknown> | null {
  const up = root().dataset.lock;
  clearTimeout(nap);
  delete root().dataset.lock;
  const host = hall();
  const kit = drawn;
  if (!up || !host || !kit) return null;
  const left = kit.ending(host, () => kit.seal(host, "leaving"));
  return calm() ? null : left;
}

function unlock() {
  clearTimeout(nap);
  delete root().dataset.lock;
  const host = hall();
  if (host) drawn?.hide(host);
}

function close() {
  unlock();
  clearTimeout(later);
  shown?.stop?.();
  shown?.drop?.();
  const box = shown?.box;
  if (box) import("./parts.tsx").then((one) => one.hide(box)).catch(() => {});
  box?.remove();
  shown = null;
}
