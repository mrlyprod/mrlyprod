import { useCallback, useEffect, useRef, useState } from 'react';
import { Btn } from '../../lib/knobs.jsx';
import { Scene } from '../../lib/scene.jsx';
import { wears } from '../life/scene.js';
import { Design, MaskView, Pane, useLife, useStatus } from '../life/widget.jsx';
import { PRESETS, SPEC, make } from './scene.js';

const PAUSE = 120;

const ring = (peak, wave) => `ring ${peak}, ${wave.toFixed(2)} cells`;

export function status(stat) {
  if (!stat) return [];
  if (stat.live === undefined) return [['error', stat.error]];
  return [
    ['gen', stat.gen],
    ['live', stat.live],
    ['share', stat.share.toFixed(3)],
    ['still', ring(stat.peak, stat.wave)],
    ['mask', ring(stat.maskPeak, stat.maskWave)],
    ['span', stat.span],
    ['cells', stat.cells],
    ['born', stat.birth],
    ['stay', stat.survive],
    ...(stat.error ? [['error', stat.error]] : []),
  ];
}

export function Widget({ value, onChange, onReady, onStatus }) {
  const units = useLife();
  const handle = useRef(null);
  const [stat, setStat] = useState(null);
  const [held, hold] = useState(value);
  const key = JSON.stringify(value);
  useEffect(() => {
    const timer = setTimeout(() => hold(value), PAUSE);
    return () => clearTimeout(timer);
  }, [key]);
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units, onStat: setStat }), [units]);
  const ready = (scene) => {
    handle.current = scene;
    onReady?.(scene);
  };
  useStatus(onStatus, status(stat));
  const folds = [
    { name: 'Run', items: ['steps', 'density', 'size', <Btn key="run" onClick={() => handle.current?.rerun()}>Run</Btn>] },
    { name: 'Rule', live: stat?.name, items: ['born', 'stay', ...PRESETS.map(([label, one]) => <Btn key={label} on={wears(value, one)} onClick={() => onChange(one)}>{label}</Btn>)] },
    {
      name: 'Mask',
      live: stat?.label,
      items: [<MaskView key="view" mask={stat?.mask} dot={8} />, <Design key="code" math={units?.math} dim={2} code={value.code} onChange={(code) => onChange({ code })} />, 'number', 'level'],
    },
  ];
  return (
    <>
      {units && <Scene make={made} value={held} onReady={ready} onClick={() => handle.current?.step()} />}
      <Pane spec={SPEC} value={value} onChange={onChange} folds={folds} />
    </>
  );
}
