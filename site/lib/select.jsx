import { useMemo, useRef } from 'react';
import { mrly } from './mrly.js';
import { stamp } from './query.js';
import { Pick, Text, Btn, Slider, Check } from './app.jsx';

const FLAT3 = [['carpet', '495'], ['runner', '127']];

function designs(dimension, bases) {
  const list = [];
  if (bases.includes(2)) {
    for (const design of JSON.parse(mrly.universe(dimension)).designs) list.push([`${design.code} · ${design.anf}`, design.code, 2]);
  }
  if (dimension === 2 && bases.includes(3)) {
    for (const [word, code] of FLAT3) list.push([`${word} · ${code} · base 3`, code, 3]);
  }
  return list;
}

export function useSeeds() {
  const seed = useRef(+(new URLSearchParams(location.search).get('seed') ?? 0));
  return useMemo(() => ({
    get: () => seed.current,
    next: () => {
      seed.current += 1;
      stamp({ seed: seed.current });
      return seed.current;
    },
    drop: () => {
      seed.current = 0;
      stamp({ seed: null });
    },
  }), []);
}

export function roll(seed, spans) {
  return mrly.random_between(seed, spans.map(([low]) => low), spans.map(([, high]) => high));
}

export function seeded(seeds, dimension, base, code) {
  return seeds.get() ? mrly.random_code(dimension, base, seeds.get()) : code;
}

export function Picker({ dimension, bases = [2], code, base = bases[0], onChange, seeds, button = true }) {
  const list = useMemo(() => designs(dimension, bases), [dimension, String(bases)]);
  const picked = list.some(([, value, b]) => value === code.trim() && b === base) ? `${code.trim()}:${base}` : '';
  return (
    <>
      <Pick label="design" value={picked} options={[['', 'type a code'], ...list.map(([text, value, b]) => [`${value}:${b}`, text])]}
        onChange={(v) => {
          if (!v) return;
          const [value, b] = v.split(':');
          seeds.drop();
          onChange(bases.length > 1 ? { code: value, base: +b } : { code: value });
        }} />
      <Text label="code" value={code} onChange={(v) => { seeds.drop(); onChange({ code: v }); }} />
      {bases.length > 1 && <Pick label="base" value={base} options={bases.map((b) => [b, b])} onChange={(v) => onChange({ base: +v })} />}
      {button && <Btn onClick={() => onChange({ code: mrly.random_code(dimension, base, seeds.next()) })}>Randomize</Btn>}
    </>
  );
}

export function Ramp({ value, onChange }) {
  return (
    <>
      <Pick label="ramp" value={value.ramp} options={['fire', 'heat', 'diverge']} onChange={(v) => onChange({ ramp: v })} />
      <Slider label="levels" value={value.levels} min={2} max={64} onChange={(v) => onChange({ levels: v })} />
      <Check label="invert" checked={value.invert} onChange={(v) => onChange({ invert: v })} />
    </>
  );
}

export function Cropper({ dimension = 2, value, onChange }) {
  const shapes = useMemo(() => JSON.parse(mrly.crop_shapes(dimension)), [dimension]);
  return (
    <>
      <Pick label="crop" value={value.crop} options={[['', 'off'], ...shapes]} onChange={(v) => onChange({ crop: v })} />
      <Slider label="radius" value={value['crop-r']} min={1} max={32} show={`${value['crop-r']}/32`} onChange={(v) => onChange({ 'crop-r': v })} />
      <Check label="anti" checked={value['crop-anti']} onChange={(v) => onChange({ 'crop-anti': v })} />
    </>
  );
}

export function cropOf(value) {
  return { shape: value.crop, rnum: value['crop-r'], rden: 32, anti: value['crop-anti'], active: value.crop !== '' };
}

export const SOURCE_FIRST = { source: 'flat', code: '495', base: 3, number: 3, level: 4, preset: 'weave', limit: 9, scode: '23', tile: 3, slevel: 2 };

