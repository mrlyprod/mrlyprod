import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Group, Pick } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { sources, spell } from './engine.js';
import { BOARD_KEYS, make } from './scene.js';

const HAND = ['', 'By hand'];

const SIDES = [['born', 'Born', 0], ['stay', 'Stay', 1]];

const at = (e) => {
  const box = e.currentTarget.getBoundingClientRect();
  return [(e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height];
};

const ROWS = [
  ['Generation', (s) => s.gen],
  ['Population', (s) => s.pop],
  ['Fate', (s) => s.fate || '-'],
  ['Rule', (s) => s.name || '-'],
];

const Fact = ({ name, value }) => (
  <label>
    <span>{name}</span>
    <span className="num" title={String(value)}>{value}</span>
  </label>
);

const clean = (text) => String(text ?? '').trim().toLowerCase();

export function Widget({ value, onChange, onReady }) {
  const [units, setUnits] = useState(null);
  const live = useRef(value);
  live.current = value;
  const handle = useRef(null);
  const painting = useRef(false);
  const [stat, setStat] = useState(null);
  useEffect(() => {
    let on = true;
    import('./unit.js').then(({ life, math, ready }) => ready.then(() => on && setUnits({ life, math })));
    return () => {
      on = false;
    };
  }, []);
  const names = useMemo(() => (units ? sources(units.life) : []), [units]);
  const options = useMemo(() => [HAND, ...names.map((name) => [name, name.replaceAll('_', ' ')])], [names]);
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units, live, onStat: setStat }), [units]);
  const ready = (scene) => {
    handle.current = scene;
    onReady?.(scene);
  };
  const down = (e) => {
    if (e.button) return;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    painting.current = true;
    handle.current?.touch(...at(e), true);
  };
  const move = (e) => {
    if (painting.current) handle.current?.touch(...at(e), false);
  };
  const up = () => {
    painting.current = false;
  };
  const chosen = (text) => (names.includes(clean(text)) ? clean(text) : '');
  const draw = (key, i) => (name) => onChange({ [key]: name || spell(stat?.counts?.[i] ?? []) });
  const board = Object.fromEntries(BOARD_KEYS.map((key) => [key, value[key]]));
  return (
    <>
      {units && <Scene make={made} value={board} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />}
      <Bar side="right">
        <Group name="Counts" className="facts">
          {ROWS.map(([name, get]) => <Fact key={name} name={name} value={stat ? get(stat) : '-'} />)}
        </Group>
      </Bar>
      <Bar side="actions">
        <Btn primary onClick={() => handle.current?.step()}>Step</Btn>
        <Btn onClick={() => handle.current?.clear()}>Clear</Btn>
      </Bar>
      {units && value.mode !== 'wolfram' && (
        <Bar side="right">
          <Group name="Sequences">
            {SIDES.map(([key, label, i]) => <Pick key={key} label={label} value={chosen(value[key])} options={options} onChange={draw(key, i)} />)}
          </Group>
        </Bar>
      )}
    </>
  );
}
