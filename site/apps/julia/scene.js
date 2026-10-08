import { fractal } from '../../lib/fractal.js';
import { pick } from '../../lib/scene.js';

const HOME = { xMin: -1.5, xMax: 1.5, yMin: -1.5, yMax: 1.5 };

const PRESETS = [[-0.4, 0.6], [-0.8, 0.156], [0.285, 0.01], [-0.7269, 0.1889], [-0.1, 0.651], [0.355, 0.355]];

export const make = (canvas, view) => fractal(canvas, view, HOME, pick(view.rand, PRESETS));
