import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/app/export.js';
import { Bar } from '../../lib/app/frame.jsx';
import { Fold } from '../../lib/app/controls.jsx';
import { Knob, gist } from '../../lib/app/knobs.jsx';
import { Art, Item, Library, Shelf } from '../../lib/app/library.jsx';
import { rng } from '../../lib/rand.js';
import { Scene } from '../../lib/app/page.jsx';
import { paints } from '../../lib/color.js';
import { useTheme } from '../../lib/designs/pick.jsx';
import { TRACKS, fit, glance, resolve, roll } from './engine.js';
import { SPEC, file, make, parked, scrub, status } from './scene.js';
import { useReroll } from '../../lib/app/hooks.js';

export const FOLDS = [
  ['Track', ['kind', 'sides', 'ring', 'wheel', 'laps']],
  ['Draw', ['at', 'ink', 'fill']],
  ['Wheel', ['number', 'level', 'mode', 'reach', 'jitter']],
];

const SLOP = 6;

const LINE = 1;

const rowOf = (key) => SPEC.find((row) => row.key === key);

export const shown = (keys, value) => keys.map(rowOf).filter((row) => !row.need || row.need(value));

export const lit = (value, patch) => Object.keys(patch).filter((key) => rowOf(key).need?.(patch) ?? true).every((key) => value[key] === patch[key]);

export const sub = ({ kind, ring, wheel, sides }) => (kind === 'in' || kind === 'out' ? `R/r = ${ring}/${wheel}` : kind === 'line' ? 'on a line' : `${sides} sides`);

export function etch(ctx, { count, samples, trace, frame }, w, h, dpr) {
  const box = fit(frame, w, h, 4 * dpr);
  ctx.strokeStyle = paints(ctx.canvas).accent;
  ctx.lineWidth = LINE * dpr;
  ctx.lineJoin = 'round';
  ctx.beginPath();
  for (let k = 0; k < count; k++) {
    for (let i = 0; i < samples; i++) {
      const j = (k * samples + i) * 2;
      const x = box.ox + trace[j] * box.k;
      const y = box.oy - trace[j + 1] * box.k;
      if (i) ctx.lineTo(x, y);
      else ctx.moveTo(x, y);
    }
  }
  ctx.stroke();
}

export function Tracks({ value, math, onChange }) {
  const tick = useTheme();
  const glimpse = (patch) => (
    <Art
      paint={(ctx, w, h, dpr) => etch(ctx, glance(math, value, resolve(math, value), patch), w, h, dpr)}
      deps={[math, patch.kind, patch.ring, patch.wheel, patch.sides, patch.laps, value.base, value.code, value.number, value.mode, value.reach, value.jitter, value.seed, tick]}
    />
  );
  return (
    <Shelf name="Tracks" grid>
      {TRACKS.map(([name, patch]) => <Item key={name} label={name} sub={sub(patch)} on={lit(value, patch)} onPick={() => onChange(patch)} art={math ? glimpse(patch) : undefined} />)}
    </Shelf>
  );
}

export function Pane({ value, at, math, onChange }) {
  const values = { ...value, at };
  const units = math ? { math } : undefined;
  return (
    <Bar side="right">
      {FOLDS.map(([fold, keys]) => {
        const rows = shown(keys, value);
        return (
          <Fold key={fold} name={fold} live={gist(rows, values)}>
            {rows.map((row) => <Knob key={row.key} row={row} value={values[row.key]} values={values} units={units} onChange={onChange} />)}
          </Fold>
        );
      })}
    </Bar>
  );
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units }) {
  const unit = units?.math;
  const [stat, setStat] = useState(null);
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const change = useRef(onChange);
  change.current = onChange;
  const was = useRef(null);
  const touch = useRef(null);
  const tell = useCallback((next) => {
    if (was.current?.playing && !next.playing) {
      const at = parked(next.part);
      if (at !== now.current.at) change.current({ at });
    }
    was.current = next;
    setStat(next);
  }, []);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit, tell }), [unit, tell]);
  useReroll(value.seed, (seed) => unit && change.current({ code: roll(unit, now.current.base, rng(seed)) }));
  const ready = (handle) => {
    live.current = handle;
    const fix = (next) => {
      if (next?.step) handle.take();
      handle.fix?.(next);
    };
    onReady?.(handle && { ...handle, fix });
  };
  const toggle = () => live.current?.toggle();
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, file(now.current, units), 'svg');
  };
  const have = Boolean(stat);
  useEffect(() => onExport?.(have ? [['SVG', keep]] : []), [have]);
  const rows = status(stat);
  const told = JSON.stringify(rows);
  useEffect(() => onStatus?.(rows), [told]);
  const playing = Boolean(stat?.playing);
  const at = playing ? parked(stat.part) : value.at;
  const down = (e) => {
    if (e.button) return;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    touch.current = { x: e.clientX, at, moved: false };
  };
  const move = (e) => {
    const held = touch.current;
    if (!held || (!held.moved && Math.abs(e.clientX - held.x) < SLOP)) return;
    held.moved = true;
    const next = scrub(held.at, e.clientX - held.x, e.currentTarget.clientWidth);
    if (next !== now.current.at) change.current({ at: next });
  };
  const up = () => {
    const held = touch.current;
    touch.current = null;
    if (held && !held.moved) toggle();
  };
  const cancel = () => {
    touch.current = null;
  };
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={cancel} />}
      <Library>
        <Tracks value={value} math={unit} onChange={onChange} />
      </Library>
      <Pane value={value} at={at} math={unit} onChange={onChange} />
    </>
  );
}
