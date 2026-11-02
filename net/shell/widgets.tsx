import { order, widgets } from "site:widgets";
import type { MouseEvent as Clicked } from "react";
import { useEffect, useLayoutEffect, useRef, useState, useSyncExternalStore } from "react";
import { flushSync } from "react-dom";
import { stamp } from "../sdk/clock.ts";
import type { Draft } from "../sdk/drag.ts";
import { placed, sortable } from "../sdk/drag.ts";
import { px } from "../sdk/px.ts";
import type { Desk } from "../sdk/rows.ts";
import { parts } from "./active.ts";
import { AppIcon } from "./appicon.tsx";
import { app, recent } from "./catalog.ts";
import { ago, gist } from "./launch.ts";
import type { Spot } from "./lens.ts";
import { measure } from "./measure.ts";
import { show } from "./parts.tsx";
import { KEY, read, root, subscribe, write } from "./prefs.ts";
import type { Wired } from "./route.ts";

/* TYPES */

export type Widget = Desk & { id: string };

export type Box = { w: number; h: number };

type Hooks = { preview: (next: Draft | null) => void; commit: (next: Draft | null) => void };

const MINUTE = 60000;
const SKIP = "a, button, input, textarea, select, [popover], .barred, .docked";
const EDGE = measure.edge;

const two = (n: number) => String(n).padStart(2, "0");
const shift = (ms = 0) => dispatchEvent(new CustomEvent("shift", { detail: ms }));
const lost = () => !document.activeElement || document.activeElement === document.body;

/* TIME */

function useNow(step = MINUTE) {
  const [now, setNow] = useState(() => new Date());
  useEffect(() => {
    let timer: ReturnType<typeof setTimeout> | 0 = 0;
    const tick = () => {
      setNow(new Date());
      timer = setTimeout(tick, step - (Date.now() % step) + 50);
    };
    const back = () => !document.hidden && setNow(new Date());
    timer = setTimeout(tick, step - (Date.now() % step) + 50);
    document.addEventListener("visibilitychange", back);
    return () => {
      clearTimeout(timer);
      document.removeEventListener("visibilitychange", back);
    };
  }, [step]);
  return now;
}

function useRecent() {
  const [list, setList] = useState(recent);
  useEffect(() => {
    const fresh = () => setList(recent());
    addEventListener("recent", fresh);
    return () => removeEventListener("recent", fresh);
  }, []);
  return list;
}

/* CLOCK */

function Clock() {
  const now = useNow();
  const hours = useSyncExternalStore(
    subscribe,
    () => read(KEY.hours),
    () => "",
  );
  const zone = Intl.DateTimeFormat().resolvedOptions().timeZone ?? "";
  const city = zone.includes("/") ? zone.split("/").pop()!.replace(/_/g, " ") : "Clock";
  return (
    <>
      <h2>{city}</h2>
      <time className="big" dateTime={`${two(now.getHours())}:${two(now.getMinutes())}`}>
        {stamp(now, hours)}
      </time>
      <p>{now.toLocaleDateString(undefined, { weekday: "long", day: "numeric", month: "long" })}</p>
    </>
  );
}

/* CONTINUE */

function Continue() {
  const list = useRecent();
  useNow();
  const last = list.find((one) => app(one.id)?.meta?.scene) ?? list[0];
  const one = last ? app(last.id) : null;
  if (!one) {
    return (
      <a className="fill" href="/menu/">
        <h2>Continue</h2>
        <p className="dim">Open an app and it waits here.</p>
      </a>
    );
  }
  return (
    <a className="fill" href={last!.url}>
      <h2>Continue</h2>
      <span className="who">
        <AppIcon id={last!.id} />
        <span>
          <b>{one.title}</b>
          <small>{gist(last!.url)}</small>
        </span>
      </span>
      <p className="dim">{ago(Number(last!.at))}</p>
    </a>
  );
}

/* CATALOG */

const OWN: Record<string, Desk> = {
  clock: { name: "Clock", size: "wide", Body: Clock },
  continue: { name: "Continue", size: "small", Body: Continue },
};

export const WIDGETS: Widget[] = order.flatMap((id) => {
  const one = widgets[id] ?? OWN[id];
  return one ? [{ id, ...one }] : [];
});

