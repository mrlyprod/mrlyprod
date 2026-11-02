import type { Props } from "./design.ts";
import type { Spot } from "./lens.ts";
import type { Lock } from "./lock.ts";
import { ending, hide, saves, veil } from "./parts.tsx";
import { calm, cycle, KEY, minutes, read, root, subscribe } from "./prefs.ts";

/* TYPES */

export type Woke = Pick<Event, "type"> & Partial<Pick<PointerEvent, "clientX" | "clientY">>;

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
  stage: HTMLElement | null;
  bind: () => { ref: (node: HTMLElement | null) => void };
  face: boolean;
  now: Date;
  state: Props["saver"]["state"];
  tick: ReturnType<typeof setTimeout> | 0;
  deaf: () => void;
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
  if (watching || !saves()) return;
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

function draw(mine: Running) {
  veil(mine.box, {
    clock: mine.face ? { now: mine.now, hours: cycle() } : null,
    state: mine.state,
    stage: mine.bind,
  });
}

function tick(mine: Running) {
  clearTimeout(mine.tick);
  if (mine.gone) return;
  mine.now = new Date();
  draw(mine);
  if (!mine.face) return;
  const now = mine.now;
  mine.tick = setTimeout(() => tick(mine), MINUTE - (now.getSeconds() * 1000 + now.getMilliseconds()) + 50);
}

/* SAVER */

const TURN = 90000;
const FADE = 1600;
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
  const stage = mine.stage ?? mine.box;
  const after = mine.live?.canvas ?? null;
  stage.insertBefore(canvas, after?.parentNode === stage ? after.nextSibling : stage.firstChild);
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
  if (shown || document.hidden || !saves()) return false;
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
  const up = Object.assign<Base, Up>(mine, {
    box,
    stage: null,
    bind: () => ({
      ref: (node: HTMLElement | null) => {
        if (node) up.stage = node;
      },
    }),
    face: read(KEY.clock) !== "off",
    now: new Date(),
    state: undefined,
    tick: 0,
    deaf: () => {},
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
  tick(up);
  up.deaf = subscribe((key) => (key === null || key === KEY.hours) && tick(up));
  root().dataset.saver = "";
  (up.stage ?? box).getBoundingClientRect();
  up.state = "on";
  draw(up);
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
  clearTimeout(mine.tick);
  mine.deaf();
  delete root().dataset.saver;
  dispatchEvent(new CustomEvent("saver", { detail: false }));
  const done = () => {
    mine.ears.abort();
    for (const one of [mine.live, ...mine.old]) {
      if (!one) continue;
      clearTimeout(one.timer);
      one.run();
    }
    hide(mine.box);
    mine.box.remove();
  };
  const left = ending(mine.box, () => {
    mine.state = "out";
    draw(mine);
  });
  if (calm() || !left) return done();
  left.then(done);
}
