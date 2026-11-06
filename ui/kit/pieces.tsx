import type { ComponentProps, FocusEvent, KeyboardEvent, PointerEvent, ReactElement, ReactNode } from "react";
import { cloneElement, isValidElement, useEffect, useId, useRef, useState, useSyncExternalStore } from "react";
import { flushSync } from "react-dom";
import { parts } from "../shell/active.ts";
import type { Act, Day, IconName, Props } from "../shell/design.ts";
import type { Action } from "./controls.tsx";

type Kind = Props["toast"]["kind"];

export type Choice = { label: string; primary?: boolean; danger?: boolean };

export type Asked = { title: string; text?: string; actions?: Choice[] };

type Note = { id: number; text: string; kind: Kind; action?: Action; ms: number };

type Pending = Asked & { id: number; done: (label: string | null) => void };

export type Tab = Props["tabs"]["items"][number];

type TabsProps = { label: string; items: Tab[]; value: string; onChange: (id: string) => void; children?: ReactNode };

type SearchProps = Omit<ComponentProps<"input">, "value" | "onChange"> & {
  label: string;
  value: string;
  onChange: (v: string) => void;
};

type DateProps = { label: string; value: string; onChange: (iso: string) => void; min?: string; max?: string };

const OK: Choice[] = [{ label: "OK", primary: true }];
const MARK: Record<Kind, IconName> = { info: "info", ok: "check", warn: "warn" };
const FEW = 3;
const LONG = 4000;
const WAIT = 500;
const FLING = 96;
const DAY = 864e5;
const MONDAY = "2024-01-01";
const STEPS: Partial<Record<string, number>> = { ArrowLeft: -1, ArrowRight: 1, ArrowUp: -7, ArrowDown: 7 };
const SIDE: Partial<Record<string, number>> = { ArrowLeft: -1, ArrowRight: 1 };
const PAGES: Partial<Record<string, number>> = { PageUp: -1, PageDown: 1 };

const named = (stem: string, id: string) => `${stem}${id.replace(/[^\w-]/g, "")}`;

let live: { notes: Note[]; asks: Pending[] } = { notes: [], asks: [] };
let made = 0;
const ears = new Set<() => void>();

const tell = (next: Partial<typeof live>) => {
  live = { ...live, ...next };
  for (const one of ears) one();
};

const listen = (fn: () => void) => {
  ears.add(fn);
  return () => {
    ears.delete(fn);
  };
};

export const useLive = () =>
  useSyncExternalStore(
    listen,
    () => live,
    () => live,
  );

const drop = (id: number) => tell({ notes: live.notes.filter((one) => one.id !== id) });

export function toast(text: string, { kind = "info", action, ms = LONG }: Partial<Note> = {}) {
  const id = ++made;
  tell({ notes: [...live.notes, { id, text, kind, action, ms }].slice(-FEW) });
  return () => drop(id);
}

export const ask = (asked: Asked) =>
  new Promise<string | null>((resolve) => {
    const one: Pending = {
      ...asked,
      id: ++made,
      done: (label) => {
        tell({ asks: live.asks.filter((each) => each !== one) });
        resolve(label);
      },
    };
    tell({ asks: [...live.asks, one] });
  });

export const sure = async (asked: Asked, yes: Choice) =>
  (await ask({ ...asked, actions: [{ label: "Cancel" }, yes] })) === yes.label;

export function Toast({ note, held }: { note: Note; held: boolean }) {
  useEffect(() => {
    if (held) return undefined;
    const timer = setTimeout(() => drop(note.id), note.ms);
    return () => clearTimeout(timer);
  }, [held]);
  const run = () => {
    drop(note.id);
    note.action?.[1]();
  };
  const action = note.action && { label: note.action[0], bind: () => ({ type: "button", onClick: run }) };
  const Part = parts.toast;
  return <Part text={note.text} kind={note.kind} icon={MARK[note.kind]} action={action} />;
}

function pull(e: PointerEvent<HTMLElement>) {
  const head = e.currentTarget;
  const sheet = head.closest("dialog");
  if (!sheet || e.button !== 0 || (e.target as Element).closest("button")) return;
  const from = e.clientY;
  const by = (at: { clientY: number }) => Math.max(0, at.clientY - from);
  head.setPointerCapture(e.pointerId);
  sheet.dataset.pull = "";
  const move = (at: globalThis.PointerEvent) => sheet.style.setProperty("--pull", `${by(at)}px`);
  const end = (at: globalThis.PointerEvent) => {
    head.removeEventListener("pointermove", move);
    delete sheet.dataset.pull;
    sheet.style.removeProperty("--pull");
    if (at.type === "pointerup" && by(at) > FLING) sheet.close();
  };
  head.addEventListener("pointermove", move);
  head.addEventListener("pointerup", end, { once: true });
  head.addEventListener("pointercancel", end, { once: true });
}

