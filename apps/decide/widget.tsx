import { useCallback, useState } from "react";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Units } from "../../net/sdk/spec.ts";
import { MILLE } from "./engine.ts";
import type { Decide, Held, Told } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Held>> & Pick<WidgetProps<Units, Held>, "value" | "onChange">;

const PLACES = 3;

export const thousandths = (n: number) => `${Math.floor(n / MILLE)}.${String(n % MILLE).padStart(PLACES, "0")}`;

export const ticks = (n: number) =>
  n === MILLE ? "1 tick" : `${n % MILLE ? thousandths(n).replace(/0+$/, "") : n / MILLE} ticks`;

export function rows(told: Told | null): Readout[] {
  if (!told) return [];
  return [
    ["granted", thousandths(told.granted)],
    ["wait", ticks(told.wait)],
    ["reach", `${told.reach} of ${told.bots}`],
    [told.over ? "ran" : "tick", `${told.tick} of ${told.length}`],
    ["wishes", `${told.posted} posted, ${told.dropped} taken back`],
  ];
}

export function Widget({ value, onReady, onStatus }: Props) {
  const [told, setTold] = useState<Told | null>(null);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, onTell: setTold }),
    [],
  );
  const ready = (handle: Handle<Decide> | null) => onReady?.(handle);
  useStatus(onStatus, rows(told));
  return (
    <Scene make={scene} value={value} onReady={ready} role="img" aria-label="A village of bots deciding on wishes" />
  );
}
