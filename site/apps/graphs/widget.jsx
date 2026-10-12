import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Fold, Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { rng } from '../../lib/scene.js';
import { Picker } from '../designs/pick.jsx';
import { deal, design, keysOf, solid as isSolid, spaceOf, turn } from './engine.js';
import { make, name } from './scene.js';

const POLL = 250;
const TAP = 6;
const WHEEL = 0.0015;
const POSES = [['corner', 'Corner'], ['edge', 'Edge'], ['face', 'Face']];
const CAMERA = { eye: 'perspective', iso: 'isometric' };

const sci = (x) => Number(x).toExponential(2);

export const note = (f, camera = 'eye') => `${f.graph} graph, level ${f.level}, ${f.layout}${f.space === 'cube' ? `, ${CAMERA[camera] ?? CAMERA.eye}` : ''}`;

export function rows(f, r, camera) {
  const out = [
    ['Design', `${f.name || 'bang'} ${f.code}`],
    ['Network', note(f, camera)],
    ['Nodes', f.nodes],
    ['Branches', f.branches],
    ['Tips', f.tips],
    ['Junctions', f.junctions],
    ['Pieces', f.pieces],
    ['Length', f.length.toFixed(2)],
    ['Box dimension', f.box.toFixed(3)],
    ['Euler', f.euler ?? 'none'],
  ];
  if (!r) return out;
  return [...out, ['Ticks', r.ticks], ...(r.ticks ? [['Energy', sci(r.energy)], ['Moved', sci(r.moved)]] : []), ['Heat', sci(r.heat)]];
}

export function useThree(wanted, given) {
  const [three, setThree] = useState(null);
  useEffect(() => {
    if (given || !wanted || three) return undefined;
    let on = true;
    import('three').then((mod) => on && setThree(mod));
    return () => {
      on = false;
    };
  }, [given, wanted, three]);
  return given ?? three;
}

export function Pane({ value, onChange, math, relax, onRelax, onReset, onPose }) {
  const keys = keysOf(spaceOf(value));
  const { dim, base, code } = design(value);
  const pick = (next) => onChange({ [keys.base]: next.base, [keys.code]: next.code });
  const running = Boolean(relax?.on);
  return (
    <>
      <Fold name="Design" live={code}>
        {math ? <Picker value={{ dim, base, code }} onChange={pick} math={math} dims={[dim]} /> : <span>loading</span>}
      </Fold>
      {value.layout === 'force' && (
        <Group name="Relax" after="Layout">
          <Btn on={running} onClick={onRelax}>{running ? 'Pause' : 'Relax'}</Btn>
          <Btn onClick={onReset}>Reset</Btn>
        </Group>
      )}
      {isSolid(value) && (
        <Group name="Pose" after="Camera">
          {POSES.map(([pose, label]) => (
            <Btn key={pose} onClick={() => onPose(pose)}>
              {label}
            </Btn>
          ))}
        </Group>
      )}
    </>
  );
}

const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);

