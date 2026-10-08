import { fractal } from '../../lib/fractal.js';

const HOME = { xMin: -2, xMax: 1, yMin: -1.5, yMax: 1.5 };

export const make = (canvas, view) => fractal(canvas, view, HOME, null);
