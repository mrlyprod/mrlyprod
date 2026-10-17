import { useCallback, useEffect, useRef, useState } from 'react';
import { Bar } from '../../lib/app/frame.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { BLENDS, COPIES, drawn, title } from '../../lib/spin/engine.js';
import { Source, read, useReroll } from '../../lib/spin/source.jsx';
import { make } from './scene.js';
import { useLatest, useUnit } from '../../lib/app/hooks.js';

const pct = (x) => `${(x * 100).toFixed(1)}%`;

const BLEND = BLENDS.map(([one]) => one).filter((one) => one !== 'meet');

export const wrap = (copies, by) => ((copies - 1 + by + COPIES) % COPIES) + 1;

export function rows(facts) {
  if (!facts) return [];
  return [
    ['name', title(facts)],
    ['side', String(facts.side)],
    ['copies', String(facts.copies)],
    ['step', `${facts.step}°`],
    ['rotation order', facts.order ? String(facts.order) : 'round'],
    ['petals', String(facts.petals)],
    ['order 0', pct(facts.share)],
    ...facts.leading.map((one) => [`order ${one.order}`, pct(one.share)]),
  ];
}

export function rolled(math, value, rand) {
  return { ...drawn(math, read(value), rand), copies: 1 + Math.floor(rand() * COPIES), blend: BLEND[Math.floor(rand() * BLEND.length)], full: 1 };
}

export function Widget({ value, onChange, onReady, onStatus, unit: given }) {
  const unit = useUnit('math', given);
  const [facts, setFacts] = useState(null);
  const now = useLatest(value);
  const change = useLatest(onChange);
  const was = useRef(value);
  useReroll(value.seed, (rand) => unit && change.current(rolled(unit, now.current, rand)));
  useEffect(() => {
    const before = was.current;
    was.current = value;
    if (Number(before.full) === 1 && Number(value.full) !== 1 && before.step === value.step) change.current({ step: 360 / value.copies });
  }, [value.full, value.step]);
  useEffect(() => onStatus?.(rows(facts)), [facts]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit, onFacts: setFacts }), [unit]);
  const copies = (by, loop) => {
    const at = now.current.copies;
    change.current({ copies: loop ? wrap(at, by) : Math.min(COPIES, Math.max(1, at + by)) });
  };
  const ready = (handle) => {
    const hands = {
      whole: () => change.current({ full: 1 }),
      copies: (by) => copies(by, false),
      step: (by) => {
        const { full: whole, step, copies: n } = now.current;
        change.current({ full: 0, step: Math.min(360, Math.max(0, (Number(whole) === 1 ? 360 / n : step) + by)) });
      },
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} onClick={() => copies(1, true)} />}
      <Bar side="right">
        <Source value={value} onChange={onChange} math={unit} />
      </Bar>
    </>
  );
}
