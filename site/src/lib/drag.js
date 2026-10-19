/* RULE */

export function placed(list, draft) {
  if (!draft) return list;
  const out = list.filter((one) => one !== draft.id);
  if (draft.index !== null && draft.index !== undefined) out.splice(Math.min(draft.index, out.length), 0, draft.id);
  return out;
}

export function slot(rects, x, y, axis = 'grid') {
  const before = {
    x: (r) => x < r.left + r.width / 2,
    y: (r) => y < r.top + r.height / 2,
    grid: (r) => y < r.top || (y < r.bottom && x < r.left + r.width / 2),
  }[axis];
  const at = rects.findIndex(before);
  return at < 0 ? rects.length : at;
}

const same = (a, b) => (a?.id ?? null) === (b?.id ?? null) && (a?.index ?? null) === (b?.index ?? null);

/* LOOK */

const LIFT = 5;
const HOLD = 400;
const DRIFT = 8;
const PAD = 24;
const EDGE = 48;
const SPEED = 14;
const GLIDE = 220;
const EASE = 'cubic-bezier(0.2, 0, 0, 1)';
const SKIP = /^(transition|animation|view-transition|anchor|position-|will-change)/;

const lists = new Set();
let now = null;
let busy = false;
let voice = null;

const calm = () => document.documentElement.dataset.motion === 'reduce' || (typeof matchMedia === 'function' && matchMedia('(prefers-reduced-motion: reduce)').matches);
const items = (one) => [...one.el.children].filter((kid) => kid.dataset?.id);
const inside = (r, x, y, pad = 0) => r.width > 0 && r.height > 0 && x >= r.left - pad && x <= r.right + pad && y >= r.top - pad && y <= r.bottom + pad;
const shift = (ms) => dispatchEvent(new CustomEvent('shift', { detail: ms }));
const spill = (box) => ({ x: box.scrollWidth > box.clientWidth, y: box.scrollHeight > box.clientHeight });
const pace = (at, lo, hi) => (at < lo + EDGE ? -SPEED : at > hi - EDGE ? SPEED : 0);

function say(text) {
  if (!voice?.isConnected) {
    voice = document.createElement('div');
    voice.className = 'visually-hidden';
    voice.setAttribute('role', 'status');
    document.body.append(voice);
  }
  voice.textContent = text;
}

/* GHOST */

function dress(from, to) {
  const css = getComputedStyle(from);
  for (const prop of css) if (!SKIP.test(prop)) to.style.setProperty(prop, css.getPropertyValue(prop));
  const a = from.children;
  const b = to.children;
  for (let i = 0; i < a.length; i++) if (b[i]) dress(a[i], b[i]);
}

function ghost(item, box) {
  const copy = item.cloneNode(true);
  dress(item, copy);
  const was = item.querySelectorAll('canvas');
  copy.querySelectorAll('canvas').forEach((one, i) => {
    try {
      one.getContext('2d')?.drawImage(was[i], 0, 0);
    } catch {}
  });
  for (const one of [copy, ...copy.querySelectorAll('[id], [data-id], [data-lens], [data-figure]')]) {
    for (const name of ['id', 'data-id', 'data-lens', 'data-figure', 'data-held']) one.removeAttribute(name);
  }
  copy.setAttribute('aria-hidden', 'true');
  copy.classList.add('ghost');
  Object.assign(copy.style, { position: 'fixed', left: `${box.left}px`, top: `${box.top}px`, width: `${box.width}px`, height: `${box.height}px`, margin: '0', translate: 'none', transform: 'none', transformOrigin: '0 0', pointerEvents: 'none', zIndex: 'var(--z-drag)', opacity: 'var(--lifting, 1)', filter: 'var(--lifted, none)', visibility: 'visible' });
  document.body.append(copy);
  return copy;
}

/* MOTION */

function snapshot() {
  const out = new Map();
  for (const one of lists) for (const kid of items(one)) out.set(kid, kid.getBoundingClientRect());
  return out;
}

function settle(before) {
  for (const kid of before.keys()) for (const run of kid.getAnimations?.() ?? []) if (run.id === 'drag') run.cancel();
  const rects = new Map();
  for (const one of lists) {
    const kids = items(one).map((kid) => [kid, kid.getBoundingClientRect()]);
    rects.set(one, kids);
  }
  if (calm()) return rects;
  let moved = false;
  for (const [, kids] of rects) {
    for (const [kid, r] of kids) {
      const was = before.get(kid);
      if (!was || kid.dataset.held !== undefined) continue;
      const dx = was.left - r.left;
      const dy = was.top - r.top;
      if (Math.abs(dx) < 1 && Math.abs(dy) < 1) continue;
      const run = kid.animate([{ translate: `${dx}px ${dy}px` }, { translate: '0 0' }], { duration: GLIDE, easing: EASE });
      run.id = 'drag';
      moved = true;
    }
  }
  if (moved) shift(GLIDE);
  return rects;
}

/* SESSION */

