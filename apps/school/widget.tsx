import { useCallback, useState } from "react";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useStatus } from "../../net/sdk/hooks.ts";
import type { Handle, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Units } from "../../net/sdk/spec.ts";
import { TENTHS } from "./engine.ts";
import type { Held, School, Told } from "./scene.ts";
import { make } from "./scene.ts";

/* TYPES */

type Props = Partial<WidgetProps<Units, Held>> & Pick<WidgetProps<Units, Held>, "value" | "onChange">;

export const tenths = (n: number) => `${Math.floor(n / TENTHS)}.${n % TENTHS}`;

export function rows(told: Told | null): Readout[] {
  if (!told) return [];
  return [
    ["skill", tenths(told.skill)],
    ["asked", String(told.asked)],
    ["taught", String(told.taught)],
    ["invented", String(told.invented)],
    ["open", String(told.open)],
    [told.over ? "ran" : "day", `${told.day} of ${told.length}`],
    [
      "world",
      `${told.pupils} pupils, ${told.puzzles} puzzles: ${told.understood} understood, ${told.told} only told; first worked out ${told.solved}; deepest understood ${told.deepest}, deepest known ${told.reach}`,
    ],
  ];
}

export function Widget({ value, onReady, onStatus }: Props) {
  const [told, setTold] = useState<Told | null>(null);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) => make(canvas, view, { ...opts, onTell: setTold }),
    [],
  );
  const ready = (handle: Handle<School> | null) => onReady?.(handle);
  useStatus(onStatus, rows(told));
  return (
    <Scene
      make={scene}
      value={value}
      onReady={ready}
      role="img"
      aria-label="A school of bots growing a tree of puzzles"
    />
  );
}
