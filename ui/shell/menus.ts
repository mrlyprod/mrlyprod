import { rows } from "site:routes";
import type { Item, Menu, Model } from "../kit/menus.ts";
import { button, item, sheet } from "../kit/pulldown.ts";
import { px } from "../kit/px.ts";
import { design } from "./active.ts";
import type { Note } from "./catalog.ts";
import { app, notes, recent, shelves } from "./catalog.ts";
import type { Pulldown } from "./design.ts";
import { siblings } from "./launch.ts";

const RECENT = 5;

const PLAIN: Pulldown = { drop: 6, button, sheet: () => sheet(), item };

const look = () => design.pulldown ?? PLAIN;

let held: Model | null = null;
let topics: Note[] = [];

const here = () => rows.find((row) => row.route === location.pathname)?.meta?.id ?? "";

function trips(id: string, sections = shelves(), last = recent()): (Item | null)[] {
  const visited = last
    .filter((one) => one.id !== id && app(one.id))
    .slice(0, RECENT)
    .map((one) => ({ label: app(one.id)!.title, href: one.url }));
  const near = siblings(sections, id)
    .filter(app)
    .map((one) => ({ label: app(one)!.title, href: app(one)!.route }));
  const parts = [
    [
      { label: "Home", href: "/" },
      { label: "Launchpad", href: "/menu/" },
    ],
    visited,
    near,
  ].filter((part) => part.length);
  return parts.flatMap((part, n) => (n ? [null, ...part] : part));
}

function full(model: Model): Menu[] {
  const id = here();
  const note = topics.find((one) => one.slug === id);
  const go = { name: "Go", items: trips(id) };
  const out = model.menus.map((one) =>
    one.name === "Help" && note
      ? { ...one, items: [...one.items, null, { label: `${note.title} Note`, href: note.href }] }
      : one,
  );
  const at = out.findIndex((one) => one.name === "Help");
  return [...out.slice(0, at < 0 ? out.length : at), go, ...out.slice(at < 0 ? out.length : at)];
}

const mac = () => /Mac|iPhone|iPad/.test(navigator.platform || navigator.userAgent || "");

let given: HTMLElement | null = null;

const bar = () => (given?.isConnected ? given : document.querySelector<HTMLElement>("[data-menus]"));
const drops = () => [...document.querySelectorAll<HTMLElement>("[data-menu][popover]")];
const buttons = () => [...(bar()?.querySelectorAll("button") ?? [])];
const shown = () => drops().find((one) => one.matches(":popover-open")) ?? null;
const live = (drop: HTMLElement) => [
  ...drop.querySelectorAll<HTMLElement>('[role^="menuitem"]:not([aria-disabled="true"])'),
];

function place(drop: HTMLElement, button: HTMLElement) {
  const gap = look().drop;
  const at = button.getBoundingClientRect();
  drop.style.left = px(Math.max(gap, Math.min(at.left, innerWidth - drop.offsetWidth - gap)));
  drop.style.top = px(at.bottom + gap);
}

function show(n: number, focus = "") {
  const button = buttons()[n];
  const drop = drops()[n];
  if (!button || !drop) return;
  if (!drop.matches(":popover-open")) drop.showPopover();
  place(drop, button);
  const items = live(drop);
  if (focus === "first") items[0]?.focus();
  if (focus === "last") items.at(-1)?.focus();
}

function hide() {
  shown()?.hidePopover();
}

function step(drop: HTMLElement, by: number) {
  const items = live(drop);
  const at = (items as readonly (Element | null)[]).indexOf(document.activeElement);
  items[(at + by + items.length) % items.length]?.focus();
}

function turn(by: number, focus?: string) {
  const all = buttons();
  const at = (drops() as readonly (HTMLElement | null)[]).indexOf(shown());
  const from = at < 0 ? (all as readonly (Element | null)[]).indexOf(document.activeElement) : at;
  const next = (from + by + all.length) % all.length;
  if (at < 0) return all[next]?.focus();
  hide();
  if (!focus) all[next]?.focus({ preventScroll: true });
  show(next, focus);
}

