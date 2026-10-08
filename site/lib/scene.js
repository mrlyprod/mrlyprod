import { palette } from '../kit/theme/palette.js';
import { dark as NIGHT, light as DAY } from '../kit/theme/theme.js';
import { HUES, tints } from '../ui/hues.js';

/* RANDOM */

export function rng(seed) {
  if (seed === undefined || seed === null) return Math.random;
  let s = seed >>> 0;
  s = Math.imul(s ^ (s >>> 16), 0x85ebca6b);
  s = Math.imul(s ^ (s >>> 13), 0xc2b2ae35);
  s = (s ^ (s >>> 16)) >>> 0 || 0x9e3779b9;
  return () => {
    s ^= s << 13;
    s ^= s >>> 17;
    s ^= s << 5;
    s >>>= 0;
    return s / 4294967296;
  };
}

export function pick(rand, list) {
  return list[Math.floor(rand() * list.length)];
}

/* COLOUR */

export function rgb(value) {
  const text = String(value).trim();
  if (text.startsWith('#')) {
    const hex = text.length < 7 ? [...text.slice(1, 4)].map((c) => c + c).join('') : text.slice(1, 7);
    const n = Number.parseInt(hex, 16);
    return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
  }
  const parts = text.match(/-?\d+(\.\d+)?/g);
  return parts ? parts.slice(0, 3).map(Number) : [0, 0, 0];
}

export function veil(color, alpha) {
  const [r, g, b] = rgb(color);
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

export { HUES };

export const TINTS = tints('Site');

export function paints(canvas, tint = '') {
  const night = typeof matchMedia === 'function' && matchMedia('(prefers-color-scheme: dark)').matches;
  const base = night ? NIGHT : DAY;
  const css = typeof getComputedStyle === 'function' && canvas ? getComputedStyle(canvas) : null;
  const read = (name, fallback) => (css && css.getPropertyValue(name).trim()) || fallback;
  const paper = read('--art', read('--ground', base.ground ?? palette.white));
  const accent = HUES.includes(tint) ? read(`--${tint}`, palette[tint]) : read('--accent', base.accent ?? palette.blue);
  return { paper, accent };
}

/* BOARD */

export function board(view, cols, rows, fit = 1) {
  const cell = Math.max(1, Math.floor(Math.min((view.w * fit) / cols, (view.h * fit) / rows)));
  const w = cell * cols;
  const h = cell * rows;
  return { x: Math.floor((view.w - w) / 2), y: Math.floor((view.h - h) / 2), w, h, cell };
}

/* LIFE */

const LAG = 100;

export function run(canvas, make, opts = {}) {
  const rand = rng(opts.seed);
  let still = typeof matchMedia === 'function' && matchMedia('(prefers-reduced-motion: reduce)').matches;
  let skin = paints(canvas, opts.tint);
  let gone = false;
  let paused = false;
  let on = false;
  const view = { rand, look: () => skin, still, w: 1, h: 1, dpr: 1, t: 0 };
  const size = () => {
    const dpr = Math.min(globalThis.devicePixelRatio || 1, 2);
    const w = Math.max(1, Math.round((canvas.clientWidth || 300) * dpr));
    const h = Math.max(1, Math.round((canvas.clientHeight || 150) * dpr));
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
  const watched = typeof IntersectionObserver === 'function';
  let timer = 0;
  let frame = 0;
  let seen = !watched;
  let last = 0;
  const loop = (now) => {
    if (last) view.t += Math.min(now - last, LAG);
    last = now;
    saver.draw();
    frame = requestAnimationFrame(loop);
  };
  const tick = () => {
    view.t += saver.every;
    saver.draw();
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
    if (still || paused) saver.draw();
  };
  const grow = () => {
    if (!size()) return;
    saver.size?.();
    if (still || paused) saver.draw();
  };
  const spy = ([entry]) => {
    seen = entry.isIntersecting;
    watch();
  };
  const eye = watched ? new IntersectionObserver(spy) : null;
  const tape = typeof ResizeObserver === 'function' ? new ResizeObserver(grow) : null;
  const shade = typeof matchMedia === 'function' ? matchMedia('(prefers-color-scheme: dark)') : null;
  const start = () => {
    if (gone) return;
    on = true;
    eye?.observe(canvas);
    tape?.observe(canvas);
    shade?.addEventListener('change', paint);
    window.addEventListener('theme', paint);
    document.addEventListener('visibilitychange', watch);
    if (still) saver.draw();
    else watch();
  };
  if (saver.font && document.fonts) document.fonts.load(`1em ${JSON.stringify(saver.font)}`).then(start, start);
  else start();
  const stop = () => {
    gone = true;
    halt();
    eye?.disconnect();
    tape?.disconnect();
    shade?.removeEventListener('change', paint);
    window.removeEventListener('theme', paint);
    document.removeEventListener('visibilitychange', watch);
    saver.stop?.();
  };
  stop.pause = () => {
    paused = true;
    halt();
  };
  stop.play = () => {
    paused = false;
    if (on && !gone) watch();
  };
  stop.scene = saver;
  return stop;
}