function target(x, y) {
  let near = null;
  for (const one of lists) {
    if (!one.el.isConnected || !one.takes(now.from.kind)) continue;
    if (one.scroller) {
      const clip = one.scroller.getBoundingClientRect();
      const over = spill(one.scroller);
      if ((over.x && (x < clip.left || x > clip.right)) || (over.y && (y < clip.top || y > clip.bottom))) continue;
    }
    const r = one.el.getBoundingClientRect();
    if (inside(r, x, y)) return one;
    if (!near && inside(r, x, y, PAD)) near = one;
  }
  return near;
}

function index(one, x, y) {
  const kids = (now.rects.get(one) ?? items(one).map((kid) => [kid, kid.getBoundingClientRect()])).filter(([kid]) => kid.dataset.id !== now.id);
  return slot(kids.map(([, r]) => r), x, y, one.axis);
}

function place(drafts) {
  const changed = [...new Set([...now.drafts.keys(), ...drafts.keys()])].filter((one) => !same(now.drafts.get(one), drafts.get(one)));
  if (!changed.length) return;
  const before = snapshot();
  for (const one of changed) {
    const next = drafts.get(one) ?? null;
    now.touched.add(one);
    if (next) now.drafts.set(one, next);
    else now.drafts.delete(one);
    one.preview?.(next);
  }
  now.rects = settle(before);
}

function follow() {
  const { px: x, py: y, from, id } = now;
  now.ghost.style.translate = `${x - now.x}px ${y - now.y}px`;
  const to = target(x, y);
  const take = to ? to.takes(from.kind) : '';
  const drafts = new Map();
  if (to) drafts.set(to, { id, index: index(to, x, y), from: from.kind });
  if (to !== from) {
    if ((to && take === 'move') || (!to && from.out)) drafts.set(from, { id, index: null, from: from.kind });
    else if (!to) drafts.set(from, { id, index: now.origin, from: from.kind });
  }
  now.over = to;
  now.ghost.toggleAttribute('data-out', !to && from.out);
  place(drafts);
}

function roll() {
  if (!now?.lifted) return;
  const box = (now.over ?? now.from).scroller;
  if (box) {
    const r = box.getBoundingClientRect();
    const { px: x, py: y } = now;
    const over = spill(box);
    const on = inside(r, x, y);
    const dx = on && over.x ? pace(x, r.left, r.right) : 0;
    const dy = on && over.y ? pace(y, r.top, r.bottom) : 0;
    const was = [box.scrollLeft, box.scrollTop];
    if (dx) box.scrollLeft += dx;
    if (dy) box.scrollTop += dy;
    if (box.scrollLeft !== was[0] || box.scrollTop !== was[1]) {
      now.rects = new Map();
      follow();
    }
  }
  now.frame = requestAnimationFrame(roll);
}

function lift() {
  clearTimeout(now.timer);
  const { item, from } = now;
  if (!item.isConnected) return quit();
  now.lifted = true;
  now.origin = items(from).indexOf(item);
  now.box = item.getBoundingClientRect();
  now.ghost = ghost(item, now.box);
  now.px ??= now.x;
  now.py ??= now.y;
  document.documentElement.classList.add('dragging');
  addEventListener('scroll', rescan, true);
  follow();
  now.frame = requestAnimationFrame(roll);
}

function move(e) {
  if (!now || e.pointerId !== now.pointer) return;
  now.px = e.clientX;
  now.py = e.clientY;
  if (!now.lifted) {
    const far = Math.hypot(e.clientX - now.x, e.clientY - now.y);
    if (now.hold) return far > DRIFT ? quit() : undefined;
    if (far < LIFT) return;
    lift();
    return;
  }
  e.preventDefault();
  follow();
  shift(0);
}

function pin(e) {
  if (now?.lifted && e.cancelable) e.preventDefault();
}

function rescan() {
  if (now) now.rects = new Map();
}

function hush(e) {
  if (now) e.preventDefault();
}

function swallow() {
  const stop = (e) => {
    e.preventDefault();
    e.stopPropagation();
  };
  addEventListener('click', stop, { capture: true, once: true });
  setTimeout(() => removeEventListener('click', stop, { capture: true }), 0);
}

function end() {
  clearTimeout(now.timer);
  cancelAnimationFrame(now.frame);
  removeEventListener('pointermove', move, true);
  removeEventListener('pointerup', up, true);
  removeEventListener('pointercancel', cancel, true);
  removeEventListener('keydown', escape, true);
  removeEventListener('contextmenu', hush, true);
  removeEventListener('scroll', rescan, true);
  now.touch.removeEventListener('touchmove', pin);
  document.documentElement.classList.remove('dragging');
  const session = now;
  now = null;
  return session;
}

function quit() {
  if (now) end();
}

