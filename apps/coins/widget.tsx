import { useCallback, useState } from "react";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Units } from "../../net/sdk/spec.ts";
import { MILLE } from "./engine.ts";
import type { Coins, Held, Told } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Held>> & Pick<WidgetProps<Units, Held>, "value" | "onChange">;

const PLACES = 3;

export const thousandths = (n: number) => `${Math.floor(n / MILLE)}.${String(n % MILLE).padStart(PLACES, "0")}`;

export function rows(told: Told | null): Readout[] {
  if (!told) return [];
  return [
    ["traded", String(told.traded)],
    ["failed", String(told.failed)],
    ["lost", thousandths(told.lost)],
    ["stuck", String(told.stuck)],
    [told.over ? "ran" : "day", `${told.day} of ${told.length}`],
    [
      "world",
      `${told.currencies === 1 ? "1 coin" : `${told.currencies} coins`}, ${told.booths === 1 ? "1 changer" : `${told.booths} changers`}, fee ${told.fee}%, ${told.traders} traders; days per trade ${told.traded ? thousandths(told.pace) : "-"}, top fifth holds ${thousandths(told.spread)}`,
    ],
  ];
}

export function Widget({ value, onReady, onStatus }: Props) {
  const [told, setTold] = useState<Told | null>(null);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, onTell: setTold }),
    [],
  );
  const ready = (handle: Handle<Coins> | null) => onReady?.(handle);
  useStatus(onStatus, rows(told));
  return (
    <Scene make={scene} value={value} onReady={ready} role="img" aria-label="Traders crossing a map of coin regions" />
  );
}
