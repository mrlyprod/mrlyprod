import { useEffect, useRef } from 'react';
import { tidy } from '../app/spec.js';
import { Fold, Pick as Select, Segment, Slider } from '../app/controls.jsx';
import { rng } from '../rand.js';
import { settle } from '../designs/engine.js';
import { Pick } from '../designs/pick.jsx';
import { CUT, LIMIT, SOURCES, cap, title } from './engine.js';
import { SOURCE } from './scene.js';
import { useLatest } from '../app/hooks.js';

export const read = (value) => tidy(SOURCE, value);

const option = (key) => SOURCE.find((row) => row.key === key).options;

export function useReroll(seed, roll) {
  const was = useRef(seed);
  const latest = useLatest(roll);
  useEffect(() => {
    if (was.current === seed) return;
    was.current = seed;
    latest.current(rng(seed));
  }, [seed]);
}

export function gist(value, math) {
  const src = read(value);
  const name = (raw, dim, base) => {
    const code = math ? settle(math, dim, base, raw, rng(value.seed)) : raw;
    return title({ name: math?.name.of(code, dim, base) ?? '', code, kind: src.source });
  };
  if (src.source === 'moire') return `Moire · ${src.preset} · ${src.limit}`;
  if (src.source === 'slice') return `Slice · ${name(src.scode, 3, src.sbase)} · ${src.tile} · ${src.slevel}`;
  return `Flat · ${name(src.code, 2, src.base)} · ${src.number} · ${src.level}`;
}

export function Rows({ value, onChange, math }) {
  const src = read(value);
  const write = (patch) => onChange(read({ ...src, ...patch }));
  return (
    <>
      <Segment label="Source" value={src.source} options={SOURCES} onChange={(v) => write({ source: v })} />
      {src.source === 'flat' && (
        <>
          {math ? <Pick value={{ dim: 2, base: src.base, code: src.code }} onChange={({ base, code }) => write({ base, code })} math={math} dims={[2]} /> : <span>loading</span>}
          <Select label="Side" value={src.number} options={option('number')} onChange={(v) => write({ number: v })} />
          <Slider label="Level" value={src.level} min={1} max={cap(src.number)} step={1} onChange={(v) => write({ level: v })} />
        </>
      )}
      {src.source === 'moire' && (
        <>
          <Select label="Preset" value={src.preset} options={option('preset')} onChange={(v) => write({ preset: v })} />
          <Slider label="Scales up to" value={src.limit} min={1} max={LIMIT} step={2} onChange={(v) => write({ limit: v })} />
        </>
      )}
      {src.source === 'slice' && (
        <>
          {math ? <Pick value={{ dim: 3, base: src.sbase, code: src.scode }} onChange={({ base, code }) => write({ sbase: base, scode: code })} math={math} dims={[3]} /> : <span>loading</span>}
          <Select label="Tile" value={src.tile} options={option('tile')} onChange={(v) => write({ tile: v })} />
          <Slider label="Level" value={src.slevel} min={1} max={cap(src.tile, CUT)} step={1} onChange={(v) => write({ slevel: v })} />
        </>
      )}
    </>
  );
}

export function Source({ value, onChange, math }) {
  return (
    <Fold name="Source" live={gist(value, math)}>
      <Rows value={value} onChange={onChange} math={math} />
    </Fold>
  );
}
