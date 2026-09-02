import { useEffect, useRef } from 'react';
import { paint, blit, signs } from './mrly.js';

export function useCanvas(draw, deps, live = false, outer) {
  const inner = useRef(null);
  const ref = outer ?? inner;
  useEffect(() => {
    draw(ref.current);
    if (!live) return;
    const on = () => draw(ref.current);
    addEventListener('resize', on);
    return () => removeEventListener('resize', on);
  }, deps);
  return ref;
}

export function Grid({ grid, on, off, canvasRef, className = 'sheet', ...rest }) {
  const ref = useCanvas((canvas) => paint(canvas, grid, on, off), [grid, on, off], false, canvasRef);
  return <canvas ref={ref} className={className} {...rest} />;
}

export function Signs({ grid, hues, canvasRef, className = 'sheet', ...rest }) {
  const ref = useCanvas((canvas) => signs(canvas, grid, hues), [grid, hues], false, canvasRef);
  return <canvas ref={ref} className={className} {...rest} />;
}

export function Pixels({ data, canvasRef, className = 'sheet', ...rest }) {
  const ref = useCanvas((canvas) => blit(canvas, data), [data], false, canvasRef);
  return <canvas ref={ref} className={className} {...rest} />;
}

export function Sketch({ draw, deps = [], onSeek, pad = 14, className = 'sheet', ...rest }) {
  const ref = useCanvas(draw, deps, true);
  const at = (event) => {
    const box = ref.current.getBoundingClientRect();
    onSeek((event.clientX - box.left - pad) / (box.width - 2 * pad));
  };
  const down = onSeek ? (event) => { ref.current.setPointerCapture(event.pointerId); at(event); } : undefined;
  const move = onSeek ? (event) => { if (event.buttons) at(event); } : undefined;
  return <canvas ref={ref} className={className} onPointerDown={down} onPointerMove={move} {...rest} />;
}

export function Markup({ svg, ...rest }) {
  return <div {...rest} dangerouslySetInnerHTML={{ __html: svg }} />;
}
