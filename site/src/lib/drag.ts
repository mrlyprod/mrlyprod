/* TYPES */

export type Axis = "x" | "y" | "grid";

export type Rect = Pick<DOMRectReadOnly, "left" | "top" | "right" | "bottom" | "width" | "height">;

export type Draft = { id: string; index: number | null; from: string };

export type Spec = {
  kind?: string;
  takes?: (from: string) => string;
  out?: boolean;
  axis?: Axis;
  scroller?: Element | null;
  fixed?: boolean;
  name?: (id: string) => string;
  preview?: (draft: Draft | null) => void;
  commit?: (draft: Draft | null) => void;
};

export type List = Spec & {
  el: HTMLElement;
  kind: string;
  takes: (from: string) => string;
  out: boolean;
  axis: Axis;
  scroller: Element | null;
};

type Placed = [HTMLElement, DOMRect];

type Rects = Map<List, Placed[]>;

type Session = {
  from: List;
  item: HTMLElement;
  touch: Element;
  id: string;
  x: number;
  y: number;
  pointer: number;
  hold: boolean;
  lifted: boolean;
  drafts: Map<List, Draft>;
  touched: Set<List>;
  rects: Rects;
  timer?: ReturnType<typeof setTimeout>;
  px?: number;
  py?: number;
  origin?: number;
  box?: DOMRect;
  ghost?: HTMLElement;
  over?: List | null;
  frame?: number;
};

type Lifted = Session & { px: number; py: number; origin: number; box: DOMRect; ghost: HTMLElement; frame: number };

/* RULE */

export function placed(list: string[], draft?: { id: string; index?: number | null } | null): string[] {
  if (!draft) return list;
  const out = list.filter((one) => one !== draft.id);
  if (draft.index !== null && draft.index !== undefined) out.splice(Math.min(draft.index, out.length), 0, draft.id);
  return out;
}

export function slot(rects: readonly Rect[], x: number, y: number, axis: Axis = "grid"): number {
  const before = {
    x: (r: Rect) => x < r.left + r.width / 2,
    y: (r: Rect) => y < r.top + r.height / 2,
    grid: (r: Rect) => y < r.top || (y < r.bottom && x < r.left + r.width / 2),
  }[axis];
  const at = rects.findIndex(before);
  return at < 0 ? rects.length : at;
}

const same = (a?: Draft | null, b?: Draft | null) =>
  (a?.id ?? null) === (b?.id ?? null) && (a?.index ?? null) === (b?.index ?? null);

/* LOOK */

const LIFT = 5;
const HOLD = 400;
const DRIFT = 8;
const PAD = 24;
const EDGE = 48;
const SPEED = 14;
const GLIDE = 220;
const EASE = "cubic-bezier(0.2, 0, 0, 1)";
const SKIP = /^(transition|animation|view-transition|anchor|position-|will-change)/;

const lists = new Set<List>();
let now: Session | null = null;
let busy = false;
let voice: HTMLElement | null = null;

const calm = () =>
  document.documentElement.dataset.motion === "reduce" ||
  (typeof matchMedia === "function" && matchMedia("(prefers-reduced-motion: reduce)").matches);
const items = (one: List) => ([...one.el.children] as HTMLElement[]).filter((kid) => kid.dataset?.id);
const inside = (r: Rect, x: number, y: number, pad = 0) =>
  r.width > 0 && r.height > 0 && x >= r.left - pad && x <= r.right + pad && y >= r.top - pad && y <= r.bottom + pad;
const shift = (ms: number) => dispatchEvent(new CustomEvent("shift", { detail: ms }));
const spill = (box: Element) => ({ x: box.scrollWidth > box.clientWidth, y: box.scrollHeight > box.clientHeight });
const pace = (at: number, lo: number, hi: number) => (at < lo + EDGE ? -SPEED : at > hi - EDGE ? SPEED : 0);

function say(text: string) {
  if (!voice?.isConnected) {
    voice = document.createElement("div");
    voice.className = "visually-hidden";
    voice.setAttribute("role", "status");
    document.body.append(voice);
  }
  voice.textContent = text;
}

