import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Bar } from '../../lib/frame.jsx';
import { Btn, Fold, Icon, Knob, Pick, Segment, gist } from '../../lib/knobs.jsx';
import { paints, veil } from '../../lib/scene.js';
import { Scene } from '../../lib/scene.jsx';
import { roll } from '../designs/engine.js';
import { Thumbs, useTheme } from '../designs/pick.jsx';
import { codeOf, codesOf, dimOf, maskName, sources, spell } from './engine.js';
import { PRESETS, SOUP, SPEC, make, runs, wears, worn } from './scene.js';

const HAND = ['', 'By hand'];
const SPOT = 160;
const DOT = 16;
const STRIP = 12;
const GROUND = 0.1;

const at = (e) => {
  const box = e.currentTarget.getBoundingClientRect();
  return [(e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height];
};

export function useLife() {
  const [units, setUnits] = useState(null);
  useEffect(() => {
    let on = true;
    import('./unit.js').then(({ life, math, ready }) => ready.then(() => on && setUnits({ life, math })));
    return () => {
      on = false;
    };
  }, []);
  return units;
}

export function useStatus(onStatus, rows) {
  const key = JSON.stringify(rows);
  useEffect(() => onStatus?.(rows), [key]);
}

export function usePaint(handle) {
  const painting = useRef(false);
  const down = (e) => {
    if (e.button) return;
    e.currentTarget.setPointerCapture?.(e.pointerId);
    painting.current = true;
    handle.current?.touch?.(...at(e), true);
  };
  const move = (e) => {
    if (painting.current) handle.current?.touch?.(...at(e), false);
  };
  const up = () => {
    painting.current = false;
  };
  return { onPointerDown: down, onPointerMove: move, onPointerUp: up, onPointerCancel: up };
}

export function Pane({ spec, value, onChange, folds }) {
  return (
    <Bar side="right">
      {folds.filter(Boolean).map(({ name, items = [], live }) => {
        const rows = items.filter((item) => typeof item === 'string').map((key) => spec.find((row) => row.key === key));
        return (
          <Fold key={name} name={name} live={live ?? gist(rows, value)}>
            {items.map((item) => {
              if (typeof item !== 'string') return item;
              const row = rows.find((one) => one.key === item);
              return row ? <Knob key={item} row={row} value={value[item]} values={value} onChange={onChange} /> : null;
            })}
          </Fold>
        );
      })}
    </Bar>
  );
}

export function MaskView({ mask, dot = DOT, label = 'The neighbourhood mask' }) {
  const at = useRef(null);
  const tick = useTheme();
  useEffect(() => {
    const canvas = at.current;
    if (!canvas || !mask) return;
    const [rows, cols] = mask.shape;
    const dpr = Math.min(globalThis.devicePixelRatio || 1, 2);
    const cell = Math.max(1, Math.min(Math.floor((SPOT * dpr) / cols), dot * dpr));
    const high = rows === 1 ? Math.max(cell, STRIP * dpr) : cell;
    canvas.width = cols * cell;
    canvas.height = rows * high;
    canvas.style.width = `${(cols * cell) / dpr}px`;
    canvas.style.height = `${(rows * high) / dpr}px`;
    const ctx = canvas.getContext('2d');
    const { accent } = paints(canvas);
    ctx.fillStyle = veil(accent, GROUND);
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    ctx.fillStyle = accent;
    runs(ctx, mask.data, cols, rows, 0, 0, cell, high);
  }, [mask, dot, tick]);
  return <canvas ref={at} className="mask" role="img" aria-label={label} style={{ display: 'block' }} />;
}

export function Design({ math, dim, code, onChange }) {
  const now = String(codeOf(code, dim));
  const options = codesOf(dim).map((one) => {
    const word = math ? maskName(math, one, dim) : '';
    return [one, word ? `${one} ${word}` : one];
  });
  const named = math && dim === 2 ? math.name.words(2, 2).map(([one]) => one) : [];
  return (
    <>
      <Pick label="Code" value={now} options={options} onChange={onChange} />
      <Btn disabled={!math} onClick={() => onChange(roll(math, dim, 2))}>
        <Icon name="casino" />
        Random
      </Btn>
      {named.length > 0 && <Thumbs math={math} dim={2} base={2} codes={named} code={now} onPick={onChange} px={40} />}
    </>
  );
}

const listed = (counts) => (counts?.length ? counts.join(' ') : 'none');

export const status = (stat) =>
  stat
    ? [
        ['gen', stat.gen],
        ['pop', stat.pop],
        ['fate', stat.fate || '-'],
        ['rule', stat.name],
        ['born', listed(stat.counts?.[0])],
        ['stay', listed(stat.counts?.[1])],
        ['mask', stat.label],
        ['cells', stat.cells],
        ['lattice', stat.lattice],
        ...(stat.error ? [['error', stat.error]] : []),
      ]
    : [];

const clean = (text) => String(text ?? '').trim().toLowerCase();

export function Widget({ value, onChange, onReady, onStatus }) {
  const units = useLife();
  const handle = useRef(null);
  const [stat, setStat] = useState(null);
  const paint = usePaint(handle);
  const names = useMemo(() => (units ? sources(units.life) : []), [units]);
  const options = useMemo(() => [HAND, ...names.map((one) => [one, one.replaceAll('_', ' ')])], [names]);
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units, onStat: setStat }), [units]);
  const ready = (scene) => {
    handle.current = scene;
    onReady?.(scene);
  };
  const act = (name) => () => handle.current?.[name]?.();
  const dim = dimOf(value);
  const chosen = (text) => (names.includes(clean(text)) ? clean(text) : '');
  const from = (key, i, label) => <Pick key={`${key}-from`} label={label} value={chosen(value[key])} options={options} onChange={(one) => onChange({ [key]: one || spell(stat?.counts?.[i] ?? []) })} />;
  useStatus(onStatus, status(stat));
  const preset = worn(PRESETS, value);
  const folds = [
    {
      name: 'Board',
      items: [
        'from',
        ...(SOUP(value) ? ['density'] : []),
        'speed',
        <Btn key="step" onClick={act('step')}>Step</Btn>,
        <Btn key="fate" onClick={act('doom')}>Run to fate</Btn>,
        <Btn key="reset" onClick={act('reset')}>Reset</Btn>,
        <Btn key="clear" onClick={act('clear')}>Clear</Btn>,
      ],
    },
    { name: 'Presets', live: preset, items: PRESETS.map(([label, one]) => <Btn key={label} on={wears(value, one)} onClick={() => onChange(one)}>{label}</Btn>) },
    { name: 'Rule', live: stat?.name, items: ['born', ...(units ? [from('born', 0, 'Born from')] : []), 'stay', ...(units ? [from('stay', 1, 'Stay from')] : []), 'wrap'] },
    {
      name: 'Mask',
      live: stat?.label,
      items: [
        <MaskView key="view" mask={stat?.mask} />,
        <Segment key="dim" label="Dim" value={dim} options={SPEC[0].options} onChange={(v) => onChange({ dim: Number(v), code: Number(v) === 1 ? '1' : '7' })} />,
        <Design key="code" math={units?.math} dim={dim} code={value.code} onChange={(code) => onChange({ code })} />,
        'number',
        'level',
      ],
    },
  ];
  return (
    <>
      {units && <Scene make={made} value={value} onReady={ready} {...paint} />}
      <Pane spec={SPEC} value={value} onChange={onChange} folds={folds} />
    </>
  );
}
