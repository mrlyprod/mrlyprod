import { useCallback } from 'react';
import { Scene } from '../../lib/app/page.jsx';
import { make } from './scene.js';

export function Widget({ units, ...props }) {
  const scene = useCallback((canvas, view, opts) => make(canvas, view, { ...opts, ...units }), [units]);
  if (!units?.math) return null;
  return <Scene make={scene} {...props} />;
}
