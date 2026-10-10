import { canvas, ink } from '../../pkgs/mrlyjs/view/index.js';
import { dark, light, tinted } from '../kit/theme/theme.js';

export const TILE = 512;
export const OPENER = 1024;
export const BUDGET = 128 * 1024 * 1024;

const SIZE = [1024, 1024];
const MARGIN = '100% 0px';
const STILL = '(prefers-reduced-motion: reduce)';
const DARK = '(prefers-color-scheme: dark)';

/* SIZE */

export function fit(box, dpr, cap, [w, h] = SIZE) {
  const long = Math.max(w, h);
  const scale = Math.min((box * dpr) / w, Math.min(cap, long) / long);
  return [Math.max(1, Math.round(w * scale)), Math.max(1, Math.round(h * scale))];
}

/* QUEUE */

export function queue({ near, paint, frame = (fn) => requestAnimationFrame(fn), cancel = (id) => cancelAnimationFrame(id) }) {
  const waiting = new Set();
  let ticket = 0;
  let busy = false;
  let epoch = 0;
  const kick = () => {
    if (!ticket && !busy && waiting.size) ticket = frame(step);
  };
  function step() {
    ticket = 0;
    let next = null;
    let best = Infinity;
    for (const one of waiting) {
      const gap = near(one);
      if (gap < best) [next, best] = [one, gap];
    }
    if (!next) return;
    waiting.delete(next);
    busy = true;
    const mine = epoch;
    Promise.resolve()
      .then(() => paint(next))
      .catch(() => {})
      .finally(() => {
        if (mine !== epoch) return;
        busy = false;
        kick();
      });
  }
  return {
    add(one) {
      waiting.add(one);
      kick();
    },
    remove: (one) => waiting.delete(one),
    drop() {
      epoch++;
      busy = false;
      waiting.clear();
      if (ticket) cancel(ticket);
      ticket = 0;
    },
    get size() {
      return waiting.size;
    },
  };
}

/* INK */

let table = { figures: {}, units: {} };
let shade = null;

function look() {
  const root = document.documentElement;
  const set = root.dataset.theme;
  const theme = set === 'dark' || set === 'light' ? set : matchMedia(DARK).matches ? 'dark' : 'light';
  const css = getComputedStyle(root);
  const accent = css.getPropertyValue('--accent').trim();
  const link = css.getPropertyValue('--link').trim();
  const key = `${theme} ${accent} ${link}`;
  if (shade?.key !== key) shade = { key, ink: ink(tinted(theme === 'dark' ? dark : light, { accent, link, theme })) };
  return shade;
}

/* UNITS */

const opened = new Map();

function open(units = {}) {
  return Promise.all(
    Object.entries(units).map(([name, unit]) => {
      if (!opened.has(name)) {
        const url = table.units[name];
        if (!url) throw new Error(`figure: no wasm for the unit ${name}`);
        opened.set(name, unit.default({ module_or_path: url }));
      }
      return opened.get(name);
    }),
  );
}

const modules = new Map();

function load(name) {
  if (!modules.has(name)) {
    const thunk = table.figures[name];
    modules.set(name, thunk ? thunk().then(async (one) => (await open(one.units), one)) : Promise.reject(new Error(`figure: no live figure ${name}`)));
  }
  return modules.get(name);
}

/* DRAW */

const hosts = new Map();
let scratch = null;
let bytes = 0;
let warned = false;

function account(one, next) {
  bytes += next - one.bytes;
  one.bytes = next;
  if (bytes > BUDGET && !warned) {
    warned = true;
    console.warn(`figure: ${Math.round(bytes / 2 ** 20)} MB of live canvases, past the ${BUDGET / 2 ** 20} MB budget`);
  }
}

function moment(figure, one) {
  const frames = figure.frames ?? 0;
  if (!figure.loop || !frames || matchMedia(STILL).matches) return figure.still ?? 0;
  one.step = one.step === undefined ? Math.round((figure.still ?? 0) * frames) % frames : (one.step + 1) % frames;
  return one.step / frames;
}

async function paint(host) {
  const one = hosts.get(host);
  if (!one || !host.isConnected) return;
  try {
    await draw(host, one);
  } finally {
    host.removeAttribute('aria-busy');
  }
}

