import { useCallback, useRef, useState } from 'react';
import { Bar } from '../../lib/app/frame.jsx';
import { Btn, Fold, Icon, Pick as Select } from '../../lib/app/controls.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { useTouch } from '../../lib/app/touch.js';
import { Pick } from '../../lib/designs/pick.jsx';
import { drawn, presetOf, presets } from '../../lib/moire/engine.js';
import { make } from './scene.js';
import { roll } from '../../lib/moire/parts.jsx';
import { useLatest, useStatus, useUnit } from '../../lib/app/hooks.js';

const exact = (r) => (r === 0 ? '0' : r.toFixed(9));

export const patch = ({ dim, ...rest }) => rest;

export function rows(f) {
  if (!f) return [];
  if (f.error) return [['error', f.error]];
  if (f.view === 'field') {
    return [
      ['design', f.name ? `${f.name} ${f.code}` : `code ${f.code}`],
      ['layers', f.layers],
      ['scales', f.scales.join(' ')],
      ['pixels', `${f.size} by ${f.size}`],
    ];
  }
  return [
    ['scale m', f.row || 'none'],
    ['scale n', f.mate || 'none earlier'],
    ['r', f.r === null ? 'none' : exact(f.r)],
    ['row of m', !f.row ? 'none' : f.prime ? 'clear against every earlier scale' : `largest ${exact(f.max)} at scale ${f.at}`],
    ['matrix', `${f.k} by ${f.k}`],
  ];
}

const title = (name) => name.charAt(0).toUpperCase() + name.slice(1);

export function Widget({ value, onChange, onReady, onStatus, unit }) {
  const math = useUnit('math', unit);
  const [facts, setFacts] = useState(null);
  const handle = useRef(null);
  const change = useLatest(onChange);
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math, onFacts: setFacts, onLimit: (limit) => change.current({ limit }) }), [math]);
  const random = () => math && change.current(drawn(math, roll()));
  const ready = (scene) => {
    const rich = scene && { ...scene, random };
    handle.current = rich;
    onReady?.(rich);
  };
  const touch = useTouch(handle, change);
  useStatus(onStatus, rows(facts));
  const list = math ? presets(math) : [];
  const preset = presetOf(list, value);
  const name = facts?.name ? `${facts.name} ${facts.code}` : `code ${value.code}`;
  const pick = (one) => {
    const found = list.find((p) => p.name === one);
    if (found) onChange({ base: found.base, code: found.code, level: found.level, combine: found.combine });
  };
  return (
    <>
      {math && <Scene make={made} value={value} onReady={ready} {...touch} />}
      <Bar side="status">{!math && <span>loading</span>}</Bar>
      {value.view !== 'matrix' && (
        <Bar side="right">
          <Fold name="Design" after="Stack" live={preset ? title(preset) : name}>
            <Select label="Preset" value={preset} options={[['', 'Custom'], ...list.map((one) => [one.name, title(one.name)])]} onChange={pick} />
            {math ? <Pick value={{ ...value, dim: 2 }} onChange={(next) => onChange(patch(next))} math={math} dims={[2]} /> : <span>loading</span>}
            <Btn disabled={!math} onClick={random}>
              <Icon name="casino" />
              Randomize
            </Btn>
          </Fold>
        </Bar>
      )}
    </>
  );
}
