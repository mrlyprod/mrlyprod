import { useEffect, useRef, useState } from "react";
import { flushSync } from "react-dom";
import type { Draft } from "../kit/drag.ts";
import { dragging, placed, sortable } from "../kit/drag.ts";
import { px } from "../kit/px.ts";
import { design, parts } from "./active.ts";
import { AppIcon } from "./appicon.tsx";
import { app, dock, launch as fold, redock } from "./catalog.ts";
import type { Tile } from "./design.ts";
import { route } from "./desk.ts";
import type { Item } from "./lens.ts";
import { lens } from "./lens.ts";
import { show } from "./parts.tsx";
import { calm, KEY, read, subscribe } from "./prefs.ts";
import type { Wired } from "./route.ts";

type Hooks = { preview: (g: number, next: Draft | null) => void; commit: (g: number, next: Draft | null) => void };

type Mark = { el: HTMLElement; wide: boolean };

type Sorted = { el: HTMLElement; off: () => void };

const RAMP = 150;
const VARS = ["--l", "--r", "--big", "--dx", "--m"];
const LAUNCH = "menu";

const takes = (from: string) => (from === "dock" ? "move" : from === "launchpad" || from === "recent" ? "copy" : "");
const shift = (ms: number) => dispatchEvent(new CustomEvent("shift", { detail: ms }));
const before = (a: Mark, b: Mark) => (a.el.compareDocumentPosition(b.el) & Node.DOCUMENT_POSITION_FOLLOWING ? -1 : 1);

const tile = (id: string, here: string): Tile[] => {
  const one = app(id);
  return one ? [{ id, title: one.title, href: one.route, art: <AppIcon id={id} />, current: one.route === here }] : [];
};