export function Sheet({ title, className, children, ...rest }: ComponentProps<"dialog"> & { title: string }) {
  const Part = parts.sheet;
  return (
    <Part title={title} tag={className} bind={{ root: () => rest, pull: () => ({ onPointerDown: pull }) }}>
      {children}
    </Part>
  );
}

export function Alert({ one }: { one: Pending }) {
  const at = useRef<HTMLDialogElement>(null);
  const id = named("alert", useId());
  const actions = one.actions ?? OK;
  useEffect(() => {
    at.current?.showModal();
    at.current?.querySelector<HTMLElement>("[data-default]")?.focus();
  }, []);
  const main = actions.findIndex((each) => each.primary && !each.danger);
  const safe = actions.findIndex((each) => !each.danger);
  const first = main < 0 ? Math.max(0, safe) : main;
  const acts: Act[] = actions.map((each, i) => ({
    ...each,
    bind: () => ({ onClick: () => at.current?.close(each.label), "data-default": i === first || undefined }),
  }));
  const root = () => ({
    ref: at,
    role: "alertdialog",
    "aria-labelledby": `${id}-title`,
    "aria-describedby": one.text ? `${id}-text` : undefined,
    onClose: () => one.done(at.current?.returnValue || null),
  });
  const Part = parts.alert;
  return (
    <Part
      title={one.title}
      text={one.text}
      actions={acts}
      bind={{ root, title: () => ({ id: `${id}-title` }), text: () => ({ id: `${id}-text` }) }}
    />
  );
}

function useTabs({ label, items, value, onChange }: Omit<TabsProps, "children">): Props["tabs"]["bind"] {
  const base = named("tab", useId());
  const key = (e: KeyboardEvent) => {
    const at = items.findIndex((one) => one.id === value);
    const by = SIDE[e.key];
    const to = e.key === "Home" ? 0 : e.key === "End" ? items.length - 1 : by && at + by;
    if (to === undefined) return;
    e.preventDefault();
    const next = items[(to + items.length) % items.length]!;
    onChange(next.id);
    document.getElementById(`${base}-${next.id}`)?.focus();
  };
  return {
    list: () => ({ role: "tablist", "aria-label": label, onKeyDown: key }),
    tab: (id) => () => ({
      id: `${base}-${id}`,
      type: "button",
      role: "tab",
      "aria-selected": id === value,
      "aria-controls": `${base}-panel`,
      tabIndex: id === value ? 0 : -1,
      onClick: () => onChange(id),
    }),
    panel: () => ({ id: `${base}-panel`, role: "tabpanel", "aria-labelledby": `${base}-${value}`, tabIndex: 0 }),
  };
}

export function Tabs({ children, ...props }: TabsProps) {
  const bind = useTabs(props);
  const Part = parts.tabs;
  return (
    <Part items={props.items} bind={bind}>
      {children}
    </Part>
  );
}

export function Table(props: Props["table"]) {
  const Part = parts.table;
  return <Part {...props} />;
}

let tipped: HTMLElement | null = null;

const quit = (e: globalThis.KeyboardEvent) => {
  if (e.key === "Escape") untip();
};

function untip() {
  if (tipped?.matches(":popover-open")) tipped.hidePopover();
  tipped = null;
  document.removeEventListener("keydown", quit);
}

export function Tooltip({ text, children }: { text: string; children: ReactElement<{ "aria-describedby"?: string }> }) {
  const id = named("tip", useId());
  const timer = useRef<ReturnType<typeof setTimeout>>(undefined);
  const show = () => {
    const tip = document.getElementById(id);
    if (!tip || tip === tipped) return;
    untip();
    tip.showPopover();
    tipped = tip;
    document.addEventListener("keydown", quit);
  };
  const hide = () => {
    clearTimeout(timer.current);
    if (tipped?.id === id) untip();
  };
  const root = () => ({
    style: { anchorName: `--${id}` },
    onPointerEnter: (e: PointerEvent) => {
      if (e.pointerType === "mouse") timer.current = setTimeout(show, WAIT);
    },
    onPointerLeave: hide,
    onPointerDown: hide,
    onFocus: (e: FocusEvent) => {
      if (e.target.matches(":focus-visible")) show();
    },
    onBlur: hide,
  });
  const tip = () => ({ id, role: "tooltip", popover: "manual", style: { positionAnchor: `--${id}` } });
  const Part = parts.tooltip;
  return (
    <Part text={text} bind={{ root, tip }}>
      {isValidElement(children) ? cloneElement(children, { "aria-describedby": id }) : children}
    </Part>
  );
}

export function Search({ label, value, onChange, placeholder, onKeyDown, ...rest }: SearchProps) {
  const own = named("find", useId());
  const id = rest.id ?? own;
  const key = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key !== "Escape" || !value) return onKeyDown?.(e);
    e.preventDefault();
    e.stopPropagation();
    onChange("");
  };
  const field = () => ({ autoComplete: "off", spellCheck: false, ...rest, id, onKeyDown: key });
  const clear = () => ({
    "aria-label": "Clear",
    onClick: () => {
      onChange("");
      document.getElementById(id)?.focus();
    },
  });
  const Part = parts.search;
  return <Part label={label} value={value} placeholder={placeholder} onChange={onChange} bind={{ field, clear }} />;
}

