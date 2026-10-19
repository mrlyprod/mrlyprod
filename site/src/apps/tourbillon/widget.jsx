import { useCallback, useMemo, useRef, useState } from 'react';
import { Bar } from '../../lib/app/frame.jsx';
import { Btn, Fold, Slider, Text } from '../../lib/app/controls.jsx';
import { Art, Item, Library, Shelf } from '../../lib/app/library.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { useTouch } from '../../lib/app/touch.js';
import { paints } from '../../lib/color.js';
import { useTheme } from '../../lib/designs/pick.jsx';
import { fixed } from '../../lib/moire/parts.jsx';
import { stamp } from '../../lib/moire/sheet.js';
import { EYES, KINDS, NUDGE, SCHEDULES, eyeOf, label, place, settle, shown, swatch, trim } from './engine.js';
import { make } from './scene.js';
import { useLatest, useReroll, useStatus, useUnit } from '../../lib/app/hooks.js';

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

export function Shelves({ value, onChange, math }) {
  const [kind, setKind] = useState('unit');
  const [find, setFind] = useState('');
  const tick = useTheme();
  const eyes = useMemo(() => (math ? math.tourbillon.eyes(EYES) : []), [math]);
  const now = eyeOf(eyes, value.increment);
  const look = [value.limit, value.set, value.weights, value.mode, value.blend, value.ramp, value.levels, value.invert, value.tint, value.seed, tick];
  const thumb = (over, at) => (ctx, w) => stamp(ctx, swatch(math, { ...value, ...over }, at, w, paints(ctx.canvas, value.tint)), w);
  return (
    <Library chips={KINDS} chip={kind} onChip={setKind} find={find} onFind={kind === 'all' ? setFind : undefined}>
      {math && kind === 'sched' && (
        <Shelf name="Schedules">
          {SCHEDULES.map(([key, text]) => <Item key={key} label={text} on={value.sched === key} onPick={() => onChange({ sched: key })} art={<Art paint={thumb({ sched: key }, value.increment)} deps={[key, value.increment, ...look]} />} />)}
        </Shelf>
      )}
      {math && kind !== 'sched' && (
        <Shelf name="Quarter turns" grid>
          {shown(eyes, kind, find).map(([eye, at]) => (
            <Item key={at} label={trim(eye.angle)} sub={eye.denom === 1 ? '' : `${eye.numer}/${eye.denom}`} title={label(eye)} on={now === String(at)} onPick={() => onChange({ increment: settle(eye.angle), sched: 'degrees' })} art={<Art px={64} paint={thumb({ sched: 'degrees' }, eye.angle)} deps={[at, ...look]} />} />
          ))}
        </Shelf>
      )}
    </Library>
  );
}

export function Widget({ value, onChange, onReady, onStatus, unit }) {
  const math = useUnit('math', unit);
  const [facts, setFacts] = useState(null);
  const handle = useRef(null);
  const change = useLatest(onChange);
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, onFacts: setFacts, onTurn: (patch) => change.current(patch) }), [math]);
  const ready = (scene) => {
    handle.current = scene;
    onReady?.(scene);
  };
  const nudge = (by) => handle.current?.nudge?.(by);
  const touch = useTouch(handle, change);
  useReroll(value.seed, () => change.current({ sched: 'random' }));
  useStatus(onStatus, rows(facts));
  const shownNow = facts?.playing ? facts.angle : value.increment;
  const sched = SCHEDULES.find(([key]) => key === value.sched)?.[1] ?? value.sched;
  const write = (text) => {
    const v = typed(text);
    if (v !== null) onChange({ increment: v });
  };
  return (
    <>
      {math && <Scene make={made} value={value} onReady={ready} {...touch} />}
      <Bar side="status">{!math && <span>loading</span>}</Bar>
      <Shelves value={value} onChange={onChange} math={math} />
      <Bar side="right">
        <Fold name="Spin" live={`${place(shownNow, 2)} deg · ${sched}`}>
          <Slider label="Increment" value={Number(shownNow.toFixed(2))} min={0} max={360} step={0.5} unit="deg" onChange={(v) => onChange({ increment: settle(v) })} />
          <Text label="Degrees" inputMode="decimal" value={trim(shownNow)} commit onChange={write} />
          <Btn disabled={!math} onClick={() => nudge(-1)}>{`Nudge down ${NUDGE} deg`}</Btn>
          <Btn disabled={!math} onClick={() => nudge(1)}>{`Nudge up ${NUDGE} deg`}</Btn>
        </Fold>
      </Bar>
    </>
  );
}
