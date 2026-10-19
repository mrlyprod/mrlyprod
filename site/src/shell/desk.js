import { lock as saver } from 'site:apps';
import { word } from './word.js';
import { KEY, read, root, screen, write } from './prefs.js';

/* NAME */

const SVG = 'http://www.w3.org/2000/svg';
const STILL = '(prefers-reduced-motion: reduce)';

function paint(svg, rows, cols, on) {
  svg.setAttribute('viewBox', `0 0 ${cols} ${rows}`);
  svg.style.setProperty('--rows', rows);
  svg.style.setProperty('--cols', cols);
  svg.replaceChildren(
    ...on.map((i) => {
      const cell = document.createElementNS(SVG, 'rect');
      for (const [k, v] of [['x', i % cols], ['y', Math.floor(i / cols)], ['width', 1], ['height', 1]]) cell.setAttribute(k, v);
      return cell;
    }),
  );
}

export function folds(mark) {
  let anim = null;
  let at = 0;
  let way = 0;
  let timer = 0;
  const halt = () => {
    clearInterval(timer);
    timer = 0;
  };
  const tick = () => {
    const next = at + way;
    if (next < 0 || next >= anim.frames.length) return halt();
    at = next;
    paint(mark.querySelector('svg'), anim.rows, anim.cols, anim.frames[at]);
  };
  const run = async (dir) => {
    const text = mark.dataset.word;
    if (!text || (dir > 0 && matchMedia(STILL).matches)) return;
    if (anim?.text !== text) {
      const { fold } = await import('../lib/font/font.js');
      if (mark.dataset.word !== text) return;
      anim = { text, ...fold(text) };
      at = 0;
    }
    way = dir;
    if (!timer) timer = setInterval(tick, 1000 / anim.fps);
  };
  mark.addEventListener('pointerenter', (e) => e.pointerType === 'mouse' && run(1));
  mark.addEventListener('pointerleave', (e) => e.pointerType === 'mouse' && run(-1));
  mark.refold = () => {
    halt();
    anim = null;
    at = 0;
  };
}

const FOLD = 'fold';

export let route = '';
let doors = [];
let drawn = FOLD;

export function visit(detail) {
  ({ route, doors } = detail);
}

export async function rename() {
  const mark = document.querySelector('.top .mark');
  if (!mark || !route) return;
  const next = word(route, doors);
  const want = route === '/' ? FOLD : next || 'X';
  mark.dataset.word = next;
  const title = mark.getAttribute('aria-label').replace(/^.*, /, '');
  mark.setAttribute('aria-label', next ? `${next}, ${title}` : title);
  if (want === drawn) return;
  drawn = want;
  mark.refold?.();
  if (want === FOLD) return arrive(mark);
  depart(mark);
  const { letters } = await import('../lib/font/font.js');
  if (drawn !== want) return;
  const { rows, cols, grid } = letters(want);
  paint(mark.querySelector('svg'), rows, cols, grid.flat().flatMap((on, i) => (on ? [i] : [])));
}

/* WELCOME */

const PLAY = 2000;
const FADE = 250;
const QUIET = 200;
const PAGING = ['ArrowDown', 'PageDown', ' ', 'End'];
const LIVE = 'main a[href], main button, main input, main select, main textarea, main summary';

let playing = false;
let asked = false;
let quiet = 0;
let watch = null;
let owed = false;
let tapped = 0;

const hall = () => document.querySelector('.welcome');
export const greeting = () => root().dataset.welcome;
const jump = (top) => scrollTo({ top, behavior: 'instant' });
const rolls = () => CSS.supports('animation-timeline', 'scroll()');

