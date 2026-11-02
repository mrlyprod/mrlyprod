import { afterEach, beforeEach, expect, test } from "bun:test";
import type { Draft, Rect, Spec } from "./drag.ts";
import { dragging, placed, slot, sortable } from "./drag.ts";

type Fake = HTMLElement & {
  on: Record<string, EventListener>;
  how: Record<string, boolean | AddEventListenerOptions | undefined>;
  children: Fake[];
};

const box = (left: number, top: number, width = 10, height = 10): Rect => ({
  left,
  top,
  width,
  height,
  right: left + width,
  bottom: top + height,
});

class Element {}

function node(id?: string | null, at = box(0, 0)): Fake {
  const on: Record<string, EventListener> = {};
  const how: Record<string, boolean | AddEventListenerOptions | undefined> = {};
  const children: Fake[] = [];
  const self: Fake = Object.assign(new Element() as HTMLElement, {
    dataset: id ? { id } : {},
    children,
    style: {},
    on,
    how,
    isConnected: true,
    classList: { add() {}, remove() {} },
    closest: (sel: string) => (sel === "[data-id]" && id ? self : null),
    getBoundingClientRect: () => at,
    getAnimations: () => [],
    querySelector: () => null,
    querySelectorAll: () => [],
    cloneNode: () => node(),
    addEventListener: (type: string, fn: EventListener, options?: boolean | AddEventListenerOptions) => {
      on[type] = fn;
      how[type] = options;
    },
    removeEventListener() {},
    setAttribute() {},
    removeAttribute() {},
    toggleAttribute() {},
    append() {},
    remove() {},
    focus() {},
  });
  return self;
}

function list(at: Rect, ...ids: string[]) {
  const el = node(null, at);
  Object.assign(el, {
    children: ids.map((id, i) => Object.assign(node(id, box(at.left + 12 * i, at.top)), { parentElement: el })),
  });
  return el;
}

const event = (type: string, props: Record<string, unknown> = {}) =>
  Object.defineProperties(
    new Event(type, { cancelable: true }),
    Object.fromEntries(Object.entries(props).map(([name, value]) => [name, { value }])),
  );
const press = (el: Fake, at: number, props: Record<string, unknown> = {}) =>
  el.on.pointerdown(
    event("pointerdown", {
      button: 0,
      target: el.children[at],
      pointerId: 1,
      pointerType: "mouse",
      clientX: 1,
      clientY: 1,
      ...props,
    }),
  );
const send = (type: string, props: Record<string, unknown> = {}) =>
  dispatchEvent(event(type, { pointerId: 1, ...props }));
const KEEP = ["Element", "document", "getComputedStyle", "requestAnimationFrame", "cancelAnimationFrame"];

let undo: (() => void)[] = [];

beforeEach(() => {
  Object.assign(globalThis, {
    Element,
    document: {
      documentElement: Object.assign(node(), { dataset: { motion: "reduce" } }),
      body: node(),
      createElement: () => node(),
      activeElement: node(),
    },
    getComputedStyle: () => [],
    requestAnimationFrame: () => 0,
    cancelAnimationFrame() {},
  });
});

afterEach(() => {
  send("pointercancel");
  for (const off of undo) off();
  undo = [];
  for (const name of KEEP) Reflect.deleteProperty(globalThis, name);
});

function rig(at: Rect, ids: string[], spec?: Spec) {
  const el = list(at, ...ids);
  const log: (Draft | null)[] = [];
  undo.push(sortable(el, { kind: "app", takes: () => "move", commit: (draft) => log.push(draft), ...spec }));
  return { el, log, off: undo.at(-1)! };
}

test("a draft moves its id to the index among the others, null takes it out, and no draft keeps the list", () => {
  const list = ["a", "b", "c"];
  expect(placed(list, null)).toBe(list);
  expect(placed(list, { id: "a", index: 1 })).toEqual(["b", "a", "c"]);
  expect(placed(list, { id: "c", index: 0 })).toEqual(["c", "a", "b"]);
  expect(placed(list, { id: "b", index: null })).toEqual(["a", "c"]);
  expect(placed(list, { id: "z", index: 9 })).toEqual(["a", "b", "c", "z"]);
});

test("the slot is the first item the pointer reads before: rows by reading order, a row by x, a column by y", () => {
  const grid = [box(0, 0), box(20, 0), box(0, 20), box(20, 20)];
  expect(slot(grid, 2, 5)).toBe(0);
  expect(slot(grid, 8, 5)).toBe(1);
  expect(slot(grid, 40, 5)).toBe(2);
  expect(slot(grid, 0, 15)).toBe(2);
  expect(slot(grid, 26, 25)).toBe(4);
  expect(slot([], 0, 0)).toBe(0);
  const row = [box(0, 10), box(20, 10)];
  expect(slot(row, 12, 0, "x")).toBe(1);
  const column = [box(0, 0, 100), box(0, 20, 100)];
  expect(slot(column, 90, 12, "y")).toBe(1);
});

