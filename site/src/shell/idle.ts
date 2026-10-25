import type { Spot } from "./lens.ts";
import type { Lock } from "./lock.ts";
import { calm, KEY, minutes, read, root, stamp, subscribe } from "./prefs.ts";

/* TYPES */

export type Woke = Pick<Event, "type"> & Partial<Pick<PointerEvent, "clientX" | "clientY">>;

type Dial = { node: HTMLDivElement; stop: () => void };

type Live = { canvas: HTMLCanvasElement; run: Lock; timer?: ReturnType<typeof setTimeout> };

type Kit = {
  on: () => string[];
  shuffle: (ids: readonly string[]) => string[];
  lock: typeof import("./lock.ts").lock;
};

type Base = { gone: boolean; only: string };

type Waiting = Base & { box: null };

type Up = {
  box: HTMLDivElement;
  face: Dial | null;
  pool: string[];
  at: number;
  id: string;
  miss: number;
  live: Live | null;
  old: Live[];
  cycle: ReturnType<typeof setTimeout> | 0;
  from: Spot | null;
  since: number;
  kit: Kit;
  ears: AbortController;
};

type Running = Base & Up;

type Mine = Waiting | Running;

/* WAKE */

const DEAF = 1000;
const SLACK = 8;

export function wakes(e: Woke, age: number, from: Spot | null) {
  if (age < DEAF) return false;
  if (e.type !== "pointermove") return true;
  return from !== null && Math.hypot(e.clientX! - from.x, e.clientY! - from.y) > SLACK;
}

/* TIMER */

const MINUTE = 60000;
const INPUT = ["pointermove", "pointerdown", "keydown", "wheel", "touchstart", "scroll"];

let watching = false;
let last = 0;
let timer: ReturnType<typeof setTimeout> | 0 = 0;
let shown: Mine | null = null;

const poke = () => {
  last = performance.now();
};

const free = () =>
  !document.hidden &&
  root().dataset.welcome !== "open" &&
  !("zen" in root().dataset) &&
  !document.querySelector("dialog[open]");

function arm() {
  clearTimeout(timer);
  timer = 0;
  const span = minutes() * MINUTE;
  if (!watching || !span || shown) return;
  timer = setTimeout(due, Math.max(0, last + span - performance.now()));
}

async function due() {
  timer = 0;
  const span = minutes() * MINUTE;
  if (!span || shown) return;
  if (performance.now() - last < span) return arm();
  if (free() && (await start())) return;
  poke();
  arm();
}

export function watch() {
  if (watching) return;
  watching = true;
  for (const type of INPUT) addEventListener(type, poke, { capture: true, passive: true });
  document.addEventListener("visibilitychange", poke);
  subscribe((key) => {
    if (key === null || key === KEY.idle) arm();
  });
  poke();
  arm();
}

/* CLOCK */

const two = (n: number) => String(n).padStart(2, "0");

function dial(): Dial {
  const node = document.createElement("div");
  node.className = "clock";
  const date = document.createElement("p");
  const time = document.createElement("time");
  node.append(date, time);
  let tick: ReturnType<typeof setTimeout> | 0 = 0;
  const paint = () => {
    clearTimeout(tick);
    const now = new Date();
    date.textContent = now.toLocaleDateString(undefined, { weekday: "long", day: "numeric", month: "long" });
    time.textContent = stamp(now, read(KEY.hours), true);
    time.dateTime = `${two(now.getHours())}:${two(now.getMinutes())}`;
    tick = setTimeout(paint, MINUTE - (now.getSeconds() * 1000 + now.getMilliseconds()) + 50);
  };
  paint();
  const deaf = subscribe((key) => (key === null || key === KEY.hours) && paint());
  return {
    node,
    stop: () => {
      clearTimeout(tick);
      deaf();
    },
  };
}

/* SAVER */

const TURN = 90000;
const FADE = 1600;
const OUT = 300;
const WAKE = ["keydown", "pointerdown", "pointermove", "wheel", "touchstart", "click", "contextmenu"];

const show = (node: HTMLElement) => {
  node.getBoundingClientRect();
  node.classList.add("on");
};

function shut() {
  try {
    for (const one of document.querySelectorAll<HTMLElement>(":popover-open")) one.hidePopover();
  } catch {}
}

