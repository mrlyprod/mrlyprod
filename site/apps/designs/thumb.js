import { rgb } from '../../lib/scene.js';
import { grow } from './engine.js';
import { PHI, camera, extent, solid, walk } from './solid.js';
import { runs } from '../../lib/svg.js';

export const LEVEL = 2;
const THETA = 0.62;
const FIT = 0.9;
const SLICE = 8;

/* COLOUR */

export function mix(paper, accent, k) {
  const a = rgb(paper);
  const b = rgb(accent);
  return `rgb(${a.map((one, i) => Math.round(one + (b[i] - one) * k)).join(', ')})`;
}

/* PAINT */

export function flat(ctx, cell, x, y, side, fill) {
  const [rows, cols] = cell.shape;
  const px = side / cols;
  ctx.fillStyle = fill;
  ctx.beginPath();
  for (const [row, col, len] of runs(cell)) {
    const left = Math.round(x + col * px);
    const top = Math.round(y + row * px);
    ctx.rect(left, top, Math.round(x + (col + len) * px) - left, Math.round(y + (row + 1) * px) - top);
  }
  ctx.fill();
}

export function cube(ctx, shape, cam, cx, cy, tones) {
  let kind = -1;
  return walk(shape, cam, cx, cy, (face, x0, y0, x1, y1, x2, y2, x3, y3) => {
    if (face !== kind) {
      kind = face;
      ctx.fillStyle = tones[face];
    }
    ctx.beginPath();
    ctx.moveTo(x0, y0);
    ctx.lineTo(x1, y1);
    ctx.lineTo(x2, y2);
    ctx.lineTo(x3, y3);
    ctx.fill();
  });
}

export function thumb(ctx, math, dim, base, code, side, { accent, paper }) {
  const cell = grow(math, dim, base, code, LEVEL);
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, side, side);
  if (dim === 2) return flat(ctx, cell, 0, 0, side, accent);
  const n = cell.shape[0];
  const [w, h] = extent(n, PHI);
  const cam = camera(THETA, PHI, (FIT * side) / Math.max(w, h));
  return cube(ctx, solid(math.three.quads(cell), n), cam, side / 2, side / 2, cam.tone.map((k) => mix(paper, accent, k)));
}

/* QUEUE */

const jobs = [];
let pending = 0;

function flush() {
  pending = 0;
  const start = performance.now();
  while (jobs.length && performance.now() - start < SLICE) jobs.shift()();
  if (jobs.length) pending = requestAnimationFrame(flush);
}

export function enqueue(job) {
  jobs.push(job);
  if (!pending) pending = requestAnimationFrame(flush);
  return () => {
    const at = jobs.indexOf(job);
    if (at >= 0) jobs.splice(at, 1);
  };
}
