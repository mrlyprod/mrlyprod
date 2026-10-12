import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group, Segment } from '../../lib/knobs.jsx';
import { share } from '../../lib/query.js';
import { Scene } from '../../lib/scene.jsx';
import { cap, codes, distinct, drawn, facts, gallery, home, next, total } from './engine.js';
import { Thumbs, read, useUnit } from './pick.jsx';
import { make, named } from './scene.js';
import { hue } from './thumb.js';

export const FILTERS = [['classes', 'Classes'], ['all', 'All'], ['named', 'Named']];

const TAP = 6;

export function filters(math, dim, base, show) {
  const options = math && codes(math, dim, base) ? FILTERS : FILTERS.slice(2);
  return { options, shown: options.some(([one]) => one === show) ? show : options[0][0] };
}

export function rows(info) {
  const out = [['Design', info.name ? `${info.name} ${info.code}` : info.code]];
  if (info.rep !== info.code) out.push(['Class', info.rep]);
  out.push(['Orbit', String(info.orbit)]);
  if (info.degree >= 0) out.push(['Degree', String(info.degree)]);
  if (info.anf) out.push(['Normal form', info.anf]);
  out.push(['Filled', `${info.fills} of ${info.grid}`], ['Ratio', info.ratio.toFixed(4)], ['Dimension', info.dimension.toFixed(4)]);
  out.push(info.dim === 2 ? ['Perimeter', String(info.perimeter)] : ['Surface', String(info.surface)]);
  out.push(['Euler', String(info.euler)], [`Distinct, dim 1 to ${info.upto}`, info.terms.join(', ')]);
  return out;
}

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

export function Widget({ value, onChange, onReady, onExport, onStatus, unit, three: given }) {
  const math = useUnit(unit);
  const { dim, base, code } = read(value);
  const three = useThree(dim, given);
  const live = useRef(null);
  const press = useRef(null);
  const change = useRef(onChange);
  change.current = onChange;
  const [info, setInfo] = useState(null);
  const tell = useCallback((next) => setInfo(next), []);
  const here = info?.dim === dim && info?.base === base ? info : null;
  const { options, shown } = filters(math, dim, base, value.show);
  const list = useMemo(() => (math ? (gallery(math, dim, base, shown) ?? []) : []), [math, dim, base, shown]);
  const forms = useMemo(() => (math && base === 2 ? new Map(list.map((one) => [one, facts(math, dim, base, one)])) : null), [math, dim, base, list]);
  const at = here ? (list.includes(here.code) ? here.code : here.rep) : code;
  const now = useRef(null);
  now.current = { list, at, code: here?.code ?? code, level: value.level, seed: value.seed, dim, base };
  useEffect(() => onStatus?.(here ? rows(here) : []), [here]);
  const space = `${dim}:${base}`;
  const was = useRef(space);
  useEffect(() => {
    if (!math) return;
    const moved = was.current !== space;
    was.current = space;
    if (moved || String(value.code ?? '').trim() === '') change.current({ code: home(math, dim, base) });
  }, [math, space]);
  const seed = useRef(value.seed);
  useEffect(() => {
    if (seed.current === value.seed) return;
    seed.current = value.seed;
    if (math) change.current({ code: drawn(math, dim, base, gallery(math, dim, base, 'classes'), now.current.code, value.seed) });
  }, [value.seed]);
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
  const tone = (one) => hue(value.tint, forms?.get(one)?.degree ?? -1);
  const note = forms ? (one) => forms.get(one)?.anf ?? '' : undefined;
  const art = Object.fromEntries(Object.entries(value).filter(([key]) => key !== 'show'));
  const heading = math ? `${total(math, dim, base).toLocaleString()} codes, ${distinct(math, dim, base).toLocaleString()} classes` : 'loading';
  return (
    <>
      {staged && <Scene make={scene} value={art} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />}
      <Bar side="left">
        <div className="controls">
          <Group name={heading}>
            <Segment label="Show" value={shown} options={options} onChange={(v) => onChange({ show: v })} />
            {math && <Thumbs math={math} dim={dim} base={base} codes={list} code={at} onPick={(v) => onChange({ code: v })} tone={tone} note={note} />}
            {dim === 3 && here && <a href={`/three/${share({ base, code: here.code, level: here.level })}`}>Open in Three</a>}
          </Group>
        </div>
      </Bar>
    </>
  );
}
