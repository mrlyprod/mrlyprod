import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Group } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { factors, formula, step, title } from './engine.js';
import { SPEC, make, said } from './scene.js';

const MAX_C = SPEC.find((row) => row.key === 'c').max;

const pct = (x, places = 1) => `${(x * 100).toFixed(places)}%`;

export function streak(line) {
  if (!line.count) return 'off the sheet';
  if (line.streak === line.count) return `all ${line.count} prime`;
  return `${line.streak}, then ${line.next.n} = ${factors(line.next.factors)}`;
}

export const pin = (f, patch) => ({ roll: 0, lattice: f.lattice, a: f.a, b: f.b, c: f.c, ...patch });

const ROLLED = [['Lattice', (f) => title(f.lattice)]];

const SHEET = [
  ['Rings', (f) => f.rings],
  ['Numbers', (f) => f.top],
  ['Primes', (f) => f.primes],
  ['Density', (f) => pct(f.density, 2)],
];

const LIT = [['Lit', (f) => f.lit]];

const LINE = [
  ['Line', (f) => formula(f.a, f.b, f.c)],
  ['Landings', (f) => f.line.count],
  ['Hits', (f) => `${f.line.hits} of ${f.line.count}`],
  ['Share', (f) => pct(f.line.share)],
  ['Streak', (f) => streak(f.line)],
];

export const counts = (f) => [...(f.rolled ? ROLLED : []), ...SHEET, ...(f.mark === 'prime' ? [] : LIT), ...(f.line ? LINE : [])].map(([name, get]) => [name, get(f)]);

const Fact = ({ name, value }) => (
  <label>
    <span>{name}</span>
    <span className="num">{value}</span>
  </label>
);

export function Widget({ value, onChange, onReady, onExport, unit: given }) {
  const [loaded, setLoaded] = useState(null);
  const unit = given ?? loaded;
  const [facts, setFacts] = useState(null);
  const [picked, setPicked] = useState(null);
  const live = useRef(null);
  const known = useRef(null);
  const now = useRef(value);
  now.current = value;
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ num, ready }) => ready.then(() => on && setLoaded(() => num)));
    return () => {
      on = false;
    };
  }, [given]);
  const told = useCallback((next) => {
    known.current = next;
    setFacts(next);
  }, []);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, num: unit, onFacts: told }), [unit, told]);
  const ready = (handle) => {
    live.current = handle;
    told(handle?.facts ?? null);
    setPicked(null);
    const hands = {
      line: (by) => {
        const f = known.current;
        if (!f || !f.line) return;
        const c = step(unit, f.c, by, MAX_C);
        if (c !== f.c) onChange(pin(f, { c }));
      },
      lattice: () => {
        const f = known.current;
        if (f) onChange(pin(f, { lattice: f.lattice === 'hex' ? 'square' : 'hex' }));
      },
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const tap = (e) => {
    const box = e.currentTarget.getBoundingClientRect();
    setPicked(live.current?.pick((e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height) ?? null);
  };
  const keep = (kind) => {
    const body = live.current?.[kind]?.();
    if (body) text(body, `ulam-${now.current.seed}`, kind);
  };
  const have = Boolean(facts);
  const lined = Boolean(facts?.line);
  useEffect(() => onExport?.(have ? [['SVG', () => keep('svg')], ...(lined ? [['CSV', () => keep('csv')]] : [])] : []), [have, lined]);
  const f = facts;
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} onClick={tap} />}
      <Bar side="status">
        {!unit && <span>loading</span>}
        {f && <span>{picked ? said(picked) : f.line ? formula(f.a, f.b, f.c) : `${f.primes} primes in ${f.top}`}</span>}
      </Bar>
      {f && (
        <Bar side="right">
          <Group name="Counts">
            {counts(f).map(([name, shown]) => <Fact key={name} name={name} value={shown} />)}
          </Group>
        </Bar>
      )}
    </>
  );
}
