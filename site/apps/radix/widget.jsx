import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Group, Icon, Pick, Text } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { classes, named, word } from '../designs/engine.js';
import { Thumbs } from '../designs/pick.jsx';
import { BASE, DIM, roll, steps } from './engine.js';
import { make } from './scene.js';

const ROWS = [
  ['Ring', (f) => f.ring],
  ['Base', (f) => f.base],
  ['Norm', (f) => f.norm],
  ['Digits', (f) => f.digits],
  ['Code', (f) => f.code],
  ['Canonical', (f) => (f.canonical ? 'yes' : 'no')],
  ['Dimension', (f) => f.dimension.toFixed(3)],
];

const LIVE = [
  ['Level', (f) => f.level],
  ['Fill', (f) => f.fill],
  ['Distinct', (f) => f.distinct],
];

const Fact = ({ name, value }) => (
  <label>
    <span>{name}</span>
    <span className="num">{value}</span>
  </label>
);

function Tiles({ math, code, onChange }) {
  const names = named(math, DIM, BASE);
  const reps = classes(math, DIM, BASE) ?? [];
  return (
    <>
      <Pick label="Named" value={word(names, code) ? code : ''} options={[['', 'Code'], ...names.map((one) => [one.code, one.name])]} onChange={(v) => v && onChange(v)} />
      <Text label="Code" value={code} commit onChange={onChange} />
      <Btn onClick={() => onChange(roll(Math.random))}>
        <Icon name="reroll" />
        Random
      </Btn>
      {reps.length > 0 && <Thumbs math={math} dim={DIM} base={BASE} codes={reps.map((one) => one.code)} code={code} names={names} onPick={onChange} px={40} />}
    </>
  );
}

export function Widget({ value, onChange, onReady, onExport, units: given }) {
  const [loaded, setLoaded] = useState(null);
  const units = given ?? loaded;
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const [facts, setFacts] = useState(null);
  const [grown, setGrown] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ math, num, ready }) => ready.then(() => on && setLoaded({ math, num })));
    return () => {
      on = false;
    };
  }, [given]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units, onLevel: setGrown }), [units]);
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    if (!handle) setGrown(null);
    onReady?.(handle && { ...handle, ...steps(handle, onChange) });
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `radix-${now.current.seed}`, 'svg');
  };
  const have = Boolean(facts);
  useEffect(() => onExport?.(have ? [['SVG', keep]] : []), [have]);
  const tile = value.shape === 'tile';
  const f = facts;
  const g = grown ?? facts;
  return (
    <>
      {units && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="right">
        <Group name="Design" hidden={!tile}>{units && tile && <Tiles math={units.math} code={value.code} onChange={(code) => onChange({ code })} />}</Group>
      </Bar>
      <Bar side="status">
        <span>{f ? (f.kind === 'tile' ? `${f.name || 'tile'} ${f.code}` : f.name) : 'loading'}</span>
        {g && <span>level {g.level}</span>}
      </Bar>
      {f && (
        <Bar side="right">
          <Group name="Counts">
            {ROWS.map(([name, get]) => <Fact key={name} name={name} value={get(f)} />)}
            {LIVE.map(([name, get]) => <Fact key={name} name={name} value={get(g)} />)}
          </Group>
        </Bar>
      )}
    </>
  );
}
