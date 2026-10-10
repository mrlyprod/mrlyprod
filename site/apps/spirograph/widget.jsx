import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { make } from './scene.js';

const pct = (x) => `${Math.round(x * 100)}%`;

const turns = (x) => (Number.isInteger(x) ? String(x) : x.toFixed(2));

const Count = ({ name, children }) => (
  <label>
    <span>{name}</span>
    <span className="num">{children}</span>
  </label>
);

export function Widget({ value, onReady, onExport }) {
  const [unit, setUnit] = useState(null);
  const [facts, setFacts] = useState(null);
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  useEffect(() => {
    let on = true;
    import('./unit.js').then(async ({ math, ready }) => {
      await ready;
      if (on) setUnit(math);
    });
    return () => {
      on = false;
    };
  }, []);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit }), [unit]);
  const ready = (handle) => {
    live.current = handle;
    if (handle) setFacts(handle.facts);
    onReady?.(handle);
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `spirograph-${now.current.seed}`, 'svg');
  };
  const have = Boolean(facts);
  useEffect(() => onExport?.(have ? [['SVG', keep]] : []), [have]);
  const f = facts;
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="status">
        {!unit && <span>loading</span>}
        {f && <span>code {f.code}</span>}
      </Bar>
      {f && (
        <Bar side="right">
          <Group name="Counts">
            <Count name="Cells">{f.width} by {f.height}</Count>
            <Count name="Pencils">{f.pencils}</Count>
            <Count name="Curves">{f.distinct}</Count>
            <Count name="Nodes">{f.nodes ?? '-'}</Count>
            <Count name="Ratio">{f.round ? `${f.ratio[0]}/${f.ratio[1]}` : '-'}</Count>
            <Count name="Turns">{turns(f.turns)}</Count>
            <Count name="Cover">{f.cover === null ? '-' : pct(f.cover)}</Count>
          </Group>
        </Bar>
      )}
    </>
  );
}
