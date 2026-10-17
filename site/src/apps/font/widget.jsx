import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Bar } from '../../lib/app/frame.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { cut, facts, gallery } from './engine.js';
import { Gallery } from './gallery.jsx';
import { PAGE, make } from './scene.js';
import { useUnit } from '../../lib/app/hooks.js';

export function status(value, plan, known) {
  if (value.view === 'text') return plan ? [['board', `${plan.cols}x${plan.rows}`], ['frames', plan.frames], ['time', `${(plan.frames / plan.fps / value.speed).toFixed(1)} s`]] : [];
  return known ? [['pens', known.pens], ['strokes', known.strokes], ['flagged', known.flagged]] : [];
}

export function added(value, plan, char) {
  const was = value.view === 'text' ? value.text || plan?.text || '' : value.text;
  return { view: 'text', text: cut(`${was}${char}`) };
}

export function Widget({ value, onChange, onReady, onExport, onStatus }) {
  const font = useUnit('font');
  const [plan, setPlan] = useState(null);
  const live = useRef(null);
  const kinds = useMemo(() => (font ? gallery(font) : null), [font]);
  const known = useMemo(() => (kinds ? facts(kinds) : null), [kinds]);
  const fps = useMemo(() => (font ? font.FPS() : 25), [font]);
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, font, kinds, onPlan: setPlan }), [font, kinds]);
  const ready = (handle) => {
    live.current = handle;
    onReady?.(handle);
  };
  const act = (name) => () => live.current && PAGE.actions[name](live.current);
  useEffect(() => onExport?.(font ? [['JSON', act('json')], ['SVG', act('svg')]] : []), [font]);
  const rows = status(value, plan, known);
  const said = JSON.stringify(rows);
  useEffect(() => onStatus?.(rows), [said]);
  const tap = (e) => {
    const canvas = e.currentTarget;
    const box = canvas.getBoundingClientRect();
    if (!box.width || !box.height) return;
    live.current?.tap?.(((e.clientX - box.left) * canvas.width) / box.width, ((e.clientY - box.top) * canvas.height) / box.height);
  };
  return (
    <>
      {font && <Scene make={made} value={value} onReady={ready} onPointerDown={tap} />}
      {kinds && (
        <Bar side="left">
          <Gallery kinds={kinds} tint={value.tint} rate={fps * value.speed} hold={value.hold} onPick={(char) => onChange(added(value, plan, char))} />
        </Bar>
      )}
    </>
  );
}
