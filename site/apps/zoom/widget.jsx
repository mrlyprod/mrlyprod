import { useRef } from 'react';
import { Scene } from '../../lib/scene.jsx';
import { fit } from '../../lib/space/hud.js';
import { make } from './scene.js';

export function Widget({ onReady, ...rest }) {
  const live = useRef(null);
  const ready = (handle) => {
    live.current = handle;
    onReady?.(handle);
  };
  const down = (e) => {
    const canvas = e.currentTarget;
    const [x, y, k] = fit(canvas.getBoundingClientRect(), canvas.width, canvas.height, e.clientX, e.clientY);
    live.current?.tap?.(x, y, k);
  };
  return <Scene make={make} onReady={ready} onPointerDown={down} {...rest} />;
}
