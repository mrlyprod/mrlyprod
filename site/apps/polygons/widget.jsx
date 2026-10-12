import { useRef } from 'react';
import { Scene } from '../../lib/scene.jsx';
import { make } from './scene.js';

export function Widget({ onReady, ...props }) {
  const live = useRef(null);
  const ready = (handle) => {
    live.current = handle;
    onReady?.(handle);
  };
  const tap = (e) => {
    const box = e.currentTarget.getBoundingClientRect();
    live.current?.poke((e.clientX - box.left) / box.width, (e.clientY - box.top) / box.height);
  };
  return <Scene make={make} onReady={ready} onPointerDown={tap} {...props} />;
}
