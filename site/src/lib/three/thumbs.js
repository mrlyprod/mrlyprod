import { blend } from '../color.js';
import { PHI, camera, extent, solid } from '../designs/solid.js';
import { cube } from '../designs/thumb.js';
import { flat, memo, region, tensor } from './engine.js';

const FULL = '255';
const NUMBER = 2;
const LEVEL = 4;
const RADIUS = 50;
const FIT = 0.9;
const LOW = 0.1;
const QUARTER = Math.PI / 4;

export const POSES = { corner: [QUARTER, PHI], edge: [QUARTER, LOW], face: [0, LOW] };

export const BLOCK = { n: 1, cells: Int32Array.of(0), masks: Uint8Array.of(63) };

const whole = memo(1);
const held = memo(8);

export function block(math, crop = '') {
  return held(crop || 'none', () => {
    const stock = whole('block', () => math.three.create(FULL, NUMBER, LEVEL, 2));
    const cell = crop ? flat(math.shape.crop(tensor(stock), region(math, crop, RADIUS, false), true)) : stock;
    return solid(math.three.quads(cell), cell.shape[0]);
  });
}

export function glance(ctx, shape, [theta, phi], side, { paper, accent }) {
  const [w, h] = extent(shape.n, phi);
  const cam = camera(theta, phi, (FIT * side) / Math.max(w, h));
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, side, side);
  cube(ctx, shape, cam, side / 2, side / 2, cam.tone.map((k) => blend(paper, accent, k)));
}
