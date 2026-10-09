import { useCallback, useRef } from 'react';
import { Scene } from '../../lib/scene.jsx';
import { make, next } from './scene.js';

export function Widget({ value, onChange, onReady }) {
  const now = useRef(value);
  const tell = useRef(onChange);
  now.current = value;
  tell.current = onChange;
  const step = (by) => tell.current?.({ world: next(now.current.world, by) });
  const made = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, live: now }), []);
  const { world, moons, rings, seed } = value;
  return <Scene make={made} value={{ world, moons, rings, seed }} onReady={(handle) => onReady?.(handle && { ...handle, step })} />;
}
