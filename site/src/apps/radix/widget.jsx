import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { text } from '../../lib/app/export.js';
import { Bar } from '../../lib/app/frame.jsx';
import { Btn, Fold, Icon, Pick, Text } from '../../lib/app/controls.jsx';
import { Knob } from '../../lib/app/knobs.jsx';
import { rng } from '../../lib/rand.js';
import { Scene } from '../../lib/app/page.jsx';
import { classes } from '../../lib/designs/engine.js';
import { Thumbs } from '../../lib/designs/pick.jsx';
import { BASES, BOX, DIM, PRESETS, RINGS, boxOf, boxed, boxy, cap, coded, dial, every, flat, pair, preset, rebase, roll, rolled, spell, toggle, turn, twisted, wears } from './engine.js';
import { SPEC, file, make } from './scene.js';
import { useUnits } from '../../lib/app/hooks.js';
import { wasm } from '../../lib/units/index.js';

const UNITS = wasm('math', 'num');

const row = (key) => SPEC.find((one) => one.key === key);

export const rows = (f) =>
  f
    ? [
        ['level', `${f.level} of ${f.cap}`],
        ['fill', f.fill],
        ['distinct', f.distinct],
        ['glue', f.glued ? 'fewer points than words' : 'none, one point a word'],
        ['dimension', f.dimension.toFixed(6)],
        ['base', `${f.base} in ${f.ring}, norm ${f.norm}`],
        ['digits', `${f.digits} of ${f.norm} classes`],
        ['units', `${f.units} on this lattice`],
        ['code', f.code],
        ['canonical', f.canonical ? 'every digit, so the code names this design' : 'not every digit, so the code names only the classes'],
        ...(f.error ? [['error', f.error]] : []),
      ]
    : [];

export const degrees = (k, n) => `${Math.round((360 * k) / n)}°`;

export function next(name, by) {
  const at = PRESETS.findIndex(([one]) => one === name);
  if (at < 0) return PRESETS[by > 0 ? 0 : PRESETS.length - 1][0];
  return PRESETS[(at + by + PRESETS.length) % PRESETS.length][0];
}

export const deeper = (value) => (value.level >= cap(value) ? 1 : value.level + 1);

export function Chips({ dialed, onToggle }) {
  if (!dialed) return null;
  const { letter, residues, held } = dialed;
  return (
    <div className="designs" role="group" aria-label="Digits">
      {residues.map((z, cls) => {
        const one = held.find((h) => h.cls === cls);
        const word = spell(letter, ...(one?.z ?? z));
        return (
          <Btn key={cls} on={Boolean(one)} aria-pressed={Boolean(one)} disabled={Boolean(one) && held.length === 1} title={`class ${cls}: ${one ? `digit ${word} held` : `${word} out`}`} onClick={() => onToggle(cls)}>
            {word}
          </Btn>
        );
      })}
    </div>
  );
}

