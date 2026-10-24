import { useCallback, useEffect, useRef, useState } from "react";
import { Bar } from "../../lib/app/frame.jsx";
import { useLatest } from "../../lib/app/hooks.js";
import { Item, Library, Shelf } from "../../lib/app/library.jsx";
import { Scene } from "../../lib/app/page.jsx";
import { drawn, NEEDLES, RPM, title } from "../../lib/spin/engine.js";
import { Moires } from "../../lib/spin/shelves.jsx";
import { read, Source, useReroll } from "../../lib/spin/source.jsx";
import { make, spinning } from "./scene.js";

const deg = (x) => `${x.toFixed(1)}°`;

const SPEEDS = { 33: "long play", 45: "single", 78: "shellac", 899: "drifts back", 900: "freezes" };

export const next = (rpm) => NEEDLES.find((at) => at > rpm) ?? NEEDLES[0];

export function rows(facts, pace) {
  if (!facts) return [];
  return [
    ["name", title(facts)],
    ...(pace
      ? [
          ["screen", `${Math.round(pace.hz)} Hz`],
          ["per frame", deg(pace.deg)],
        ]
      : []),
    ...(pace && facts.order ? [["seen", deg(pace.seen)]] : []),
    ["side", String(facts.side)],
    ...(facts.fills === null ? [] : [["filled", String(facts.fills)]]),
    ["mass of the rings", facts.mass.toFixed(1)],
    ["dark disc to", facts.disc.toFixed(2)],
    ["brightest ring", facts.peak.toFixed(3)],
    ["last ring at", facts.reach.toFixed(1)],
    ["rotation order", facts.order ? String(facts.order) : "round"],
  ];
}

export function Needles({ rpm, onChange }) {
  return (
    <Shelf name="Needles">
      {NEEDLES.map((at) => (
        <Item key={at} label={`${at} rpm`} sub={SPEEDS[at]} on={rpm === at} onPick={() => onChange({ rpm: at })} />
      ))}
    </Shelf>
  );
}

export function Widget({ value, onChange, onReady, onStatus, units }) {
  const unit = units?.math;
  const [facts, setFacts] = useState(null);
  const [pace, setPace] = useState(null);
  const live = useRef(null);
  const now = useLatest(value);
  const change = useLatest(onChange);
  const table = spinning(value);
  useReroll(
    value.seed,
    (rand) => unit && change.current({ ...drawn(unit, read(now.current), rand), rpm: Math.floor(rand() * (RPM + 1)) }),
  );
  useEffect(() => onStatus?.(rows(facts, pace)), [facts, pace]);
  const scene = useCallback(
    (canvas, view, opts) => make(canvas, view, { ...opts, math: unit, onPace: setPace, onFacts: setFacts }),
    [unit],
  );
  const needle = () => {
    if (live.current?.playing?.() === false) live.current.toggle();
    change.current({ rpm: next(now.current.rpm) });
  };
  const ready = (handle) => {
    live.current = handle;
    setPace(null);
    const hands = {
      speed: (by) => change.current({ rpm: now.current.rpm + by }),
      needle: (rpm) => change.current({ rpm }),
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
