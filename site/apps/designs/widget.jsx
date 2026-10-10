import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group, Segment } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { codes, distinct, gallery, named, next, roll, total } from './engine.js';
import { Thumbs, read, useUnit } from './pick.jsx';
import { make } from './scene.js';

const FILTERS = [['all', 'All'], ['classes', 'Classes'], ['named', 'Named']];

const ROWS = [
  ['Name', (info) => info.title],
  ['Normal form', (info) => info.anf],
  ['Degree', (info) => (info.degree >= 0 ? info.degree : '')],
  ['Class', (info) => info.rep],
  ['Orbit', (info) => info.orbit],
  ['Level', (info) => info.level],
  ['Side', (info) => info.side],
  ['Fills', (info) => info.fills],
  ['Voids', (info) => info.voids],
  ['Ratio', (info) => (info.fills / (info.fills + info.voids)).toFixed(4)],
  ['Perimeter', (info) => info.perimeter],
  ['Surface', (info) => info.surface],
  ['Euler', (info) => info.euler],
  ['Dimension', (info) => info.dimension.toFixed(3)],
];

const Fact = ({ name, value }) => (
  <label>
    <span>{name}</span>
    <span className="num">{value}</span>
  </label>
);

export function useThree(dim, given) {
  const [three, setThree] = useState(null);
  useEffect(() => {
    if (given || dim !== 3 || three) return undefined;
    let on = true;
    import('three').then((mod) => on && setThree(mod));
    return () => {
      on = false;
    };
  }, [given, dim, three]);
  return given ?? three;
}

export function Widget({ value, onChange, onReady, onExport, unit, three: given }) {
  const math = useUnit(unit);
  const { dim, base, code } = read(value);
  const three = useThree(dim, given);
  const live = useRef(null);
  const now = useRef(null);
  const from = useRef(null);
  const [info, setInfo] = useState(null);
  const [filter, setFilter] = useState('all');
  const listed = Boolean(math && codes(math, dim, base));
  const shown = listed ? filter : 'named';
  const list = useMemo(() => (math ? gallery(math, dim, base, shown) : []), [math, dim, base, shown]);
  const names = useMemo(() => (math ? named(math, dim, base) : []), [math, dim, base]);
  now.current = { list, code: info?.code ?? code, level: value.level, seed: value.seed, dim, base };
  const staged = Boolean(math && (dim !== 3 || three));
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, three }), [math, three]);
  const ready = (handle) => {
    live.current = handle;
    setInfo(handle?.info ?? null);
    const hands = {
      random: () => onChange({ code: roll(math, now.current.dim, now.current.base) }),
      step: (by) => onChange({ code: next(now.current.list, now.current.code, by) }),
      deeper: (by) => onChange({ level: now.current.level + by }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const down = (e) => {
    if (e.button) return;
    from.current = [e.clientX, e.clientY];
    e.currentTarget.setPointerCapture?.(e.pointerId);
  };
  const move = (e) => {
    if (!from.current) return;
    const box = e.currentTarget.getBoundingClientRect();
    live.current?.turn?.((e.clientX - from.current[0]) / box.width, (e.clientY - from.current[1]) / box.height);
    from.current = [e.clientX, e.clientY];
  };
  const up = () => {
    from.current = null;
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `designs-${now.current.seed}`, 'svg');
  };
  const flat = info?.dim === 2;
  useEffect(() => onExport?.(flat ? [['SVG', keep]] : []), [flat]);
  const heading = math ? `${total(math, dim, base).toLocaleString()} codes, ${distinct(math, dim, base).toLocaleString()} classes` : 'loading';
  return (
    <>
      {staged && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />}
      <Bar side="left">
        <div className="controls">
          <Group name={heading}>
            <Segment label="Show" value={shown} options={listed ? FILTERS : FILTERS.slice(2)} onChange={setFilter} />
            {math && <Thumbs math={math} dim={dim} base={base} codes={list ?? []} code={info?.code ?? code} names={names} onPick={(v) => onChange({ code: v })} />}
          </Group>
        </div>
      </Bar>
      {info && (
        <Bar side="right">
          <Group name="Counts" className="facts">
            {ROWS.filter(([, get]) => get(info) !== '' && get(info) !== undefined).map(([name, get]) => <Fact key={name} name={name} value={get(info)} />)}
          </Group>
        </Bar>
      )}
    </>
  );
}
