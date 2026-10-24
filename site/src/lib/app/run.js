import { paints } from "../color.js";
import { fed } from "../glass/feed.js";
import { rng } from "../rand.js";

/* BOARD */

export function board(view, cols, rows, fit = 1) {
  const cell = Math.max(1, Math.floor(Math.min((view.w * fit) / cols, (view.h * fit) / rows)));
  const w = cell * cols;
  const h = cell * rows;
  return { x: Math.floor((view.w - w) / 2), y: Math.floor((view.h - h) / 2), w, h, cell };
}

/* LIFE */

const LAG = 100;

const SHORT = 540;

const DEBT = 4;

const calm = () =>
  (typeof document !== "undefined" && document.documentElement?.dataset?.motion === "reduce") ||
  (typeof matchMedia === "function" && matchMedia("(prefers-reduced-motion: reduce)").matches);

export function run(canvas, make, opts = {}, how = {}) {
  let still = Boolean(how.still) || calm();
  let skin = paints(canvas, opts.tint);
  let gone = false;
  let paused = false;
  let on = false;
  let held = opts.step || opts.frame ? { step: opts.step, frame: opts.frame } : null;
  let late = 0;
  let debt = 0;
  let lent = false;
  let fall = null;
  let owed = 0;
  const view = {
    rand: rng(opts.seed),
    look: () => skin,
    still,
    lock: Boolean(how.lock),
    w: 1,
    h: 1,
    dpr: 1,
    t: 0,
    fixed: false,
    drew: () => fed(canvas),
  };
  const size = () => {
    const shot = held?.frame;
    view.fixed = Boolean(shot) && !held.dpr;
    const dpr = shot ? (held.dpr ?? Math.min(shot[0], shot[1]) / SHORT) : Math.min(globalThis.devicePixelRatio || 1, 2);
    const w = shot ? shot[0] : Math.max(1, Math.round((canvas.clientWidth || 300) * dpr));
    const h = shot ? shot[1] : Math.max(1, Math.round((canvas.clientHeight || 150) * dpr));
    if (w === view.w && h === view.h && dpr === view.dpr) return false;
    view.dpr = dpr;
    view.w = w;
    view.h = h;
    canvas.width = w;
    canvas.height = h;
    return true;
  };
  size();
  const saver = make(canvas, view, opts);
  const draw = () => {
    saver.draw();
    fed(canvas);
  };
  if (saver.still) {
    still = true;
    view.still = true;
  }
  const watched = typeof IntersectionObserver === "function";
  let timer = 0;
  let frame = 0;
  let seen = !watched;
  let last = 0;
  const after = () => held?.after?.({ t: view.t, late });
  const paced = (now, step) => {
    debt = last ? debt + now - last : step;
    last = now;
    if (debt < step) return;
    debt -= step;
    view.t += step;
    draw();
    if (debt > DEBT * step) {
      late += Math.floor(debt / step);
      debt = 0;
    }
    after();
  };
  const rate = (now) => {
    if (!fall) return 1;
    const k = (now - fall.from) / fall.span;
    if (k < 1) return (1 - k) ** 2;
    fall = null;
    paused = true;
    return 0;
  };
  const loop = (now) => {
    frame = 0;
    const r = rate(now);
    if (held?.step) paced(now, held.step);
    else {
      if (last) view.t += Math.min(now - last, LAG) * r;
      last = now;
      draw();
      after();
    }
    if (paused) return halt();
    frame = requestAnimationFrame(loop);
  };
  const tick = () => {
    owed += rate(performance.now());
    if (owed >= 1) {
      owed -= 1;
      view.t += saver.every;
      draw();
      after();
    }
    if (paused) halt();
  };
  const go = () => {
    if (still || paused || timer || frame) return;
    if (saver.every) timer = setInterval(tick, saver.every);
    else frame = requestAnimationFrame(loop);
  };
  const halt = () => {
    if (timer) clearInterval(timer);
    if (frame) cancelAnimationFrame(frame);
    timer = 0;
    frame = 0;
    last = 0;
  };
  const watch = () => (seen && !document.hidden ? go() : halt());
  view.wake = () => {
    if (!still || gone) return;
    still = false;
    view.still = false;
    watch();
  };
  const paint = () => {
    skin = paints(canvas, opts.tint);
    saver.theme?.();
    if (still || paused) draw();
  };
  const grow = () => {
    if (held?.frame || !size()) return;
    saver.size?.();
    if (still || paused) draw();
  };
  const spy = ([entry]) => {
    seen = entry.isIntersecting;
    watch();
  };
  const eye = watched ? new IntersectionObserver(spy) : null;
  const tape = typeof ResizeObserver === "function" ? new ResizeObserver(grow) : null;
  const shade = typeof matchMedia === "function" ? matchMedia("(prefers-color-scheme: dark)") : null;
  const start = () => {
    if (gone) return;
    on = true;
    eye?.observe(canvas);
    tape?.observe(canvas);
    shade?.addEventListener("change", paint);
    window.addEventListener("theme", paint);
    document.addEventListener("visibilitychange", watch);
    if (still || saver.every) draw();
    if (!still) watch();
  };
  if (saver.font && document.fonts) document.fonts.load(`1em ${JSON.stringify(saver.font)}`).then(start, start);
  else start();
  const stop = () => {
    gone = true;
    halt();
    eye?.disconnect();
    tape?.disconnect();
    shade?.removeEventListener("change", paint);
    window.removeEventListener("theme", paint);
    document.removeEventListener("visibilitychange", watch);
    saver.stop?.();
  };
  stop.pause = () => {
    paused = true;
    halt();
  };
  stop.play = () => {
    paused = false;
    fall = null;
    owed = 0;
    if (on && !gone) watch();
  };
  stop.coast = (span) => {
    if (gone || still || paused || fall) return;
    fall = { from: performance.now(), span };
  };
  stop.fix = (next) => {
    if (gone) return;
    if (next?.step && still) {
      lent = true;
      view.wake();
    } else if (!next?.step && lent) {
      lent = false;
      still = true;
      view.still = true;
      halt();
    }
    held = next ?? null;
    late = 0;
    debt = 0;
    last = 0;
    if (size()) saver.size?.();
    if (still || paused) draw();
  };
  stop.set = (next) => {
    if (gone || !saver.set) return false;
    if (next.seed !== opts.seed) view.rand = rng(next.seed);
    opts = next;
    skin = paints(canvas, opts.tint);
    if (saver.set(next) === false) return false;
    if (still || paused) draw();
    return true;
  };
  Object.defineProperty(stop, "late", { get: () => late });
  stop.scene = saver;
  return stop;
}
