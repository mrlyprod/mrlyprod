import type { CSSProperties, ReactNode, RefCallback } from "react";
import { createContext, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { sourced, tap } from "../glass/feed.ts";
import { handle } from "../handle.ts";
import { held, hold } from "../keep.ts";
import type { IconName } from "./controls.tsx";
import { Icon } from "./controls.tsx";

/* TYPES */

export type Side = "left" | "right";

export type Slot = Side | "tail" | "status" | "hints" | "actions";

export type Pane = { name: string; key: string; keep: string; icons: [IconName, IconName] };

export type Nodes = Partial<Record<Slot, HTMLElement | null>>;

export type Panes = {
  claims: Partial<Record<Slot, number>>;
  nodes: Nodes;
  ref: (side: Slot) => RefCallback<HTMLElement>;
  claim: (side: Slot) => () => void;
  has: (side: Slot) => boolean;
  open: Record<Side, boolean>;
  flip: (side: Side, on?: boolean) => void;
  widths: Record<Side, number>;
  size: (side: Side, px: number, keep: boolean) => void;
};

export type Framed = Pick<Panes, "claim" | "nodes">;

export type Doc = Pick<Document, "querySelector">;

export type FrameProps = { label?: string; gestures?: boolean; panes?: Panes; children?: ReactNode };

export type BarProps = { side: Slot; children?: ReactNode };

export type OpenerProps = { side: Side; panes: Panes };

type Vars = CSSProperties & Record<`--${string}`, string>;

const Slots = createContext<Framed | null>(null);

export const SIDES: Record<Side, Pane> = {
  left: { name: "Library", key: "[", keep: "library", icons: ["left_panel_open", "left_panel_close"] },
  right: { name: "Knobs", key: "]", keep: "knobs", icons: ["right_panel_open", "right_panel_close"] },
};

export const WIDTH = 280;
const LEAST = 220;
const MOST = 440;
const GAP = 12;
const ROOM = 360;

export function calm(box: EventTarget | null): () => void {
  let by = "";
  const on: Record<string, EventListener> = {
    pointerdown: () => (by = "pointer"),
    keydown: () => (by = "key"),
    change: (e) => by === "pointer" && (e.target as Partial<HTMLElement>).blur?.(),
  };
  for (const [name, fn] of Object.entries(on)) box!.addEventListener(name, fn);
  return () => {
    for (const [name, fn] of Object.entries(on)) box!.removeEventListener(name, fn);
  };
}

export function slot(frame: Framed | null, side: Slot, doc: Doc | undefined = globalThis.document): Element | null {
  if (frame) return frame.nodes[side] ?? null;
  return side === "right" || side === "tail" ? (doc?.querySelector("#right .controls") ?? null) : null;
}

/* PANES */

export const pushed = (open: Partial<Record<Side, boolean>>, widths: Record<Side, number>, gap = GAP) =>
  Math.max(0, ...(Object.keys(SIDES) as Side[]).map((side) => (open[side] ? widths[side] + 2 * gap : 0)));

export const floats = (width: number, push: number, room = ROOM) => width - 2 * push < room;

/* FACTS */

export function fit(rights: number[], room: number, least = 1, lefts: number[] = [], end = 0): [number, number] {
  const short = (rights[least - 1] ?? 0) - room;
  let hints = lefts.length;
  if (short > 0.01)
    hints = Math.max(
      0,
      lefts.findLastIndex((left) => end - left >= short),
    );
  const wide = room + (hints < lefts.length ? end - lefts[hints] : 0);
  const over = rights.findIndex((right) => right > wide + 0.01);
  return [Math.min(rights.length, Math.max(least, over < 0 ? rights.length : over)), hints];
}

function trim(pill: Element, box: Element, end: Element) {
  const facts = [...box.children];
  const kids = [...end.children];
  const hints = kids.filter((kid) => kid.tagName === "SPAN");
  pill.toggleAttribute("data-measure", true);
  for (const kid of [...facts, ...kids]) kid.removeAttribute("data-drop");
  const { left, width } = box.getBoundingClientRect();
  const keys =
    kids.find((kid) => kid.tagName !== "SPAN")?.getBoundingClientRect().left ?? end.getBoundingClientRect().right;
  const least = facts.findIndex((kid) => kid.tagName === "SPAN") + 1 || 1;
  const [shown, said] = fit(
    facts.map((kid) => kid.getBoundingClientRect().right - left),
    width,
    least,
    hints.map((kid) => kid.getBoundingClientRect().left),
    keys,
  );
  for (const [i, kid] of facts.entries()) kid.toggleAttribute("data-drop", i >= shown);
  for (const [i, kid] of hints.entries()) kid.toggleAttribute("data-drop", i >= said);
  pill.removeAttribute("data-measure");
}

const phone = () => typeof document !== "undefined" && document.documentElement?.dataset?.form === "phone";

export function usePanes(): Panes {
  const [claims, setClaims] = useState<Partial<Record<Slot, number>>>({});
  const [nodes, setNodes] = useState<Nodes>({});
  const refs = useRef<Partial<Record<Slot, RefCallback<HTMLElement>>>>({});
  const [open, setOpen] = useState<Record<Side, boolean>>(() =>
    phone() ? { left: false, right: false } : { left: true, right: true, ...held("panes", {}) },
  );
  const [widths, setWidths] = useState<Record<Side, number>>(() => ({
    left: held(SIDES.left.keep, WIDTH),
    right: held(SIDES.right.keep, WIDTH),
  }));
  const claim = useCallback((side: Slot) => {
    setClaims((was) => ({ ...was, [side]: (was[side] ?? 0) + 1 }));
    return () => setClaims((was) => ({ ...was, [side]: was[side]! - 1 }));
  }, []);
  const ref = (side: Slot) =>
    (refs.current[side] ??= (node) => setNodes((was) => (was[side] === node ? was : { ...was, [side]: node })));
  const has = (side: Slot) => (claims[side] ?? 0) > 0 || (side === "right" && (claims.tail ?? 0) > 0);
  const flip = useCallback(
    (side: Side, on?: boolean) =>
      setOpen((was) => {
        const next = { ...was, [side]: on ?? !was[side] };
        if (!phone()) hold("panes", next);
        return next;
      }),
    [],
  );
  const size = useCallback((side: Side, px: number, keep: boolean) => {
    setWidths((was) => (was[side] === px ? was : { ...was, [side]: px }));
    if (keep) hold(SIDES[side].keep, px === WIDTH ? null : Math.round(px));
  }, []);
  return { claims, nodes, ref, claim, has, open, flip, widths, size };
}

function Grip({ side, panes }: OpenerProps) {
  const at = useRef<HTMLDivElement>(null);
  const latest = useRef(panes);
  latest.current = panes;
  useEffect(
    () =>
      handle(at.current!, {
        get: () => latest.current.widths[side],
        set: (px) => latest.current.size(side, px, false),
        done: (px) => latest.current.size(side, px, true),
        reset: WIDTH,
        min: LEAST,
        max: MOST,
        rate: side === "left" ? 1 : -1,
        label: `${SIDES[side].name} width`,
        busy: (on) => at.current?.closest(".frame")?.toggleAttribute("data-drag", on),
      }),
    [side],
  );
  return <div className="grip" ref={at} />;
}

export function Opener({ side, panes }: OpenerProps) {
  const { name, key, icons } = SIDES[side];
  const on = panes.open[side];
  return (
    <button
      type="button"
      className="icon side"
      aria-controls={side}
      aria-expanded={on}
      aria-label={name}
      title={`${name} (${key})`}
      onClick={() => panes.flip(side)}
    >
      <Icon name={icons[on ? 1 : 0]} />
    </button>
  );
}

export function Frame({ label, gestures, panes: given, children }: FrameProps) {
  const mine = usePanes();
  const panes = given ?? mine;
  const { claim, nodes, ref, has, open, flip, widths } = panes;
  const box = useRef<HTMLElement>(null);
  const [wide, setWide] = useState(Infinity);
  useLayoutEffect(() => {
    const el = box.current;
    if (!el || typeof ResizeObserver !== "function") return undefined;
    const eye = new ResizeObserver(([one]) => setWide(one.contentRect.width));
    eye.observe(el);
    return () => eye.disconnect();
  }, []);
  const [fed, setFed] = useState(false);
  useEffect(() => {
    const look = () => setFed(sourced().some((one) => box.current?.contains(one)));
    look();
    return tap(look);
  }, []);
  const stage = useRef<HTMLDivElement>(null);
  const status = has("status") || has("hints");
  useLayoutEffect(() => {
    const { status: facts, hints } = nodes;
    const el = facts?.parentElement;
    if (!el || !hints || typeof ResizeObserver !== "function") return undefined;
    let live = true;
    const go = () => live && trim(el, facts!, hints);
    const eye = new ResizeObserver(go);
    eye.observe(stage.current!);
    const watch = typeof MutationObserver === "function" ? new MutationObserver(go) : null;
    watch?.observe(el, { childList: true, characterData: true, subtree: true });
    document.fonts?.ready.then(go);
    return () => {
      live = false;
      eye.disconnect();
      watch?.disconnect();
    };
  }, [nodes.status, nodes.hints]);
  const glass = (name: string, kind: string) =>
    fed ? { className: name, "data-lens": kind } : { className: `${name} frost` };
  const latest = useRef(has);
  latest.current = has;
  useEffect(() => {
    const press = (e: KeyboardEvent) => {
      if (!(e.metaKey || e.ctrlKey) || e.altKey || e.shiftKey) return;
      const side = (Object.keys(SIDES) as Side[]).find((one) => SIDES[one].key === e.key);
      if (!side || !latest.current(side)) return;
      e.preventDefault();
      flip(side);
    };
    addEventListener("keydown", press);
    return () => removeEventListener("keydown", press);
  }, [flip]);
  const frame = useMemo(() => ({ claim, nodes }), [claim, nodes]);
  const shown = Object.fromEntries(
    (Object.keys(SIDES) as Side[]).map((side): [Side, boolean] => [side, has(side) && open[side]]),
  );
  const float = phone() || floats(wide, pushed(shown, widths));
  const sides = (Object.keys(SIDES) as Side[]).filter(has);
  const style: Vars = { "--left": `${widths.left}px`, "--right": `${widths.right}px` };
  return (
    <Slots.Provider value={frame}>
      <section
        className="frame"
        aria-label={label}
        ref={box}
        style={style}
        data-left={has("left") ? (open.left ? "open" : "shut") : undefined}
        data-right={has("right") ? (open.right ? "open" : "shut") : undefined}
        data-float={float ? "" : undefined}
      >
        <div className="canvas" ref={stage} data-gestures={gestures ? "" : undefined}>
          {children}
        </div>
        {has("left") && (
          <aside
            {...glass("pane left", "panel")}
            id="left"
            aria-label={SIDES.left.name}
            inert={!open.left ? true : undefined}
          >
            <div className="body" ref={ref("left")} />
            <Grip side="left" panes={panes} />
          </aside>
        )}
        {has("right") && (
          <aside
            {...glass("pane right", "panel")}
            id="right"
            aria-label={SIDES.right.name}
            inert={!open.right ? true : undefined}
          >
            <div className="body" ref={calm}>
              <section className="controls" aria-label="Controls" ref={ref("right")} />
              <div className="controls tail" ref={ref("tail")} />
            </div>
            <Grip side="right" panes={panes} />
          </aside>
        )}
        {(has("actions") || (float && sides.length > 0)) && (
          <div {...glass("actions", "pill")}>
            {float && sides.includes("left") && <Opener side="left" panes={panes} />}
            <div className="buttons" ref={ref("actions")} />
            {float && sides.includes("right") && <Opener side="right" panes={panes} />}
          </div>
        )}
        {status && (
          <div {...glass("status", "pill")}>
            <div className="facts" ref={ref("status")} />
            <div className="hints" ref={ref("hints")} />
          </div>
        )}
      </section>
    </Slots.Provider>
  );
}

export function Bar({ side, children }: BarProps) {
  const frame = useContext(Slots);
  const claim = frame?.claim;
  useLayoutEffect(() => claim?.(side), [claim, side]);
  const at = typeof document === "undefined" ? null : slot(frame, side);
  return at ? createPortal(children, at) : null;
}
