import { useCallback, useEffect, useRef, useState } from 'react';
import { Art, Item, Library, Shelf } from '../../lib/app/library.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { useLatest, useReroll, useStatus } from '../../lib/app/hooks.js';
import { paints } from '../../lib/color.js';
import { useTheme } from '../../lib/designs/pick.jsx';
import { float } from '../../lib/fractal/orbit.js';
import { pan, zoomAt, zoomOf } from '../../lib/fractal/view.js';
import { KINDS, PLACES, roll, thumb } from './places.js';
import { at, go, limits, make, seat, spot, where } from './scene.js';

const WHEEL = 1 / 400;
const LINE = 16;
const KEPT = 8;
const short = (v) => Number(v.toPrecision(7));
const three = (v) => +v.toFixed(3);
const complex = ([re, im]) => `${three(re)}${im < 0 ? '-' : '+'}${three(Math.abs(im))}i`;

/* VIEW */

export function point(box, e) {
  const side = Math.min(box.width, box.height) || 1;
  return [(e.clientX - box.left - box.width / 2) / side, (box.top + box.height / 2 - e.clientY) / side];
}

export function scale(v, p, k, [least, most]) {
  const span = Math.min(most, Math.max(least, v.span / k));
  return zoomAt(v, p, v.span / span);
}

export function pinch(from, [a, b], [c, d], edges) {
  const mid = (p, q) => [(p[0] + q[0]) / 2, (p[1] + q[1]) / 2];
  const gap = (p, q) => Math.hypot(p[0] - q[0], p[1] - q[1]) || 1e-9;
  const [m0, m1] = [mid(a, b), mid(c, d)];
  return pan(scale(from, m0, gap(c, d) / gap(a, b), edges), [m1[0] - m0[0], m1[1] - m0[1]]);
}

const mag = (zoom) => {
  const n = 2 ** zoom;
  return `×${n < 1000 ? +n.toPrecision(3) : n.toExponential(1).replace('e+', 'e')}`;
};

export const rows = (value, v) => [
  ['zoom', mag(zoomOf(v.span))],
  ['x', String(short(float(v.centre[0])))],
  ['y', String(short(float(v.centre[1])))],
  ...(value.kind === 'julia' ? [['c', complex([short(value.cx), short(value.cy)])]] : []),
];

/* PLACES */

function Places({ value, onChange }) {
  const tick = useTheme();
  const [find, setFind] = useState('');
  const want = find.trim().toLowerCase();
  return (
    <Library name="Places" find={find} onFind={setFind}>
      {KINDS.map(([kind, name]) => {
        const list = PLACES.filter((one) => one.kind === kind && one.name.toLowerCase().includes(want));
        if (!list.length) return null;
        return (
          <Shelf key={kind} name={name} grid>
            {list.map((one) => (
              <Item key={one.id} label={one.name} sub={one.c ? `c ${complex(one.c)}` : mag(one.zoom)} on={at(value, one)} onPick={() => onChange(go(one))} art={<Art deps={[one.id, value.tint, tick]} paint={(ctx, w, h) => thumb(ctx, one, w, h, paints(ctx.canvas, value.tint))} />} />
            ))}
          </Shelf>
        );
      })}
    </Library>
  );
}

/* WIDGET */

export function Widget({ value, onChange, onReady, onStatus }) {
  const live = useRef(null);
  const now = useLatest(value);
  const change = useLatest(onChange);
  const grab = useRef({ points: new Map(), from: null, start: null });
  const mine = useRef({ view: null, wrote: [] });
  const forget = () => (mine.current = { view: null, wrote: [] });
  const [flight, setFlight] = useState(null);
  if (mine.current.view && !mine.current.wrote.includes(where(value))) forget();
  useStatus(onStatus, rows(value, flight ?? seat(value)));
  useReroll(value.seed, (seed) => onChange(go(roll(now.current.kind, seed))));
  const rest = useCallback((place) => {
    live.current?.pause();
    setFlight(null);
    if (!place) return;
    mine.current = { view: null, wrote: [] };
    change.current(place);
  }, []);
  const tell = useCallback((v) => setFlight(v), []);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, onRest: rest, onTell: tell }), []);
  const base = () => mine.current.view ?? live.current?.at();
  const put = (v) => {
    const patch = spot(now.current.kind, v);
    const kept = where({ ...now.current, ...patch });
    mine.current = { view: v, wrote: [...mine.current.wrote, kept].slice(-KEPT) };
    change.current(patch);
  };
  const land = () => {
    const handle = live.current;
    if (!handle?.flying()) return;
    const place = handle.halt();
    handle.pause();
    setFlight(null);
    forget();
    change.current(place);
  };
  const zoom = (p, k) => {
    land();
    const v = base();
    if (v) put(scale(v, p, k, limits(now.current)));
  };
  const box = () => live.current?.canvas.getBoundingClientRect();
  const wheel = useLatest((e) => {
    e.preventDefault();
    zoom(point(box(), e), 2 ** (-e.deltaY * (e.deltaMode ? LINE : 1) * WHEEL));
  });
  const unwheel = useRef(() => {});
  useEffect(() => () => unwheel.current(), []);
  const ready = (handle) => {
    unwheel.current();
    live.current = handle;
    if (!handle) return onReady?.(null);
    const turn = (e) => wheel.current(e);
    handle.canvas.addEventListener('wheel', turn, { passive: false });
    unwheel.current = () => handle.canvas.removeEventListener('wheel', turn);
    const toggle = () => {
      if (handle.flying()) return land();
      handle.fly();
      handle.play();
    };
    const fix = (next) => {
      if (next?.step && !handle.flying()) handle.fly();
      if (next?.step) handle.play();
      handle.fix(next);
    };
    const julia = () => {
      const [cx, cy] = handle.at().centre.map((one) => Number(float(one).toFixed(9)));
      change.current({ kind: 'julia', cx, cy, ...go({ kind: 'julia', x: '0', y: '0', zoom: 0 }) });
    };
    onReady?.({ ...handle, toggle, fix, julia, playing: () => handle.flying(), zoom: (by) => zoom([0, 0], 2 ** by) });
  };
  const restart = () => {
    const g = grab.current;
    g.from = base();
    g.start = [...g.points.values()];
  };
  const down = (e) => {
    if (e.button || !live.current) return;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    land();
    grab.current.points.set(e.pointerId, point(box(), e));
    restart();
  };
  const move = (e) => {
    const g = grab.current;
    if (!g.points.has(e.pointerId) || !g.from) return;
    g.points.set(e.pointerId, point(box(), e));
    const [a, b] = g.start;
    const [c, d] = [...g.points.values()];
    put(b && d ? pinch(g.from, [a, b], [c, d], limits(now.current)) : pan(g.from, [c[0] - a[0], c[1] - a[1]]));
  };
  const up = (e) => {
    if (!grab.current.points.delete(e.pointerId)) return;
    restart();
  };
  const twice = (e) => zoom(point(box(), e), e.shiftKey ? 0.5 : 2);
  return (
    <>
      <Scene make={scene} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} onDoubleClick={twice} style={{ cursor: 'grab' }} />
      <Places value={value} onChange={onChange} />
    </>
  );
}
