import type { MouseEvent as Clicked, ComponentType } from "react";
import { useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { flushSync } from "react-dom";
import type { Mrly } from "../../apps/shared/designs/engine.ts";
import { classes, dimension } from "../../apps/shared/designs/engine.ts";
import { useTheme } from "../../apps/shared/designs/pick.tsx";
import { thumb } from "../../apps/shared/designs/thumb.ts";
import { stamp } from "../sdk/clock.ts";
import { paints } from "../sdk/color.ts";
import type { Draft } from "../sdk/drag.ts";
import { placed, sortable } from "../sdk/drag.ts";
import { useUnit } from "../sdk/hooks.ts";
import { px } from "../sdk/px.ts";
import { parts } from "./active.ts";
import { AppIcon } from "./appicon.tsx";
import type { Note } from "./catalog.ts";
import { app, KNOWN, notes, recent } from "./catalog.ts";
import { ago, daily, gist, today } from "./launch.ts";
import type { Spot } from "./lens.ts";
import { measure } from "./measure.ts";
import { show } from "./parts.tsx";
import { KEY, read, root, subscribe, write } from "./prefs.ts";
import type { Wired } from "./route.ts";

/* TYPES */

export type Widget = { id: string; name: string; size: "wide" | "small"; Body: ComponentType };

export type Daily = { dim: number; base: number; code: string; label: string; href: string };

export type Box = { w: number; h: number };

type Hooks = { preview: (next: Draft | null) => void; commit: (next: Draft | null) => void };

const MINUTE = 60000;
const SPACES: [number, number][] = [
  [2, 2],
  [3, 2],
  [2, 3],
];
const SKIP = "a, button, input, textarea, select, [popover], .bar, .docked";
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

/* DAY */

export function chosen(math: Mrly, day: string): Daily | null {
  const pool = SPACES.flatMap(([dim, base]) =>
    (classes(math, dim, base) ?? [])
      .map((one) => String(one.code))
      .filter((code) => {
        const d = dimension(math, dim, base, code);
        return d > dim / 2 && d < dim;
      })
      .map((code) => ({ dim, base, code })),
  );
  const one = pool[daily(day, pool.length)];
  if (!one) return null;
  const name = math.name.of(one.code, one.dim, one.base) ?? "code";
  return {
    ...one,
    label: `${name} ${one.code} · ${one.dim}D${one.base === 3 ? ", base 3" : ""}`,
    href: `/designs/?dim=${one.dim}&base=${one.base}&code=${one.code}`,
  };
}

function Day() {
  const math = useUnit("math");
  const tick = useTheme();
  const now = useNow();
  const day = today(now);
  const pick = useMemo(() => (math ? chosen(math, day) : null), [math, day]);
  const art = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    const canvas = art.current;
    if (!canvas || !pick) return;
    const side = Math.round((canvas.getBoundingClientRect().width || 96) * Math.min(devicePixelRatio || 1, 2));
    canvas.width = side;
    canvas.height = side;
    thumb(canvas.getContext("2d")!, math!, pick.dim, pick.base, pick.code, side, paints(canvas));
  }, [pick, tick]);
  return (
    <a className="fill" href={pick?.href ?? "/designs/"}>
      <h2>Design of the day</h2>
      <canvas className="art" ref={art} aria-hidden="true" />
      <p>{pick?.label ?? "Designs"}</p>
    </a>
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

/* RESEARCH */

function Research() {
  const [topics, setTopics] = useState<Note[] | null>(null);
  useEffect(() => {
    let on = true;
    notes().then((list) => on && setTopics(list));
    return () => {
      on = false;
    };
  }, []);
  const top = topics?.length
    ? [...topics].sort((a, b) => (b.date ?? "").localeCompare(a.date ?? "") || a.title.localeCompare(b.title))[0]
    : null;
  if (!top) {
    return (
      <a className="fill" href="/research/">
        <h2>Research</h2>
        <p className="dim">{topics ? "The notes are on their way." : ""}</p>
      </a>
    );
  }
  const kin = KNOWN.has(top.slug) ? app(top.slug) : null;
  const when = top.date
    ? new Date(`${top.date}T12:00`).toLocaleDateString(undefined, { month: "short", day: "numeric" })
    : "";
  const n = top.figures?.length ?? 0;
  return (
    <>
      <a className="fill" href={top.href}>
        <span className="visually-hidden">{top.title}</span>
      </a>
      <h2>Research{when ? ` · ${when}` : ""}</h2>
      <h3>{top.title}</h3>
      <p className="lead">{top.lead}</p>
      <p className="tags">
        {n > 0 && <span className="chip">{n === 1 ? "1 figure" : `${n} figures`}</span>}
        {kin && (
          <a className="chip" href={kin.route}>
            {kin.title} app
          </a>
        )}
      </p>
    </>
  );
}

/* CATALOG */

export const WIDGETS: Widget[] = [
  { id: "clock", name: "Clock", size: "wide", Body: Clock },
  { id: "day", name: "Design of the day", size: "small", Body: Day },
  { id: "continue", name: "Continue", size: "small", Body: Continue },
  { id: "research", name: "Research", size: "wide", Body: Research },
];

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
      else if (!ours(target) && !target.closest(".bar")) setEditing(false);
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

  useEffect(() => {
    const box = pop.current;
    if (!menu || !box) return undefined;
    box.showPopover?.();
    const { x, y } = clamp(menu, box.offsetWidth, box.offsetHeight, { w: innerWidth, h: innerHeight });
    box.style.setProperty("--x", px(x));
    box.style.setProperty("--y", px(y));
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

  return (
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
      menu={
        menu && {
          list: () => ({ ref: pop, popover: "auto" }),
          edit: () => ({
            onClick: () => {
              setMenu(null);
              setEditing(true);
            },
          }),
          wallpaper: () => ({ href: "/settings/desktop/", onClick: () => pop.current?.hidePopover?.() }),
        }
      }
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
  );
}

export function start() {
  if (!parts.widgets) return;
  const host = document.createElement("div");
  host.className = "desked";
  document.body.append(host);
  show(host, <Widgets />);
}
