import { useCallback, useMemo, useRef, useState } from 'react';
import { Btn, Icon, Pick, Segment } from '../../lib/app/controls.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { roll } from '../../lib/designs/engine.js';
import { Thumbs } from '../../lib/designs/pick.jsx';
import { codeOf, codesOf, dimOf, maskName, sources, spell } from '../../lib/life/engine.js';
import { SOUP } from '../../lib/life/scene.js';
import { PRESETS, SPEC, make, wears, worn } from './scene.js';
import { MaskView, Pane, useLife, usePaint } from '../../lib/life/parts.jsx';
import { useStatus } from '../../lib/app/hooks.js';

const HAND = ['', 'By hand'];

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
