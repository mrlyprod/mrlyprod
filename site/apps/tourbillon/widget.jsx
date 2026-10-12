import { useCallback, useMemo, useRef, useState } from 'react';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Fold, Knob, Pick as Select, Slider, Text } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { useTouch } from '../../lib/touch.js';
import { fixed, roll, useLatest, useMath, useStatus } from '../moire/widget.jsx';
import { EYES, SCHEDULES, eyeOf, label, place, settle, trim } from './engine.js';
import { SPEC, make } from './scene.js';

const OFF = ['', 'Off the lattice'];

const SCHED = SPEC.find((row) => row.key === 'sched');

const load = () => import('./unit.js');

export function rows(f) {
  if (!f) return [];
  if (f.error) return [['error', f.error]];
  return [
    ['angle', `${place(f.angle, 2)} deg`],
    ['layers', f.layers],
    ['quarter turn', f.period === null ? 'never closes' : `${f.period} increments`],
    ['classes', f.classes],
    ['pairs sharing', f.pairs],
    ['mean', fixed(f.mean)],
    ['rms', fixed(f.rms)],
    ['rms x root L', fixed(f.faded)],
    ['centre', fixed(f.centre)],
    ['scales', f.scales.join(' ')],
    ['angles', f.angles.map((a) => place(a, 1)).join(' ')],
    ['peaks', f.peaks.map(([x, y, v]) => `${fixed(x)} ${fixed(y)} ${fixed(v)}`).join(', ')],
    ...(f.unused ? [['weights', 'unused by this blend']] : []),
    ['pixels', `${f.size} by ${f.size}`],
  ];
}

export const typed = (text) => {
  const v = Number(text);
  return String(text).trim() && Number.isFinite(v) && v >= 0 && v <= 360 ? settle(v) : null;
};

export function Widget({ value, onChange, onReady, onStatus, unit }) {
  const math = useMath(unit, load);
  const [facts, setFacts] = useState(null);
  const handle = useRef(null);
  const change = useLatest(onChange);
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, onFacts: setFacts, onTurn: (patch) => change.current(patch) }), [math]);
  const random = () => change.current({ sched: 'random', seed: roll() });
  const ready = (scene) => {
    const rich = scene && { ...scene, random };
    handle.current = rich;
    onReady?.(rich);
  };
  const nudge = (by) => handle.current?.nudge?.(by);
  const touch = useTouch(handle, change);
  useStatus(onStatus, rows(facts));
  const eyes = useMemo(() => (math ? math.tourbillon.eyes(EYES) : []), [math]);
  const shown = facts?.playing ? facts.angle : value.increment;
  const sched = SCHEDULES.find(([key]) => key === value.sched)?.[1] ?? value.sched;
  const write = (text) => {
    const v = typed(text);
    if (v !== null) onChange({ increment: v });
  };
  return (
    <>
      {math && <Scene make={made} value={value} onReady={ready} {...touch} />}
      <Bar side="status">{!math && <span>loading</span>}</Bar>
      <Bar side="right">
        <Fold name="Spin" live={`${place(shown, 2)} deg · ${sched}`}>
          <Slider label="Increment" value={Number(shown.toFixed(2))} min={0} max={360} step={0.5} unit="deg" onChange={(v) => onChange({ increment: settle(v) })} />
          <Text label="Degrees" inputMode="decimal" value={trim(shown)} commit onChange={write} />
          <Btn disabled={!math} onClick={() => nudge(-1)}>- 0.05</Btn>
          <Btn disabled={!math} onClick={() => nudge(1)}>+ 0.05</Btn>
          <Select label="Quarter turn" value={eyeOf(eyes, shown)} options={[OFF, ...eyes.map((eye, i) => [String(i), label(eye)])]} onChange={(v) => v !== '' && onChange({ increment: settle(eyes[+v].angle), sched: 'degrees' })} />
          <Knob row={SCHED} value={value.sched} values={value} onChange={onChange} />
        </Fold>
      </Bar>
    </>
  );
}
