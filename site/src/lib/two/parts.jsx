import { useEffect } from 'react';
import { download, text } from '../app/export.js';
import { Fold } from '../app/controls.jsx';
import { Knob } from '../app/knobs.jsx';
import { rng } from '../rand.js';
import { canon, roll } from '../designs/engine.js';
import { useReroll as onRoll } from '../app/hooks.js';
import { DIM, named, read } from './engine.js';
import { SIZES } from './scene.js';

const WAIT = 90;

export function useReroll(value, onChange, math) {
  onRoll(value.seed, (seed) => math && onChange({ code: roll(math, DIM, read(value).base, rng(seed)) }));
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
  const { base, code } = read(value);
  const shown = unit ? canon(unit, DIM, base, code) : code;
  const name = unit ? named(unit, shown, base) : '';
  const word = spec.find((one) => one.key === 'number').label.toLowerCase();
  return {
    name: 'Design',
    live: [name || shown, `${word} ${value.number}`, `level ${value.level}`].join(' · '),
    body: knobs(['number', 'level'], value, onChange, spec, unit && { math: unit }),
  };
}
