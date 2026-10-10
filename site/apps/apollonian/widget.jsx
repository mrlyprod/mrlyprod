import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { said, step } from './engine.js';
import { make } from './scene.js';

const ROWS = [
  ['Circles', (f) => f.circles],
  ['Ford', (f) => f.ford],
  ['Nodes', (f) => f.nodes],
  ['Brightness', (f) => (f.bright === null ? null : `${f.bright} of ${f.want}`)],
];

const at = (e) => {
  const box = e.currentTarget.getBoundingClientRect();
  return [(e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height];
};

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
  const [tapped, setTapped] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ num, ready }) => ready.then(() => on && setLoaded(() => num)));
    return () => {
      on = false;
    };
  }, [given]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, num: unit, once: true }), [unit]);
  const deeper = (by) => {
    const next = step(now.current.cap, by);
    if (next !== now.current.cap) onChange({ cap: next });
  };
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    setTapped(null);
    const again = () => {
      handle.again();
      setTapped(null);
    };
    onReady?.(handle && { ...handle, deeper, again });
  };
  const down = (e) => {
    if (e.button) return;
    setTapped(live.current?.tap(...at(e)) ?? null);
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `apollonian-${now.current.seed}`, 'svg');
  };
  const dump = () => {
    const rows = live.current?.json?.();
    if (rows) text(rows, `apollonian-${now.current.seed}`, 'json');
  };
  const have = Boolean(facts);
  useEffect(() => onExport?.(have ? [['SVG', keep], ['JSON', dump]] : []), [have]);
  const f = facts;
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} />}
      <Bar side="status">
        {!unit && <span>loading</span>}
        {f && <span>{f.title}</span>}
        {tapped && <span>{said(tapped)}</span>}
      </Bar>
      {f && (
        <Bar side="right">
          <Group name="Counts">
            {ROWS.map(([name, get]) => (get(f) === null ? null : <Fact key={name} name={name} value={get(f)} />))}
          </Group>
        </Bar>
      )}
    </>
  );
}
