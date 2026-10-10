import { useCallback, useEffect, useRef, useState } from 'react';
import { download, text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { roll } from '../designs/engine.js';
import { Pick, read } from '../designs/pick.jsx';
import { DIM, tag } from './engine.js';
import { SIZES, make } from './scene.js';

const ROWS = [
  ['Level', (f) => (f.capped ? `${f.level} capped` : f.level)],
  ['Tile', (f) => `${f.side} by ${f.side}`],
  ['Sheet', (f) => `${f.cols} by ${f.rows}`],
  ['Fill', (f) => f.fills],
  ['Void', (f) => f.voids],
  ['Perimeter', (f) => f.perimeter],
  ['Euler', (f) => f.euler],
  ['Dimension', (f) => f.dimension.toFixed(3)],
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
      random: () => onChange({ code: roll(unit, DIM, read(now.current).base) }),
      deeper: (by) => onChange({ level: now.current.level + by }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const design = ({ dim, ...patch }) => onChange(patch);
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `two-${now.current.seed}`, 'svg');
  };
  const shot = (size) => live.current?.shot?.(size).then((blob) => download(blob, `two-${now.current.seed}-${size}.png`));
  const have = Boolean(facts);
  useEffect(() => onExport?.(have ? [...SIZES.map((size) => [`PNG ${size}`, () => shot(size)]), ['SVG', keep]] : []), [have]);
  const f = facts;
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="right">
        <Group name="Design">{unit && <Pick value={{ ...read(value), dim: DIM }} onChange={design} math={unit} dims={[DIM]} />}</Group>
      </Bar>
      <Bar side="status">
        <span>{f ? tag(f) : 'loading'}</span>
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
