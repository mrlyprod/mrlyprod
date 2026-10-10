import { useEffect, useRef, useState } from 'react';
import { tidy } from '../../lib/knobs.js';
import { Btn, Icon, Pick as Select, Segment, Slider, Text } from '../../lib/knobs.jsx';
import { paints } from '../../lib/scene.js';
import { cap, classes, named, roll, word } from './engine.js';
import { DESIGN } from './scene.js';
import { enqueue, thumb } from './thumb.js';

const STRIP = 32;
const FILL = { width: '100%' };

/* UNIT */

export function useUnit(given) {
  const [math, setMath] = useState(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(async (unit) => {
      await unit.ready;
      if (on) setMath(() => unit.math);
    });
    return () => {
      on = false;
    };
  }, [given]);
  return given ?? math;
}

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

/* THUMBS */

export function Thumb({ math, dim, base, code, px = 56, tick = 0 }) {
  const at = useRef(null);
  useEffect(() => {
    const canvas = at.current;
    if (!canvas || !math) return undefined;
    return enqueue(() => {
      const dpr = Math.min(globalThis.devicePixelRatio || 1, 2);
      const side = Math.round(px * dpr);
      canvas.width = side;
      canvas.height = side;
      thumb(canvas.getContext('2d'), math, dim, base, code, side, paints(canvas));
    });
  }, [math, dim, base, code, px, tick]);
  return <canvas ref={at} width={px} height={px} aria-hidden="true" />;
}

export function Thumbs({ math, dim, base, codes, code, names, onPick, px, className = 'designs' }) {
  const tick = useTheme();
  return (
    <div className={className} style={FILL}>
      {codes.map((one) => {
        const name = word(names, one);
        return (
          <button type="button" key={one} className={one === String(code) ? 'on' : undefined} aria-pressed={one === String(code)} aria-label={`${name || 'bang'} ${one}`} title={name || undefined} onClick={() => onPick(one)}>
            <Thumb math={math} dim={dim} base={base} code={one} px={px} tick={tick} />
            <small>{name || one}</small>
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
  const names = math ? named(math, dim, base) : [];
  const reps = math ? (classes(math, dim, base) ?? []) : [];
  const write = (patch) => onChange(tidy(DESIGN, { dim, base, code, ...patch }));
  return (
    <>
      {spaces.length > 1 && <Segment label={DESIGN[0].label} value={dim} options={spaces} onChange={(v) => write({ dim: v })} />}
      <Segment label={DESIGN[1].label} value={base} options={DESIGN[1].options} onChange={(v) => write({ base: v })} />
      <Select label="Named" value={word(names, code) ? code : ''} options={[['', 'Code'], ...names.map((one) => [one.code, one.name])]} onChange={(v) => v && write({ code: v })} />
      <Text label={DESIGN[2].label} value={code} commit onChange={(v) => write({ code: v })} />
      {'level' in value && <Slider label="Level" value={value.level} min={1} max={cap(dim)} step={1} onChange={(v) => onChange({ level: v })} />}
      <Btn disabled={!math} onClick={() => write({ code: roll(math, dim, base) })}>
        <Icon name="reroll" />
        Random
      </Btn>
      {reps.length > 0 && reps.length <= STRIP && <Thumbs math={math} dim={dim} base={base} codes={reps.map((one) => one.code)} code={code} names={names} onPick={(v) => write({ code: v })} px={40} />}
    </>
  );
}

export function Pick({ value, onChange, math: given, dims }) {
  const math = useUnit(given);
  return <Picker value={value} onChange={onChange} math={math} dims={dims} />;
}
