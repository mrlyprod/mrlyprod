import { useCallback, useEffect, useRef, useState } from "react";
import { Bar } from "../../lib/app/frame.tsx";
import type { Readout } from "../../lib/app/hooks.ts";
import { useLatest } from "../../lib/app/hooks.ts";
import { Item, Library, Shelf } from "../../lib/app/library.tsx";
import type { Handle, Mathy, WidgetProps } from "../../lib/app/page.tsx";
import { Scene } from "../../lib/app/page.tsx";
import type { Opts, View } from "../../lib/app/run.ts";
import type { Patch } from "../../lib/app/spec.ts";
import { drawn, NEEDLES, RPM, title } from "../../lib/spin/engine.ts";
import { Moires } from "../../lib/spin/shelves.tsx";
import { read, Source, useReroll } from "../../lib/spin/source.tsx";
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
  useReroll(
    value.seed,
    (rand) => unit && change.current({ ...drawn(unit, read(now.current), rand), rpm: Math.floor(rand() * (RPM + 1)) }),
  );
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
    const hands = {
      speed: (by: number) => change.current({ rpm: now.current.rpm + by }),
      needle: (rpm: number) => change.current({ rpm }),
    };
    onReady?.(handle && { ...handle, ...hands });
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
