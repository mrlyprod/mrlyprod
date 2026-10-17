import { useCallback, useRef, useState } from 'react';
import { Bar } from '../../lib/app/frame.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { useTouch } from '../../lib/app/touch.js';
import { fixed, roll } from '../../lib/moire/parts.jsx';
import { LEAST, MOST } from './engine.js';
import { make } from './scene.js';
import { useLatest, useStatus, useUnit } from '../../lib/app/hooks.js';

const sign = (n) => (n > 0 ? `+${n}` : String(n));

export const drawn = (rand) => ({ layers: LEAST + 2 * Math.floor(rand() * ((MOST - LEAST) / 2 + 1)), seed: roll() });

export function rows(f) {
  if (!f) return [];
  if (f.error) return [['error', f.error]];
  return [
    ['layers', f.layers],
    ['excess x L', fixed(f.scaled, 6)],
    ['slope', f.slope === null ? 'needs L divisible by four' : fixed(f.slope, 6)],
    ['width law', fixed(f.target, 6)],
    ['exact', `${f.exact} of ${f.layers} layers`],
    ['plus (ln L)/4', fixed(f.logged, 10)],
    ['C', fixed(f.constant, 10)],
    ['L mod 4', f.branch],
    ['residual x L^2', fixed(f.residual, 8)],
    ['predicted', f.predicted === null ? 'odd L has none' : fixed(f.predicted, 8)],
    ['deepest side', f.deepest],
    ['chi_8 at the deepest', sign(f.chi)],
    ['arm ink', fixed(f.arm, 6)],
    ['hexagon ink', fixed(f.hexagon, 6)],
    ['excess', fixed(f.excess, 6)],
    ['band', f.half === 0 ? 'the exact diameter x = y' : `${f.half} cells about it`],
    ['arm shares', f.ribbon.map((one) => `${one.arm}${one.exact ? '' : ` (law ${one.law})`}`).join(' ')],
    ['ladder', f.ladder.map((one) => `L ${one.count}: ${fixed(one.scaled, 6)}`).join(', ')],
    ['ink range', `${fixed(f.low, 4)} to ${fixed(f.high, 4)}`],
    ['pixels', `${f.size} by ${f.size}`],
  ];
}

export function Widget({ value, onChange, onReady, onStatus, unit }) {
  const math = useUnit('math', unit);
  const [facts, setFacts] = useState(null);
  const handle = useRef(null);
  const change = useLatest(onChange);
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, onFacts: setFacts }), [math]);
  const random = () => change.current(drawn(Math.random));
  const ready = (scene) => {
    const rich = scene && { ...scene, random };
    handle.current = rich;
    onReady?.(rich);
  };
  const touch = useTouch(handle, change);
  useStatus(onStatus, rows(facts));
  return (
    <>
      {math && <Scene make={made} value={value} onReady={ready} {...touch} />}
      <Bar side="status">{!math && <span>loading</span>}</Bar>
    </>
  );
}
