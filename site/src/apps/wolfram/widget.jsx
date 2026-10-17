import { useCallback, useMemo, useRef, useState } from 'react';
import { Btn, Icon, Text } from '../../lib/app/controls.jsx';
import { Scene } from '../../lib/app/page.jsx';
import { MaskView, Pane, useLife, usePaint } from '../../lib/life/parts.jsx';
import { SPEC, card, make } from './scene.js';
import { useStatus } from '../../lib/app/hooks.js';

const RULES = 256;
const SEEDS = 4294967296;
const LEGEND = 'corner i = 4 x0 + 2 x1 + x2: the top row is l = 0, the columns run 00, 01, 10, 11 in (c, r)';
const CENSUS = 'Exactly 30 of the 256 rules are onto and exactly 6 are reversible: 15, 51, 85, 170, 204 and 240.';

const yes = (on, word) => (on ? word : `not ${word}`);

export const status = (rule, read, stat) =>
  read && stat
    ? [
        ['rule', rule],
        ['design', read.name],
        ['popcount', `${read.popcount}, lambda ${read.lambda.toFixed(3)}`],
        ['degree', read.degree],
        ['genus', read.genus],
        ['traits', [yes(read.affine, 'affine'), yes(read.onto, 'onto'), yes(read.reversible, 'reversible')].join(', ')],
        ['B/S', read.bs || 'none'],
        ['classes', `Wolfram ${read.klass[0]} of ${read.klass.length}, cube ${read.mates[0].rule} of ${read.mates.length}, NPN ${read.npn}`],
        ...(read.gasket ? [['draws', read.gasket]] : []),
        ['rows', stat.rows],
        ['across', stat.cols],
        ['live', stat.live],
        ['seed', stat.how],
      ]
    : [];

function Chips({ rules, rule, onPick, badge = () => '' }) {
  return (
    <div className="designs">
      {rules.map((one) => {
        const n = one.rule ?? one;
        return (
          <Btn key={n} on={n === rule} aria-pressed={n === rule} onClick={() => onPick(n)}>
            {n}
            <small>{badge(one) || '\u00a0'}</small>
          </Btn>
        );
      })}
    </div>
  );
}

export function Widget({ value, onChange, onReady, onStatus }) {
  const units = useLife();
  const handle = useRef(null);
  const now = useRef(value);
  now.current = value;
  const change = useRef(onChange);
  change.current = onChange;
  const [stat, setStat] = useState(null);
  const paint = usePaint(handle);
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units, onStat: setStat }), [units]);
  const turn = (by) => change.current({ rule: (now.current.rule + by + RULES) % RULES });
  const random = () => change.current({ rule: Math.floor(Math.random() * RULES), seed: Math.floor(Math.random() * SEEDS) });
  const ready = (scene) => {
    const rich = scene && { ...scene, turn, random };
    handle.current = rich;
    onReady?.(rich);
  };
  const rule = value.rule;
  const read = useMemo(() => (units ? card(units.life, rule) : null), [units, rule]);
  const stamp = useMemo(() => read && { shape: [2, 4], data: read.corners }, [read]);
  const pick = (one) => onChange({ rule: one });
  useStatus(onStatus, status(rule, read, stat));
  const folds = [
    {
      name: 'Rule',
      live: read ? `${rule} · ${read.name}` : String(rule),
      items: [
        'rule',
        <Text key="type" label="Type" type="number" inputMode="numeric" min={0} max={RULES - 1} step={1} value={String(rule)} commit onChange={(v) => Number.isFinite(Number.parseInt(v, 10)) && onChange({ rule: Number.parseInt(v, 10) })} />,
        <Btn key="prev" onClick={() => turn(-1)}>Prev</Btn>,
        <Btn key="next" onClick={() => turn(1)}>Next</Btn>,
        <Btn key="random" onClick={random}>
          <Icon name="casino" />
          Random
        </Btn>,
      ],
    },
    read && {
      name: 'Card',
      live: `${read.popcount} of 8`,
      items: [<MaskView key="stamp" mask={stamp} dot={32} label="The eight corner bits of the rule" />, <small key="legend">{LEGEND}</small>, <small key="census">{CENSUS}</small>],
    },
    read && {
      name: 'Classes',
      live: `cube ${read.mates[0].rule}, Wolfram ${read.klass[0]}`,
      items: [
        <small key="cube">The cube orbit: {read.mates.length} rules</small>,
        <Chips key="mates" rules={read.mates} rule={rule} onPick={pick} badge={(one) => (one.onto ? 'onto' : '')} />,
        <small key="wolfram">The Wolfram class: {read.klass.length} rules</small>,
        <Chips key="klass" rules={read.klass} rule={rule} onPick={pick} />,
        <small key="mixed">{read.mixed ? 'Onto is not constant on this orbit, so the cube symmetry is not a symmetry of the dynamics.' : 'Every rule in this orbit answers onto the same way.'}</small>,
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
