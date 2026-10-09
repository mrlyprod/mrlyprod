import { createContext, useCallback, useContext, useLayoutEffect, useMemo, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import { Icon } from './knobs.jsx';

const Slots = createContext(null);

const BOTTOM = ['status', 'hints', 'actions'];

const OPENERS = { left: ['Browse', 'paneLeft'], right: ['Knobs', 'knobs'] };

export function calm(box) {
  let by = '';
  const on = { pointerdown: () => (by = 'pointer'), keydown: () => (by = 'key'), change: (e) => by === 'pointer' && e.target.blur?.() };
  for (const [name, fn] of Object.entries(on)) box.addEventListener(name, fn);
  return () => {
    for (const [name, fn] of Object.entries(on)) box.removeEventListener(name, fn);
  };
}

export function slot(frame, side, doc = globalThis.document) {
  if (side === 'top') return doc?.querySelector('.subheader .actions') ?? null;
  if (frame) return frame.nodes[side] ?? null;
  return side === 'right' ? (doc?.querySelector('#right .controls') ?? null) : null;
}

export function Opener({ side, label, icon, few }) {
  return (
    <button type="button" className="icon" data-pane={side} aria-controls={side} aria-expanded="false" aria-label={label} hidden={few ? true : undefined} data-few={few}>
      <Icon name={icon} />
    </button>
  );
}

export function Frame({ label, gestures, children }) {
  const [held, setHeld] = useState({});
  const [nodes, setNodes] = useState({});
  const refs = useRef({});
  const ref = (side) => (refs.current[side] ??= (node) => setNodes((was) => (was[side] === node ? was : { ...was, [side]: node })));
  const claim = useCallback((side) => {
    setHeld((was) => ({ ...was, [side]: (was[side] ?? 0) + 1 }));
    return () => setHeld((was) => ({ ...was, [side]: was[side] - 1 }));
  }, []);
  const frame = useMemo(() => ({ claim, nodes }), [claim, nodes]);
  const has = (side) => held[side] > 0;
  const sides = Object.keys(OPENERS).filter(has);
  const bottom = sides.length > 0 || BOTTOM.some(has);
  return (
    <Slots.Provider value={frame}>
      <section className="frame" aria-label={label}>
        {has('left') && <aside className="pane left" id="left" aria-label={OPENERS.left[0]} ref={ref('left')} />}
        <div className="canvas" data-gestures={gestures ? '' : undefined}>{children}</div>
        {bottom && (
          <div className="bottom">
            {has('status') && <div className="status" ref={ref('status')} />}
            {has('hints') && <div className="hints" ref={ref('hints')} />}
            <div className="end">
              {has('actions') && <div className="actions" ref={ref('actions')} />}
              {sides.length > 0 && <span className="openers">{sides.map((side) => <Opener key={side} side={side} label={OPENERS[side][0]} icon={OPENERS[side][1]} />)}</span>}
            </div>
          </div>
        )}
        {has('right') && (
          <aside className="pane right" id="right" aria-label={OPENERS.right[0]} ref={calm}>
            <section className="controls" aria-label="Controls" ref={ref('right')} />
            <button type="button" className="done" data-pane="right" aria-controls="right">Done</button>
          </aside>
        )}
        {sides.length > 0 && <div className="scrim"></div>}
      </section>
    </Slots.Provider>
  );
}

export function Bar({ side, children }) {
  const frame = useContext(Slots);
  const claim = frame?.claim;
  useLayoutEffect(() => claim?.(side), [claim, side]);
  const at = typeof document === 'undefined' ? null : slot(frame, side);
  return at ? createPortal(children, at) : null;
}
