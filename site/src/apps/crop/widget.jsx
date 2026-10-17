import { useCallback, useRef, useState } from 'react';
import { Bar } from '../../lib/app/frame.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { tag } from '../../lib/two/engine.js';
import { Folds, design, useReroll, useShots, useStatus } from '../../lib/two/parts.jsx';
import { RMAX, top } from './engine.js';
import { SPEC, make, name as called } from './scene.js';
import { useUnit } from '../../lib/app/hooks.js';

/* ROWS */

export function rows(f) {
  const crop = [
    ['design', tag(f)],
    ['in', f.in],
    ['cut', f.cut],
    ['out', f.out],
    ['exposed before', f.before],
    ['exposed after', f.after],
    ['side', f.side],
    ['policy', f.note],
    ['level', f.level],
  ];
  const c = f.circle;
  if (!c) return crop;
  return [['N(r)', c.seen], ['inside(r)', c.inside], ['C(r)', c.cut], ['count side', c.side], ...crop, ['r', c.r], ['d', c.d.toFixed(7)], ['fill of one tile', c.mass]];
}

/* PANES */

export const panes = (value, onChange, unit) => [design(value, onChange, unit, SPEC)];

/* WIDGET */

export function Widget({ value, onChange, onReady, onExport, onStatus, unit: given }) {
  const unit = useUnit('math', given);
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const grip = useRef(false);
  const [have, setHave] = useState(false);
  useReroll(value, onChange, unit);
  useStatus(live, `${have}:${JSON.stringify(value)}`, onStatus, rows);
  useShots(onExport, live, have, () => called(now.current));
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit }), [unit]);
  const ready = (handle) => {
    live.current = handle;
    setHave(Boolean(handle));
    if (!handle) return onReady?.(null);
    const at = () => now.current;
    const hands = {
      flip: () => onChange({ mode: at().mode === 'anti' ? 'crop' : 'anti' }),
      grow: (by) => onChange({ radius: Math.max(0, Math.min(RMAX, at().radius + by)) }),
      deeper: (by) => onChange({ level: Math.max(1, Math.min(top(unit, at().number), at().level + by)) }),
    };
    onReady?.({ ...handle, ...hands });
  };
  const point = (e) => {
    const canvas = e.currentTarget;
    const box = canvas.getBoundingClientRect();
    return [((e.clientX - box.left) * canvas.width) / (box.width || 1), ((e.clientY - box.top) * canvas.height) / (box.height || 1)];
  };
  const move = (e) => {
    if (!grip.current) return;
    const patch = live.current?.drag?.(...point(e));
    if (patch && patch.radius !== now.current.radius) onChange(patch);
  };
  const down = (e) => {
    if (!live.current?.drag) return;
    grip.current = true;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    move(e);
  };
  const up = () => {
    grip.current = false;
  };
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />}
      <Bar side="right">
        <Folds groups={panes(value, onChange, unit)} />
      </Bar>
    </>
  );
}
