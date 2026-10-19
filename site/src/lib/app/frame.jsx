import { createContext, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import { handle } from '../handle.js';
import { held, hold } from '../keep.js';
import { Icon } from './controls.jsx';

const Slots = createContext(null);

export const SIDES = {
  left: { name: 'Library', key: '[', keep: 'library', icons: ['left_panel_open', 'left_panel_close'] },
  right: { name: 'Knobs', key: ']', keep: 'knobs', icons: ['right_panel_open', 'right_panel_close'] },
};

export const WIDTH = 280;
const LEAST = 220;
const MOST = 440;
const GAP = 12;
const ROOM = 360;

export function calm(box) {
  let by = '';
  const on = { pointerdown: () => (by = 'pointer'), keydown: () => (by = 'key'), change: (e) => by === 'pointer' && e.target.blur?.() };
  for (const [name, fn] of Object.entries(on)) box.addEventListener(name, fn);
  return () => {
    for (const [name, fn] of Object.entries(on)) box.removeEventListener(name, fn);
  };
}

export function slot(frame, side, doc = globalThis.document) {
  if (frame) return frame.nodes[side] ?? null;
  return side === 'right' || side === 'tail' ? (doc?.querySelector('#right .controls') ?? null) : null;
}

/* PANES */

export const pushed = (open, widths, gap = GAP) => Math.max(0, ...Object.keys(SIDES).map((side) => (open[side] ? widths[side] + 2 * gap : 0)));

export const floats = (width, push, room = ROOM) => width - 2 * push < room;

const phone = () => typeof document !== 'undefined' && document.documentElement?.dataset?.form === 'phone';

export function usePanes() {
  const [claims, setClaims] = useState({});
  const [nodes, setNodes] = useState({});
  const refs = useRef({});
  const [open, setOpen] = useState(() => (phone() ? { left: false, right: false } : { left: true, right: true, ...held('panes', {}) }));
  const [widths, setWidths] = useState(() => ({ left: held(SIDES.left.keep, WIDTH), right: held(SIDES.right.keep, WIDTH) }));
  const claim = useCallback((side) => {
    setClaims((was) => ({ ...was, [side]: (was[side] ?? 0) + 1 }));
    return () => setClaims((was) => ({ ...was, [side]: was[side] - 1 }));
  }, []);
  const ref = (side) => (refs.current[side] ??= (node) => setNodes((was) => (was[side] === node ? was : { ...was, [side]: node })));
  const has = (side) => claims[side] > 0 || (side === 'right' && claims.tail > 0);
  const flip = useCallback((side, on) => setOpen((was) => {
    const next = { ...was, [side]: on ?? !was[side] };
    if (!phone()) hold('panes', next);
    return next;
  }), []);
  const size = useCallback((side, px, keep) => {
    setWidths((was) => (was[side] === px ? was : { ...was, [side]: px }));
    if (keep) hold(SIDES[side].keep, px === WIDTH ? null : Math.round(px));
  }, []);
  return { claims, nodes, ref, claim, has, open, flip, widths, size };
}

function Grip({ side, panes }) {
  const at = useRef(null);
  const latest = useRef(panes);
  latest.current = panes;
  useEffect(() => handle(at.current, {
    get: () => latest.current.widths[side],
    set: (px) => latest.current.size(side, px, false),
    done: (px) => latest.current.size(side, px, true),
    reset: WIDTH,
    min: LEAST,
    max: MOST,
    rate: side === 'left' ? 1 : -1,
    label: `${SIDES[side].name} width`,
    busy: (on) => at.current?.closest('.frame')?.toggleAttribute('data-drag', on),
  }), [side]);
  return <div className="grip" ref={at} />;
}

export function Opener({ side, panes }) {
  const { name, key, icons } = SIDES[side];
  const on = panes.open[side];
  return (
    <button type="button" className="icon side" aria-controls={side} aria-expanded={String(on)} aria-label={name} title={`${name} (${key})`} onClick={() => panes.flip(side)}>
      <Icon name={icons[on ? 1 : 0]} />
    </button>
  );
}

export function Frame({ label, gestures, panes: given, children }) {
  const mine = usePanes();
  const panes = given ?? mine;
  const { claim, nodes, ref, has, open, flip, widths } = panes;
  const box = useRef(null);
  const [wide, setWide] = useState(Infinity);
  useLayoutEffect(() => {
    const el = box.current;
    if (!el || typeof ResizeObserver !== 'function') return undefined;
    const eye = new ResizeObserver(([one]) => setWide(one.contentRect.width));
    eye.observe(el);
    return () => eye.disconnect();
  }, []);
  const latest = useRef(has);
  latest.current = has;
  useEffect(() => {
    const press = (e) => {
      if (!(e.metaKey || e.ctrlKey) || e.altKey || e.shiftKey) return;
      const side = Object.keys(SIDES).find((one) => SIDES[one].key === e.key);
      if (!side || !latest.current(side)) return;
      e.preventDefault();
      flip(side);
    };
    addEventListener('keydown', press);
    return () => removeEventListener('keydown', press);
  }, [flip]);
  const frame = useMemo(() => ({ claim, nodes }), [claim, nodes]);
  const shown = Object.fromEntries(Object.keys(SIDES).map((side) => [side, has(side) && open[side]]));
  const float = phone() || floats(wide, pushed(shown, widths));
  const sides = Object.keys(SIDES).filter(has);
  const style = { '--left': `${widths.left}px`, '--right': `${widths.right}px` };
  const status = has('status') || has('hints');
  return (
    <Slots.Provider value={frame}>
      <section className="frame" aria-label={label} ref={box} style={style} data-left={has('left') ? (open.left ? 'open' : 'shut') : undefined} data-right={has('right') ? (open.right ? 'open' : 'shut') : undefined} data-float={float ? '' : undefined}>
        <div className="canvas" data-gestures={gestures ? '' : undefined}>{children}</div>
        {has('left') && (
          <aside className="pane left" id="left" aria-label={SIDES.left.name} data-lens="panel" inert={!open.left ? true : undefined}>
            <div className="body" ref={ref('left')} />
            <Grip side="left" panes={panes} />
          </aside>
        )}
        {has('right') && (
          <aside className="pane right" id="right" aria-label={SIDES.right.name} data-lens="panel" inert={!open.right ? true : undefined}>
            <div className="body" ref={calm}>
              <section className="controls" aria-label="Controls" ref={ref('right')} />
              <div className="controls tail" ref={ref('tail')} />
            </div>
            <Grip side="right" panes={panes} />
          </aside>
        )}
        {(has('actions') || (float && sides.length > 0)) && (
          <div className="actions" data-lens="pill">
            {float && sides.includes('left') && <Opener side="left" panes={panes} />}
            <div className="buttons" ref={ref('actions')} />
            {float && sides.includes('right') && <Opener side="right" panes={panes} />}
          </div>
        )}
        {status && (
          <div className="status" data-lens="pill">
            <div className="facts" ref={ref('status')} />
            <div className="hints" ref={ref('hints')} />
          </div>
        )}
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