export function Pane({ value, units, dialed, facts, name, label, onChange, change, moves }) {
  const letter = dialed?.letter ?? 'w';
  const held = dialed?.held ?? [];
  const turned = held.filter(({ u }) => u > 0).length;
  const fold = (title, summary, children) => (
    <Fold key={title} name={title} live={summary}>
      {children}
    </Fold>
  );
  const baseOptions = (BASES[value.ring] ?? []).map((one) => {
    const [a, c] = pair(one);
    return [one, `${spell(letter, a, c)}${dialed ? `, norm ${units.num.gauss.Ring.norm(dialed.ring, a, c)}` : ''}`];
  });
  const box = dialed ? boxOf(dialed) : '';
  const square = Boolean(units) && boxy(value);
  const names = square ? units.math.name.words(DIM, BOX) : [];
  const reps = square ? (classes(units.math, DIM, BOX) ?? []) : [];
  const load = (patch) => units && change(patch);
  return [
    fold(
      'Design',
      `${label} · level ${value.level} · ${value.line ? 'line' : 'cells'}`,
      <>
        <Pick label="Preset" value={name} options={[['', 'Custom'], ...PRESETS.map(([one, title]) => [one, title])]} disabled={!units} onChange={(one) => one && load(preset(units.num, units.math, one))} />
        <Chips dialed={dialed} onToggle={(cls) => change(toggle(dialed, cls))} />
        <span className="tabs">
          <Btn disabled={!dialed} onClick={moves.every}>Every residue</Btn>
          <Btn disabled={!twisted(value)} onClick={moves.flat}>Drop the twists</Btn>
        </span>
        <Knob row={row('level')} value={value.level} values={value} onChange={onChange} />
        <Knob row={row('line')} value={value.line} values={value} onChange={onChange} />
      </>,
    ),
    fold(
      'Twists',
      turned ? `${turned} of ${held.length} turned` : 'none',
      held.map(({ cls, z, u }) => (
        <Pick key={cls} label={`Twist on ${spell(letter, ...z)}`} value={String(u)} options={dialed.units.map((w, k) => [String(k), `${degrees(k, dialed.units.length)} · ${spell(letter, ...w)}`])} onChange={(v) => change(turn(dialed, cls, Number(v)))} />
      )),
    ),
    fold(
      'Base',
      `Z[${letter}] · ${spell(letter, ...pair(value.base))}`,
      <>
        <Pick label="Ring" value={value.ring} options={RINGS} disabled={!units} onChange={(ring) => load(rebase(units.num, units.math, ring, value.base))} />
        <Pick label="Base" value={value.base} options={baseOptions} disabled={!units} onChange={(base) => load(rebase(units.num, units.math, value.ring, base))} />
        <Text label="Code" value={facts?.code ?? ''} commit inputMode="numeric" onChange={(code) => dialed && change(coded(dialed, units.math, code))} />
      </>,
    ),
    square &&
      fold(
        'Box',
        names.find(([one]) => one === box)?.[1] ?? box,
        <>
          <Pick label="Named" value={names.some(([one]) => one === box) ? box : ''} options={[['', 'Code'], ...names]} onChange={(code) => code && load(boxed(units.num, units.math, code))} />
          <Btn onClick={() => load(boxed(units.num, units.math, roll(Math.random)))}>
            <Icon name="casino" />
            Random
          </Btn>
          {reps.length > 0 && <Thumbs math={units.math} dim={DIM} base={BOX} codes={reps.map((one) => one.code)} code={box} onPick={(code) => load(boxed(units.num, units.math, code))} px={40} />}
        </>,
      ),
  ];
}

export function Widget({ value, onChange, onReady, onExport, onStatus, units: given }) {
  const units = useUnits(UNITS, given);
  const live = useRef(null);
  const [facts, setFacts] = useState(null);
  const dialed = useMemo(() => units && dial(units.num, units.math, value), [units, value.ring, value.base, value.d, value.t]);
  const presets = useMemo(() => (units ? PRESETS.map(([name]) => [name, preset(units.num, units.math, name)]) : []), [units]);
  const name = presets.find(([, one]) => wears(value, one))?.[0] ?? '';
  const label = PRESETS.find(([one]) => one === name)?.[1] ?? 'Custom';
  const now = useRef(null);
  now.current = { value, onChange, dialed, units, name };
  const change = (patch) => patch && now.current.onChange(patch);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units, onFacts: setFacts }), [units]);
  const moves = {
    flat: () => now.current.dialed && change(flat(now.current.dialed)),
    every: () => now.current.dialed && change(every(now.current.dialed)),
    deeper: (by) => change({ level: now.current.value.level + by }),
    step: (by) => {
      const { units: held, name: was } = now.current;
      if (held) change(preset(held.num, held.math, next(was, by)));
    },
  };
  const ready = (handle) => {
    live.current = handle;
    if (!handle) setFacts(null);
    onReady?.(handle && { ...handle, ...moves });
  };
  const seen = useRef(value.seed);
  useEffect(() => {
    if (seen.current === value.seed) return;
    seen.current = value.seed;
    if (dialed) change(rolled(dialed, rng(value.seed)));
  }, [value.seed]);
  const told = facts ? [['design', label], ...rows(facts)] : [];
  const key = JSON.stringify(told);
  useEffect(() => onStatus?.(told), [key]);
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, file(now.current.value), 'svg');
  };
  const have = Boolean(units);
  useEffect(() => onExport?.(have ? [['SVG', keep]] : []), [have]);
  const spelled = spell(dialed?.letter ?? 'w', ...pair(value.base));
  return (
    <>
      {units && <Scene make={scene} value={value} onReady={ready} onClick={() => change({ level: deeper(value) })} role="img" aria-label={`The level ${value.level} words of the radix design ${label} at base ${spelled}`} />}
      <Bar side="right">
        <Pane value={value} units={units} dialed={dialed} facts={facts} name={name} label={label} onChange={onChange} change={change} moves={moves} />
      </Bar>
    </>
  );
}
