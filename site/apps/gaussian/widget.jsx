import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { FATES, STEP, name, product, verdict } from './engine.js';
import { make } from './scene.js';

const TAP = 8;

const pct = (x) => `${(x * 100).toFixed(1)}%`;

const ROWS = [
  ['Reach', (f) => f.reach],
  ['Points', (f) => f.points],
  ['Primes', (f) => f.primes],
  ['Split', (f) => f.split],
  ['Inert', (f) => f.inert],
  ['Ramified', (f) => f.ramified],
  ['Units', (f) => f.units],
  ['Density', (f) => pct(f.density)],
  ['Symmetry', (f) => `${f.symmetry}-fold`],
  ['Busiest norm', (f) => `${f.busy}, ${f.most} points`],
];

const Fact = ({ label, value }) => (
  <label>
    <span>{label}</span>
    <span className="num">{value}</span>
  </label>
);

export function Widget({ value, onChange, onReady, onExport, unit: given }) {
  const [loaded, setLoaded] = useState(null);
  const unit = given ?? loaded;
  const live = useRef(null);
  const from = useRef(null);
  const now = useRef(value);
  now.current = value;
  const [facts, setFacts] = useState(null);
  const [pick, setPick] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ num, ready }) => ready.then(() => on && setLoaded(() => num)));
    return () => {
      on = false;
    };
  }, [given]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, num: unit }), [unit]);
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    setPick(null);
    const pin = (patch) => onChange({ roll: 0, ring: handle.facts.word, limit: handle.facts.reach, look: handle.facts.look, ...patch });
    const hands = {
      reach: (by) => pin({ limit: handle.facts.reach + STEP * by }),
      turn: () => pin({ ring: handle.facts.word === 'hex' ? 'square' : 'hex' }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const down = (e) => {
    from.current = [e.clientX, e.clientY];
  };
  const up = (e) => {
    const was = from.current;
    from.current = null;
    if (!was || Math.hypot(e.clientX - was[0], e.clientY - was[1]) > TAP) return;
    const box = e.currentTarget.getBoundingClientRect();
    setPick(live.current?.at?.((e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height) ?? null);
  };
  const cancel = () => {
    from.current = null;
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, `gaussian-${now.current.seed}`, 'svg');
  };
  const sheet = () => {
    const rows = live.current?.csv?.();
    if (rows) text(rows, `gaussian-${now.current.seed}`, 'csv');
  };
  const have = Boolean(facts);
  useEffect(() => onExport?.(have ? [['SVG', keep], ['CSV', sheet]] : []), [have]);
  const f = facts;
  const p = pick;
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} onPointerDown={down} onPointerUp={up} onPointerCancel={cancel} />}
      <Bar side="status">
        {!unit && <span>loading</span>}
        {f && !p && <span>{f.ring}, {f.primes} primes</span>}
        {f && p && <span>{name(f.word, p.a, p.b)}</span>}
        {f && p && <span>{verdict(f.word, p)}</span>}
      </Bar>
      {f && p && (
        <Bar side="right">
          <Group name="Point">
            <Fact label="Point" value={name(f.word, p.a, p.b)} />
            <Fact label="Norm" value={p.norm} />
            <Fact label="Fate" value={FATES[p.fate]} />
            <Fact label="Factors" value={p.factors.length ? product(p.factors) : '-'} />
            <Fact label="Conjugate" value={name(f.word, p.conjugate[0], p.conjugate[1])} />
            <Fact label="Associates" value={p.associates.map(([a, b]) => name(f.word, a, b)).join(', ')} />
          </Group>
        </Bar>
      )}
      {f && (
        <Bar side="right">
          <Group name="Counts">
            {ROWS.map(([label, get]) => <Fact key={label} label={label} value={get(f)} />)}
          </Group>
        </Bar>
      )}
    </>
  );
}
