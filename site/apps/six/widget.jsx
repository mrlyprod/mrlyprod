import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { Pick, read } from '../designs/pick.jsx';
import { DIM, PRESETS, VIEWS, reroll } from './engine.js';
import { make, name } from './scene.js';

const ORDER = VIEWS.map(([key]) => key);

const by = ([w, h]) => `${w} x ${h}`;

export function rows(f) {
  if (!f) return [];
  return [
    ['design', f.name ? `${f.name} ${f.code}` : f.title],
    ['side', f.side],
    ['level', `${f.level} of ${f.top}`],
    ['copies', f.copies],
    ['filled', f.fills],
    ['exposed', f.exposed],
    ['one copy exposed', f.alone],
    ['tile', by(f.tile)],
    ['sheet', by(f.sheet)],
    ['triangles', f.triangles],
    ['empty', f.voids],
    ['pieces', f.pieces],
    ['holes', f.holes],
    ['corners', f.corners],
    ['edges', f.edges],
    ['euler', f.euler],
  ].map(([name, v]) => [name, String(v)]);
}

export const label = (f) => `${f ? f.name || f.title : 'a cube design'} repeated into a sheet`;

export function Presets({ value, onChange }) {
  return (
    <Group name="Presets" after="Sheet">
      {PRESETS.map(([x, y]) => (
        <Btn key={`${x}.${y}`} on={value.sheet !== 'disc' && value.x === x && value.y === y} onClick={() => onChange({ sheet: 'rect', x, y })}>
          tile({x},{y})
        </Btn>
      ))}
    </Group>
  );
}

export function Pane({ value, onChange, unit }) {
  return (
    <>
      <Group name="Design">{unit ? <Pick value={{ ...read(value), dim: DIM }} onChange={({ base, code }) => onChange({ base, code })} math={unit} dims={[DIM]} /> : <span>loading</span>}</Group>
      <Presets value={value} onChange={onChange} />
    </>
  );
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units }) {
  const unit = units?.math;
  const [facts, setFacts] = useState(null);
  const live = useRef(null);
  const now = useRef(value);
  const change = useRef(onChange);
  const seed = useRef(value.seed);
  now.current = value;
  change.current = onChange;
  useEffect(() => {
    if (seed.current === value.seed) return;
    seed.current = value.seed;
    if (unit) change.current({ code: reroll(unit, read(now.current).base, value.seed) });
  }, [value.seed]);
  useEffect(() => onStatus?.(rows(facts)), [facts]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit, onFacts: setFacts }), [unit]);
  const hands = {
    crop: () => change.current({ overhang: now.current.overhang === 1 ? 0 : 1 }),
    view: (step) => change.current({ view: ORDER[(ORDER.indexOf(now.current.view) + step + ORDER.length) % ORDER.length] }),
    deeper: (step) => change.current({ level: now.current.level + step }),
  };
  const ready = (handle) => {
    live.current = handle;
    onReady?.(handle && { ...handle, ...hands });
  };
  const keep = (kind, tail) => () => {
    const picture = live.current?.[kind]?.();
    if (picture) text(picture, `${name(now.current, units)}${tail}`, 'svg');
  };
  const have = Boolean(unit);
  useEffect(() => onExport?.(have ? [['SVG', keep('svg', '')], ['SVG tile', keep('tile', '-tile')]] : []), [have]);
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} role="img" aria-label={label(facts)} onClick={() => hands.view(1)} />}
      <Bar side="right">
        <Pane value={value} onChange={onChange} unit={unit} />
      </Bar>
    </>
  );
}
