import { useCallback, useState } from "react";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Units } from "../../net/sdk/spec.ts";
import { MILLE } from "./engine.ts";
import type { Held, Roots, Told } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Held>> & Pick<WidgetProps<Units, Held>, "value" | "onChange">;

const PLACES = 3;

export const thousandths = (n: number) => `${Math.floor(n / MILLE)}.${String(n % MILLE).padStart(PLACES, "0")}`;

export function rows(told: Told | null): Readout[] {
  if (!told) return [];
  return [
    ["top", thousandths(told.top)],
    ["mobility", told.pairs ? thousandths(told.mobility) : "-"],
    ["made", thousandths(told.made)],
    ["from nothing", thousandths(told.nothing)],
    [told.over ? "ran" : "year", `${told.year} of ${told.length}`],
    [
      "world",
      `${told.families} families, generation ${told.generations}; rent ${thousandths(told.rented)}, shared ${thousandths(told.shared)}, commons ${thousandths(told.commons)}; richest ${told.richest}, poorest ${told.poorest}`,
    ],
  ];
}

export function Widget({ value, onReady, onStatus }: Props) {
  const [told, setTold] = useState<Told | null>(null);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, onTell: setTold }),
    [],
  );
  const ready = (handle: Handle<Roots> | null) => onReady?.(handle);
  useStatus(onStatus, rows(told));
  return (
    <Scene make={scene} value={value} onReady={ready} role="img" aria-label="Generations of bot families as a river" />
  );
}
