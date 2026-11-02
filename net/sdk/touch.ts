import type { PointerEvent, RefObject } from "react";
import { useRef } from "react";
import type { Patch } from "./spec.ts";

/* TYPES */

export type Spot = [number, number];

export type Touched<P = Patch> = {
  touch?: (u: number, v: number, start: boolean) => P | null | undefined | false;
  lift?: (u: number, v: number) => P | null | undefined | false;
};

export type Change<P = Patch> = (patch: P) => void;

export type Hands = {
  onPointerDown: (e: PointerEvent<Element>) => void;
  onPointerMove: (e: PointerEvent<Element>) => void;
  onPointerUp: (e: PointerEvent<Element>) => void;
  onPointerCancel: (e: PointerEvent<Element>) => void;
};

const at = (e: PointerEvent<Element>): Spot => {
  const box = e.currentTarget.getBoundingClientRect();
  return [(e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height];
};

export function useTouch<P = Patch>(
  handle: RefObject<Touched<P> | null | undefined>,
  change: RefObject<Change<P> | null | undefined>,
): Hands {
  const down = useRef(false);
  const send = (e: PointerEvent<Element>, start: boolean) => {
    const patch = handle.current?.touch?.(...at(e), start);
    if (patch) change.current?.(patch);
  };
  const up = (e: PointerEvent<Element>) => {
    if (!down.current) return;
    down.current = false;
    const patch = handle.current?.lift?.(...at(e));
    if (patch) change.current?.(patch);
  };
  return {
    onPointerDown: (e) => {
      if (e.button) return;
      e.currentTarget.setPointerCapture?.(e.pointerId);
      down.current = true;
      send(e, true);
    },
    onPointerMove: (e) => down.current && send(e, false),
    onPointerUp: up,
    onPointerCancel: up,
  };
}
