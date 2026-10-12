import { HUES, rgb } from '../../lib/scene.js';
import { grow } from './engine.js';
import { THETA } from './mesh.js';
import { PHI, camera, extent, solid, walk } from './solid.js';
import { path, runs, sheet } from '../../lib/svg.js';

export const LEVEL = 2;
export const DEGREES = ['blue', 'orange', 'yellow', 'green', 'pink'];
const FIT = 0.9;
const SLICE = 8;
const ISO = 1024;
const LOW = 0.1;

/* COLOUR */

export function hue(tint, degree) {
  if (tint !== 'degree') return HUES.includes(tint) ? tint : '';
  return degree >= 0 ? DEGREES[degree % DEGREES.length] : '';
}

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

export function iso(math, cell, theta, phi, paper, accent) {
  const n = cell.shape[0];
  const up = Math.max(LOW, phi);
  const [w, h] = extent(n, up);
  const cam = camera(theta, up, (FIT * ISO) / Math.max(w, h));
  const tones = cam.tone.map((k) => mix(paper, accent, k));
  const parts = [];
  let kind = -1;
  let d = '';
  const flush = () => {
    if (d) parts.push(`<path fill="${tones[kind]}" d="${d}"/>`);
    d = '';
  };
  walk(solid(math.three.quads(cell), n), cam, ISO / 2, ISO / 2, (face, ...points) => {
    if (face !== kind) {
      flush();
      kind = face;
    }
    d += path(points, true);
  });
  flush();
  return sheet(ISO, ISO, `<rect width="${ISO}" height="${ISO}" fill="${paper}"/>${parts.join('')}`);
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
