import { tidy } from '../../lib/knobs.js';
import { TINTS, pick } from '../../lib/scene.js';
import { DESIGNS, LEVELS, NUMBERS, ink, tile } from '../../lib/tiles.js';

const named = (word) => word[0].toUpperCase() + word.slice(1);

const MIXED = ['', 'Mixed'];

export const SPEC = [
  { key: 'design', label: 'Design', kind: 'pick', def: '', options: [MIXED, ...DESIGNS.map((design) => [design, named(design)])], group: 'Shape' },
  { key: 'number', label: 'Number', kind: 'pick', def: '', options: [MIXED, ...NUMBERS.map((n) => [n, String(n)])], group: 'Shape' },
  { key: 'level', label: 'Level', kind: 'pick', def: '', options: [MIXED, ...LEVELS.map((n) => [n, String(n)])], group: 'Shape' },
  { key: 'speed', label: 'Speed', kind: 'slider', def: 240, min: 60, max: 600, step: 20, unit: 'px/s', group: 'Motion' },
  { key: 'size', label: 'Size', kind: 'slider', def: 0.15, min: 0.08, max: 0.4, step: 0.01, group: 'Size' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = { spec: SPEC };

export function bounce(p, v, max) {
  if (p <= 0) return { p: 0, v: Math.abs(v), hit: true };
  if (p >= max) return { p: max, v: -Math.abs(v), hit: true };
  return { p, v, hit: false };
}

export function make(canvas, view, opts) {
  const ctx = canvas.getContext('2d');
  const rand = view.rand;
  const { design, number, level, speed, size: share } = tidy(SPEC, opts);
  let mark;
  let side = 1;
  let vx = rand() < 0.5 ? 1 : -1;
  let vy = rand() < 0.5 ? 1 : -1;
  let x = 0;
  let y = 0;
  let last = view.t;
  const fit = () => {
    const n = mark.size;
    side = Math.max(1, Math.round((Math.min(view.w, view.h) * share) / n)) * n;
    x = Math.min(x, Math.max(0, view.w - side));
    y = Math.min(y, Math.max(0, view.h - side));
  };
  const roll = () => {
    mark = tile(design || pick(rand, DESIGNS), number || pick(rand, NUMBERS), level || pick(rand, LEVELS));
    fit();
  };
  const paint = () => {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.fillStyle = view.look().accent;
    ink(ctx, mark, Math.round(x), Math.round(y), side);
  };
  const draw = () => {
    const run = (speed * view.dpr * (view.t - last)) / 1000;
    last = view.t;
    if (run > 0) {
      const hx = bounce(x + vx * run, vx, Math.max(0, view.w - side));
      const hy = bounce(y + vy * run, vy, Math.max(0, view.h - side));
      x = hx.p;
      vx = hx.v;
      y = hy.p;
      vy = hy.v;
      if (hx.hit || hy.hit) roll();
    }
    paint();
  };
  roll();
  x = rand() * Math.max(0, view.w - side);
  y = rand() * Math.max(0, view.h - side);
  return { draw, size: fit, theme: paint };
}
