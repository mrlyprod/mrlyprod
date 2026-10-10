import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { make } from './scene.js';

const tee = (t) => (Number.isInteger(t) ? String(t) : t.toFixed(2));

const ROWS = {
  walk: [
    ['From', (f) => tee(f.from)],
    ['To', (f) => tee(f.to)],
    ['Zeros', (f) => f.zeros],
    ['First zero', (f) => (f.zeros ? f.first : '-')],
    ['Reach', (f) => f.reach.toFixed(2)],
  ],
  stairs: [
    ['Up to', (f) => f.x],
    ['Zeros', (f) => f.zeros],
    ['Last zero', (f) => (f.zeros ? f.last.toFixed(2) : '-')],
    ['psi', (f) => f.psi.toFixed(3)],
    ['Formula', (f) => f.formula.toFixed(3)],
    ['Gap', (f) => f.gap.toFixed(3)],
  ],
};

const Fact = ({ name, value }) => (
  <label>
    <span>{name}</span>
    <span className="num">{value}</span>
  </label>
);

export function Widget({ value, onReady, onExport, unit: given }) {
  const [loaded, setLoaded] = useState(null);
  const unit = given ?? loaded;
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const [facts, setFacts] = useState(null);
  const [pass, setPass] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ num, ready }) => ready.then(() => on && setLoaded(() => num)));
    return () => {
      on = false;
    };
  }, [given]);
  const tell = useCallback((count, t) => setPass({ count, t }), []);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, num: unit, onPass: tell }), [unit, tell]);
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    if (!handle) setPass(null);
    onReady?.(handle);
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `zeta-${now.current.seed}`, 'svg');
  };
  const sheet = () => {
    const rows = live.current?.csv?.();
    if (rows) text(rows, `zeta-${now.current.seed}`, 'csv');
  };
  const have = Boolean(facts);
  useEffect(() => onExport?.(have ? [['SVG', keep], ['CSV', sheet]] : []), [have]);
  const f = facts;
  const p = pass;
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="status">
        {!unit && <span>loading</span>}
        {f && <span>{p?.count ?? 0} of {f.zeros} zeros</span>}
        {f && p?.count > 0 && <span>zero {f.first + p.count - 1} at {p.t.toFixed(3)}</span>}
      </Bar>
      {f && (
        <Bar side="right">
          <Group name="Counts">
            {ROWS[f.face].map(([name, get]) => <Fact key={name} name={name} value={get(f)} />)}
          </Group>
        </Bar>
      )}
    </>
  );
}
