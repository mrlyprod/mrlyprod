import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { rng } from '../../lib/scene.js';
import { roll } from '../designs/engine.js';
import { Pick, read } from '../designs/pick.jsx';
import { DIM } from './engine.js';
import { useReroll, useThree, useUnit } from './hooks.js';
import { make, name } from './scene.js';

const load = () => import('./unit.js');
const WHEEL = 0.0015;
const FRESH = 80;
const LOOKS = [['corner', 'Corner'], ['edge', 'Edge'], ['face', 'Face']];
const COMBINE = ['sum', 'xor'];

const warn = { position: 'absolute', top: 12, left: '50%', transform: 'translateX(-50%)', maxWidth: '80%', textAlign: 'center', fontSize: 13, opacity: 0.85, pointerEvents: 'none' };

export const hands = (at, onChange, math) => ({
  random: () => {
    const patch = { code: roll(math, DIM, read(at()).base) };
    if (at().source === 'field') patch.combine = COMBINE[Math.floor(Math.random() * COMBINE.length)];
    onChange(patch);
  },
  deeper: (by) => onChange({ level: at().level + by }),
  widen: (by) => onChange({ radius: at().radius + by }),
  toggle: () => onChange({ spin: Number(at().spin) === 1 ? 0 : 1 }),
  playing: () => Number(at().spin) === 1,
});

export function Widget({ value, onChange, onReady, onExport, onStatus, unit, three: given }) {
  const math = useUnit(unit, load);
  const three = useThree(given);
  const live = useRef(null);
  const now = useRef(value);
  const touch = useRef({ points: new Map(), mode: 'turn', last: [0, 0], at: 0 });
  const [info, setInfo] = useState(null);
  now.current = value;
  const tell = useCallback((next) => setInfo(next), []);
  useEffect(() => onStatus?.(info?.rows ?? []), [info]);
  const rolled = (seed) => {
    if (!math) return;
    const base = read(now.current).base;
    const rand = rng(seed);
    const patch = { code: roll(math, DIM, base, rand) };
    if (now.current.source === 'field') patch.combine = COMBINE[Math.floor(rand() * COMBINE.length)];
    onChange(patch);
  };
  useReroll(value.seed, rolled);
  const mode = useRef(`${value.source} ${Boolean(value.crop)}`);
  useEffect(() => {
    const next = `${value.source} ${Boolean(value.crop)}`;
    if (mode.current === next) return;
    const calm = value.source === 'field' || Boolean(value.crop);
    const was = mode.current;
    mode.current = next;
    if (calm && was === 'design false') onChange({ spin: 0 });
  }, [value.source, value.crop]);
  const wheel = useRef(null);
  const ready = (handle) => {
    live.current = handle;
    wheel.current?.();
    wheel.current = null;
    const canvas = handle?.canvas;
    if (canvas?.addEventListener) {
      const wheeled = (e) => {
        e.preventDefault();
        live.current?.zoom?.(Math.exp(-e.deltaY * WHEEL));
      };
      const menu = (e) => e.preventDefault();
      canvas.addEventListener('wheel', wheeled, { passive: false });
      canvas.addEventListener('contextmenu', menu);
      wheel.current = () => {
        canvas.removeEventListener('wheel', wheeled);
        canvas.removeEventListener('contextmenu', menu);
      };
    }
    onReady?.(handle && { ...handle, ...hands(() => now.current, onChange, math) });
  };
  useEffect(() => () => wheel.current?.(), []);
  const down = (e) => {
    const state = touch.current;
    state.points.set(e.pointerId, [e.clientX, e.clientY]);
    state.mode = e.button === 2 || e.shiftKey ? 'pan' : 'turn';
    state.last = [0, 0];
    e.currentTarget.setPointerCapture?.(e.pointerId);
  };
  const move = (e) => {
    const state = touch.current;
    const from = state.points.get(e.pointerId);
    if (!from) return;
    const rect = e.currentTarget.getBoundingClientRect();
    const scene = live.current;
    if (state.points.size === 2) {
      const gap = () => {
        const [a, b] = [...state.points.values()];
        return Math.hypot(a[0] - b[0], a[1] - b[1]);
      };
      const before = gap();
      state.points.set(e.pointerId, [e.clientX, e.clientY]);
      if (before > 0) scene?.zoom?.(gap() / before);
      scene?.pan?.((e.clientX - from[0]) / rect.width / 2, (e.clientY - from[1]) / rect.height / 2);
      return;
    }
    const dx = (e.clientX - from[0]) / rect.width;
    const dy = (e.clientY - from[1]) / rect.height;
    state.points.set(e.pointerId, [e.clientX, e.clientY]);
    if (state.mode === 'pan') scene?.pan?.(dx, dy);
    else scene?.turn?.(dx, dy);
    state.last = [dx, dy];
    state.at = e.timeStamp;
  };
  const up = (e) => {
    const state = touch.current;
    const had = state.points.size;
    state.points.delete(e.pointerId);
    if (had === 1 && state.mode === 'turn' && e.timeStamp - state.at < FRESH) live.current?.fling?.(...state.last);
  };
  const keep = (body, kind) => body && text(body, name(now.current), kind);
  const obj = () => keep(live.current?.obj?.(), 'obj');
  useEffect(() => onExport?.(math ? [['OBJ', obj]] : []), [Boolean(math)]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, three, tell }), [math, three]);
  const pick = ({ base, code }) => onChange({ base, code });
  return (
    <>
      {math && three && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />}
      {info?.error && <p style={warn}>{info.error}</p>}
      <Bar side="right">
        <Group name="Design">{math ? <Pick value={{ ...read(value), dim: DIM }} onChange={pick} math={math} dims={[DIM]} /> : <span>loading</span>}</Group>
        <Group name="Look from">
          {LOOKS.map(([key, label]) => (
            <Btn key={key} onClick={() => live.current?.look?.(key)}>
              {label}
            </Btn>
          ))}
        </Group>
      </Bar>
    </>
  );
}
