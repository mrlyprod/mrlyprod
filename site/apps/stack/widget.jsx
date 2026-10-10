import { useCallback, useEffect, useMemo, useState } from 'react';
import { Bar } from '../../lib/frame.jsx';
import { Group, Pick as Select } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { Pick } from '../designs/pick.jsx';
import { EYES, eyeOf, label } from './engine.js';
import { make } from './scene.js';

const OFF = ['', 'Off the lattice'];

const fixed = (x, places = 4) => (Number.isFinite(x) ? x.toFixed(places) : 'none');

const brief = (scales) => (scales.length <= 6 ? scales.join(' ') : `${scales.slice(0, 4).join(' ')} ... ${scales.at(-1)}`);

const Count = ({ name, children }) => (
  <label>
    <span>{name}</span>
    <span className="num">{children}</span>
  </label>
);

export const patch = ({ dim, ...rest }) => rest;

export function said(f) {
  if (f.kind === 'moire') return `${f.name || 'code'} ${f.code}`;
  if (f.kind === 'tourbillon') return `${f.layers} layers`;
  return `${f.layers} cuts`;
}

export function rows(f) {
  if (f.kind === 'moire') {
    return [
      ['Layers', f.layers],
      ['Scales', brief(f.scales)],
      ['Largest r', f.prime ? '0' : `${fixed(f.max)} at ${f.at}`],
      ['Scale ' + f.limit, f.prime ? 'prime, row clear' : 'shares a factor'],
    ];
  }
  if (f.kind === 'tourbillon') {
    return [
      ['Layers', f.layers],
      ['Set', f.set],
      ['Weights', f.weights],
      ['Period', f.period ?? 'none'],
      ['Classes', f.classes],
      ['Pairs', f.pairs],
      ['Mean', fixed(f.mean)],
      ['Contrast', fixed(f.rms)],
      ['Centre', fixed(f.centre)],
    ];
  }
  return [
    ['Cuts', f.layers],
    ['Deepest', f.limit],
    ['Excess x L', fixed(f.scaled)],
    ['Settles', fixed(f.logged)],
    ['Slope', f.slope === null ? 'needs L mod 4 = 0' : fixed(f.slope)],
    ['Branch', f.branch],
  ];
}

export function Widget({ value, onChange, onReady, unit: given }) {
  const [loaded, setLoaded] = useState(null);
  const unit = given ?? loaded;
  const [facts, setFacts] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ math, ready }) => ready.then(() => on && setLoaded(() => math)));
    return () => {
      on = false;
    };
  }, [given]);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit }), [unit]);
  const ready = (handle) => {
    setFacts(handle?.facts ?? null);
    onReady?.(handle);
  };
  const eyes = useMemo(() => (unit ? unit.tourbillon.eyes(EYES) : []), [unit]);
  const moire = value.kind === 'moire';
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="status">
        {!unit && <span>loading</span>}
        {facts && <span>{said(facts)}</span>}
      </Bar>
      {moire && (
        <Bar side="right">
          <Group name="Design">{unit ? <Pick value={{ ...value, dim: 2 }} onChange={(next) => onChange(patch(next))} math={unit} dims={[2]} /> : <span>loading</span>}</Group>
        </Bar>
      )}
      {facts && (
        <Bar side="right">
          <Group name="Readings">
            {rows(facts).map(([name, text]) => (
              <Count key={name} name={name}>
                {text}
              </Count>
            ))}
            {facts.kind === 'tourbillon' && <Select label="Eye" value={eyeOf(eyes, value.increment)} options={[OFF, ...eyes.map((eye, i) => [String(i), label(eye)])]} onChange={(v) => v !== '' && onChange({ increment: eyes[+v].angle })} />}
          </Group>
        </Bar>
      )}
    </>
  );
}
