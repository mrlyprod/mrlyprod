import { useEffect, useRef } from 'react';
import { download, text } from '../app/export.js';
import { Fold } from '../app/controls.jsx';
import { Knob } from '../app/knobs.jsx';
import { rng } from '../rand.js';
import { roll, settle } from '../designs/engine.js';
import { Pick, read } from '../designs/pick.jsx';
import { DIM, named } from './engine.js';
import { SIZES } from './scene.js';

const WAIT = 90;

export function useReroll(value, onChange, math) {
  const seen = useRef(value.seed);
  useEffect(() => {
    if (value.seed === seen.current || !math) return;
    seen.current = value.seed;
    onChange({ code: roll(math, DIM, read(value).base, rng(value.seed)) });
  }, [value.seed, math]);
}

export function useStatus(live, key, onStatus, rows) {
  useEffect(() => {
    if (!onStatus) return undefined;
    const timer = setTimeout(() => {
      const facts = live.current?.facts?.();
      if (facts) onStatus(rows(facts));
    }, WAIT);
    return () => clearTimeout(timer);
  }, [key]);
}

export function useShots(onExport, live, have, name) {
  const svg = () => {
    const made = live.current?.svg?.();
    if (made) text(made, name(), 'svg');
  };
  const shot = (size) => live.current?.shot?.(size).then((blob) => download(blob, `${name()}-${size}.png`));
  useEffect(() => onExport?.(have ? [...SIZES.map((size) => [`PNG ${size}`, () => shot(size)]), ['SVG', svg]] : []), [have]);
}

export function Folds({ groups }) {
  return groups.map(({ name, live, body }) => (
    <Fold key={name} name={name} live={live}>
      {body}
    </Fold>
  ));
}

export function knobs(keys, value, onChange, spec, units) {
  return keys.map((key) => {
    const one = spec.find((it) => it.key === key);
    return <Knob key={key} row={one} value={value[key]} values={value} units={units} onChange={onChange} />;
  });
}

export function design(value, onChange, unit, spec) {
  const pick = ({ dim, ...patch }) => onChange(patch);
  const { base, code } = read(value);
  const shown = unit ? settle(unit, DIM, base, code, rng(value.seed)) : code;
  const name = unit ? named(unit, shown, base) : '';
  const word = spec.find((one) => one.key === 'number').label.toLowerCase();
  return {
    name: 'Design',
    live: [name || shown, `${word} ${value.number}`, `level ${value.level}`].join(' · '),
    body: (
      <>
        {unit && <Pick value={{ ...read(value), dim: DIM }} onChange={pick} math={unit} dims={[DIM]} />}
        {knobs(['number', 'level'], value, onChange, spec, unit && { math: unit })}
      </>
    ),
  };
}