/* GHOST */

function dress(from: Element, to: Element & ElementCSSInlineStyle) {
  const css = getComputedStyle(from);
  for (const prop of css) if (!SKIP.test(prop)) to.style.setProperty(prop, css.getPropertyValue(prop));
  const a = from.children;
  const b = to.children;
  for (let i = 0; i < a.length; i++) if (b[i]) dress(a[i], b[i] as Element & ElementCSSInlineStyle);
}

function ghost(item: HTMLElement, box: DOMRect) {
  const copy = item.cloneNode(true) as HTMLElement;
  dress(item, copy);
  const was = item.querySelectorAll("canvas");
  copy.querySelectorAll("canvas").forEach((one, i) => {
    try {
      one.getContext("2d")?.drawImage(was[i], 0, 0);
    } catch {}
  });
  for (const one of [copy, ...copy.querySelectorAll("[id], [data-id], [data-lens], [data-figure]")]) {
    for (const name of ["id", "data-id", "data-lens", "data-figure", "data-held"]) one.removeAttribute(name);
  }
  copy.setAttribute("aria-hidden", "true");
  copy.classList.add("ghost");
  Object.assign(copy.style, {
    position: "fixed",
    left: `${box.left}px`,
    top: `${box.top}px`,
    width: `${box.width}px`,
    height: `${box.height}px`,
    margin: "0",
    translate: "none",
    transform: "none",
    transformOrigin: "0 0",
    pointerEvents: "none",
    zIndex: "var(--z-drag)",
    opacity: "var(--lifting, 1)",
    filter: "var(--lifted, none)",
    visibility: "visible",
  });
  document.body.append(copy);
  return copy;
}

/* MOTION */

function snapshot() {
  const out = new Map<HTMLElement, DOMRect>();
  for (const one of lists) for (const kid of items(one)) out.set(kid, kid.getBoundingClientRect());
  return out;
}

function settle(before: Map<HTMLElement, DOMRect>): Rects {
  for (const kid of before.keys()) for (const run of kid.getAnimations?.() ?? []) if (run.id === "drag") run.cancel();
  const rects: Rects = new Map();
  for (const one of lists) {
    const kids: Placed[] = items(one).map((kid) => [kid, kid.getBoundingClientRect()]);
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
      const run = kid.animate([{ translate: `${dx}px ${dy}px` }, { translate: "0 0" }], {
        duration: GLIDE,
        easing: EASE,
      });
      run.id = "drag";
      moved = true;
    }
  }
  if (moved) shift(GLIDE);
  return rects;
}

/* SESSION */

