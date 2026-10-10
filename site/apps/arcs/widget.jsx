import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { Pick, read } from '../designs/pick.jsx';
import { DIM, looping } from './engine.js';
import { make } from './scene.js';

const ROWS = [
  ['Side', (f) => f.side],
  ['Filled', (f) => `${f.filled} of ${f.cells}`],
  ['Loops', (f) => f.loops],
  ['Strands', (f) => `${f.strands} = 2 x ${f.side}`],
  ['Law', (f) => (f.law ? `L(n) = ${f.law.formula}` : 'none proved')],
  ['Law gives', (f) => (f.law ? `${f.law.loops}, ${f.law.loops === f.loops ? 'meets' : 'misses'} the count` : '-')],
];

const Fact = ({ name, value }) => (
  <label>
    <span>{name}</span>
    <span className="num">{value}</span>
  </label>
);

export function Widget({ value, onChange, onReady, onExport, unit: given }) {
  const [loaded, setLoaded] = useState(null);
  const unit = given ?? loaded;
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const [facts, setFacts] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ math, ready }) => ready.then(() => on && setLoaded(() => math)));
    return () => {
      on = false;
    };
  }, [given]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit }), [unit]);
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    const hands = {
      random: () => onChange({ code: looping(unit, read(now.current).base, now.current.level) }),
      deeper: (by) => onChange({ level: now.current.level + by }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const design = ({ dim, ...patch }) => onChange(patch);
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `arcs-${now.current.seed}`, 'svg');
  };
  const have = Boolean(facts);
  useEffect(() => onExport?.(have ? [['SVG', keep]] : []), [have]);
  const f = facts;
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="right">
        <Group name="Design">{unit && <Pick value={{ ...read(value), dim: DIM }} onChange={design} math={unit} dims={[DIM]} />}</Group>
      </Bar>
      <Bar side="status">
        <span>{f ? (f.name ? `${f.name} ${f.code}` : `code ${f.code}`) : 'loading'}</span>
      </Bar>
      {f && (
        <Bar side="right">
          <Group name="Counts">
            {ROWS.map(([name, get]) => <Fact key={name} name={name} value={get(f)} />)}
          </Group>
        </Bar>
      )}
    </>
  );
}
