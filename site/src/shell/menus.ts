import { rows } from "site:routes";
import type { IconName } from "../lib/app/controls.tsx";
import type { Item, Menu, Model } from "../lib/app/menus.ts";
import { label } from "../lib/app/menus.ts";
import { ICONS, VIEWBOX } from "../lib/icons.gen.js";
import type { Note } from "./catalog.ts";
import { app, notes, recent, shelves } from "./catalog.ts";
import { siblings } from "./launch.ts";

const RECENT = 5;
const DROP = 6;
const SVG = "http://www.w3.org/2000/svg";

/* MODEL */

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

/* DRAW */

const mac = () => /Mac|iPhone|iPad/.test(navigator.platform || navigator.userAgent || "");

function glyph(name: IconName) {
  const svg = document.createElementNS(SVG, "svg");
  svg.setAttribute("viewBox", VIEWBOX);
  svg.setAttribute("aria-hidden", "true");
  const path = document.createElementNS(SVG, "path");
  path.setAttribute("d", ICONS[name]);
  svg.append(path);
  return svg;
}

function chord(keys: string[]) {
  const out = document.createElement("span");
  out.className = "chord";
  const apple = mac();
  const parts = keys.map((one) =>
    one === "mod"
      ? apple
        ? glyph("keyboard_command_key")
        : "Ctrl"
      : one === "shift"
        ? apple
          ? glyph("shift")
          : "Shift"
        : label(one),
  );
  parts.forEach((part, n) => {
    if (n && !apple) out.append("+");
    out.append(part);
  });
  out.setAttribute(
    "aria-label",
    keys
      .map((one) => (one === "mod" ? (apple ? "Command" : "Control") : one === "shift" ? "Shift" : label(one)))
      .join(" "),
  );
  return out;
}

function item(row: Item | null) {
  if (!row) {
    const rule = document.createElement("hr");
    rule.setAttribute("role", "separator");
    return rule;
  }
  const one = document.createElement(row.href ? "a" : "button");
  if (row.href) (one as HTMLAnchorElement).href = row.href;
  else (one as HTMLButtonElement).type = "button";
  one.setAttribute("role", row.radio ? "menuitemradio" : "on" in row ? "menuitemcheckbox" : "menuitem");
  if ("on" in row) one.setAttribute("aria-checked", String(Boolean(row.on)));
  if (row.off) one.setAttribute("aria-disabled", "true");
  one.tabIndex = -1;
  if (row.act) one.dataset.act = row.act;
  const tick = document.createElement("span");
  tick.className = "tick";
  if (row.on) tick.append(glyph("check"));
  const text = document.createElement("span");
  text.className = "label";
  text.textContent = String(row.label);
  one.append(tick, text);
  if (row.keys?.length) one.append(chord(row.keys));
  return one;
}

/* OPEN */

const bar = () => document.querySelector(".trio .menus");
const drops = () => [...document.querySelectorAll<HTMLElement>(".pulldown[data-menu]")];
const buttons = () => [...(bar()?.querySelectorAll("button") ?? [])];
const shown = () => drops().find((one) => one.matches(":popover-open")) ?? null;
const live = (drop: HTMLElement) => [
  ...drop.querySelectorAll<HTMLElement>('[role^="menuitem"]:not([aria-disabled="true"])'),
];

function place(drop: HTMLElement, button: HTMLElement) {
  const at = button.getBoundingClientRect();
  drop.style.left = `${Math.max(DROP, Math.min(at.left, innerWidth - drop.offsetWidth - DROP))}px`;
  drop.style.top = `${at.bottom + DROP}px`;
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

/* RENDER */

function render() {
  const nav = bar();
  const was = (drops() as readonly (HTMLElement | null)[]).indexOf(shown());
  const spot = was < 0 ? -1 : (live(drops()[was]!) as readonly (Element | null)[]).indexOf(document.activeElement);
  for (const one of drops()) one.remove();
  if (!nav) return;
  nav.replaceChildren();
  if (!held) return;
  full(held).forEach((menu, n) => {
    const button = document.createElement("button");
    button.type = "button";
    button.textContent = menu.name;
    button.setAttribute("aria-haspopup", "menu");
    button.setAttribute("aria-expanded", "false");
    const drop = document.createElement("div");
    drop.className = "pulldown";
    drop.id = `menu-${menu.name.toLowerCase()}`;
    drop.dataset.menu = menu.name;
    drop.dataset.lens = "menu";
    drop.popover = "auto";
    drop.setAttribute("role", "menu");
    drop.setAttribute("aria-label", menu.name);
    button.setAttribute("aria-controls", drop.id);
    drop.append(...menu.items.map(item));
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

/* WIRE */

function pick(e: Event) {
  const one =
    e.target instanceof Element ? e.target.closest<HTMLElement>('.pulldown[data-menu] [role^="menuitem"]') : null;
  if (!one) return;
  if (one.getAttribute("aria-disabled") === "true") return e.preventDefault();
  hide();
  if (one.dataset.act) held?.run(one.dataset.act);
}

function press(e: KeyboardEvent) {
  const drop = shown();
  const inBar = e.target instanceof Element && e.target.closest(".trio .menus");
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