export function Progress({ value = null, max = 1, label }: { value?: number | null; max?: number; label: string }) {
  const Part = parts.progress;
  return <Part value={value} max={max} label={label} />;
}

export function Empty({
  icon,
  title,
  text,
  action,
}: {
  icon?: IconName;
  title: string;
  text?: string;
  action?: Action;
}) {
  const Part = parts.empty;
  const act = action && { label: action[0], bind: () => ({ type: "button", onClick: action[1] }) };
  return <Part icon={icon} title={title} text={text} action={act} />;
}

const at = (iso: string) => new Date(`${iso}T00:00:00Z`);

const isoOf = (time: number) => new Date(time).toISOString().slice(0, 10);

const plus = (iso: string, days: number) => isoOf(at(iso).getTime() + days * DAY);

const weekday = (iso: string) => (at(iso).getUTCDay() + 6) % 7;

const said = (iso: string, how: Intl.DateTimeFormatOptions) =>
  at(iso).toLocaleDateString(undefined, { ...how, timeZone: "UTC" });

function today() {
  const now = new Date();
  return isoOf(Date.UTC(now.getFullYear(), now.getMonth(), now.getDate()));
}

function shift(iso: string, by: number) {
  const day = at(iso);
  const [year, month] = [day.getUTCFullYear(), day.getUTCMonth() + by];
  return isoOf(Date.UTC(year, month, Math.min(day.getUTCDate(), new Date(Date.UTC(year, month + 1, 0)).getUTCDate())));
}

function useCalendar({ label, value, onChange, min = "", max = "" }: DateProps): Props["calendar"] {
  const id = named("date", useId());
  const clamp = (iso: string) => (min && iso < min ? min : max && iso > max ? max : iso);
  const [focus, setFocus] = useState(() => clamp(value || today()));
  const now = today();
  const land = (iso: string) => {
    const to = clamp(iso);
    flushSync(() => setFocus(to));
    document.getElementById(id)?.querySelector<HTMLElement>(`[data-iso="${to}"]`)?.focus();
  };
  const key = (e: KeyboardEvent) => {
    const by = e.key === "Home" ? -weekday(focus) : e.key === "End" ? 6 - weekday(focus) : STEPS[e.key];
    const page = PAGES[e.key];
    if (by === undefined && page === undefined) return;
    e.preventDefault();
    land(page === undefined ? plus(focus, by!) : shift(focus, page));
  };
  const pick = (iso: string) => {
    onChange(iso);
    document.getElementById(id)?.hidePopover();
  };
  const month = focus.slice(0, 7);
  const start = plus(focus, 1 - at(focus).getUTCDate());
  const first = plus(start, -weekday(start));
  const off = (iso: string) => Boolean((min && iso < min) || (max && iso > max));
  const days: Day[] = Array.from({ length: 42 }, (_, i) => {
    const iso = plus(first, i);
    const out = iso.slice(0, 7) !== month;
    return {
      iso,
      n: at(iso).getUTCDate(),
      out,
      bind: () => ({
        type: "button",
        tabIndex: iso === focus ? 0 : -1,
        disabled: out || off(iso),
        "data-iso": iso,
        "aria-label": said(iso, { dateStyle: "full" }),
        "aria-pressed": iso === value,
        "aria-current": iso === now ? "date" : undefined,
        onClick: () => pick(iso),
      }),
    };
  });
  const turn = (by: number) => () => ({
    type: "button",
    "aria-label": by < 0 ? "Previous month" : "Next month",
    disabled: by < 0 ? Boolean(min) && month <= min.slice(0, 7) : Boolean(max) && month >= max.slice(0, 7),
    onClick: () => setFocus(clamp(shift(focus, by))),
  });
  return {
    label,
    text: value ? said(value, { dateStyle: "medium" }) : "",
    month: said(focus, { month: "long", year: "numeric" }),
    week: Array.from({ length: 7 }, (_, i) => said(plus(MONDAY, i), { weekday: "short" })),
    days,
    bind: {
      field: () => ({
        type: "button",
        popoverTarget: id,
        "aria-haspopup": "dialog",
        "aria-label": value ? `${label}, ${said(value, { dateStyle: "full" })}` : label,
        style: { anchorName: `--${id}` },
      }),
      pop: () => ({
        id,
        popover: "auto",
        role: "dialog",
        "aria-label": label,
        style: { positionAnchor: `--${id}` },
        onToggle: (e: { currentTarget: Element }) => {
          if (e.currentTarget.matches(":popover-open")) land(value || today());
        },
      }),
      prev: turn(-1),
      next: turn(1),
      grid: () => ({ role: "group", "aria-label": said(focus, { month: "long", year: "numeric" }), onKeyDown: key }),
    },
  };
}

export function DateField(props: DateProps) {
  const calendar = useCalendar(props);
  const Part = parts.calendar;
  return <Part {...calendar} />;
}
