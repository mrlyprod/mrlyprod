import type { RefObject } from "react";
import { Fragment, useEffect, useRef, useState } from "react";
import { flushSync } from "react-dom";
import type { Draft } from "../lib/drag.ts";
import { dragging, placed, sortable } from "../lib/drag.ts";
import { AppIcon } from "./appicon.tsx";
import { app, dock, redock } from "./catalog.ts";
import { route } from "./desk.ts";
import type { Item } from "./lens.ts";
import { BIG, lens } from "./lens.ts";
import { show } from "./parts.tsx";
import { calm, KEY, subscribe } from "./prefs.ts";
import type { Wired } from "./route.ts";
import { Tile } from "./tile.tsx";

/* TYPES */

type Hooks = { preview: (g: number, next: Draft | null) => void; commit: (g: number, next: Draft | null) => void };

type GroupProps = {
  g: number;
  ids: string[];
  draft: Draft | null;
  foreign: string | undefined;
  here: string;
  hooks: RefObject<Hooks | null>;
};

const RAMP = 150;
const VARS = ["--l", "--r", "--big", "--dx", "--m"];

const takes = (from: string) => (from === "dock" ? "move" : from === "launchpad" || from === "recent" ? "copy" : "");
const shift = (ms: number) => dispatchEvent(new CustomEvent("shift", { detail: ms }));

/* GROUP */

function Group({ g, ids, draft, foreign, here, hooks }: GroupProps) {
  const list = useRef<HTMLUListElement>(null);
  useEffect(
    () =>
      sortable(list.current!, {
        kind: "dock",
        axis: "x",
        out: true,
        scroller: list.current!.closest(".dock"),
        takes,
        name: (id) => app(id)?.title ?? id,
        preview: (next) => hooks.current!.preview(g, next),
        commit: (next) => hooks.current!.commit(g, next),
      }),
    [g],
  );
  const held = draft && draft.index !== null ? draft.id : null;
  const shown = placed(ids, draft).filter((id) => id === held || id !== foreign);
  return (
    <ul className="apps" ref={list}>
      {shown.map((id) => (
        <li key={id} data-id={id} data-held={id === held ? "" : undefined}>
          <Tile id={id} name={false} current={app(id)?.route === here} />
        </li>
      ))}
    </ul>
  );
}

/* DOCK */

function Dock() {
  const [groups, setGroups] = useState(dock);
  const [drafts, setDrafts] = useState<(Draft | null)[]>([]);
  const [here, setHere] = useState("");
  const live = useRef(groups);
  const nav = useRef<HTMLElement>(null);
  const hooks = useRef<Hooks | null>(null);
  const poke = useRef<(() => void) | null>(null);

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

  useEffect(() => {
    const box = nav.current!;
    const html = document.documentElement;
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
      const held = dragging() || !box.offsetWidth;
      const goal = want && !held ? 1 : 0;
      amount = held ? 0 : Math.min(1, Math.max(0, amount + ((goal ? 1 : -1) * Math.max(0, t - last)) / RAMP));
      last = t;
      const parts = [...box.querySelectorAll<HTMLElement>(".launcher, .split")];
      if (!amount && !goal) {
        stop();
        for (const el of [box, ...parts]) for (const name of VARS) el.style.removeProperty(name);
        return shift(RAMP);
      }
      const items: Item[] = parts.map((el) => {
        const r = el.getBoundingClientRect();
        return {
          c: r.left + r.width / 2,
          dx: Number.parseFloat(el.style.getPropertyValue("--dx")) || 0,
          w: el.classList.contains("launcher") ? r.width : 0,
        };
      });
      const out = lens(items, x, amount * amount * (3 - 2 * amount));
      moved = false;
      put(box, "--l", `${out.l}px`);
      put(box, "--r", `${out.r}px`);
      put(box, "--big", String(BIG));
      parts.forEach((el, i) => {
        put(el, "--dx", `${out.dx[i]}px`);
        if (items[i].w) put(el, "--m", out.m[i].toFixed(3));
      });
      if (moved) shift(RAMP);
      else if (amount === goal) stop();
    };
    const kick = () => {
      if (frame) return;
      last = performance.now();
      frame = requestAnimationFrame(tick);
    };
    const redo = () => {
      if (amount) kick();
    };
    const move = (e: PointerEvent) => {
      x = e.clientX;
      want =
        e.pointerType !== "touch" &&
        !dragging() &&
        html.hasAttribute("data-magnify") &&
        html.dataset.form !== "phone" &&
        !calm();
      if (want) kick();
      else redo();
    };
    const leave = () => {
      want = false;
      redo();
    };
    poke.current = redo;
    addEventListener("wire", wire);
    addEventListener("dock", fresh);
    addEventListener("resize", redo);
    const sub = subscribe((key) => (!key || key === KEY.dock) && fresh());
    box.addEventListener("pointermove", move);
    box.addEventListener("pointerleave", leave);
    setHere(route);
    return () => {
      removeEventListener("wire", wire);
      removeEventListener("dock", fresh);
      removeEventListener("resize", redo);
      sub();
      box.removeEventListener("pointermove", move);
      box.removeEventListener("pointerleave", leave);
      stop();
    };
  }, []);

  useEffect(() => {
    shift(0);
    poke.current?.();
  });

  const foreign = (g: number) => drafts.find((one, k) => k !== g && one && one.index !== null)?.id;

  return (
    <nav className="dock" data-lens="dock" aria-label="Dock" ref={nav}>
      <a className="launcher launch" href="/menu/" aria-label="Launchpad" data-name="Launchpad" draggable="false">
        <AppIcon id="menu" />
      </a>
      {groups.map((ids, g) => (
        <Fragment key={g}>
          <span className="split" aria-hidden="true" />
          <Group g={g} ids={ids} draft={drafts[g] ?? null} foreign={foreign(g)} here={here} hooks={hooks} />
        </Fragment>
      ))}
    </nav>
  );
}

export function start() {
  const host = document.createElement("div");
  host.className = "docked";
  document.body.append(host);
  show(host, <Dock />);
}
