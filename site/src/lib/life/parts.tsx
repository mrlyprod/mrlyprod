import type { Tensor } from "@mrlyprod/mrly/life";
import type { PointerEvent, ReactNode, RefObject } from "react";
import { useEffect, useRef } from "react";
import { Fold } from "../app/controls.tsx";
import { Bar } from "../app/frame.tsx";
import { useUnits } from "../app/hooks.ts";
import type { Change } from "../app/knobs.tsx";
import { gist, Knob } from "../app/knobs.tsx";
import type { Spec, Units, Value } from "../app/spec.ts";
import type { Hands, Spot, Touched } from "../app/touch.ts";
import { paints, veil } from "../color.ts";
import { useTheme } from "../designs/pick.tsx";
import { wasm } from "../units/index.ts";
import { runs } from "./scene.ts";

/* TYPES */

export type Section = { name: string; items?: readonly ReactNode[]; live?: ReactNode };

export type PaneProps<U = Units> = {
  spec: Spec<U>;
  value: Value;
  onChange: Change;
  folds: readonly (Section | null | false | undefined)[];
};

export type MaskViewProps = { mask?: Tensor | null; dot?: number; label?: string };

const LIFE = wasm("life", "math");

const SPOT = 160;

const DOT = 16;

const STRIP = 12;

const GROUND = 0.1;

const at = (e: PointerEvent<Element>): Spot => {
  const box = e.currentTarget.getBoundingClientRect();
  return [(e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height];
};

export const useLife = () => useUnits(LIFE);

export function usePaint(handle: RefObject<Touched<unknown> | null | undefined>): Hands {
  const painting = useRef(false);
  const down = (e: PointerEvent<Element>) => {
    if (e.button) return;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    painting.current = true;
    handle.current?.touch?.(...at(e), true);
  };
  const move = (e: PointerEvent<Element>) => {
    if (painting.current) handle.current?.touch?.(...at(e), false);
  };
  const up = () => {
    painting.current = false;
  };
  return { onPointerDown: down, onPointerMove: move, onPointerUp: up, onPointerCancel: up };
}

export function Pane<U = Units>({ spec, value, onChange, folds }: PaneProps<U>) {
  return (
    <Bar side="right">
      {(folds.filter(Boolean) as Section[]).map(({ name, items = [], live }) => {
        const rows = items
          .filter((item) => typeof item === "string")
          .map((key) => spec.find((row) => row.key === key)!);
        return (
          <Fold key={name} name={name} live={live ?? gist(rows, value)}>
            {items.map((item) => {
              if (typeof item !== "string") return item;
              const row = rows.find((one) => one.key === item);
              return row ? <Knob key={item} row={row} value={value[item]} values={value} onChange={onChange} /> : null;
            })}
          </Fold>
        );
      })}
    </Bar>
  );
}

export function MaskView({ mask, dot = DOT, label = "The neighbourhood mask" }: MaskViewProps) {
  const at = useRef<HTMLCanvasElement>(null);
  const tick = useTheme();
  useEffect(() => {
    const canvas = at.current;
    if (!canvas || !mask) return;
    const [rows, cols] = mask.shape;
    const dpr = Math.min(globalThis.devicePixelRatio || 1, 2);
    const cell = Math.max(1, Math.min(Math.floor((SPOT * dpr) / cols), dot * dpr));
    const high = rows === 1 ? Math.max(cell, STRIP * dpr) : cell;
    canvas.width = cols * cell;
    canvas.height = rows * high;
    canvas.style.width = `${(cols * cell) / dpr}px`;
    canvas.style.height = `${(rows * high) / dpr}px`;
    const ctx = canvas.getContext("2d")!;
    const { accent } = paints(canvas);
    ctx.fillStyle = veil(accent, GROUND);
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    ctx.fillStyle = accent;
    runs(ctx, mask.data, cols, rows, 0, 0, cell, high);
  }, [mask, dot, tick]);
  return <canvas ref={at} className="mask" role="img" aria-label={label} style={{ display: "block" }} />;
}
