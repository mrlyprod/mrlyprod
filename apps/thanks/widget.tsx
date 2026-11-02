import { useCallback, useState } from "react";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Units } from "../../net/sdk/spec.ts";
import type { Held, Thanks, Told } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Held>> & Pick<WidgetProps<Units, Held>, "value" | "onChange">;

const RULES = { runner: "the runner", maker: "the maker", all: "everyone" };

export const percent = (n: number) => `${(n / 10).toFixed(1)}%`;

export function rows(told: Told | null): Readout[] {
  if (!told) return [];
  return [
    ["made by robots", percent(told.share)],
    ["top fifth", percent(told.top)],
    ["nothing to do", String(told.idle)],
    ["robots built", String(told.built)],
    [told.over ? "ran" : "day", `${told.day} of ${told.length}`],
    [
      "world",
      `thanks to ${RULES[told.credit]}, ${told.bots} bots, ${told.alive} robots now, ${told.building} building; makers got ${percent(told.makers)} of robot thanks, ${told.paid} thanks paid for robots`,
    ],
  ];
}

export function Widget({ value, onReady, onStatus }: Props) {
  const [told, setTold] = useState<Told | null>(null);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, onTell: setTold }),
    [],
  );
  const ready = (handle: Handle<Thanks> | null) => onReady?.(handle);
  useStatus(onStatus, rows(told));
  return (
    <Scene
      make={scene}
      value={value}
      onReady={ready}
      role="img"
      aria-label="Bots and robots in a workshop town, thanks flowing between them"
    />
  );
}
