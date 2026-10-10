import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { Pick, read } from '../designs/pick.jsx';
import { DIM, tag } from './engine.js';
import { make } from './scene.js';

const KEEP = { flex: '0 0.01 auto', minWidth: 0, overflow: 'hidden', textOverflow: 'ellipsis' };

const Fact = ({ name, value }) => (
  <label>
    <span>{name}</span>
    <span className="num">{value}</span>
  </label>
);

export function Widget({ value, onChange, onReady, onExport, units: given }) {
  const [loaded, setLoaded] = useState(null);
  const units = given ?? loaded;
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const [facts, setFacts] = useState(null);
  const [tile, setTile] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ math, num, ready }) => ready.then(() => on && setLoaded({ math, num })));
    return () => {
      on = false;
    };
  }, [given]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units, onTile: setTile }), [units]);
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    if (!handle) setTile(null);
    onReady?.(handle);
  };
  const design = ({ dim, ...patch }) => onChange(patch);
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `snail-${now.current.seed}`, 'svg');
  };
  const have = Boolean(facts);
  useEffect(() => onExport?.(have ? [['SVG', keep]] : []), [have]);
  const f = facts;
  return (
    <>
      {units && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="right">
        <Group name="Design">{units && <Pick value={{ ...read(value), dim: DIM }} onChange={design} math={units.math} dims={[DIM]} />}</Group>
      </Bar>
      <Bar side="status">
        <span style={KEEP}>{f ? tag(f) : 'loading'}</span>
        {tile && <span>n {tile.n}{tile.prime ? ' prime' : ''}</span>}
      </Bar>
      {f && (
        <Bar side="right">
          <Group name="Counts">
            <Fact name="Numbers" value={f.tiles} />
            <Fact name="Primes" value={f.primes} />
            <Fact name="Grown" value={f.grown} />
            {f.levels.map(({ level, count, side }) => <Fact key={level} name={`Side ${side}`} value={count} />)}
            <Fact name="Largest" value={`${f.side} by ${f.side}`} />
            <Fact name="Area" value={f.area} />
            <Fact name="Box" value={`${f.width} by ${f.height}`} />
          </Group>
        </Bar>
      )}
    </>
  );
}
