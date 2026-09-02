import { useEffect, useRef } from 'react';
import { stage } from './stage.js';

export function Stage({ onStage, deps = [], className = 'stage', ...rest }) {
  const ref = useRef(null);
  const live = useRef(null);
  useEffect(() => {
    live.current ??= stage(ref.current);
    onStage(live.current);
  }, deps);
  return <canvas ref={ref} className={className} {...rest} />;
}
