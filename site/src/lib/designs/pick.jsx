import { useEffect, useRef, useState } from 'react';
import { tidy } from '../app/spec.js';
import { Btn, Icon, Pick as Select, Segment, Slider, Text } from '../app/controls.jsx';
import { paints } from '../color.js';
import { DESIGN, cap, classes, roll } from './engine.js';
import { enqueue, thumb } from './thumb.js';
import { useUnit } from '../app/hooks.js';

const STRIP = 32;
const FILL = { width: '100%' };
const MARGIN = '160px';

/* UNIT */

export function useTheme() {
  const [tick, setTick] = useState(0);
  useEffect(() => {
    const bump = () => setTick((was) => was + 1);
    const shade = typeof matchMedia === 'function' ? matchMedia('(prefers-color-scheme: dark)') : null;
    addEventListener('theme', bump);
    shade?.addEventListener('change', bump);
    return () => {
      removeEventListener('theme', bump);
      shade?.removeEventListener('change', bump);
    };
  }, []);
  return tick;
}

/* SIGHT */

const sights = new Map();
let eye = null;

export function watch(node, fn) {
  if (!node || typeof IntersectionObserver !== 'function') return undefined;
  eye ??= new IntersectionObserver((rows) => {
    for (const row of rows) {
      if (!row.isIntersecting) continue;
      const seen = sights.get(row.target);
      eye.unobserve(row.target);
      sights.delete(row.target);
      seen?.();
    }
  }, { rootMargin: MARGIN });
  sights.set(node, fn);
  eye.observe(node);
  return () => {
    eye.unobserve(node);
    sights.delete(node);
  };
}

/* THUMBS */

export function Thumb({ math, dim, base, code, px = 56, tick = 0, hue = '', level }) {
  const at = useRef(null);
  const [seen, setSeen] = useState(() => typeof IntersectionObserver !== 'function');
  useEffect(() => (seen ? undefined : watch(at.current, () => setSeen(true))), [seen]);
  useEffect(() => {
    const canvas = at.current;
    if (!seen || !canvas || !math) return undefined;
    return enqueue(() => {
      const dpr = Math.min(globalThis.devicePixelRatio || 1, 2);
      const side = Math.round(px * dpr);
      canvas.width = side;
      canvas.height = side;
      thumb(canvas.getContext('2d'), math, dim, base, code, side, paints(canvas, hue), level);
    });
  }, [seen, math, dim, base, code, px, tick, hue, level]);
  return <canvas ref={at} width={px} height={px} aria-hidden="true" />;
}

export function Thumbs({ math, dim, base, codes, code, onPick, px, className = 'designs', tone, note }) {
  const tick = useTheme();
  return (
    <div className={className} style={FILL}>
      {codes.map((one) => {
        const name = math.name.of(one, dim, base) ?? '';
        const form = note?.(one) ?? '';
        const label = note ? `${one}${name ? ` ${name}` : ''}` : name || one;
        return (
          <button type="button" key={one} className={one === String(code) ? 'on' : undefined} aria-pressed={one === String(code)} aria-label={`${name || 'bang'} ${one}${form ? `, ${form}` : ''}`} title={[name, form].filter(Boolean).join(', ') || undefined} onClick={() => onPick(one)}>
            <Thumb math={math} dim={dim} base={base} code={one} px={px} tick={tick} hue={tone?.(one) ?? ''} />
            <small>{label}</small>
            {form && <small>{form}</small>}
          </button>
        );
      })}
    </div>
  );
}

/* PICK */

export const read = (value) => tidy(DESIGN, value);

export function Picker({ value, onChange, math, dims = [2, 3] }) {
  const { dim, base, code } = read(value);
  const spaces = DESIGN[0].options.filter(([one]) => dims.includes(one));
  const names = math ? math.name.words(dim, base) : [];
  const reps = math ? (classes(math, dim, base) ?? []) : [];
  const write = (patch) => onChange(tidy(DESIGN, { dim, base, code, ...patch }));
  return (
    <>
      {spaces.length > 1 && <Segment label={DESIGN[0].label} value={dim} options={spaces} onChange={(v) => write({ dim: v })} />}
      <Segment label={DESIGN[1].label} value={base} options={DESIGN[1].options} onChange={(v) => write({ base: v })} />
      <Select label="Named" value={names.some(([one]) => one === String(code)) ? code : ''} options={[['', 'Code'], ...names]} onChange={(v) => v && write({ code: v })} />
      <Text label={DESIGN[2].label} value={code} commit onChange={(v) => write({ code: v })} />
      {'level' in value && <Slider label="Level" value={value.level} min={1} max={math ? cap(math, dim) : undefined} step={1} onChange={(v) => onChange({ level: v })} />}
      <Btn disabled={!math} onClick={() => write({ code: roll(math, dim, base) })}>
        <Icon name="casino" />
        Random
      </Btn>
      {reps.length > 0 && reps.length <= STRIP && <Thumbs math={math} dim={dim} base={base} codes={reps.map((one) => one.code)} code={code} onPick={(v) => write({ code: v })} px={40} />}
    </>
  );
}

export function Pick({ value, onChange, math: given, dims }) {
  const math = useUnit('math', given);
  return <Picker value={value} onChange={onChange} math={math} dims={dims} />;
}
