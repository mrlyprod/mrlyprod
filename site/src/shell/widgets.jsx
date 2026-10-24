import { useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { flushSync } from "react-dom";
import { useUnit } from "../lib/app/hooks.js";
import { paints } from "../lib/color.js";
import { classes, dimension } from "../lib/designs/engine.js";
import { useTheme } from "../lib/designs/pick.jsx";
import { thumb } from "../lib/designs/thumb.js";
import { placed, sortable } from "../lib/drag.js";
import { AppIcon } from "./appicon.jsx";
import { app, KNOWN, notes, recent } from "./catalog.js";
import { ago, daily, gist, today } from "./launch.js";
import { Glyph, show } from "./parts.jsx";
import { KEY, read, root, stamp, subscribe, write } from "./prefs.js";

const MINUTE = 60000;
const SPACES = [
  [2, 2],
  [3, 2],
  [2, 3],
];
const SKIP = "a, button, input, textarea, select, .bar, .dock, .catalog, .menu";
const EDGE = 8;
const BARE = ".widgets, .widgets .column";
const KEEP = ".widgets, .catalog, .bar";

const two = (n) => String(n).padStart(2, "0");
const shift = (ms = 0) => dispatchEvent(new CustomEvent("shift", { detail: ms }));
const lost = () => !document.activeElement || document.activeElement === document.body;

/* TIME */

function useNow(step = MINUTE) {
  const [now, setNow] = useState(() => new Date());
  useEffect(() => {
    let timer = 0;
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
  const city = zone.includes("/") ? zone.split("/").pop().replace(/_/g, " ") : "Clock";
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

export function chosen(math, day) {
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
  const art = useRef(null);
  useEffect(() => {
    const canvas = art.current;
    if (!canvas || !pick) return;
    const side = Math.round((canvas.getBoundingClientRect().width || 96) * Math.min(devicePixelRatio || 1, 2));
    canvas.width = side;
    canvas.height = side;
    thumb(canvas.getContext("2d"), math, pick.dim, pick.base, pick.code, side, paints(canvas));
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
    <a className="fill" href={last.url}>
      <h2>Continue</h2>
      <span className="who">
        <AppIcon id={last.id} />
        <span>
          <b>{one.title}</b>
          <small>{gist(last.url)}</small>
        </span>
      </span>
      <p className="dim">{ago(last.at)}</p>
    </a>
  );
}

/* RESEARCH */

function Research() {
  const [topics, setTopics] = useState(null);
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

export const WIDGETS = [
  { id: "clock", name: "Clock", size: "wide", Body: Clock },
  { id: "day", name: "Design of the day", size: "small", Body: Day },
  { id: "continue", name: "Continue", size: "small", Body: Continue },
  { id: "research", name: "Research", size: "wide", Body: Research },
];

const BY = new Map(WIDGETS.map((one) => [one.id, one]));

export function kept(text) {
  try {
    const list = text ? JSON.parse(text) : null;
    if (!Array.isArray(list)) return WIDGETS.map((one) => one.id);
    return [...new Set(list)].filter((id) => BY.has(id));
  } catch {
    return WIDGETS.map((one) => one.id);
  }
}

function Card({ one, editing, held, lens = true, onRemove }) {
  const { Body } = one;
  return (
    <article
      className={lens ? "widget" : "widget frost"}
      data-id={lens ? one.id : undefined}
      data-size={one.size}
      data-lens={lens ? "widget" : undefined}
      data-held={held ? "" : undefined}
      inert={!lens}
      aria-label={one.name}
    >
      {editing && (
        <button
          type="button"
          className="remove"
          aria-label={`Remove ${one.name}`}
          title={`Remove ${one.name}`}
          onClick={() => onRemove(one.id)}
        >
          <Glyph art="minus" />
        </button>
      )}
      <Body />
    </article>
  );
}

function Gallery({ list, done, onAdd, onDone }) {
  const shelf = useRef(null);
  const box = useRef(null);
  useEffect(() => sortable(shelf.current, { kind: "catalog", axis: "x", name: (id) => BY.get(id)?.name ?? id }), []);
  useEffect(() => done.current?.focus(), []);
  useEffect(() => {
    const watch = new ResizeObserver(() => root().style.setProperty("--gallery", `${box.current.offsetHeight}px`));
    watch.observe(box.current);
    return () => {
      watch.disconnect();
      root().style.removeProperty("--gallery");
    };
  }, []);
  return (
    <section className="catalog" ref={box} data-lens="window" aria-label="Widget gallery">
      <header>
        <h2>Widgets</h2>
        <button type="button" className="primary" ref={done} onClick={onDone}>
          Done
        </button>
      </header>
      <div className="row" ref={shelf}>
        {WIDGETS.map((one) => {
          const added = list.includes(one.id);
          return (
            <div key={one.id} className="pick" data-id={added ? undefined : one.id} data-size={one.size}>
              <Card one={one} lens={false} />
              <span>{one.name}</span>
              <button type="button" disabled={added} onClick={() => onAdd(one.id)}>
                {added ? "Added" : "Add"}
              </button>
            </div>
          );
        })}
      </div>
    </section>
  );
}

/* MENU */

export const clamp = (at, w, h, view) => ({
  x: Math.max(EDGE, Math.min(at.x, view.w - w - EDGE)),
  y: Math.max(EDGE, Math.min(at.y, view.h - h - EDGE)),
});

function Menu({ at, onEdit, onClose }) {
  const box = useRef(null);
  useEffect(() => {
    const pop = box.current;
    if (!pop) return undefined;
    pop.showPopover?.();
    const { x, y } = clamp(at, pop.offsetWidth, pop.offsetHeight, { w: innerWidth, h: innerHeight });
    pop.style.setProperty("--x", `${x}px`);
    pop.style.setProperty("--y", `${y}px`);
    pop.querySelector("button, a")?.focus();
    const gone = (e) => e.newState === "closed" && onClose();
    pop.addEventListener("toggle", gone);
    return () => pop.removeEventListener("toggle", gone);
  }, [at]);
  return (
    <div ref={box} className="pop menu" popover="auto" data-lens="menu">
      <button type="button" onClick={onEdit}>
        Edit Widgets
      </button>
      <a className="button" href="/settings/desktop/" onClick={() => box.current?.hidePopover?.()}>
        Change Wallpaper
      </a>
    </div>
  );
}

/* WIDGETS */

function Widgets() {
  const [list, setList] = useState(() => kept(read(KEY.widgets)));
  const [draft, setDraft] = useState(null);
  const [editing, setEditing] = useState(false);
  const [menu, setMenu] = useState(null);
  const live = useRef(list);
  const column = useRef(null);
  const edit = useRef(null);
  const done = useRef(null);
  const was = useRef(false);
  const hooks = useRef(null);

  const keep = (next) => {
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

  useEffect(
    () =>
      sortable(column.current, {
        kind: "widgets",
        scroller: column.current.parentElement,
        takes: (from) => (from === "widgets" ? "move" : from === "catalog" ? "copy" : ""),
        name: (id) => BY.get(id)?.name ?? id,
        preview: (next) => hooks.current.preview(next),
        commit: (next) => hooks.current.commit(next),
      }),
    [],
  );

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
    const key = (e) => {
      if (e.key !== "Escape" || e.defaultPrevented) return;
      e.preventDefault();
      setEditing(false);
    };
    let armed = null;
    const press = (e) => {
      const target = e.target instanceof Element ? e.target : null;
      armed = null;
      if (!target) return;
      if (target.matches(BARE)) armed = e.pointerId;
      else if (!target.closest(KEEP)) setEditing(false);
    };
    const up = (e) => e.pointerId === armed && setEditing(false);
    const cancel = () => {
      armed = null;
    };
    const wire = (e) => e.detail?.route !== "/" && setEditing(false);
    document.addEventListener("keydown", key);
    document.addEventListener("pointerdown", press);
    document.addEventListener("pointerup", up);
    document.addEventListener("pointercancel", cancel);
    addEventListener("wire", wire);
    return () => {
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
    const open = (e) => {
      const target = e.target instanceof Element ? e.target : null;
      if (
        e.defaultPrevented ||
        !root().hasAttribute("data-desk") ||
        root().dataset.welcome === "open" ||
        !target ||
        target.closest(SKIP)
      )
        return;
      e.preventDefault();
      setMenu({ x: e.clientX, y: e.clientY });
    };
    document.addEventListener("contextmenu", open);
    return () => document.removeEventListener("contextmenu", open);
  }, []);

  useEffect(() => {
    shift();
  });

  const inert = (e) => {
    if (editing && e.target instanceof Element && e.target.closest("a")) e.preventDefault();
  };
  const remove = (gone) => {
    const next = live.current[live.current.indexOf(gone) + 1];
    flushSync(() => keep(live.current.filter((one) => one !== gone)));
    ((next && column.current?.querySelector(`[data-id="${next}"] > .remove`)) || done.current)?.focus();
  };
  const shown = placed(list, draft);
  const held = draft && draft.index !== null ? draft.id : null;

  return (
    <>
      <aside className="widgets" aria-label="Widgets" data-editing={editing ? "" : undefined}>
        <div className="column" ref={column} onClickCapture={inert}>
          {shown.map((id) => (
            <Card key={id} one={BY.get(id)} editing={editing} held={id === held} onRemove={remove} />
          ))}
        </div>
        {!editing && (
          <button type="button" className="edit" ref={edit} onClick={() => setEditing(true)}>
            Edit Widgets
          </button>
        )}
      </aside>
      {editing && (
        <Gallery list={list} done={done} onAdd={(id) => keep([...live.current, id])} onDone={() => setEditing(false)} />
      )}
      {menu && (
        <Menu
          at={menu}
          onEdit={() => {
            setMenu(null);
            setEditing(true);
          }}
          onClose={() => setMenu(null)}
        />
      )}
    </>
  );
}

export function start() {
  const host = document.createElement("div");
  host.className = "desked";
  document.body.append(host);
  show(host, <Widgets />);
}
