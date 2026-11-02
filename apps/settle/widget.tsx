import { useCallback, useState } from "react";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Units } from "../../net/sdk/spec.ts";
import { MILLE } from "./engine.ts";
import type { Held, Settle, Told } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Held>> & Pick<WidgetProps<Units, Held>, "value" | "onChange">;

const PLACES = 3;

export const thousandths = (n: number) => `${Math.floor(n / MILLE)}.${String(n % MILLE).padStart(PLACES, "0")}`;

const many = (n: number, word: string) => `${n} ${word}${n === 1 ? "" : "s"}`;

export function rows(told: Told | null): Readout[] {
  if (!told) return [];
  return [
    ["housed", thousandths(told.housed)],
    ["fed", thousandths(told.fed)],
    ["green", thousandths(told.green)],
    ["reach", thousandths(told.reach)],
    [told.over ? "ran" : "day", `${told.tick} of ${told.length}`],
    [
      "world",
      `${told.ashore} of ${told.bots} bots ashore, ${many(told.towns, "town")}, ${many(told.tents, "tent")}, ${told.fenced} fenced idle`,
    ],
  ];
}

export function Widget({ value, onReady, onStatus }: Props) {
  const [told, setTold] = useState<Told | null>(null);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, onTell: setTold }),
    [],
  );
  const ready = (handle: Handle<Settle> | null) => onReady?.(handle);
  useStatus(onStatus, rows(told));
  return <Scene make={scene} value={value} onReady={ready} role="img" aria-label="Bots settling an empty island" />;
}