function Dock() {
  const [groups, setGroups] = useState(dock);
  const [drafts, setDrafts] = useState<(Draft | null)[]>([]);
  const [here, setHere] = useState("");
  const [magnify, setMagnify] = useState(() => read(KEY.magnify) === "1");
  const live = useRef(groups);
  const nav = useRef<HTMLElement | null>(null);
  const hooks = useRef<Hooks | null>(null);
  const poke = useRef<(() => void) | null>(null);
  const pointer = useRef<{ move: (e: PointerEvent) => void; leave: () => void } | null>(null);
  const lists = useRef(new Map<number, HTMLElement>());
  const sorted = useRef(new Map<number, Sorted>());
  const marks = useRef(new Map<string, Mark>());
  const refs = useRef(new Map<string, (el: HTMLElement | null) => void>());

  const keep = (next: string[][]) => {
    live.current = next;
    redock(next);
    setGroups(next);
  };

  hooks.current = {
    preview: (g, next) => flushSync(() => setDrafts((was) => Object.assign([...was], { [g]: next }))),
    commit: (g, next) =>
      flushSync(() => {
        setDrafts((was) => Object.assign([...was], { [g]: null }));
        if (next)
          keep(
            live.current.map((ids, k) =>
              k === g ? placed(ids, next) : next.index !== null ? ids.filter((id) => id !== next.id) : ids,
            ),
          );
      }),
  };

  const ref = (key: string, fn: (el: HTMLElement | null) => void) => {
    const had = refs.current.get(key);
    if (had) return had;
    refs.current.set(key, fn);
    return fn;
  };
  const mark = (key: string, wide: boolean) =>
    ref(`lens ${key}`, (el) => {
      if (el) marks.current.set(key, { el, wide });
      else marks.current.delete(key);
    });
  const listed = (g: number) =>
    ref(`group ${g}`, (el) => {
      if (el) lists.current.set(g, el);
      else lists.current.delete(g);
    });

  useEffect(() => {
    const html = document.documentElement;
    const shape = design.lens;
    let x = 0;
    let want = false;
    let amount = 0;
    let last = 0;
    let frame = 0;
    let moved = false;
    const wire = (e: CustomEvent<Wired>) => e.detail?.route && setHere(e.detail.route);
    const fresh = () => {
      live.current = dock();
      setGroups(live.current);
    };
    const stop = () => {
      cancelAnimationFrame(frame);
      frame = 0;
    };
    const put = (el: HTMLElement, name: string, value: string) => {
      if (el.style.getPropertyValue(name) === value) return;
      el.style.setProperty(name, value);
      moved = true;
    };
    const tick = (t: number) => {
      frame = requestAnimationFrame(tick);
      const box = nav.current;
      const shown = [...marks.current.values()].filter(({ el }) => el.isConnected).sort(before);
      if (!box || !shown.length || !shape) return stop();
      const held = dragging() || !box.offsetWidth;
      const goal = want && !held ? 1 : 0;
      amount = held ? 0 : Math.min(1, Math.max(0, amount + ((goal ? 1 : -1) * Math.max(0, t - last)) / RAMP));
      last = t;
      const els = shown.map(({ el }) => el);
      if (!amount && !goal) {
        stop();
        for (const el of [box, ...els]) for (const name of VARS) el.style.removeProperty(name);
        return shift(RAMP);
      }
      const items: Item[] = shown.map(({ el, wide }) => {
        const r = el.getBoundingClientRect();
        return {
          c: r.left + r.width / 2,
          dx: Number.parseFloat(el.style.getPropertyValue("--dx")) || 0,
          w: wide ? r.width : 0,
        };
      });
      const out = lens(items, x, shape, amount * amount * (3 - 2 * amount));
      moved = false;
      put(box, "--l", px(out.l));
      put(box, "--r", px(out.r));
      put(box, "--big", String(shape.big));
      els.forEach((el, i) => {
        put(el, "--dx", px(out.dx[i]));
        if (items[i].w) put(el, "--m", out.m[i].toFixed(3));
      });
      if (moved) shift(RAMP);
      else if (amount === goal) stop();
    };
    const kick = () => {
      if (frame || !marks.current.size) return;
      last = performance.now();
      frame = requestAnimationFrame(tick);
    };
    const redo = () => {
      if (amount) kick();
    };
    pointer.current = {
      move: (e) => {
        x = e.clientX;
        want =
          Boolean(shape) && e.pointerType !== "touch" && !dragging() && html.hasAttribute("data-magnify") && !calm();
        if (want) kick();
        else redo();
      },
      leave: () => {
        want = false;
        redo();
      },
    };
    poke.current = redo;
    addEventListener("wire", wire);
    addEventListener("dock", fresh);
    addEventListener("resize", redo);
    const sub = subscribe((key) => {
      if (!key || key === KEY.dock) fresh();
      if (!key || key === KEY.magnify) setMagnify(read(KEY.magnify) === "1");
    });
    setHere(route);
    return () => {
      removeEventListener("wire", wire);
      removeEventListener("dock", fresh);
      removeEventListener("resize", redo);
      sub();
      stop();
      pointer.current = null;
    };
  }, []);

  useEffect(() => {
    for (const [g, one] of sorted.current) {
      if (lists.current.get(g) === one.el) continue;
      one.off();
      sorted.current.delete(g);
    }
    for (const [g, el] of lists.current) {
      if (sorted.current.has(g)) continue;
      const off = sortable(el, {
        kind: "dock",
        axis: "x",
        out: true,
        scroller: nav.current,
        takes,
        name: (id) => app(id)?.title ?? id,
        preview: (next) => hooks.current!.preview(g, next),
        commit: (next) => hooks.current!.commit(g, next),
      });
      sorted.current.set(g, { el, off });
    }
    shift(0);
    poke.current?.();
  });

  useEffect(
    () => () => {
      for (const one of sorted.current.values()) one.off();
      sorted.current.clear();
    },
    [],
  );

  if (!parts.dock) return null;
  const foreign = (g: number) => drafts.find((one, k) => k !== g && one && one.index !== null)?.id;
  const held = new Set(drafts.flatMap((one) => (one && one.index !== null ? [one.id] : [])));
  const shown = groups.map((ids, g) => {
    const draft = drafts[g] ?? null;
    const mine = draft && draft.index !== null ? draft.id : null;
    return placed(ids, draft)
      .filter((id) => id === mine || id !== foreign(g))
      .flatMap((id) => tile(id, here));
  });
  const launch = { id: LAUNCH, title: "Launchpad", href: "/menu/", art: <AppIcon id={LAUNCH} /> };

  return (
    <parts.dock
      launch={launch}
      groups={shown}
      magnify={magnify}
      bind={{
        root: () => ({
          ref: nav,
          onPointerMove: (e: { nativeEvent: PointerEvent }) => pointer.current?.move(e.nativeEvent),
          onPointerLeave: () => pointer.current?.leave(),
        }),
        launch: () => ({ onClick: fold }),
        group: (g) => () => ({ ref: listed(g) }),
        tile: (id) => () => ({ "data-id": id, "data-held": held.has(id) ? "" : undefined }),
        lens: { item: (id) => () => ({ ref: mark(id, true) }), gap: (g) => () => ({ ref: mark(`gap ${g}`, false) }) },
      }}
    />
  );
}

export function start() {
  if (!parts.dock) return;
  const host = document.createElement("div");
  host.className = "docked";
  document.body.append(host);
  show(host, <Dock />);
}
