import { useCallback, useEffect, useRef, useState } from "react";
import { Bar } from "../../net/sdk/frame.tsx";
import type { Readout } from "../../net/sdk/hooks.ts";
import { useLatest } from "../../net/sdk/hooks.ts";
import { Item, Library, Shelf } from "../../net/sdk/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../net/sdk/page.tsx";
import { Scene } from "../../net/sdk/page.tsx";
import type { Opts, View } from "../../net/sdk/run.ts";
import type { Patch } from "../../net/sdk/spec.ts";
import { NEEDLES, title } from "../shared/spin/engine.ts";
import { Moires } from "../shared/spin/shelves.tsx";
import { Source } from "../shared/spin/source.tsx";
import type { Facts, Pace, Spin, Spun } from "./scene.ts";
import { make, spinning } from "./scene.ts";

/* TYPES */

export type NeedlesProps = { rpm: number; onChange: (patch: Patch) => void };

type Props = WidgetProps<Mathy, Spun> & { value: Spin & { seed: number } };

const deg = (x: number) => `${x.toFixed(1)}°`;

const SPEEDS: Record<number, string> = {
  33: "long play",
  45: "single",
  78: "shellac",
  899: "drifts back",
  900: "freezes",
};

export const next = (rpm: number) => NEEDLES.find((at) => at > rpm) ?? NEEDLES[0];

export function rows(facts: Facts | null, pace: Pace | null): Readout[] {
  if (!facts) return [];
  return [
    ["name", title(facts)],
    ...(pace
      ? [["screen", `${Math.round(pace.hz)} Hz`] satisfies Readout, ["per frame", deg(pace.deg)] satisfies Readout]
      : []),
    ...(pace && facts.order ? [["seen", deg(pace.seen)] satisfies Readout] : []),
    ["side", String(facts.side)],
    ...(facts.fills === null ? [] : [["filled", String(facts.fills)] satisfies Readout]),
    ["mass of the rings", facts.mass.toFixed(1)],
    ["dark disc to", facts.disc.toFixed(2)],
    ["brightest ring", facts.peak.toFixed(3)],
    ["last ring at", facts.reach.toFixed(1)],
    ["rotation order", facts.order ? String(facts.order) : "round"],
  ];
}

export function Needles({ rpm, onChange }: NeedlesProps) {
  return (
    <Shelf name="Needles">
      {NEEDLES.map((at) => (
        <Item key={at} label={`${at} rpm`} sub={SPEEDS[at]} on={rpm === at} onPick={() => onChange({ rpm: at })} />
      ))}
    </Shelf>
  );
}

export function Widget({ value, onChange, onReady, onStatus, units }: Props) {
  const unit = units?.math;
  const [facts, setFacts] = useState<Facts | null>(null);
  const [pace, setPace] = useState<Pace | null>(null);
  const live = useRef<Handle | null>(null);
  const now = useLatest(value);
  const change = useLatest(onChange);
  const table = spinning(value);
  useEffect(() => onStatus?.(rows(facts, pace)), [facts, pace]);
  const scene = useCallback(
    (canvas: HTMLCanvasElement, view: View, opts: Opts) =>
      make(canvas, view, { ...opts, math: unit, onPace: setPace, onFacts: setFacts }),
    [unit],
  );
  const needle = () => {
    if (live.current?.playing?.() === false) live.current.toggle!();
    change.current({ rpm: next(now.current.rpm) });
  };
  const ready = (handle: Handle | null) => {
    live.current = handle;
    setPace(null);
    onReady?.(handle);
  };
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} onClick={table ? needle : undefined} />}
      <Library>
        {table && <Needles rpm={value.rpm} onChange={onChange} />}
        <Moires value={value} onChange={onChange} math={unit} />
      </Library>
      <Bar side="right">
        <Source value={value} onChange={onChange} math={unit} />
      </Bar>
    </>
  );
}