async function draw(host, one) {
  try {
    const figure = await load(one.name);
    if (hosts.get(host) !== one || !host.isConnected) return;
    const size = figure.size ?? SIZE;
    const node = host.querySelector('canvas');
    if (size[0] !== size[1]) node.style.aspectRatio = `${size[0]} / ${size[1]}`;
    const box = node.getBoundingClientRect().width || host.getBoundingClientRect().width;
    const [w, h] = fit(box, devicePixelRatio || 1, one.cap, size);
    if (node.width !== w || node.height !== h) {
      node.width = w;
      node.height = h;
      account(one, w * h * 4);
    }
    const pen = look();
    const start = performance.now();
    scratch ??= document.createElement('canvas');
    if (scratch.width !== size[0] || scratch.height !== size[1]) [scratch.width, scratch.height] = size;
    const ctx = scratch.getContext('2d');
    figure.default(canvas(ctx, size[0], size[1], pen.ink.ground), pen.ink, moment(figure, one));
    const out = node.getContext('2d', { willReadFrequently: one.eager });
    out.imageSmoothingQuality = 'high';
    out.clearRect(0, 0, w, h);
    out.drawImage(scratch, 0, 0, w, h);
    if (one.eager) out.getImageData(0, 0, 1, 1);
    one.key = pen.key;
    host.dataset.ms = (performance.now() - start).toFixed(1);
    host.dispatchEvent(new CustomEvent('drawn', { detail: Number(host.dataset.ms), bubbles: true }));
    if (figure.loop && figure.frames && !matchMedia(STILL).matches) {
      one.loop = true;
      clearTimeout(one.timer);
      one.timer = setTimeout(() => one.seen && !document.hidden && line.add(host), (figure.loop * 1000) / figure.frames);
    }
  } catch (error) {
    console.error(`figure ${one.name}:`, error);
  }
}

function near(host) {
  const box = host.getBoundingClientRect();
  return box.bottom < 0 ? -box.bottom : box.top > innerHeight ? box.top - innerHeight : 0;
}

const line = typeof document === 'undefined' ? null : queue({ near, paint });

function want(host) {
  clearTimeout(hosts.get(host)?.timer);
  host.setAttribute('aria-busy', 'true');
  line.add(host);
}

function skip(host) {
  host.removeAttribute('aria-busy');
  line.remove(host);
}

/* WATCH */

let eye = null;

export function due(one, key) {
  return one.seen && (one.loop || one.key !== key);
}

function seen(entries) {
  const key = look().key;
  for (const entry of entries) {
    const one = hosts.get(entry.target);
    if (!one) continue;
    one.seen = entry.isIntersecting || one.eager;
    if (due(one, key)) want(entry.target);
    else if (!one.seen) skip(entry.target);
  }
}

function resume() {
  if (document.hidden) return;
  const key = look().key;
  for (const [host, one] of hosts) if (due(one, key)) want(host);
}

function repaint() {
  const key = look().key;
  for (const [host, one] of hosts) if (one.key && one.key !== key && one.seen) want(host);
}

let started = false;

export function start(given) {
  table = given;
  if (started) return;
  started = true;
  matchMedia(DARK).addEventListener('change', repaint);
  window.addEventListener('theme', repaint);
  document.addEventListener('visibilitychange', resume);
}

export function scan(root, eager = false) {
  eye ??= new IntersectionObserver(seen, { rootMargin: MARGIN });
  for (const host of root.querySelectorAll('figure[data-figure]')) {
    if (hosts.has(host)) continue;
    const cap = host.closest('.tile') ? TILE : OPENER;
    hosts.set(host, { name: host.dataset.figure, cap, bytes: 0, seen: eager, eager, key: '', timer: 0, loop: false });
    eye.observe(host);
    host.setAttribute('aria-busy', 'true');
    if (eager) line.add(host);
  }
}

export function drop() {
  line?.drop();
  eye?.disconnect();
  for (const [host, one] of hosts) {
    clearTimeout(one.timer);
    account(one, 0);
    const node = host.querySelector('canvas');
    if (node) node.width = node.height = 0;
    host.removeAttribute('aria-busy');
  }
  hosts.clear();
}
