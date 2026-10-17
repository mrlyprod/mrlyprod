import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/app/export.js';
import { Bar } from '../../lib/app/frame.jsx';
import { Fold, Pick as Select, Segment, Text } from '../../lib/app/controls.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { rng } from '../../lib/rand.js';
import { classes, next } from '../../lib/designs/engine.js';
import { Thumbs } from '../../lib/designs/pick.jsx';
import { BASES, DIM, looping, names, resolve } from './engine.js';
import { file, make } from './scene.js';

const STRIP = 32;

export function rows(f) {
  if (!f) return [];
  const design = ['design', f.name ? `${f.name} ${f.code}` : `code ${f.code}`];
  if (f.fault) return [design, ['fault', f.fault]];
  const law = f.law ? [['law', `L(n) = ${f.law.formula}`], [`L(${f.level})`, `${f.law.loops}, the law ${f.law.loops === f.loops ? 'meets' : 'misses'} the count`]] : [['law', 'no closed form proved for this code']];
  return [design, ['loops', String(f.loops)], ['strands', `${f.strands} = 2 x ${f.side}, proved`], ['side', String(f.side)], ['filled', `${f.filled} of ${f.cells}`], ...law];
}

export function named(math, base) {
  return names(math, base).map(([code, word]) => [code, base === 2 ? `${word} · ${math.bang.universe.anf_string(code, DIM)}` : word]);
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units }) {
  const unit = units?.math;
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef({});
  hooks.current = { onChange, onStatus };
  const seed = useRef(value.seed);
  const [facts, setFacts] = useState(null);
  useEffect(() => {
    if (!unit || seed.current === value.seed) return;
    seed.current = value.seed;
    onChange({ code: looping(unit, value.base, value.level, rng(value.seed)) });
  }, [unit, value.seed]);
  const tell = useCallback((f) => {
    setFacts(f);
    hooks.current.onStatus?.(rows(f));
  }, []);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit, onTell: tell }), [unit, tell]);
  const step = (by) => {
    const { base, code } = now.current;
    hooks.current.onChange({ code: next(names(unit, base).map(([one]) => one), resolve(unit, base, code), by) });
  };
  const deeper = (by) => hooks.current.onChange({ level: now.current.level + by });
  const ready = (handle) => {
    live.current = handle;
    if (!handle) hooks.current.onStatus?.([]);
    onReady?.(handle && { ...handle, step, deeper });
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, file(now.current, units), 'svg');
  };
  const have = Boolean(unit);
  useEffect(() => onExport?.(have ? [['SVG', keep]] : []), [have]);
  const code = facts?.code ?? value.code;
  const list = unit ? named(unit, value.base) : [];
  const reps = unit && value.base === 2 ? (classes(unit, DIM, 2) ?? []) : [];
  const label = `${facts?.name || 'a design'} at level ${value.level} drawn in arcs`;
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} onClick={() => live.current?.trace?.()} role="img" aria-label={label} />}
      <Bar side="right">
        <Fold name="Design" live={facts ? `${facts.name || 'code'} ${code} · base ${value.base}` : ''}>
          <Segment label="Base" value={value.base} options={BASES} onChange={(v) => onChange({ base: Number(v), code: '' })} />
          <Select label="Named" value={list.some(([one]) => one === code) ? code : ''} options={[['', 'Code'], ...list]} onChange={(v) => v && onChange({ code: v })} />
          <Text label="Code" value={value.code || code} commit onChange={(v) => onChange({ code: v })} />
          {facts?.fault && (
            <p className="note" role="alert">
              {facts.fault}
            </p>
          )}
          {reps.length > 0 && reps.length <= STRIP && <Thumbs math={unit} dim={DIM} base={2} codes={reps.map((one) => one.code)} code={code} onPick={(v) => onChange({ code: v })} px={40} />}
        </Fold>
      </Bar>
    </>
  );
}
