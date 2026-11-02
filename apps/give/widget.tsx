import { useCallback, useState } from "react";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Units } from "../../net/sdk/spec.ts";
import { MILLE } from "./engine.ts";
import type { Give, Held, Told } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Held>> & Pick<WidgetProps<Units, Held>, "value" | "onChange">;

const PLACES = 3;

export const thousandths = (n: number) => `${Math.floor(n / MILLE)}.${String(n % MILLE).padStart(PLACES, "0")}`;

export function rows(told: Told | null): Readout[] {
  if (!told) return [];
  return [
    ["fed", thousandths(told.fed)],
    ["idle", thousandths(told.idle)],
    ["top", `${thousandths(told.top)}, fair ${thousandths(told.fair)}`],
    ["hungry", `${told.hungry} of ${told.bots}`],
    [told.over ? "ran" : "tick", `${told.tick} of ${told.length}`],
    ["things", `${told.made} made, ${told.used} used, ${told.spilled} spilled`],
  ];
}

export function Widget({ value, onReady, onStatus }: Props) {
  const [told, setTold] = useState<Told | null>(null);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, onTell: setTold }),
    [],
  );
  const ready = (handle: Handle<Give> | null) => onReady?.(handle);
  useStatus(onStatus, rows(told));
  return (
    <Scene
      make={scene}
      value={value}
      onReady={ready}
      role="img"
      aria-label="A village of bots making and sharing things"
    />
  );
}