export function Widget({ value, onChange, onReady, onExport, onStatus, units, three: given }) {
  const math = units?.math;
  const cube = isSolid(value);
  const three = useThree(cube, given);
  const live = useRef(null);
  const now = useRef(value);
  const said = useRef(value.seed);
  const touch = useRef({ points: new Map(), from: null });
  const wheel = useRef(null);
  const tell = useRef(onChange);
  const [seen, setSeen] = useState(null);
  now.current = value;
  tell.current = onChange;
  const sync = () => {
    const handle = live.current;
    const next = handle ? { facts: handle.facts(), relax: handle.relaxed() } : null;
    setSeen((was) => (same(was, next) ? was : next));
  };
  const relax = () => {
    if (now.current.layout !== 'force') return tell.current({ layout: 'force' });
    live.current?.toggle?.();
    sync();
  };
  const reset = () => {
    live.current?.reset?.();
    sync();
  };
  const ready = (handle) => {
    if (wheel.current) wheel.current();
    wheel.current = null;
    live.current = handle;
    sync();
    if (!handle) return onReady?.(null);
    const spin = (e) => {
      if (!live.current?.zoom) return;
      e.preventDefault();
      live.current.zoom(Math.exp(-e.deltaY * WHEEL));
    };
    handle.canvas?.addEventListener?.('wheel', spin, { passive: false });
    wheel.current = () => handle.canvas?.removeEventListener?.('wheel', spin);
    onReady?.({
      ...handle,
      toggle: relax,
      deeper: (by) => tell.current({ level: now.current.level + by }),
      next: (by) => tell.current({ graph: turn(now.current, by) }),
    });
  };
  useEffect(() => () => wheel.current?.(), []);
  useEffect(() => {
    if (!math || said.current === value.seed) return;
    said.current = value.seed;
    onChange(deal(math, value, rng(value.seed)));
  }, [value.seed, math]);
  const have = Boolean(seen);
  useEffect(() => {
    if (!have) return undefined;
    const timer = setInterval(sync, POLL);
    return () => clearInterval(timer);
  }, [have]);
  useEffect(() => onStatus?.(seen ? rows(seen.facts, seen.relax, value.camera) : []), [seen, value.camera]);
  useEffect(() => {
    sync();
    const facts = live.current?.facts?.();
    if (facts && facts.space === spaceOf(value) && facts.code !== design(value).code) onChange({ [keysOf(facts.space).code]: facts.code });
  }, [JSON.stringify(value), have]);
  const keep = (body, kind) => body && text(body, name(now.current), kind);
  const svg = () => keep(live.current?.svg?.(), 'svg');
  const json = () => keep(live.current?.json?.(), 'json');
  useEffect(() => onExport?.(have ? [['SVG', svg], ['JSON', json]] : []), [have]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, three }), [math, three]);
  const down = (e) => {
    const { points } = touch.current;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    points.set(e.pointerId, [e.clientX, e.clientY]);
    if (points.size === 1) {
      touch.current.from = { x: e.clientX, y: e.clientY, far: 0, tap: e.button === 0, pan: e.button === 2 || e.shiftKey };
      live.current?.grab?.();
    } else if (touch.current.from) touch.current.from.tap = false;
  };
  const move = (e) => {
    const { points, from } = touch.current;
    const was = points.get(e.pointerId);
    if (!was || !from) return;
    const box = e.currentTarget.getBoundingClientRect();
    const at = [e.clientX, e.clientY];
    if (points.size === 2) {
      const [a, b] = [...points.values()];
      points.set(e.pointerId, at);
      const [c, d] = [...points.values()];
      const span = Math.hypot(a[0] - b[0], a[1] - b[1]) || 1;
      live.current?.zoom?.(Math.hypot(c[0] - d[0], c[1] - d[1]) / span);
      live.current?.pan?.((c[0] + d[0] - a[0] - b[0]) / 2 / box.width, (c[1] + d[1] - a[1] - b[1]) / 2 / box.height);
      return;
    }
    points.set(e.pointerId, at);
    from.far = Math.max(from.far, Math.hypot(at[0] - from.x, at[1] - from.y));
    const dx = (at[0] - was[0]) / box.width;
    const dy = (at[1] - was[1]) / box.height;
    if (from.pan) live.current?.pan?.(dx, dy);
    else live.current?.turn?.(dx, dy);
  };
  const up = (e) => {
    const { points, from } = touch.current;
    if (!points.delete(e.pointerId) || points.size) return;
    live.current?.release?.();
    touch.current.from = null;
    if (!cube && e.type === 'pointerup' && from?.tap && from.far < TAP) relax();
  };
  const hush = (e) => cube && e.preventDefault();
  return (
    <>
      {math && (!cube || three) && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} onContextMenu={hush} />}
      <Bar side="right">
        <Pane value={value} onChange={onChange} math={math} relax={seen?.relax} onRelax={relax} onReset={reset} onPose={(pose) => live.current?.snap?.(pose)} />
      </Bar>
    </>
  );
}