function drawMoire(seed) {
  const names = mrly.moire_names();
  const [p, l] = roll(seed, [[0, names.length - 1], [1, 41]]);
  return { preset: names[p], limit: l | 1 };
}

export function seedSource(seeds, first) {
  const s = seeds.get();
  if (!s) return first;
  const params = new URLSearchParams(location.search);
  const source = params.get('source') ?? first.source;
  if (source === 'flat') return { ...first, code: mrly.random_code(2, +params.get('base') || first.base, s) };
  if (source === 'slice') return { ...first, scode: mrly.random_code(3, 2, s) };
  return { ...first, ...drawMoire(s) };
}

export function Sources({ value, onChange, seeds, onSeed }) {
  const presets = useMemo(() => [...mrly.moire_names()], []);
  const randomize = () => {
    const s = seeds.next();
    if (value.source === 'flat') onChange({ code: mrly.random_code(2, value.base, s) });
    else if (value.source === 'slice') onChange({ scode: mrly.random_code(3, 2, s) });
    else onChange(drawMoire(s));
    if (onSeed) onSeed(s);
  };
  return (
    <>
      <Pick label="source" value={value.source} options={[['flat', 'flat design'], ['moire', 'moire stack'], ['slice', 'hex slice']]} onChange={(v) => onChange({ source: v })} />
      {value.source === 'flat' && (
        <span className="set">
          <Picker dimension={2} bases={[3, 2]} code={value.code} base={value.base} seeds={seeds} button={false} onChange={onChange} />
          <Pick label="side" value={value.number} options={[[3, 3], [5, 5], [7, 7]]} onChange={(v) => onChange({ number: +v })} />
          <Slider label="level" value={Math.min(value.level, mrly.level_cap(value.number, 1, 243))} min={1} max={mrly.level_cap(value.number, 1, 243)} onChange={(v) => onChange({ level: v })} />
        </span>
      )}
      {value.source === 'moire' && (
        <span className="set">
          <Pick label="preset" value={value.preset} options={presets} onChange={(v) => onChange({ preset: v })} />
          <Slider label="scales up to" value={value.limit} min={1} max={41} step={2} onChange={(v) => onChange({ limit: v })} />
        </span>
      )}
      {value.source === 'slice' && (
        <span className="set">
          <Picker dimension={3} code={value.scode} seeds={seeds} button={false} onChange={(patch) => onChange(patch.code === undefined ? patch : { scode: patch.code })} />
          <Pick label="tile" value={value.tile} options={[[3, 3], [5, 5]]} onChange={(v) => onChange({ tile: +v })} />
          <Slider label="level" value={Math.min(value.slevel, mrly.level_cap(value.tile, 1, 27))} min={1} max={mrly.level_cap(value.tile, 1, 27)} onChange={(v) => onChange({ slevel: v })} />
        </span>
      )}
      <Btn onClick={randomize}>Randomize</Btn>
    </>
  );
}

export function readSource(value) {
  if (value.source === 'flat') {
    const level = Math.min(value.level, mrly.level_cap(value.number, 1, 243));
    const grid = mrly.two_grid(value.code, value.number, level, 0, value.base);
    const fills = JSON.parse(mrly.two_census(value.code, value.number, level, 0, value.base)).fills;
    return { kind: 'flat', grid, field: Float32Array.from(grid.types), size: grid.width, name: mrly.name_of(value.code, 2, value.base), fills };
  }
  if (value.source === 'moire') {
    return { kind: 'moire', grid: null, field: mrly.moire_field(value.preset, value.limit, 256), size: 256, name: value.preset, fills: '' };
  }
  const level = Math.min(value.slevel, mrly.level_cap(value.tile, 1, 27));
  const grid = mrly.slice_grid(value.scode, value.tile, level, 2, 384);
  const fills = JSON.parse(mrly.slice_census(value.scode, value.tile, level, 2)).fills;
  return { kind: 'slice', grid, field: Float32Array.from(grid.types), size: 384, name: mrly.name_of(value.scode, 3, 2), fills };
}
