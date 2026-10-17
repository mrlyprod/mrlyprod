import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/app/export.js';
import { Bar } from '../../lib/app/frame.jsx';
import { Group, Slider } from '../../lib/app/controls.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { rng } from '../../lib/rand.js';
import { roll } from '../../lib/designs/engine.js';
import { Pick, read } from '../../lib/designs/pick.jsx';
import { DIM } from '../../lib/three/engine.js';

import { make, name } from './scene.js';
import { useReroll, useUnit } from '../../lib/app/hooks.js';

export function Widget({ value, onChange, onReady, onExport, onStatus, unit }) {
  const math = useUnit('math', unit);
  const live = useRef(null);
  const now = useRef(value);
  const held = useRef(false);
  const [info, setInfo] = useState(null);
  const seen = useRef(null);
  const plane = useRef(`${value.code} ${value.base} ${value.level}`);
  now.current = value;
  seen.current = info;
  const tell = useCallback((next) => setInfo(next), []);
  useEffect(() => onStatus?.(info?.rows ?? []), [info]);
  useEffect(() => {
    const next = `${value.code} ${value.base} ${value.level}`;
    if (plane.current === next) return;
    plane.current = next;
    if (String(value.height ?? '') !== '') onChange({ height: '' });
  }, [value.code, value.base, value.level]);
  useReroll(value.seed, (seed) => math && onChange({ code: roll(math, DIM, read(now.current).base, rng(seed)), height: '' }));
  const cut = info?.cut;
  const at = info?.heights?.[0];
  const ready = (handle) => {
    live.current = handle;
    const hands = {
      random: () => onChange({ code: roll(math, DIM, read(now.current).base), height: '' }),
      slide: (by) => {
        const got = seen.current;
        if (!got || got.cut.empty) return;
        onChange({ height: Math.min(got.cut.high, Math.max(got.cut.low, got.heights[0] + by)), both: 0 });
      },
      deeper: (by) => onChange({ level: now.current.level + by }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const spot = (e) => {
    const canvas = e.currentTarget;
    const rect = canvas.getBoundingClientRect();
    return live.current?.seek?.(((e.clientX - rect.left) / rect.width) * canvas.width, ((e.clientY - rect.top) / rect.height) * canvas.height) ?? null;
  };
  const go = (height) => height !== null && onChange({ height, both: 0 });
  const down = (e) => {
    const height = spot(e);
    if (height === null) return;
    held.current = true;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    go(height);
  };
  const move = (e) => held.current && go(spot(e));
  const up = () => {
    held.current = false;
  };
  const svg = () => live.current?.svg?.().then((body) => text(body, name(now.current), 'svg'));
  useEffect(() => onExport?.(math ? [['SVG', svg]] : []), [Boolean(math)]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, tell }), [math]);
  const both = Number(value.both) === 1;
  return (
    <>
      {math && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />}
      <Bar side="right">
        <Group name="Design">{math ? <Pick value={{ ...read(value), dim: DIM }} onChange={({ base, code }) => onChange({ base, code, height: '' })} math={math} dims={[DIM]} /> : <span>loading</span>}</Group>
        {cut && !cut.empty && (
          <Group name="Plane">
            <Slider label="Height" value={at} min={cut.low} max={cut.high} step={1} disabled={both} onChange={(v) => onChange({ height: v })} />
          </Group>
        )}
      </Bar>
    </>
  );
}
