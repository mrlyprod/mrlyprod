import { useCallback, useEffect, useRef, useState } from 'react';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { NEEDLES, RPM, drawn, title } from './engine.js';
import { make, spinning } from './scene.js';
import { Source, read, useLatest, useReroll, useUnit } from './source.jsx';

const deg = (x) => `${x.toFixed(1)}°`;

export const next = (rpm) => NEEDLES.find((at) => at > rpm) ?? NEEDLES[0];

export function rows(facts, pace) {
  if (!facts) return [];
  return [
    ['name', title(facts)],
    ...(pace ? [['screen', `${Math.round(pace.hz)} Hz`], ['per frame', deg(pace.deg)]] : []),
    ...(pace && facts.order ? [['seen', deg(pace.seen)]] : []),
    ['side', String(facts.side)],
    ...(facts.fills === null ? [] : [['filled', String(facts.fills)]]),
    ['mass of the rings', facts.mass.toFixed(1)],
    ['dark disc to', facts.disc.toFixed(2)],
    ['brightest ring', facts.peak.toFixed(3)],
    ['last ring at', facts.reach.toFixed(1)],
    ['rotation order', facts.order ? String(facts.order) : 'round'],
  ];
}

export function Needles({ rpm, onChange }) {
  return (
    <Group name="Needles" after="">
      {NEEDLES.map((at) => <Btn key={at} on={rpm === at} onClick={() => onChange({ rpm: at })}>{at}</Btn>)}
    </Group>
  );
}

export function Pane({ value, onChange, math }) {
  return (
    <>
      <Source value={value} onChange={onChange} math={math} />
      {spinning(value) && <Needles rpm={value.rpm} onChange={onChange} />}
    </>
  );
}

export function Widget({ value, onChange, onReady, onStatus, unit: given }) {
  const unit = useUnit(given);
  const [facts, setFacts] = useState(null);
  const [pace, setPace] = useState(null);
  const live = useRef(null);
  const now = useLatest(value);
  const change = useLatest(onChange);
  const table = spinning(value);
  useReroll(value.seed, (rand) => unit && change.current({ ...drawn(unit, read(now.current), rand), rpm: Math.floor(rand() * (RPM + 1)) }));
  useEffect(() => onStatus?.(rows(facts, pace)), [facts, pace]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit, onPace: setPace, onFacts: setFacts }), [unit]);
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
      <Bar side="right">
        <Pane value={value} onChange={onChange} math={unit} />
      </Bar>
    </>
  );
}
