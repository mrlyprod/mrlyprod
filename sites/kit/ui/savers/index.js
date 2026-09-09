import { run } from './frame.js';
import { matrix } from './matrix.js';
import { sleep } from './sleep.js';
import { mandelbrot, julia } from './fractal.js';

export const SAVERS = ['matrix', 'sleep', 'mandelbrot', 'julia'];

const MAKE = { matrix, sleep, mandelbrot, julia };

export function saver(canvas, name = SAVERS[0], opts = {}) {
  const make = MAKE[name];
  if (!canvas || !make) return () => {};
  return run(canvas, make, opts);
}
