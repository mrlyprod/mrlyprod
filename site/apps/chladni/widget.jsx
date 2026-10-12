import { useCallback, useRef } from 'react';
import { Scene } from '../../lib/scene.jsx';
import { useTouch } from '../../lib/touch.js';
import { make } from './scene.js';

const SIGNS = { 1: '+', '-1': '-' };

export function rows(stat) {
  if (!stat) return [];
  if (stat.error) return [['error', stat.error]];
  if (stat.view === 'carpet') {
    return [
      ['k', `${stat.k} of ${stat.count}`],
      ['lambda', stat.lambda.toFixed(6)],
      ['domains', `${stat.domains} of ${stat.bound}`],
      ['grains left', String(stat.grains)],
    ];
  }
  return [
    ['figure', `(${[stat.n, stat.m, SIGNS[stat.sign]].filter((one) => one !== undefined).join(', ')})`],
    ['pitch', stat.pitch === null ? 'none' : stat.pitch.toFixed(3)],
    ['grains', String(stat.grains)],
  ];
}

export function Widget({ value, onChange, onReady, onStatus, units }) {
  const live = useRef(null);
  const change = useRef(onChange);
  const status = useRef(onStatus);
  change.current = onChange;
  status.current = onStatus;
  const tell = useCallback((stat) => status.current?.(rows(stat)), []);
  const push = useCallback((patch) => change.current?.(patch), []);
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units, onTell: tell, onWalk: push }), [units, tell, push]);
  const ready = (handle) => {
    live.current = handle;
    if (!handle) status.current?.([]);
    onReady?.(handle);
  };
  const touch = useTouch(live, change);
  if (!units?.num) return null;
  return <Scene make={scene} value={value} onReady={ready} {...touch} role="img" aria-label="Sand on a Chladni plate" style={{ cursor: 'crosshair' }} />;
}
