import { palette, dark as NIGHT, light as DAY } from '../palette.js';

/* RANDOM */

export function rng(seed) {
  if (seed === undefined || seed === null) return Math.random;
  let s = (seed >>> 0) || 0x9e3779b9;
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

export function paints(canvas) {
  const night = typeof matchMedia === 'function' && matchMedia('(prefers-color-scheme: dark)').matches;
  const base = night ? NIGHT : DAY;
  const css = typeof getComputedStyle === 'function' && canvas ? getComputedStyle(canvas) : null;
  const read = (name, fallback) => (css && css.getPropertyValue(name).trim()) || fallback;
  const paper = read('--art', read('--ground', base.ground ?? palette.white));
  const accent = read('--accent', base.accent ?? palette.blue);
  return { paper, accent };
}

/* LIFE */

export function run(canvas, make, opts = {}) {
  const rand = rng(opts.seed);
  const still = typeof matchMedia === 'function' && matchMedia('(prefers-reduced-motion: reduce)').matches;
  let skin = paints(canvas);
  const view = { rand, look: () => skin, still, w: 1, h: 1, dpr: 1 };
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
  const loop = () => {
    saver.draw();
    frame = requestAnimationFrame(loop);
  };
  const play = () => {
    if (still || timer || frame) return;
    if (saver.every) timer = setInterval(() => saver.draw(), saver.every);
    else frame = requestAnimationFrame(loop);
  };
  const halt = () => {
    if (timer) clearInterval(timer);
    if (frame) cancelAnimationFrame(frame);
    timer = 0;
    frame = 0;
  };
  const wake = () => (seen && !document.hidden ? play() : halt());
  const paint = () => {
    skin = paints(canvas);
    saver.theme?.();
    if (still) saver.draw();
  };
  const grow = () => {
    if (!size()) return;
    saver.size?.();
    if (still) saver.draw();
  };
  const spy = ([entry]) => {
    seen = entry.isIntersecting;
    wake();
  };
  const eye = watched ? new IntersectionObserver(spy) : null;
  const tape = typeof ResizeObserver === 'function' ? new ResizeObserver(grow) : null;
  const shade = typeof matchMedia === 'function' ? matchMedia('(prefers-color-scheme: dark)') : null;
  eye?.observe(canvas);
  tape?.observe(canvas);
  shade?.addEventListener('change', paint);
  window.addEventListener('theme', paint);
  document.addEventListener('visibilitychange', wake);
  if (still) saver.draw();
  else wake();
  return () => {
    halt();
    eye?.disconnect();
    tape?.disconnect();
    shade?.removeEventListener('change', paint);
    window.removeEventListener('theme', paint);
    document.removeEventListener('visibilitychange', wake);
    saver.stop?.();
  };
}
