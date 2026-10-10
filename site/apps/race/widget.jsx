import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { rng } from '../../lib/scene.js';
import { Scene } from '../../lib/scene.jsx';
import { pair, roots } from './engine.js';
import { make } from './scene.js';

export const LABELS = ['A', 'B'];

const ROWS = [
  ['Name', (f) => f.name || f.title],
  ['Fill', (f) => `${f.fills} of ${f.of}`],
  ['Dimension', (f) => f.dimension.toFixed(3)],
  ['Room', (f) => `${f.cells} of ${f.lit}`],
];

const cells = (x) => x.toFixed(1);

const roll = () => Math.floor(Math.random() * 4294967296);

const ENDS = { stall: 'stalled', limit: 'time up' };

export function said(stat) {
  const { over, reach, tally } = stat;
  const lead = reach[0] > reach[1] ? 0 : reach[1] > reach[0] ? 1 : -1;
  const ahead = (side, verb) => (side < 0 ? 'level' : `${LABELS[side]} ${verb}`);
  const word = !over ? ahead(lead, 'leads') : over.how === 'goal' ? (over.winner < 0 ? 'draw' : ahead(over.winner, 'wins')) : `${ENDS[over.how]}, ${ahead(over.winner, 'ahead')}`;
  const score = tally[0] + tally[1] + tally[2] ? ` ${tally[0]}-${tally[1]}` : '';
  return `${word}${score}`;
}

const Fact = ({ name, value }) => (
  <label>
    <span>{name}</span>
    <span className="num">{value}</span>
  </label>
);

export function Widget({ value, onChange, onReady, onExport, unit: given }) {
  const [loaded, setLoaded] = useState(null);
  const unit = given ?? loaded;
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const [facts, setFacts] = useState(null);
  const [stat, setStat] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ math, ready }) => ready.then(() => on && setLoaded(() => math)));
    return () => {
      on = false;
    };
  }, [given]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit, onStat: setStat }), [unit]);
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    setStat(handle?.stat?.() ?? null);
    const hands = {
      random: () => {
        const seed = roll();
        const [a, b] = pair(unit, now.current, roots(rng(seed)).code);
        onChange({ seed, a, b });
      },
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `race-${now.current.seed}`, 'svg');
  };
  const log = () => {
    const rows = live.current?.csv?.();
    if (rows) text(rows, `race-${now.current.seed}`, 'csv');
  };
  const have = Boolean(facts);
  useEffect(() => onExport?.(have ? [['SVG', keep], ['CSV', log]] : []), [have]);
  const f = facts;
  const s = stat;
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="status">
        {!unit && <span>loading</span>}
        {s && <span>{said(s)}</span>}
        {s && <span>step {s.tick}</span>}
        {s && <span>{LABELS[0]} {cells(s.reach[0])}</span>}
        {s && <span>{LABELS[1]} {cells(s.reach[1])}</span>}
      </Bar>
      {f && (
        <Bar side="right">
          {f.sides.map((side, i) => (
            <Group key={LABELS[i]} name={`${LABELS[i]} ${side.code}`}>
              {ROWS.map(([name, get]) => <Fact key={name} name={name} value={get(side)} />)}
            </Group>
          ))}
          <Group name="Course">
            <Fact name="Board" value={`${f.side} by ${f.side}`} />
            <Fact name="Goal" value={`${f.goal} cells`} />
            <Fact name="Walkers" value={f.walkers} />
          </Group>
        </Bar>
      )}
    </>
  );
}