const BY = new Map(WIDGETS.map((one) => [one.id, one] as const));

export function kept(text: string): string[] {
  try {
    const list: unknown = text ? JSON.parse(text) : null;
    if (!Array.isArray(list)) return WIDGETS.map((one) => one.id);
    return [...new Set(list as unknown[])].filter((id): id is string => typeof id === "string" && BY.has(id));
  } catch {
    return WIDGETS.map((one) => one.id);
  }
}

/* MENU */

const DESK = [{ label: "Edit Widgets" }, { label: "Change Wallpaper", href: "/settings/desktop/" }];

export const clamp = (at: Spot, w: number, h: number, view: Box): Spot => ({
  x: Math.max(EDGE, Math.min(at.x, view.w - w - EDGE)),
  y: Math.max(EDGE, Math.min(at.y, view.h - h - EDGE)),
});

/* WIDGETS */

function Widgets() {
  const [list, setList] = useState(() => kept(read(KEY.widgets)));
  const [draft, setDraft] = useState<Draft | null>(null);
  const [editing, setEditing] = useState(false);
  const [menu, setMenu] = useState<Spot | null>(null);
  const live = useRef(list);
  const aside = useRef<HTMLElement | null>(null);
  const column = useRef<HTMLElement | null>(null);
  const edit = useRef<HTMLElement | null>(null);
  const done = useRef<HTMLElement | null>(null);
  const gallery = useRef<HTMLElement | null>(null);
  const shelf = useRef<HTMLElement | null>(null);
  const pop = useRef<HTMLElement | null>(null);
  const was = useRef(false);
  const hooks = useRef<Hooks | null>(null);

  const keep = (next: string[]) => {
    live.current = next;
    write(KEY.widgets, JSON.stringify(next));
    setList(next);
  };

  hooks.current = {
    preview: (next) => flushSync(() => setDraft(next)),
    commit: (next) =>
      flushSync(() => {
        setDraft(null);
        if (next) keep(placed(live.current, next));
      }),
  };

  const ours = (target: Element) =>
    [aside.current, column.current, gallery.current].some((one) => one?.contains(target));

  useEffect(() => {
    const el = column.current;
    if (!el) return undefined;
    return sortable(el, {
      kind: "widgets",
      scroller: el.parentElement,
      takes: (from) => (from === "widgets" ? "move" : from === "catalog" ? "copy" : ""),
      name: (id) => BY.get(id)?.name ?? id,
      preview: (next) => hooks.current!.preview(next),
      commit: (next) => hooks.current!.commit(next),
    });
  }, []);

  useEffect(
    () =>
      subscribe((key) => {
        if (key && key !== KEY.widgets) return;
        const next = kept(read(KEY.widgets));
        if (JSON.stringify(next) === JSON.stringify(live.current)) return;
        live.current = next;
        setList(next);
      }),
    [],
  );

  useEffect(() => {
    if (editing) root().dataset.editing = "";
    else delete root().dataset.editing;
    if (!editing) return undefined;
    const row = shelf.current;
    const off = row ? sortable(row, { kind: "catalog", axis: "x", name: (id) => BY.get(id)?.name ?? id }) : null;
    const box = gallery.current;
    const watch = new ResizeObserver(() => box && root().style.setProperty("--gallery", px(box.offsetHeight)));
    if (box) watch.observe(box);
    done.current?.focus();
    const key = (e: KeyboardEvent) => {
      if (e.key !== "Escape" || e.defaultPrevented) return;
      e.preventDefault();
      setEditing(false);
    };
    let armed: number | null = null;
    const press = (e: PointerEvent) => {
      const target = e.target instanceof Element ? e.target : null;
      armed = null;
      if (!target) return;
      if (target === aside.current || target === column.current) armed = e.pointerId;
      else if (!ours(target) && !target.closest(".barred")) setEditing(false);
    };
    const up = (e: PointerEvent) => e.pointerId === armed && setEditing(false);
    const cancel = () => {
      armed = null;
    };
    const wire = (e: CustomEvent<Wired>) => e.detail?.route !== "/" && setEditing(false);
    document.addEventListener("keydown", key);
    document.addEventListener("pointerdown", press);
    document.addEventListener("pointerup", up);
    document.addEventListener("pointercancel", cancel);
    addEventListener("wire", wire);
    return () => {
      off?.();
      watch.disconnect();
      root().style.removeProperty("--gallery");
      document.removeEventListener("keydown", key);
      document.removeEventListener("pointerdown", press);
      document.removeEventListener("pointerup", up);
      document.removeEventListener("pointercancel", cancel);
      removeEventListener("wire", wire);
    };
  }, [editing]);

  useEffect(() => {
    if (was.current && !editing && lost()) edit.current?.focus();
    was.current = editing;
  }, [editing]);

  useEffect(() => {
    const open = (e: MouseEvent) => {
      const target = e.target instanceof Element ? e.target : null;
      if (
        e.defaultPrevented ||
        !root().hasAttribute("data-desk") ||
        root().dataset.welcome === "open" ||
        !target ||
        target.closest(SKIP) ||
        gallery.current?.contains(target)
      )
        return;
      e.preventDefault();
      setMenu({ x: e.clientX, y: e.clientY });
    };
    document.addEventListener("contextmenu", open);
    return () => document.removeEventListener("contextmenu", open);
  }, []);

  useLayoutEffect(() => {
    const box = pop.current;
    if (!menu || !box) return undefined;
    if (!box.matches(":popover-open")) box.showPopover?.();
    const at = clamp(menu, box.offsetWidth, box.offsetHeight, { w: innerWidth, h: innerHeight });
    if (at.x !== menu.x || at.y !== menu.y) setMenu(at);
    box.querySelector<HTMLElement>("button, a")?.focus();
    const gone = (e: ToggleEvent) => e.newState === "closed" && setMenu(null);
    box.addEventListener("toggle", gone);
    return () => box.removeEventListener("toggle", gone);
  }, [menu]);

  useEffect(() => {
    shift();
  });

  if (!parts.widgets) return null;
  const inert = (e: Clicked<HTMLElement>) => {
    if (editing && e.target instanceof Element && e.target.closest("a")) e.preventDefault();
  };
  const remove = (gone: string) => {
    const next = live.current[live.current.indexOf(gone) + 1];
    flushSync(() => keep(live.current.filter((one) => one !== gone)));
    ((next && column.current?.querySelector<HTMLElement>(`[data-id="${next}"] button`)) || done.current)?.focus();
  };
  const held = draft && draft.index !== null ? draft.id : null;
  const cards = placed(list, draft).flatMap((id) => {
    const one = BY.get(id);
    return one ? [{ id, name: one.name, size: one.size, body: <one.Body />, held: id === held }] : [];
  });

  const picks = [
    () => {
      setMenu(null);
      setEditing(true);
    },
    () => pop.current?.hidePopover?.(),
  ];

  return (
    <>
      <parts.widgets
        cards={cards}
        editing={editing}
        gallery={WIDGETS.map((one) => ({
          id: one.id,
          name: one.name,
          size: one.size,
          body: <one.Body />,
          added: list.includes(one.id),
        }))}
        onEdit={setEditing}
        onAdd={(id) => keep([...live.current, id])}
        onRemove={remove}
        bind={{
          root: () => ({ ref: aside }),
          column: () => ({ ref: column, onClickCapture: inert }),
          card: (id) => () => ({ "data-id": id }),
          edit: () => ({ ref: edit }),
          gallery: () => ({ ref: gallery }),
          shelf: () => ({ ref: shelf }),
          pick: (id) => () => ({ "data-id": list.includes(id) ? undefined : id }),
          done: () => ({ ref: done }),
        }}
      />
      {menu && (
        <parts.menu
          name="Desktop"
          items={DESK}
          anchor={new DOMRect(menu.x, menu.y, 0, 0)}
          mac={false}
          bind={{ list: () => ({ ref: pop, popover: "auto" }), item: (i) => () => ({ onClick: picks[i] }) }}
        />
      )}
    </>
  );
}

export function start() {
  if (!parts.widgets) return;
  const host = document.createElement("div");
  host.className = "desked";
  document.body.append(host);
  show(host, <Widgets />);
}