test("Alt with an arrow is always claimed in a list that reorders itself, even at its edge, and left alone in one that does not", () => {
  const { el, log } = rig(box(0, 0, 40), ["a", "b"]);
  const edge = event("keydown", { key: "ArrowLeft", altKey: true, target: el.children[0] });
  el.on.keydown(edge);
  expect(edge.defaultPrevented).toBe(true);
  expect(log).toEqual([]);
  el.on.keydown(event("keydown", { key: "ArrowRight", altKey: true, target: el.children[0] }));
  expect(log).toEqual([{ id: "a", index: 1, from: "app" }]);
  const shelf = rig(box(0, 50, 40), ["c", "d"], { takes: () => "" });
  const left = event("keydown", { key: "ArrowRight", altKey: true, target: shelf.el.children[0] });
  shelf.el.on.keydown(left);
  expect(left.defaultPrevented).toBe(false);
});

test("a pen holds like a finger: a short move does not lift it, though it would lift a mouse", () => {
  const { el } = rig(box(0, 0, 40), ["a", "b"]);
  press(el, 0, { pointerType: "pen" });
  send("pointermove", { clientX: 8, clientY: 1 });
  expect(dragging()).toBe(false);
});

test("a list blocks touch scrolling from setup but only once its item lifts, then also from the pressed item, and never from the page", () => {
  const { el } = rig(box(0, 0, 40), ["a", "b"]);
  expect(el.how.touchmove).toEqual({ passive: false });
  const before = event("touchmove");
  el.on.touchmove(before);
  expect(before.defaultPrevented).toBe(false);
  press(el, 0);
  send("pointermove", { clientX: 9, clientY: 1 });
  const during = event("touchmove");
  el.on.touchmove(during);
  expect(during.defaultPrevented).toBe(true);
  const held = event("touchmove");
  el.children[0].on.touchmove(held);
  expect(held.defaultPrevented).toBe(true);
  const page = event("touchmove");
  dispatchEvent(page);
  expect(page.defaultPrevented).toBe(false);
});

test("a pointercancel from another pointer leaves the drag alone; its own cancels it", () => {
  const { el, log } = rig(box(0, 0, 40), ["a", "b"]);
  press(el, 0);
  send("pointermove", { clientX: 9, clientY: 1 });
  send("pointercancel", { pointerId: 2 });
  expect(dragging()).toBe(true);
  send("pointercancel");
  expect(dragging()).toBe(false);
  expect(log.at(-1)).toBe(null);
});

test("Escape cancels a lift and eats the click its release makes", () => {
  const { el } = rig(box(0, 0, 40), ["a", "b"]);
  press(el, 0);
  send("pointermove", { clientX: 9, clientY: 1 });
  dispatchEvent(event("keydown", { key: "Escape" }));
  expect(dragging()).toBe(false);
  send("pointerup", { pointerId: 2 });
  const other = event("click");
  dispatchEvent(other);
  expect(other.defaultPrevented).toBe(false);
  send("pointerup");
  const click = event("click");
  dispatchEvent(click);
  expect(click.defaultPrevented).toBe(true);
});

test("a release that ends in pointercancel after Escape leaves the next click alone", () => {
  const { el } = rig(box(0, 0, 40), ["a", "b"]);
  press(el, 0);
  send("pointermove", { clientX: 9, clientY: 1 });
  dispatchEvent(event("keydown", { key: "Escape" }));
  send("pointercancel");
  send("pointerup");
  const click = event("click");
  dispatchEvent(click);
  expect(click.defaultPrevented).toBe(false);
});

test("a source list that unmounts mid-drag releases the lists the drag touched", async () => {
  const from = rig(box(0, 0, 40), ["a", "b"]);
  const to = rig(box(0, 100, 40), ["c"]);
  press(from.el, 0);
  send("pointermove", { clientX: 5, clientY: 105 });
  expect(dragging()).toBe(true);
  from.off();
  expect(to.log).toEqual([]);
  await Promise.resolve();
  expect(to.log).toEqual([null]);
});

test("a drag near the end of a sideways scroller rolls it along x and leaves y alone", () => {
  const scroller = Object.assign(node(null, box(0, 0, 200, 10)), {
    scrollWidth: 400,
    clientWidth: 200,
    scrollHeight: 10,
    clientHeight: 10,
    scrollLeft: 0,
    scrollTop: 0,
  });
  const { el } = rig(box(0, 0, 400), ["a", "b"], { axis: "x", scroller });
  let tick = null as (() => void) | null;
  Object.assign(globalThis, {
    requestAnimationFrame: (fn: () => void) => {
      tick = fn;
      return 1;
    },
  });
  press(el, 0);
  send("pointermove", { clientX: 190, clientY: 5 });
  tick!();
  expect(scroller.scrollLeft).toBe(14);
  expect(scroller.scrollTop).toBe(0);
});
