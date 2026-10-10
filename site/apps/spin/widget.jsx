import { useCallback, useEffect, useRef, useState } from 'react';
import { Bar } from '../../lib/frame.jsx';
import { tidy } from '../../lib/knobs.js';
import { Btn, Group, Pick as Select, Slider } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { named, roll, word } from '../designs/engine.js';
import { Pick } from '../designs/pick.jsx';
import { DIM, NEEDLES, cap } from './engine.js';
import { DESIGN, make } from './scene.js';

const BARS = { width: 240, height: 48, gap: 1 };

const pct = (x) => `${(x * 100).toFixed(1)}%`;

const deg = (x) => `${x.toFixed(1)}°`;

export const read = (value) => tidy(DESIGN, value);

const Fact = ({ name, children }) => (
  <label>
    <span>{name}</span>
    <span className="num">{children}</span>
  </label>
);

/* PARTS */

export function Spectrum({ power, copies }) {
  let top = 0;
  for (let order = 1; order < power.length; order++) top = Math.max(top, power[order]);
  const w = BARS.width / power.length;
  return (
    <svg viewBox={`0 0 ${BARS.width} ${BARS.height}`} width="100%" height={BARS.height} role="img" aria-label="Circular-harmonic power by order">
      {Array.from(power, (p, order) => {
        if (p <= 0) return null;
        const h = Math.min(1, p / (top || 1)) * BARS.height;
        const kept = order % copies === 0;
        return <rect key={order} x={order * w} y={BARS.height - h} width={Math.max(0, w - BARS.gap)} height={h} fill={order === 0 ? 'var(--fg)' : kept ? 'var(--accent)' : 'var(--dim)'} opacity={order === 0 || !kept ? 0.5 : 1} />;
      })}
    </svg>
  );
}

export function Profile({ profile, peak, disc, inner, reach }) {
  const last = Math.max(1, profile.length - 1);
  const top = peak || 1;
  const points = Array.from(profile, (v, k) => `${((k / last) * BARS.width).toFixed(1)},${(BARS.height - (Math.max(0, v) / top) * BARS.height).toFixed(1)}`).join(' ');
  const at = (r) => ((r / reach) * BARS.width).toFixed(1);
  const mark = (r, stroke) => <line x1={at(r)} x2={at(r)} y1={0} y2={BARS.height} stroke={stroke} strokeDasharray="3 3" opacity={0.6} />;
  return (
    <svg viewBox={`0 0 ${BARS.width} ${BARS.height}`} width="100%" height={BARS.height} role="img" aria-label="Circle mean by radius">
      <polygon points={`0,${BARS.height} ${points} ${BARS.width},${BARS.height}`} fill="var(--accent)" opacity={0.25} />
      <polyline points={points} fill="none" stroke="var(--accent)" strokeWidth={1.5} />
      {disc > 0 && mark(disc, 'var(--dim)')}
      {mark(inner, 'var(--fg)')}
    </svg>
  );
}

export function Needles({ rpm, onChange }) {
  return (
    <Group name="Needles">
      {NEEDLES.map((at) => <Btn key={at} on={rpm === at} onClick={() => onChange({ rpm: at })}>{at}</Btn>)}
    </Group>
  );
}

export function Design({ value, onChange, math }) {
  const design = read(value);
  const top = cap(design.number);
  const pick = ({ dim, ...patch }) => onChange(patch);
  const number = (v) => {
    const next = read({ ...design, number: v });
    onChange({ number: next.number, level: next.level });
  };
  return (
    <Group name="Design">
      {math ? <Pick value={{ dim: DIM, base: design.base, code: design.code }} onChange={pick} math={math} dims={[DIM]} /> : <span>loading</span>}
      <Select label="Number" value={design.number} options={DESIGN[2].options} onChange={number} />
      <Slider label="Level" value={design.level} min={1} max={top} step={1} commit onChange={(v) => onChange({ level: v })} />
    </Group>
  );
}

/* WIDGET */

export function Widget({ value, onChange, onReady, unit: given }) {
  const [unit, setUnit] = useState(() => given ?? null);
  const [facts, setFacts] = useState(null);
  const [pace, setPace] = useState(null);
  const now = useRef(null);
  useEffect(() => {
    if (given) return undefined;
    let on = true;
    import('./unit.js').then(({ math, ready }) => ready.then(() => on && setUnit(() => math)));
    return () => {
      on = false;
    };
  }, [given]);
  const design = read(value);
  now.current = { ...design, rpm: value.rpm };
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit, onPace: setPace }), [unit]);
  const ready = (handle) => {
    setFacts(handle?.facts ?? null);
    setPace(null);
    const hands = {
      random: () => onChange({ code: roll(unit, DIM, now.current.base) }),
      speed: (by) => onChange({ rpm: now.current.rpm + by }),
      needle: (rpm) => onChange({ rpm }),
    };
    onReady?.(handle && { ...handle, ...hands });
  };
  const name = facts && unit ? word(named(unit, DIM, design.base), facts.code) : '';
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} />}
      <Bar side="right">
        <Design value={value} onChange={onChange} math={unit} />
        <Needles rpm={value.rpm} onChange={onChange} />
      </Bar>
      <Bar side="status">
        {!unit && <span>loading</span>}
        {facts && <span>{name ? `${name} ${facts.code}` : `code ${facts.code}`}</span>}
        {pace && <span>{`${Math.round(pace.hz)} Hz, step ${deg(pace.deg)}, seen ${deg(pace.seen)}`}</span>}
      </Bar>
      {facts && (
        <Bar side="right">
          <Group name="Facts">
            <Fact name="Side">{facts.side}</Fact>
            <Fact name="Fills">{facts.fills}</Fact>
            <Fact name="Mass">{facts.mass.toFixed(1)}</Fact>
            <Fact name="Peak">{facts.peak.toFixed(3)}</Fact>
            <Fact name="Dark disc">{facts.disc.toFixed(2)}</Fact>
            <Fact name="Edge">{facts.inner.toFixed(1)}</Fact>
            <Fact name="Reach">{facts.reach.toFixed(1)}</Fact>
            <Fact name="Order">{facts.order || 'round'}</Fact>
            <Fact name="Petals">{facts.petals || '-'}</Fact>
            <Fact name="Order 0">{pct(facts.share[0])}</Fact>
            {facts.leading.map((one) => <Fact key={one.order} name={`Order ${one.order}`}>{pct(one.share)}</Fact>)}
          </Group>
          <Group name="Profile">
            <Profile profile={facts.profile} peak={facts.peak} disc={facts.disc} inner={facts.inner} reach={facts.reach} />
          </Group>
          <Group name="Spectrum">
            <Spectrum power={facts.power} copies={value.copies} />
          </Group>
        </Bar>
      )}
    </>
  );
}
