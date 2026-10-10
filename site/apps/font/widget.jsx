import { useCallback, useEffect, useRef, useState } from 'react';
import { Bar } from '../../lib/frame.jsx';
import { Scene } from '../../lib/scene.jsx';
import { cut } from './engine.js';
import { Gallery } from './gallery.jsx';
import { PAGE, make } from './scene.js';

export function Widget({ value, onChange, onReady, onExport }) {
  const [font, setFont] = useState(null);
  const [plan, setPlan] = useState(null);
  const live = useRef(null);
  useEffect(() => {
    let on = true;
    import('./unit.js').then(({ font: unit, ready }) => ready.then(() => on && setFont(() => unit)));
    return () => {
      on = false;
    };
  }, []);
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, font }), [font]);
  const ready = (handle) => {
    live.current = handle;
    setPlan(handle?.plan ?? null);
    onReady?.(handle);
  };
  const act = (name) => () => live.current && PAGE.actions[name](live.current);
  const have = Boolean(plan);
  useEffect(() => onExport?.(have ? [['JSON', act('json')], ['SVG', act('svg')]] : []), [have]);
  const add = (char) => onChange({ text: cut(`${value.text || plan?.text || ''}${char}`) });
  return (
    <>
      {font && <Scene make={made} value={value} onReady={ready} />}
      {font && (
        <Bar side="left">
          <Gallery font={font} tint={value.tint} onPick={add} />
        </Bar>
      )}
      {plan && (
        <Bar side="status">
          <span>{plan.cols}x{plan.rows} · {plan.frames.length} frames · {(plan.frames.length / plan.fps / value.speed).toFixed(1)} s</span>
        </Bar>
      )}
    </>
  );
}
