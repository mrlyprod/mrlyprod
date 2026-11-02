import type { ReactNode, RefCallback, RefObject } from "react";
import { createContext, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { parts } from "../shell/active.ts";
import type { Props } from "../shell/design.ts";
import { measure } from "../shell/measure.ts";
import { handle } from "./handle.ts";
import { held, hold } from "./keep.ts";

/* TYPES */

export type Side = "left" | "right";

export type Slot = Side | "tail" | "status" | "hints" | "actions";

export type Pane = { name: string; key: string; keep: string };

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

const Slots = createContext<Framed | null>(null);

export const SIDES: Record<Side, Pane> = {
  left: { name: "Library", key: "[", keep: "library" },
  right: { name: "Knobs", key: "]", keep: "knobs" },
};

const { width: WIDTH, least: LEAST, most: MOST, gap: GAP, stage: ROOM } = measure.panes;

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
    if (keep) hold(SIDES[side].keep, px === WIDTH ? null : Math.round(px / measure.step) * measure.step);
  }, []);
  return { claims, nodes, ref, claim, has, open, flip, widths, size };
}

function useGrips(panes: Panes, root: RefObject<HTMLElement | null>) {
  const latest = useRef(panes);
  latest.current = panes;
  return useMemo(
    () =>
      Object.fromEntries(
        (Object.keys(SIDES) as Side[]).map((side): [Side, RefCallback<HTMLElement>] => [
          side,
          (el) =>
            el
              ? handle(el, {
                  get: () => latest.current.widths[side],
                  set: (px) => latest.current.size(side, px, false),
                  done: (px) => latest.current.size(side, px, true),
                  reset: WIDTH,
                  min: LEAST,
                  max: MOST,
                  rate: side === "left" ? 1 : -1,
                  label: `${SIDES[side].name} width`,
                  busy: (on) => root.current?.toggleAttribute("data-drag", on),
                })
              : undefined,
        ]),
      ) as Record<Side, RefCallback<HTMLElement>>,
    [root],
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
  const grips = useGrips(panes, box);
  const frame = useMemo(() => ({ claim, nodes }), [claim, nodes]);
  const shown = Object.fromEntries(
    (Object.keys(SIDES) as Side[]).map((side): [Side, boolean] => [side, has(side) && open[side]]),
  );
  const float = phone() || floats(wide, pushed(shown, widths));
  const to = (slot: Slot) => () => ({ ref: ref(slot) });
  const slots: Props["app"]["slots"] = {
    stage: () => ({ ref: stage, "data-gestures": gestures ? "" : undefined }),
    ...(has("left") && { left: to("left") }),
    ...(has("right") && { right: to("right"), tail: to("tail") }),
    ...(has("actions") && { actions: to("actions") }),
    ...(status && { status: to("status"), hints: to("hints") }),
  };
  const bind: Props["app"]["bind"] = {
    root: () => ({ ref: box }),
    pane: (side) => () => ({
      id: side,
      "aria-label": SIDES[side].name,
      inert: !open[side] ? true : undefined,
      ...(side === "right" && { ref: calm }),
    }),
    grip: (side) => () => ({ ref: grips[side] }),
    opener: (side) => () => ({
      "aria-controls": side,
      "aria-expanded": open[side],
      "aria-label": SIDES[side].name,
      title: `${SIDES[side].name} (${SIDES[side].key})`,
      onClick: () => flip(side),
    }),
  };
  return (
    <Slots.Provider value={frame}>
      <parts.app title={label ?? ""} slots={slots} open={open} widths={widths} float={float} onFlip={flip} bind={bind}>
        {children}
      </parts.app>
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
