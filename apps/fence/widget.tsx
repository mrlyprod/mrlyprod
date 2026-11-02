import { useCallback, useState } from "react";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Units } from "../../net/sdk/spec.ts";
import type { Fence, Held, Told } from "./scene.ts";
import { make, SCARCE } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Held>> & Pick<WidgetProps<Units, Held>, "value" | "onChange">;

const AFTER = {
  none: "nothing follows a break",
  removal: "a break means the quiet house",
  repair: "a break is mended together",
};

export function rows(told: Told | null): Readout[] {
  if (!told) return [];
  return [
    ["breaks", String(told.breaks)],
    ["repeat", String(told.repeats)],
    ["repaired", String(told.repaired)],
    ["removed", `${told.lost} hours`],
    ["trust", `${Math.floor(told.trust / 10)}%`],
    ["desperate", String(told.desperate)],
    [told.over ? "ran" : "hour", `${told.hour} of ${told.length}`],
    [
      "world",
      `${AFTER[told.prison]}; scarcity ${SCARCE[told.scarcity]}, ${told.bots} bots, ${told.hungry} starving, ${told.homeless} without a home, ${told.patched} patched by the village`,
    ],
  ];
}

export function Widget({ value, onReady, onStatus }: Props) {
  const [told, setTold] = useState<Told | null>(null);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, onTell: setTold }),
    [],
  );
  const ready = (handle: Handle<Fence> | null) => onReady?.(handle);
  useStatus(onStatus, rows(told));
  return (
    <Scene
      make={scene}
      value={value}
      onReady={ready}
      role="img"
      aria-label="A village of bots around a fenced garden"
    />
  );
}
