import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { text } from '../../lib/app/export.js';
import { Bar } from '../../lib/app/frame.jsx';
import { Group } from '../../lib/app/controls.jsx';
import { Library } from '../../lib/app/library.jsx';
import { share } from '../../lib/app/query.js';
import { Scene } from '../../lib/app/page.jsx';
import { Browser } from '../../lib/designs/browse.jsx';
import { cap, drawn, gallery, next } from '../../lib/designs/engine.js';
import { choose, place, read } from '../../lib/designs/store.js';
import { PAGE, make, named } from './scene.js';
import { useReroll, useThree, useUnit } from '../../lib/app/hooks.js';

const TAP = 6;

const SLOT = PAGE.design;

export const steps = (math, dim, base) => gallery(math, dim, base, 'classes') ?? gallery(math, dim, base, 'named');

export function rows(info) {
  const out = [['Design', info.name ? `${info.name} ${info.code}` : info.code]];
  if (info.rep !== info.code) out.push(['Class', info.rep]);
  out.push(['Orbit', String(info.orbit)]);
  if (info.degree >= 0) out.push(['Degree', String(info.degree)]);
  if (info.anf) out.push(['Normal form', info.anf]);
  out.push(['Filled', `${info.fills} of ${info.grid}`], ['Ratio', info.ratio.toFixed(4)], ['Dimension', info.dimension.toFixed(4)]);
  out.push(info.dim === 2 ? ['Perimeter', String(info.perimeter)] : ['Surface', String(info.surface)]);
  out.push(['Euler', String(info.euler)], ['Codes', info.codes.toLocaleString()], ['Classes', info.classes.toLocaleString()], [`Distinct, dim 1 to ${info.upto}`, info.terms.join(', ')]);
  return out;
}

export function Widget({ value, onChange, onReady, onExport, onStatus, unit, three: given }) {
  const math = useUnit('math', unit);
  const { dim, base, code } = read(SLOT, value);
  const three = useThree(dim === 3, given);
  const live = useRef(null);
  const press = useRef(null);
  const change = useRef(onChange);
  change.current = onChange;
  const [info, setInfo] = useState(null);
  const tell = useCallback((next) => setInfo(next), []);
  const here = info?.dim === dim && info?.base === base ? info : null;
  const list = useMemo(() => (math ? (steps(math, dim, base) ?? []) : []), [math, dim, base]);
  const at = here ? (list.includes(here.code) ? here.code : here.rep) : code;
  const now = useRef(null);
  now.current = { list, at, code: here?.code ?? code, level: value.level, dim, base };
  useEffect(() => onStatus?.(here ? rows(here) : []), [here]);
  useReroll(value.seed, (seed) => math && onChange({ code: drawn(math, dim, base, gallery(math, dim, base, 'classes'), now.current.code, seed) }));
  const hands = {
    grow: () => change.current({ level: now.current.level >= cap(math, now.current.dim) ? 1 : now.current.level + 1 }),
    step: (by) => change.current({ code: next(now.current.list, now.current.at, by) }),
    deeper: (by) => change.current({ level: now.current.level + by }),
  };
  const staged = Boolean(math && (dim !== 3 || three));
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, three, tell }), [math, three, tell]);
  const ready = (handle) => {
    live.current = handle;
    onReady?.(handle && { ...handle, ...hands });
  };
  const down = (e) => {
    if (e.button) return;
    press.current = { x: e.clientX, y: e.clientY, far: 0 };
    e.currentTarget.setPointerCapture?.(e.pointerId);
  };
  const move = (e) => {
    const from = press.current;
    if (!from) return;
    const box = e.currentTarget.getBoundingClientRect();
    const dx = e.clientX - from.x;
    const dy = e.clientY - from.y;
    press.current = { x: e.clientX, y: e.clientY, far: from.far + Math.hypot(dx, dy) };
    live.current?.turn?.(dx / box.width, dy / box.height);
  };
  const up = (e) => {
    const from = press.current;
    press.current = null;
    if (from && e.type === 'pointerup' && from.far < TAP) hands.step(1);
  };
  const keep = (body, kind) => body && text(body, named(now.current), kind);
  useEffect(() => onExport?.([['SVG', () => keep(live.current?.svg?.(), 'svg')], ...(dim === 3 ? [['OBJ', () => keep(live.current?.obj?.(), 'obj')]] : [])]), [dim]);
  const pick = (one) => {
    onChange(place(SLOT, one));
    choose(one);
  };
  return (
    <>
      {staged && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />}
      <Library name="Gallery">
        <Browser compact slot={SLOT} value={value} math={math} onPick={pick} />
      </Library>
      {dim === 3 && here && (
        <Bar side="right">
          <Group after="Look">
            <a href={`/three/${share({ base, code: here.code, level: here.level })}`}>Open in Three</a>
          </Group>
        </Bar>
      )}
    </>
  );
}
