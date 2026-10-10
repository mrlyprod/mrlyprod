import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { roll } from '../designs/engine.js';
import { Pick, read } from '../designs/pick.jsx';
import { FADES, ROLES, turn } from './engine.js';
import { make } from './scene.js';

const ROWS = [
  ['Name', (f) => f.title],
  ['Level', (f) => f.level],
  ['Side', (f) => f.side],
  ['Nodes', (f) => f.nodes],
  ['Branches', (f) => f.branches],
  ['Pieces', (f) => f.pieces],
  ['Length', (f) => f.length.toFixed(1)],
  ['Box dimension', (f) => f.box.toFixed(3)],
  ['Euler', (f) => f.euler],
];

const Fact = ({ name, value }) => (
  <label>
    <span>{name}</span>
    <span className="num">{value}</span>
  </label>
);

const LEGEND = [3, 2, 1, 0];

const STATUS = { flex: '0 100 auto', minWidth: 0, overflow: 'hidden', textOverflow: 'ellipsis' };

export const note = (f) => `${f.name || 'bang'} ${f.code}, ${f.graph} graph, ${f.layout}`;

const swatch = (role, tint) => ({
  display: 'inline-block',
  width: '0.75em',
  height: '0.75em',
  marginRight: '0.6em',
  borderRadius: '50%',
  background: `color-mix(in srgb, var(${tint ? `--${tint}` : '--accent'}) ${Math.round((1 - FADES[role]) * 100)}%, var(--art))`,
});

export function Legend({ roles, tint = '' }) {
  return (
    <Group name="Shades">
      {LEGEND.filter((role) => roles[role] > 0).map((role) => (
        <Fact
          key={role}
          name={
            <>
              <i aria-hidden="true" style={swatch(role, tint)} />
              {ROLES[role]}
            </>
          }
          value={roles[role]}
        />
      ))}
    </Group>
  );
}

export function useUnit(given) {
  const [math, setMath] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ math, ready }) => ready.then(() => on && setMath(() => math)));
    return () => {
      on = false;
    };
  }, [given]);
  return given ?? math;
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

export function Widget({ value, onChange, onReady, onExport, unit, three: given }) {
  const math = useUnit(unit);
  const solid = read(value).dim === 3;
  const three = useThree(solid, given);
  const live = useRef(null);
  const now = useRef(value);
  const from = useRef(null);
  const [facts, setFacts] = useState(null);
  now.current = value;
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    const hands = {
      random: () => {
        const { dim, base } = read(now.current);
        onChange({ code: roll(math, dim, base) });
      },
      deeper: (by) => onChange({ level: now.current.level + by }),
      next: (by) => onChange({ graph: turn(now.current.graph || handle.facts.graph, by) }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const down = (e) => {
    if (e.button) return;
    from.current = [e.clientX, e.clientY];
    e.currentTarget.setPointerCapture?.(e.pointerId);
  };
  const move = (e) => {
    if (!from.current) return;
    const rect = e.currentTarget.getBoundingClientRect();
    live.current?.turn?.((e.clientX - from.current[0]) / rect.width, (e.clientY - from.current[1]) / rect.height);
    from.current = [e.clientX, e.clientY];
  };
  const up = () => {
    from.current = null;
  };
  const keep = (body, kind) => body && text(body, `graphs-${now.current.seed}`, kind);
  const svg = () => keep(live.current?.svg?.(), 'svg');
  const json = () => keep(live.current?.json?.(), 'json');
  const have = Boolean(facts);
  useEffect(() => onExport?.(have ? [['SVG', svg], ['JSON', json]] : []), [have]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, three }), [math, three]);
  const pick = ({ dim, base, code }) => onChange({ dim, base, code });
  const f = facts;
  return (
    <>
      {math && (!solid || three) && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} onPointerMove={move} onPointerUp={up} onPointerCancel={up} />}
      <Bar side="right">
        <Group name="Design">{math ? <Pick value={read(value)} onChange={pick} math={math} /> : <span>loading</span>}</Group>
      </Bar>
      <Bar side="status">
        <span style={STATUS}>{f ? note(f) : 'loading'}</span>
      </Bar>
      {f && (
        <Bar side="right">
          <Group name="Counts">
            {ROWS.map(([name, get]) => <Fact key={name} name={name} value={get(f)} />)}
          </Group>
          <Legend roles={f.roles} tint={value.tint} />
        </Bar>
      )}
    </>
  );
}