function finish(session, back) {
  const land = back ? null : [...session.drafts].find(([, draft]) => draft.index !== null);
  const spot = land ? items(land[0]).find((kid) => kid.dataset.id === session.id) : null;
  const gone = !back && !land && session.from.out;
  const done = () => {
    session.ghost.remove();
    for (const one of session.touched) one.commit?.(back ? null : (session.drafts.get(one) ?? null));
    busy = false;
    shift(GLIDE);
    if (land) say(`${land[0].name?.(session.id) ?? session.id}, ${land[1].index + 1} of ${items(land[0]).length}`);
    else if (gone) say(`${session.from.name?.(session.id) ?? session.id} removed`);
  };
  busy = true;
  if (calm()) return done();
  const r = spot?.getBoundingClientRect() ?? session.box;
  const from = { translate: session.ghost.style.translate || '0 0', scale: '1', opacity: 1 };
  const to = gone ? { translate: from.translate, scale: '0.4', opacity: 0 } : { translate: `${r.left - session.box.left}px ${r.top - session.box.top}px`, scale: String(r.width / session.box.width || 1), opacity: 1 };
  session.ghost.animate([from, to], { duration: GLIDE, easing: EASE, fill: 'forwards' }).finished.then(done, done);
}

function up(e) {
  if (!now || e.pointerId !== now.pointer) return;
  if (!now.lifted) return quit();
  swallow();
  finish(end(), false);
}

function cancel(e) {
  if (!now || (e?.pointerId !== undefined && e.pointerId !== now.pointer)) return;
  if (!now.lifted) return quit();
  finish(end(), true);
}

function escape(e) {
  if (e.key !== 'Escape' || !now?.lifted) return;
  e.preventDefault();
  e.stopPropagation();
  const id = now.pointer;
  const off = () => {
    removeEventListener('pointerup', hit, true);
    removeEventListener('pointercancel', drop, true);
  };
  const hit = (e) => {
    if (e.pointerId !== id) return;
    off();
    swallow();
  };
  const drop = (e) => {
    if (e.pointerId === id) off();
  };
  addEventListener('pointerup', hit, true);
  addEventListener('pointercancel', drop, true);
  cancel();
}

function grab(one, e) {
  if (now || busy || e.button !== 0 || e.ctrlKey || e.metaKey || e.altKey || e.shiftKey) return;
  const item = e.target instanceof Element ? e.target.closest('[data-id]') : null;
  if (!item || item.parentElement !== one.el || one.fixed) return;
  const control = e.target.closest('button, input, select, textarea, [contenteditable="true"]');
  if (control && control !== item && item.contains(control)) return;
  now = { from: one, item, touch: e.target, id: item.dataset.id, x: e.clientX, y: e.clientY, pointer: e.pointerId, hold: e.pointerType !== 'mouse', lifted: false, drafts: new Map(), touched: new Set(), rects: new Map() };
  if (now.hold) now.timer = setTimeout(lift, HOLD);
  e.target.addEventListener('touchmove', pin, { passive: false });
  addEventListener('pointermove', move, true);
  addEventListener('pointerup', up, true);
  addEventListener('pointercancel', cancel, true);
  addEventListener('keydown', escape, true);
  addEventListener('contextmenu', hush, true);
}

/* KEYS */

function nudge(one, e) {
  if (now || busy || !e.altKey || e.metaKey || e.ctrlKey) return;
  const by = { ArrowLeft: -1, ArrowUp: -1, ArrowRight: 1, ArrowDown: 1 }[e.key];
  const item = e.target instanceof Element ? e.target.closest('[data-id]') : null;
  if (!by || !item || item.parentElement !== one.el || !one.takes(one.kind)) return;
  e.preventDefault();
  const list = items(one);
  const to = list.indexOf(item) + by;
  if (to < 0 || to >= list.length) return;
  const id = item.dataset.id;
  const focus = document.activeElement;
  const before = snapshot();
  one.commit?.({ id, index: to, from: one.kind });
  settle(before);
  if (focus?.isConnected) focus.focus({ preventScroll: true });
  else one.el.querySelector(`[data-id="${CSS.escape(id)}"]`)?.querySelector('a, button, [tabindex]')?.focus({ preventScroll: true });
  say(`${one.name?.(id) ?? id}, ${to + 1} of ${list.length}`);
}

/* LIST */

export function sortable(el, spec) {
  const one = { el, kind: '', takes: () => '', out: false, axis: 'grid', scroller: null, ...spec };
  lists.add(one);
  const down = (e) => grab(one, e);
  const key = (e) => nudge(one, e);
  const native = (e) => e.preventDefault();
  el.addEventListener('pointerdown', down);
  el.addEventListener('keydown', key);
  el.addEventListener('dragstart', native);
  el.addEventListener('touchmove', pin, { passive: false });
  return () => {
    lists.delete(one);
    el.removeEventListener('pointerdown', down);
    el.removeEventListener('keydown', key);
    el.removeEventListener('dragstart', native);
    el.removeEventListener('touchmove', pin);
    if (now?.from === one) {
      const session = end();
      session.ghost?.remove();
      queueMicrotask(() => {
        for (const other of session.touched) if (other !== one && lists.has(other)) other.commit?.(null);
      });
    } else now?.touched.delete(one);
  };
}

export const dragging = () => Boolean(now?.lifted);
