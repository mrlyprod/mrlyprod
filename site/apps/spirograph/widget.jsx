import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Fold, Knob, gist } from '../../lib/knobs.jsx';
import { rng } from '../../lib/scene.js';
import { Scene } from '../../lib/scene.jsx';
import { Pick, read } from '../designs/pick.jsx';
import { rebase, roll } from './engine.js';
import { SPEC, file, make, parked, scrub, status } from './scene.js';

export const FOLDS = [
  ['Track', ['kind', 'sides', 'ring', 'wheel', 'laps']],
  ['Draw', ['at', 'ink', 'fill']],
  ['Wheel', ['number', 'level', 'mode', 'reach', 'jitter']],
];

const SLOP = 6;

const SEEDS = 4294967296;

const rowOf = (key) => SPEC.find((row) => row.key === key);

export const shown = (keys, value) => keys.map(rowOf).filter((row) => !row.need || row.need(value));

export const called = (value, facts) => (facts?.name && facts.code === value.code ? `${facts.name} ${value.code}` : `code ${value.code}`);

export function Pane({ value, at, math, facts, onChange, onDesign }) {
  const values = { ...value, at };
  const units = math ? { math } : undefined;
  return (
    <Bar side="right">
      {FOLDS.map(([fold, keys]) => {
        const rows = shown(keys, value);
        const wheel = fold === 'Wheel';
        const live = wheel ? `${called(value, facts)} · ${gist(rows, values)}` : gist(rows, values);
        return (
          <Fold key={fold} name={fold} live={live}>
            {wheel && math && <Pick value={{ ...read(value), dim: 2 }} onChange={onDesign} math={math} dims={[2]} />}
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
  const reroll = () => {
    const seed = Math.floor(Math.random() * SEEDS);
    change.current({ seed, code: roll(unit, now.current.base, rng(seed)) });
  };
  const ready = (handle) => {
    live.current = handle;
    const fix = (next) => {
      if (next?.step) handle.take();
      handle.fix?.(next);
    };
    onReady?.(handle && { ...handle, reroll, fix });
  };
  const toggle = () => live.current?.toggle();
  const design = ({ dim, ...patch }) => {
    const moved = unit && patch.base !== undefined && patch.base !== value.base;
    onChange(moved ? { ...patch, code: rebase(unit, patch.code ?? value.code, value.base, patch.base, value.number, rng(value.seed)) } : patch);
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, file(now.current), 'svg');
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
      <Pane value={value} at={at} math={unit} facts={stat?.facts} onChange={onChange} onDesign={design} />
    </>
  );
}
