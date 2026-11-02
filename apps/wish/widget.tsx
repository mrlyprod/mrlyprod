import { useCallback, useState } from "react";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Units } from "../../net/sdk/spec.ts";
import { MILLE } from "./engine.ts";
import type { Held, Told, Wish } from "./scene.ts";
import { make, RULES } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Held>> & Pick<WidgetProps<Units, Held>, "value" | "onChange">;

const hours = (n: number) => (n === 1 ? "1 hour" : `${n} hours`);

const tenths = (n: number) => (n / MILLE).toFixed(1);

export function rows(told: Told | null): Readout[] {
  if (!told) return [];
  return [
    ["granted", String(told.granted)],
    ["waiting", String(told.waiting)],
    ["oldest wait", hours(told.oldest)],
    ["nobody can grant", String(told.nobody)],
    ["let go", String(told.gone)],
    [told.over ? "ran" : "hour", `${told.hour} of ${told.length}`],
    [
      "world",
      `${RULES[told.choose]}, ${told.bots} bots; a granted wish waited ${told.granted ? `${tenths(told.wait)} hours` : "-"}`,
    ],
  ];
}

export function Widget({ value, onReady, onStatus }: Props) {
  const [told, setTold] = useState<Told | null>(null);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, onTell: setTold }),
    [],
  );
  const ready = (handle: Handle<Wish> | null) => onReady?.(handle);
  useStatus(onStatus, rows(told));
  return <Scene make={scene} value={value} onReady={ready} role="img" aria-label="Bots granting each other's wishes" />;
}
