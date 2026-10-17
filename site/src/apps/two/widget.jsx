import { useCallback, useRef, useState } from 'react';
import { Bar } from '../../lib/app/frame.jsx';
import { Btn } from '../../lib/app/controls.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { cap, tag } from '../../lib/two/engine.js';
import { PRESETS, SPEC, make, name as called } from './scene.js';
import { Folds, design, knobs, useReroll, useShots, useStatus } from '../../lib/two/parts.jsx';
import { useUnit } from '../../lib/app/hooks.js';

/* ROWS */

export const rows = (f) => [
  ['design', tag(f)],
  ['copies', f.copies],
  ['exposed', f.exposed],
  ['one copy exposed', f.one],
  ['fills', f.fills],
  ['voids', f.voids],
  ['tile', `${f.side} by ${f.side}`],
  ['sheet', `${f.cols} by ${f.rows}`],
  ['level', f.level],
  ['corners', f.corners],
  ['edges', f.edges],
  ['euler', f.euler],
  ['dimension', f.dimension.toFixed(4)],
];

/* PANES */

export function panes(value, onChange, unit) {
  return [
    design(value, onChange, unit, SPEC),
    {
      name: 'Sheet',
      live: `${value.x} by ${value.y}`,
      body: (
        <>
          {knobs(['x', 'y'], value, onChange, SPEC)}
          <span className="tabs" role="group" aria-label="Presets">
            {PRESETS.map(([x, y]) => (
              <Btn key={`${x}.${y}`} on={value.x === x && value.y === y} onClick={() => onChange({ x, y })}>
                {`${x} by ${y}`}
              </Btn>
            ))}
          </span>
        </>
      ),
    },
  ];
}

/* WIDGET */

export function Widget({ value, onChange, onReady, onExport, onStatus, unit: given }) {
  const unit = useUnit('math', given);
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
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
      seams: () => onChange({ seams: at().seams ? 0 : 1 }),
      deeper: (by) => onChange({ level: Math.max(1, Math.min(cap(unit, at().number, at().x * at().y), at().level + by)) }),
      wider: (by) => onChange({ x: Math.max(1, Math.min(12, at().x + by)) }),
    };
    onReady?.({ ...handle, ...hands });
  };
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="right">
        <Folds groups={panes(value, onChange, unit)} />
      </Bar>
    </>
  );
}