function render() {
  const nav = bar();
  const was = (drops() as readonly (HTMLElement | null)[]).indexOf(shown());
  const spot = was < 0 ? -1 : (live(drops()[was]!) as readonly (Element | null)[]).indexOf(document.activeElement);
  for (const one of drops()) one.remove();
  if (!nav) return;
  nav.replaceChildren();
  if (!held) return;
  const apple = mac();
  const drawn = look();
  full(held).forEach((menu, n) => {
    const button = drawn.button(menu.name);
    button.setAttribute("aria-haspopup", "menu");
    button.setAttribute("aria-expanded", "false");
    const drop = drawn.sheet(menu.name);
    drop.id = `menu-${menu.name.toLowerCase()}`;
    drop.dataset.menu = menu.name;
    drop.popover = "auto";
    drop.setAttribute("role", "menu");
    drop.setAttribute("aria-label", menu.name);
    button.setAttribute("aria-controls", drop.id);
    drop.append(...menu.items.map((row) => drawn.item(row, apple)));
    drop.addEventListener("toggle", (e) => {
      const open = e.newState === "open";
      button.setAttribute("aria-expanded", String(open));
      button.classList.toggle("on", open);
      if (!open) return;
      place(drop, button);
      dispatchEvent(new CustomEvent("shift", { detail: 300 }));
    });
    button.popoverTargetElement = drop;
    button.addEventListener("pointerenter", () => {
      if (shown() && !drop.matches(":popover-open")) {
        hide();
        show(n);
      }
    });
    nav.append(button);
    document.body.append(drop);
  });
  if (was < 0) return;
  show(was);
  if (spot >= 0) live(drops()[was]!)[spot]?.focus();
}

function pick(e: Event) {
  const one =
    e.target instanceof Element ? e.target.closest<HTMLElement>('[data-menu][popover] [role^="menuitem"]') : null;
  if (!one) return;
  if (one.getAttribute("aria-disabled") === "true") return e.preventDefault();
  hide();
  if (one.dataset.act) held?.run(one.dataset.act);
}

function press(e: KeyboardEvent) {
  const drop = shown();
  const inBar = e.target instanceof Element && e.target.closest("[data-menus]");
  if (!drop && !inBar) return;
  const keys: Partial<Record<string, () => void>> = drop
    ? {
        ArrowDown: () => step(drop, 1),
        ArrowUp: () => step(drop, -1),
        ArrowRight: () => turn(1, "first"),
        ArrowLeft: () => turn(-1, "first"),
        Home: () => live(drop)[0]?.focus(),
        End: () => live(drop).at(-1)?.focus(),
      }
    : {
        ArrowDown: () => show((buttons() as readonly (EventTarget | null)[]).indexOf(e.target), "first"),
        ArrowUp: () => show((buttons() as readonly (EventTarget | null)[]).indexOf(e.target), "last"),
        ArrowRight: () => turn(1),
        ArrowLeft: () => turn(-1),
      };
  const fn = keys[e.key];
  if (fn) {
    e.preventDefault();
    fn();
    return;
  }
  if (drop && e.key === "Escape") {
    e.preventDefault();
    const at = drops().indexOf(drop);
    hide();
    buttons()[at]?.focus();
  }
}

let asked = false;

function learn() {
  if (asked) return;
  asked = true;
  notes().then((list) => {
    topics = list;
    if (held) render();
  });
}

export function host(one: HTMLElement | null) {
  if (!one || one === given) return;
  given = one;
  render();
}

export function menus() {
  addEventListener("menus", (e: CustomEvent<Model | null>) => {
    held = e.detail ?? null;
    if (held) learn();
    render();
  });
  addEventListener("wire", () => {
    if (held) render();
  });
  document.addEventListener("click", pick);
  document.addEventListener("keydown", press);
}
