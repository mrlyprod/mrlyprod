import type { PointerEvent } from "react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { Had, Readout } from "../../lib/app/hooks.ts";
import { useUnit } from "../../lib/app/hooks.ts";
import type { Handle, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Facts } from "./engine.ts";
import { cut, facts, gallery } from "./engine.ts";
import { Pens } from "./gallery.tsx";
import type { Knobs, Told, Writer } from "./scene.ts";
import { make, PAGE } from "./scene.ts";

/* TYPES */

export type Seen = Pick<Knobs, "view"> & Partial<Pick<Knobs, "speed">>;

type Props = WidgetProps<Had<"font">, Writer> & { value: Knobs };

export function status(value: Seen, plan: Told | null, known: Facts | null): Readout[] {
  if (value.view === "text")
    return plan
      ? [
          ["board", `${plan.cols}x${plan.rows}`],
          ["frames", plan.frames],
          ["time", `${(plan.frames / plan.fps / value.speed!).toFixed(1)} s`],
        ]
      : [];
  return known
    ? [
        ["pens", known.pens],
        ["strokes", known.strokes],
        ["flagged", known.flagged],
      ]
    : [];
}

export function added(value: Pick<Knobs, "view" | "text">, plan: Told | null, char: string) {
  const was = value.view === "text" ? value.text || plan?.text || "" : value.text;
  return { view: "text", text: cut(`${was}${char}`) };
}

export function Widget({ value, onChange, onReady, onExport, onStatus }: Props) {
  const font = useUnit("font");
  const [plan, setPlan] = useState<Told | null>(null);
  const live = useRef<Handle<Writer> | null>(null);
  const kinds = useMemo(() => (font ? gallery(font) : null), [font]);
  const known = useMemo(() => (kinds ? facts(kinds) : null), [kinds]);
  const made = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, font, kinds, onPlan: setPlan }),
    [font, kinds],
  );
  const ready = (handle: Handle<Writer> | null) => {
    live.current = handle;
    onReady?.(handle);
  };
  const act = (name: keyof typeof PAGE.actions) => () => live.current && PAGE.actions[name](live.current);
  useEffect(
    () =>
      onExport?.(
        font
          ? [
              ["JSON", act("json")],
              ["SVG", act("svg")],
            ]
          : [],
      ),
    [font],
  );
  const rows = status(value, plan, known);
  const said = JSON.stringify(rows);
  useEffect(() => onStatus?.(rows), [said]);
  const tap = (e: PointerEvent<HTMLCanvasElement>) => {
    const canvas = e.currentTarget;
    const box = canvas.getBoundingClientRect();
    if (!box.width || !box.height) return;
    live.current?.tap?.(
      ((e.clientX - box.left) * canvas.width) / box.width,
      ((e.clientY - box.top) * canvas.height) / box.height,
    );
  };
  return (
    <>
      {font && <Scene make={made} value={value} onReady={ready} onPointerDown={tap} />}
      {kinds && <Pens kinds={kinds} tint={value.tint} onPick={(char) => onChange(added(value, plan, char))} />}
    </>
  );
}
