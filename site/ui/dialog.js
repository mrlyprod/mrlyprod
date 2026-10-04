import { load, mount, unmount } from './islands.js';
import { grab, layer, read } from './router.js';

const jobs = new Map();
const boxes = new Map();
const state = () => (history.state && typeof history.state === 'object' ? history.state : {});

/* VIEW */

function clear(box) {
  for (const one of box.querySelectorAll('[data-view]')) one.removeAttribute('data-view');
  delete box.dataset.view;
}

function view(box, id) {
  const was = box.dataset.view ?? '';
  const spot = id ? box.querySelector(`[id="${CSS.escape(id)}"]`) : null;
  const now = spot ? id : '';
  if (now === was) return;
  clear(box);
  if (spot) {
    spot.setAttribute('data-view', '');
    box.dataset.view = id;
  }
  box.scrollTo(0, 0);
  const next = spot ? spot.querySelector('a[href], button') : box.querySelector(`a[href="#${CSS.escape(was)}"]`);
  next?.focus({ preventScroll: true });
}

/* SYNC */

function hide(box) {
  unmount(box);
  box.close();
}

function sync() {
  const { dialog: page, view: id = '' } = state();
  for (const box of boxes.values()) if (box.open && box.dataset.page !== page) hide(box);
  const box = boxes.get(page);
  if (!box) return;
  if (!box.open) {
    clear(box);
    box.showModal();
    mount(box).catch(() => {});
  }
  view(box, id);
}

/* EVENTS */

function up() {
  const { view: id, ...rest } = state();
  if (!id) return history.back();
  history.replaceState(rest, '');
  sync();
}

function click(event) {
  if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
  const target = event.target instanceof Element ? event.target : null;
  if (target?.closest('dialog > .top button')) return up();
  const a = target?.closest('a[href^="#"]');
  if (!a) return;
  event.preventDefault();
  history.replaceState({ ...state(), view: a.getAttribute('href').slice(1) }, '');
  sync();
}

function cancel(event) {
  if (!event.cancelable) return;
  event.preventDefault();
  up();
}

function closed(event) {
  const box = event.currentTarget;
  if (!box.open && state().dialog === box.dataset.page) history.back();
}

/* BUILD */

async function build(a) {
  const page = a.getAttribute('href');
  const doc = read(await grab(new URL(a.href), true));
  const main = doc.querySelector('main');
  if (!main) throw new Error(`dialog: ${page} has no main`);
  await load(doc);
  const box = document.createElement('dialog');
  const top = document.createElement('header');
  const shut = document.createElement('button');
  const body = document.createElement('div');
  box.dataset.page = page;
  box.setAttribute('aria-label', a.getAttribute('aria-label') ?? doc.title);
  top.className = 'top';
  shut.type = 'button';
  shut.className = 'glyph';
  shut.setAttribute('aria-label', 'Close');
  shut.append(...[...a.children].map((one) => one.cloneNode(true)));
  top.append(shut);
  body.className = main.className;
  body.append(...document.importNode(main, true).childNodes);
  box.append(top, body);
  box.addEventListener('click', click);
  box.addEventListener('cancel', cancel);
  box.addEventListener('close', closed);
  document.body.append(box);
  boxes.set(page, box);
}

/* OPEN */

export async function open(a, live = () => true) {
  const page = a.getAttribute('href');
  if (!jobs.has(page)) jobs.set(page, build(a));
  try {
    await jobs.get(page);
  } catch (error) {
    jobs.delete(page);
    throw error;
  }
  if (!live()) return;
  if (state().dialog !== page) layer({ dialog: page });
  sync();
}

/* BOOT */

addEventListener('popstate', sync);
addEventListener('wire', sync);
