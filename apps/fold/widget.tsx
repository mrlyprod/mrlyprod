import type { MouseEvent, PointerEvent } from "react";
import { useCallback, useRef, useState } from "react";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Units } from "../../net/sdk/spec.ts";
import type { Fold, Held, Told } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Held>> & Pick<WidgetProps<Units, Held>, "value" | "onChange">;

export function rows(told: Told | null): Readout[] {
  if (!told) return [];
  const said = told.over === "done" ? "done in" : told.over === "stuck" ? "stuck after" : "moves";
  return [
    [said, told.moves],
    ["par", told.par],
    ["holes", `${told.holes} of ${told.of}`],
  ];
}

export function Widget({ value, onChange, onReady, onStatus }: Props) {
  const live = useRef<Handle<Fold> | null>(null);
  const now = useRef(value);
  const change = useRef(onChange);
  now.current = value;
  change.current = onChange;
  const [told, setTold] = useState<Told | null>(null);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, onTell: setTold }),
    [],
  );
  const ready = (handle: Handle<Fold> | null) => {
    live.current = handle;
    onReady?.(handle && { ...handle, again: () => change.current({ seed: (Number(now.current.seed) + 1) >>> 0 }) });
  };
  const at = (e: PointerEvent<HTMLCanvasElement> | MouseEvent<HTMLCanvasElement>): [number, number] => {
    const box = e.currentTarget.getBoundingClientRect();
    return [(e.clientX - box.left) / (box.width || 1), (e.clientY - box.top) / (box.height || 1)];
  };
  const hover = (e: PointerEvent<HTMLCanvasElement>) => {
    if (e.pointerType === "mouse") live.current?.hover(...at(e));
  };
  const tap = (e: MouseEvent<HTMLCanvasElement>) => live.current?.tap(...at(e));
  useStatus(onStatus, rows(told));
  return (
    <Scene
      make={scene}
      value={value}
      onReady={ready}
      onPointerMove={hover}
      onClick={tap}
      role="img"
      aria-label="A sheet of paper to fold and punch into lace"
    />
  );
}
