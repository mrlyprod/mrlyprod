import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/export.js';
import { Bar } from '../../lib/frame.jsx';
import { Scene } from '../../lib/scene.jsx';
import { rng } from '../../lib/scene.js';
import { REACH, factors, formula, shuffle, step } from './engine.js';
import { make, named } from './scene.js';

const pct = (x, places = 1) => `${(x * 100).toFixed(places)}%`;

const LABEL = 'The sheet: the whole numbers wound from one on the lattice, the marked ones lit and the quadratic in orange and blue';

export function streak(line) {
  if (!line.count) return 'off the sheet';
  if (line.streak === line.count) return `all ${line.count} prime`;
  return `${line.streak}, then ${line.next.n} = ${factors(line.next.factors)}`;
}

export const values = (line) => (line.values.length ? `${line.values.join(' ')}${line.more ? ' …' : ''}` : 'none on the sheet');

export function verdict(pick) {
  if (pick.n === 1) return 'one, neither prime nor composite';
  return pick.prime ? 'prime' : `composite, ${factors(pick.factors)}`;
}

export const rolled = (unit, seed, was) => (unit && seed !== was ? shuffle(unit, rng(seed)) : null);

export function readouts(f) {
  const pick = f.pick?.off ? [['Cell', 'off the sheet']] : f.pick ? [['Cell', `${f.pick.n} on ring ${f.pick.ring} at ${f.pick.x}, ${f.pick.y}`], ['Verdict', verdict(f.pick)]] : [];
  const line = f.line ? [['Values', values(f.line)], ['Streak', streak(f.line)], ['Hits', `${f.line.hits} of ${f.line.count}`], ['Share', pct(f.line.share)]] : [];
  const lit = f.mark === 'prime' ? [] : [['Lit', f.lit]];
  return [...(f.line ? [['Line', formula(f.a, f.b, f.c)]] : []), ...pick, ...line, ['Numbers', f.top], ['Primes', f.primes], ['Density', pct(f.density, 2)], ...lit];
}

export function Widget({ value, onChange, onReady, onExport, onStatus, unit: given }) {
  const [loaded, setLoaded] = useState(null);
  const unit = given ?? loaded;
  const [facts, setFacts] = useState(null);
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const change = useRef(onChange);
  change.current = onChange;
  const seed = useRef(value.seed);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ num, ready }) => ready.then(() => on && setLoaded(() => num)));
    return () => {
      on = false;
    };
  }, [given]);
  useEffect(() => {
    const next = rolled(unit, value.seed, seed.current);
    if (!next) return;
    seed.current = value.seed;
    onChange(next);
  }, [unit, value.seed]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, num: unit, onFacts: setFacts }), [unit]);
  const ready = (handle) => {
    live.current = handle;
    setFacts(handle?.facts ?? null);
    const line = (by) => {
      const { a, c } = now.current;
      const next = a > 0 ? step(unit, c, by, REACH) : c;
      if (next !== c) change.current({ c: next });
    };
    onReady?.(handle && { ...handle, line });
  };
  const tap = (e) => {
    const box = e.currentTarget.getBoundingClientRect();
    live.current?.pick((e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height);
  };
  const keep = (kind) => {
    const body = live.current?.[kind]?.();
    if (body) text(body, named(now.current), kind);
  };
  const have = Boolean(facts);
  const lined = Boolean(facts?.line);
  useEffect(() => onExport?.(have ? [['SVG', () => keep('svg')], ...(lined ? [['CSV', () => keep('csv')]] : [])] : []), [have, lined]);
  const rows = facts ? readouts(facts) : [];
  const said = JSON.stringify(rows);
  useEffect(() => onStatus?.(rows), [said]);
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} onClick={tap} aria-label={LABEL} />}
      {!unit && (
        <Bar side="status">
          <span>loading</span>
        </Bar>
      )}
    </>
  );
}
