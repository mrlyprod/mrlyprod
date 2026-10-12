import { useCallback, useEffect, useRef, useState } from 'react';
import { download, text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Fold, Knob } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { rng } from '../../lib/scene.js';
import { roll, settle } from '../designs/engine.js';
import { Pick, read } from '../designs/pick.jsx';
import { DIM, cap, named, tag } from './engine.js';
import { PRESETS, SIZES, SPEC, make, name as called } from './scene.js';

const WAIT = 90;

/* SHARED */

export function useMath(load, given) {
  const [loaded, setLoaded] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    load().then(({ math, ready }) => ready.then(() => on && setLoaded(() => math)));
    return () => {
      on = false;
    };
  }, [given]);
  return given ?? loaded;
}

export function useReroll(value, onChange, math) {
  const seen = useRef(value.seed);
  useEffect(() => {
    if (value.seed === seen.current || !math) return;
    seen.current = value.seed;
    onChange({ code: roll(math, DIM, read(value).base, rng(value.seed)) });
  }, [value.seed, math]);
}

export function useStatus(live, key, onStatus, rows) {
  useEffect(() => {
    if (!onStatus) return undefined;
    const timer = setTimeout(() => {
      const facts = live.current?.facts?.();
      if (facts) onStatus(rows(facts));
    }, WAIT);
    return () => clearTimeout(timer);
  }, [key]);
}

export function useShots(onExport, live, have, name) {
  const svg = () => {
    const made = live.current?.svg?.();
    if (made) text(made, name(), 'svg');
  };
  const shot = (size) => live.current?.shot?.(size).then((blob) => download(blob, `${name()}-${size}.png`));
  useEffect(() => onExport?.(have ? [...SIZES.map((size) => [`PNG ${size}`, () => shot(size)]), ['SVG', svg]] : []), [have]);
}

export function Folds({ groups }) {
  return groups.map(({ name, live, body }) => (
    <Fold key={name} name={name} live={live}>
      {body}
    </Fold>
  ));
}

export function knobs(keys, value, onChange, spec, units) {
  return keys.map((key) => {
    const one = spec.find((it) => it.key === key);
    return <Knob key={key} row={one} value={value[key]} values={value} units={units} onChange={onChange} />;
  });
}

export function design(value, onChange, unit, spec) {
  const pick = ({ dim, ...patch }) => onChange(patch);
  const { base, code } = read(value);
  const shown = unit ? settle(unit, DIM, base, code, rng(value.seed)) : code;
  const name = unit ? named(unit, shown, base) : '';
  const word = spec.find((one) => one.key === 'number').label.toLowerCase();
  return {
    name: 'Design',
    live: [name || shown, `${word} ${value.number}`, `level ${value.level}`].join(' · '),
    body: (
      <>
        {unit && <Pick value={{ ...read(value), dim: DIM }} onChange={pick} math={unit} dims={[DIM]} />}
        {knobs(['number', 'level'], value, onChange, spec, unit && { math: unit })}
      </>
    ),
  };
}

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
  const unit = useMath(() => import('./unit.js'), given);
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