function hear(mine: Running, e: Event & Partial<Pick<PointerEvent, "clientX" | "clientY">>) {
  const age = performance.now() - mine.since;
  if (e.type === "pointermove") {
    if (mine.gone) return;
    if (wakes(e, age, mine.from)) return stop();
    if (age < DEAF || !mine.from) mine.from = { x: e.clientX!, y: e.clientY! };
    return;
  }
  e.preventDefault();
  e.stopPropagation();
  if (!mine.gone && wakes(e, age, mine.from)) stop();
}

function deal(mine: Running) {
  const fresh = mine.kit.shuffle(mine.kit.on());
  if (!fresh.length) return mine.pool;
  if (fresh.length > 1 && fresh[0] === mine.id) fresh.push(fresh.shift()!);
  return fresh;
}

function retire(mine: Running, old: Live) {
  const drop = () => {
    mine.old = mine.old.filter((one) => one !== old);
    old.run();
    old.canvas.remove();
  };
  if (calm()) return drop();
  mine.old.push(old);
  old.timer = setTimeout(drop, FADE);
}

async function next(mine: Running) {
  clearTimeout(mine.cycle);
  if (mine.gone) return;
  if (document.hidden) {
    mine.cycle = setTimeout(() => next(mine), TURN);
    return;
  }
  if (mine.at >= mine.pool.length) {
    mine.pool = deal(mine);
    mine.at = 0;
  }
  const id = mine.pool[mine.at];
  mine.at += 1;
  const canvas = document.createElement("canvas");
  mine.box.insertBefore(canvas, mine.face?.node ?? null);
  const run = await mine.kit.lock(canvas, id, { pool: mine.pool }).catch(() => null);
  if (mine.gone) {
    run?.();
    canvas.remove();
    return;
  }
  if (!run?.id) {
    run?.();
    canvas.remove();
    mine.miss += 1;
    if (mine.miss >= mine.pool.length) return stop();
    return next(mine);
  }
  mine.miss = 0;
  mine.id = run.id;
  const old = mine.live;
  mine.live = { canvas, run };
  show(canvas);
  if (old) retire(mine, old);
  if (!mine.only) mine.cycle = setTimeout(() => next(mine), TURN);
}

export async function start({ only }: { only?: string } = {}) {
  if (shown || document.hidden) return false;
  const mine: Waiting = { gone: false, only: only || "", box: null };
  shown = mine;
  clearTimeout(timer);
  const [{ on, shuffle }, { lock }] = await Promise.all([import("./savers.ts"), import("./lock.ts")]);
  const pool = only ? [only] : shuffle(on());
  if (mine.gone || shown !== mine || !pool.length || document.hidden) {
    if (shown === mine) shown = null;
    if (!mine.gone) {
      poke();
      arm();
    }
    return false;
  }
  const box = document.createElement("div");
  box.className = "saver";
  box.setAttribute("aria-hidden", "true");
  const face = read(KEY.clock) === "off" ? null : dial();
  if (face) box.append(face.node);
  const up = Object.assign<Base, Up>(mine, {
    box,
    face,
    pool,
    at: 0,
    id: "",
    miss: 0,
    live: null,
    old: [],
    cycle: 0,
    from: null,
    since: performance.now(),
    kit: { on, shuffle, lock },
    ears: new AbortController(),
  });
  for (const type of WAKE)
    addEventListener(type, (e) => hear(up, e), { capture: true, passive: false, signal: up.ears.signal });
  shut();
  document.body.append(box);
  root().dataset.saver = "";
  show(box);
  dispatchEvent(new CustomEvent("saver", { detail: true }));
  next(up);
  return true;
}

export function stop() {
  const mine = shown;
  if (!mine) return;
  shown = null;
  mine.gone = true;
  poke();
  arm();
  if (!mine.box) return;
  clearTimeout(mine.cycle);
  mine.face?.stop();
  delete root().dataset.saver;
  dispatchEvent(new CustomEvent("saver", { detail: false }));
  const done = () => {
    mine.ears.abort();
    for (const one of [mine.live, ...mine.old]) {
      if (!one) continue;
      clearTimeout(one.timer);
      one.run();
    }
    mine.box.remove();
  };
  mine.box.classList.add("out");
  mine.box.classList.remove("on");
  if (calm()) return done();
  setTimeout(done, OUT);
}