function glide(path, time, done) {
  playing = true;
  const span = matchMedia(STILL).matches ? 0 : time;
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

function calm() {
  if (greeting() === 'open') return;
  if (performance.now() < quiet) return setTimeout(calm, QUIET);
  unlisten();
}

function settle() {
  unlock();
  shown?.stop?.coast?.(SETTLE);
  const top = Math.max(0, scrollY - (hall()?.offsetHeight ?? 0));
  write(KEY.welcome, '1');
  root().dataset.welcome = 'shut';
  jump(top);
  quiet = performance.now() + QUIET;
  setTimeout(calm, QUIET);
}

function enter() {
  if (greeting() !== 'open') return;
  asked = playing;
  if (playing) return;
  const mine = shown;
  const go = () => {
    if (shown !== mine || playing || greeting() !== 'open') return;
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
  if (playing || greeting() !== 'shut' || !hall()) return;
  const at = scrollY;
  playing = true;
  asked = false;
  clearTimeout(later);
  shown?.stop?.play?.();
  root().dataset.welcome = 'open';
  listen();
  jump(at + hall().offsetHeight);
  setTimeout(() => {
    jump(hall().offsetHeight);
    glide((t, height) => height * (1 - t), PLAY, () => (asked ? enter() : lid()));
  }, matchMedia(STILL).matches ? 0 : FADE);
}

function hold(e) {
  const open = greeting() === 'open';
  if (!open && !playing) {
    if (e.type === 'wheel' && performance.now() < quiet) {
      e.preventDefault();
      quiet = performance.now() + QUIET;
    }
    return;
  }
  const target = e.target instanceof Element ? e.target : null;
  if (e.type === 'wheel') {
    if (Math.abs(e.deltaX) > Math.abs(e.deltaY)) return;
    e.preventDefault();
    return enter();
  }
  if (e.type === 'touchmove') {
    if (e.touches.length > 1) return;
    e.preventDefault();
    return enter();
  }
  if (e.type === 'keydown') {
    if (e.metaKey || e.ctrlKey || e.altKey || !PAGING.includes(e.key)) return;
    e.preventDefault();
    return enter();
  }
  if (target?.closest(LIVE)) {
    if (open && !playing) settle();
    return;
  }
  if (e.type === 'pointerdown') return enter();
  e.preventDefault();
}

function drift() {
  if (greeting() === 'open' && !playing && scrollY > 0) enter();
}

export function listen() {
  if (watch) return;
  watch = new AbortController();
  const options = { passive: false, capture: true, signal: watch.signal };
  for (const type of ['wheel', 'touchmove', 'keydown', 'pointerdown', 'click']) addEventListener(type, hold, options);
  addEventListener('scroll', drift, { passive: true, signal: watch.signal });
  addEventListener('pointermove', stir, { passive: true, signal: watch.signal });
}

function unlisten() {
  watch?.abort();
  watch = null;
}

export function knock(mark) {
  mark.addEventListener('click', (e) => {
    if (e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
    const now = performance.now();
    const twice = now - tapped < TWICE;
    tapped = twice ? 0 : now;
    if (route === '/' && greeting() === 'shut' && hall()) {
      e.preventDefault();
      return greet();
    }
    if (!twice) return;
    e.preventDefault();
    if (!greeting()) owed = true;
  });
}

async function arrive(mark) {
  const { markup, welcome } = await import('./welcome.js');
  if (drawn !== FOLD) return;
  const model = welcome();
  const svg = mark.querySelector('svg');
  svg.setAttribute('class', 'glyphs fold');
  svg.setAttribute('viewBox', `0 0 ${model.cols} 5`);
  svg.style.setProperty('--rows', 5);
  svg.style.setProperty('--cols', model.cols);
  svg.innerHTML = markup(model);
  mark.style.setProperty('--cols', model.cols);
  const due = owed;
  owed = false;
  if (!rolls()) return;
  root().dataset.welcome = read(KEY.welcome) || matchMedia(STILL).matches ? 'shut' : 'open';
  if (greeting() === 'open') {
    listen();
    lid();
  }
  if (due) greet();
}

function depart(mark) {
  owed = false;
  close();
  delete root().dataset.welcome;
  unlisten();
  mark.style.removeProperty('--cols');
  mark.querySelector('svg')?.setAttribute('class', 'glyphs');
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

export async function desk() {
  if (shown) return;
  const box = document.createElement('div');
  box.className = 'desk';
  box.setAttribute('aria-hidden', 'true');
  box.append(document.createElement('canvas'));
  const mine = { box, stop: null };
  shown = mine;
  document.body.append(box);
  const want = read(KEY.saver);
  const use = pinned?.pick === want ? pinned.as : want;
  const stop = await saver().then(({ lock }) => lock(box.firstChild, use)).catch(() => null);
  if (shown !== mine) return stop?.();
  if (!stop) return close();
  if (use === want && stop.pick !== want) screen(stop.pick);
  pinned = { pick: read(KEY.saver), as: stop.as };
  mine.stop = stop;
  box.classList.add('on');
  if (greeting() === 'open') return lid();
  later = setTimeout(() => stop.coast?.(SETTLE), HOLD);
}

function lid() {
  if (!shown?.stop || playing || greeting() !== 'open') return;
  root().dataset.lock = 'wake';
  doze();
}

function doze() {
  clearTimeout(nap);
  nap = setTimeout(() => {
    if (root().dataset.lock) root().dataset.lock = 'rest';
  }, IDLE);
}

function stir(e) {
  if (!root().dataset.lock || e.pointerType === 'touch') return;
  root().dataset.lock = 'wake';
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
  shown?.box.remove();
  shown = null;
}