function target(x: number, y: number) {
  let near: List | null = null;
  for (const one of lists) {
    if (!one.el.isConnected || !one.takes(now!.from.kind)) continue;
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

function index(one: List, x: number, y: number) {
  const kids = (now!.rects.get(one) ?? items(one).map((kid): Placed => [kid, kid.getBoundingClientRect()])).filter(
    ([kid]) => kid.dataset.id !== now!.id,
  );
  return slot(
    kids.map(([, r]) => r),
    x,
    y,
    one.axis,
  );
}

function place(drafts: Map<List, Draft>) {
  const changed = [...new Set([...now!.drafts.keys(), ...drafts.keys()])].filter(
    (one) => !same(now!.drafts.get(one), drafts.get(one)),
  );
  if (!changed.length) return;
  const before = snapshot();
  for (const one of changed) {
    const next = drafts.get(one) ?? null;
    now!.touched.add(one);
    if (next) now!.drafts.set(one, next);
    else now!.drafts.delete(one);
    one.preview?.(next);
  }
  now!.rects = settle(before);
}

function follow() {
  const { px: x, py: y, from, id } = now as Lifted;
  (now as Lifted).ghost.style.translate = `${x - (now as Lifted).x}px ${y - (now as Lifted).y}px`;
  const to = target(x, y);
  const take = to ? to.takes(from.kind) : "";
  const drafts = new Map<List, Draft>();
  if (to) drafts.set(to, { id, index: index(to, x, y), from: from.kind });
  if (to !== from) {
    if ((to && take === "move") || (!to && from.out)) drafts.set(from, { id, index: null, from: from.kind });
    else if (!to) drafts.set(from, { id, index: (now as Lifted).origin, from: from.kind });
  }
  (now as Lifted).over = to;
  (now as Lifted).ghost.toggleAttribute("data-out", !to && from.out);
  place(drafts);
}

function roll() {
  if (!now?.lifted) return;
  const box = (now.over ?? now.from).scroller;
  if (box) {
    const r = box.getBoundingClientRect();
    const { px: x, py: y } = now as Lifted;
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
  clearTimeout(now!.timer);
  const { item, from } = now!;
  if (!item.isConnected) return quit();
  now!.lifted = true;
  now!.origin = items(from).indexOf(item);
  now!.box = item.getBoundingClientRect();
  now!.ghost = ghost(item, now!.box);
  now!.px ??= now!.x;
  now!.py ??= now!.y;
  document.documentElement.classList.add("dragging");
  addEventListener("scroll", rescan, true);
  follow();
  now!.frame = requestAnimationFrame(roll);
}

function move(e: PointerEvent) {
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

function pin(e: Event) {
  if (now?.lifted && e.cancelable) e.preventDefault();
}

function rescan() {
  if (now) now.rects = new Map();
}

function hush(e: Event) {
  if (now) e.preventDefault();
}

function swallow() {
  const stop = (e: Event) => {
    e.preventDefault();
    e.stopPropagation();
  };
  addEventListener("click", stop, { capture: true, once: true });
  setTimeout(() => removeEventListener("click", stop, { capture: true }), 0);
}

function end() {
  clearTimeout(now!.timer);
  cancelAnimationFrame(now!.frame ?? 0);
  removeEventListener("pointermove", move, true);
  removeEventListener("pointerup", up, true);
  removeEventListener("pointercancel", cancel, true);
  removeEventListener("keydown", close, true);
  removeEventListener("contextmenu", hush, true);
  removeEventListener("scroll", rescan, true);
  now!.touch.removeEventListener("touchmove", pin);
  document.documentElement.classList.remove("dragging");
  const session = now!;
  now = null;
  return session;
}

function quit() {
  if (now) end();
}

function finish(session: Lifted, back: boolean) {
  const land = back ? null : [...session.drafts].find(([, draft]) => draft.index !== null);
  const spot = land ? items(land[0]).find((kid) => kid.dataset.id === session.id) : null;
  const gone = !back && !land && session.from.out;
  const done = () => {
    session.ghost.remove();
    for (const one of session.touched) one.commit?.(back ? null : (session.drafts.get(one) ?? null));
    busy = false;
    shift(GLIDE);
    if (land) say(`${land[0].name?.(session.id) ?? session.id}, ${land[1].index! + 1} of ${items(land[0]).length}`);
    else if (gone) say(`${session.from.name?.(session.id) ?? session.id} removed`);
  };
  busy = true;
  if (calm()) return done();
  const r = spot?.getBoundingClientRect() ?? session.box;
  const from = { translate: session.ghost.style.translate || "0 0", scale: "1", opacity: 1 };
  const to = gone
    ? { translate: from.translate, scale: "0.4", opacity: 0 }
    : {
        translate: `${r.left - session.box.left}px ${r.top - session.box.top}px`,
        scale: String(r.width / session.box.width || 1),
        opacity: 1,
      };
  session.ghost.animate([from, to], { duration: GLIDE, easing: EASE, fill: "forwards" }).finished.then(done, done);
}

function up(e: PointerEvent) {
  if (!now || e.pointerId !== now.pointer) return;
  if (!now.lifted) return quit();
  swallow();
  finish(end() as Lifted, false);
}

function cancel(e?: PointerEvent) {
  if (!now || (e?.pointerId !== undefined && e.pointerId !== now.pointer)) return;
  if (!now.lifted) return quit();
  finish(end() as Lifted, true);
}

function close(e: KeyboardEvent) {
  if (e.key !== "Escape" || !now?.lifted) return;
  e.preventDefault();
  e.stopPropagation();
  const id = now.pointer;
  const off = () => {
    removeEventListener("pointerup", hit, true);
    removeEventListener("pointercancel", drop, true);
  };
  const hit = (e: PointerEvent) => {
    if (e.pointerId !== id) return;
    off();
    swallow();
  };
  const drop = (e: PointerEvent) => {
    if (e.pointerId === id) off();
  };
  addEventListener("pointerup", hit, true);
  addEventListener("pointercancel", drop, true);
  cancel();
}

function grab(one: List, e: PointerEvent) {
  if (now || busy || e.button !== 0 || e.ctrlKey || e.metaKey || e.altKey || e.shiftKey) return;
  const item = e.target instanceof Element ? e.target.closest<HTMLElement>("[data-id]") : null;
  if (!item || item.parentElement !== one.el || one.fixed) return;
  const control = (e.target as Element).closest('button, input, select, textarea, [contenteditable="true"]');
  if (control && control !== item && item.contains(control)) return;
  now = {
    from: one,
    item,
    touch: e.target as Element,
    id: item.dataset.id!,
    x: e.clientX,
    y: e.clientY,
    pointer: e.pointerId,
    hold: e.pointerType !== "mouse",
    lifted: false,
    drafts: new Map(),
    touched: new Set(),
    rects: new Map(),
  };
  if (now.hold) now.timer = setTimeout(lift, HOLD);
  (e.target as Element).addEventListener("touchmove", pin, { passive: false });
  addEventListener("pointermove", move, true);
  addEventListener("pointerup", up, true);
  addEventListener("pointercancel", cancel, true);
  addEventListener("keydown", close, true);
  addEventListener("contextmenu", hush, true);
}

/* KEYS */

function nudge(one: List, e: KeyboardEvent) {
  if (now || busy || !e.altKey || e.metaKey || e.ctrlKey) return;
  const steps: Partial<Record<string, number>> = { ArrowLeft: -1, ArrowUp: -1, ArrowRight: 1, ArrowDown: 1 };
  const by = steps[e.key];
  const item = e.target instanceof Element ? e.target.closest<HTMLElement>("[data-id]") : null;
  if (!by || !item || item.parentElement !== one.el || !one.takes(one.kind)) return;
  e.preventDefault();
  const list = items(one);
  const to = list.indexOf(item) + by;
  if (to < 0 || to >= list.length) return;
  const id = item.dataset.id!;
  const focus = document.activeElement as HTMLElement | SVGElement | null;
  const before = snapshot();
  one.commit?.({ id, index: to, from: one.kind });
  settle(before);
  if (focus?.isConnected) focus.focus({ preventScroll: true });
  else
    one.el
      .querySelector(`[data-id="${CSS.escape(id)}"]`)
      ?.querySelector<HTMLElement>("a, button, [tabindex]")
      ?.focus({ preventScroll: true });
  say(`${one.name?.(id) ?? id}, ${to + 1} of ${list.length}`);
}

/* LIST */

export function sortable(el: HTMLElement, spec: Spec): () => void {
  const one: List = { el, kind: "", takes: () => "", out: false, axis: "grid", scroller: null, ...spec };
  lists.add(one);
  const down = (e: PointerEvent) => grab(one, e);
  const key = (e: KeyboardEvent) => nudge(one, e);
  const native = (e: Event) => e.preventDefault();
  el.addEventListener("pointerdown", down);
  el.addEventListener("keydown", key);
  el.addEventListener("dragstart", native);
  el.addEventListener("touchmove", pin, { passive: false });
  return () => {
    lists.delete(one);
    el.removeEventListener("pointerdown", down);
    el.removeEventListener("keydown", key);
    el.removeEventListener("dragstart", native);
    el.removeEventListener("touchmove", pin);
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
