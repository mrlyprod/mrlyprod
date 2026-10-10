import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { roll } from '../designs/engine.js';
import { Pick, read } from '../designs/pick.jsx';
import { DIM, PROJECTIONS } from './engine.js';
import { make } from './scene.js';

const VIEWS = PROJECTIONS.map(([key]) => key);

const COUNTS = [
  ['Triangles', (f) => f.triangles],
  ['Filled', (f) => f.fills],
  ['Empty', (f) => f.voids],
  ['Pieces', (f) => f.pieces],
  ['Holes', (f) => f.holes],
  ['Euler', (f) => f.euler],
  ['Copies', (f) => (f.copies > 1 ? f.copies : '')],
];

const Count = ({ name, children }) => (
  <label>
    <span>{name}</span>
    <span className="num">{children}</span>
  </label>
);

export function Widget({ value, onChange, onReady, onExport }) {
  const [unit, setUnit] = useState(null);
  const [facts, setFacts] = useState(null);
  const live = useRef(null);
  const now = useRef(value);
  const change = useRef(onChange);
  now.current = value;
  change.current = onChange;
  useEffect(() => {
    let on = true;
    import('./unit.js').then(({ math, ready }) => ready.then(() => on && setUnit(() => math)));
    return () => {
      on = false;
    };
  }, []);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit }), [unit]);
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    const hands = {
      random: () => change.current({ code: roll(unit, DIM, read(now.current).base) }),
      view: (by) => change.current({ projection: VIEWS[(VIEWS.indexOf(now.current.projection) + by + VIEWS.length) % VIEWS.length] }),
      deeper: (by) => change.current({ level: now.current.level + by }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const pick = ({ base, code }) => onChange({ base, code });
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `six-${now.current.seed}`, 'svg');
  };
  const have = Boolean(facts);
  useEffect(() => onExport?.(have ? [['SVG', keep]] : []), [have]);
  const f = facts;
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="right">
        <Group name="Design">{unit ? <Pick value={{ ...read(value), dim: DIM }} onChange={pick} math={unit} dims={[DIM]} /> : <span>loading</span>}</Group>
      </Bar>
      <Bar side="status">
        {!unit && <span>loading</span>}
        {f && <span>{f.name ? `${f.name} ${f.code}` : f.title}</span>}
        {f && <span>side {f.side}</span>}
      </Bar>
      {f && (
        <Bar side="right">
          <Group name="Counts">
            {COUNTS.filter(([, get]) => get(f) !== '').map(([name, get]) => <Count key={name} name={name}>{get(f)}</Count>)}
          </Group>
        </Bar>
      )}
    </>
  );
}
