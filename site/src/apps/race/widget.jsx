import { useCallback, useEffect, useRef, useState } from 'react';
import { text } from '../../lib/app/export.js';
import { Bar } from '../../lib/app/frame.jsx';
import { rng } from '../../lib/rand.js';
import { Scene } from '../../lib/app/page.jsx';
import { pair } from './engine.js';
import { make, named } from './scene.js';
import { useUnit } from '../../lib/app/hooks.js';

const ENDS = { stall: 'stalled', limit: 'time up' };

const roll = () => Math.floor(Math.random() * 4294967296);

export const again = (seed) => (Number(seed) + 1) >>> 0;

const call = (side) => side.name || `code ${side.code}`;

export function fate({ phase, reach, over, sides }) {
  const name = (s) => call(sides[s]);
  if (over) {
    const when = `at step ${over.tick}`;
    if (over.how === 'goal') return over.winner < 0 ? `dead heat ${when}` : `${name(over.winner)} wins ${when}`;
    return `${ENDS[over.how]} ${when}, ${over.winner < 0 ? 'level' : `${name(over.winner)} ahead`}`;
  }
  if (phase === 'ready') return 'at the start';
  const lead = reach[0] > reach[1] ? 0 : reach[1] > reach[0] ? 1 : -1;
  return `${lead < 0 ? 'level' : `${name(lead)} leads`}${phase === 'held' ? ', paused' : ''}`;
}

const both = (sides, show) => {
  const [a, b] = sides.map(show);
  return a === b ? a : `${a} · ${b}`;
};

export function rows(stat) {
  if (!stat) return [];
  const { sides, reach, goal } = stat;
  const names = sides.map(call);
  const tags = names[0] === names[1] ? names.map((name, i) => `${name} ${'AB'[i]}`) : names;
  return [
    ['race', fate(stat)],
    ['step', String(stat.tick)],
    [tags[0], `${reach[0].toFixed(1)} of ${goal} cells`],
    [tags[1], `${reach[1].toFixed(1)} of ${goal} cells`],
    ['mass', both(sides, (side) => `${side.fills} of ${side.of}`)],
    ['dimension', both(sides, (side) => side.dimension.toFixed(3))],
    ['room', both(sides, (side) => `${side.cells} of ${side.lit}`)],
    ['board', `${stat.side} by ${stat.side}`],
    ['walkers', String(stat.walkers)],
  ];
}

export function Widget({ value, onChange, onReady, onExport, onStatus, unit: given }) {
  const unit = useUnit('math', given);
  const live = useRef(null);
  const now = useRef(value);
  now.current = value;
  const hooks = useRef({});
  hooks.current = { onChange, onStatus };
  const [stat, setStat] = useState(null);
  const tell = useCallback((next) => {
    setStat(next);
    hooks.current.onStatus?.(rows(next));
  }, []);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, math: unit, onTell: tell }), [unit, tell]);
  const go = () => {
    const handle = live.current;
    if (!handle) return;
    if (handle.phase?.() === 'done') hooks.current.onChange?.({ seed: again(now.current.seed) });
    else handle.go?.();
  };
  const fresh = () => {
    if (!unit) return;
    const [a, b] = pair(unit, now.current, rng(roll()));
    hooks.current.onChange?.({ a, b });
  };
  const ready = (handle) => {
    live.current = handle;
    if (!handle) hooks.current.onStatus?.([]);
    onReady?.(handle && { ...handle, go, toggle: go, pair: fresh });
  };
  const keep = () => {
    const picture = live.current?.svg?.();
    if (picture) text(picture, named(now.current), 'svg');
  };
  const log = () => {
    const list = live.current?.csv?.();
    if (list) text(list, named(now.current), 'csv');
  };
  const have = Boolean(unit);
  useEffect(() => onExport?.(have ? [['SVG', keep], ['CSV', log]] : []), [have]);
  const names = stat?.sides.map(call);
  return (
    <>
      {unit && <Scene make={scene} value={value} onReady={ready} onClick={go} role="img" aria-label={names ? `${names[0]} against ${names[1]}` : 'Two boards of walkers'} style={{ cursor: 'pointer' }} />}
      {!unit && (
        <Bar side="status">
          <span>loading</span>
        </Bar>
      )}
    </>
  );
}
