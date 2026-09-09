import { pick } from './frame.js';
import { DESIGNS, LEVELS, NUMBERS, tile } from './tiles.js';

const SPEED = 4;
const SHARE = 0.15;

export function bounce(p, v, max) {
  if (p <= 0) return { p: 0, v: Math.abs(v), hit: true };
  if (p >= max) return { p: max, v: -Math.abs(v), hit: true };
  return { p, v, hit: false };
}

export function sleep(canvas, view) {
  const ctx = canvas.getContext('2d');
  const rand = view.rand;
  const wide = () => view.w / view.dpr;
  const tall = () => view.h / view.dpr;
  const side = () => Math.min(wide(), tall()) * SHARE;
  let mark;
  let vx = rand() < 0.5 ? SPEED : -SPEED;
  let vy = rand() < 0.5 ? SPEED : -SPEED;
  let x = 0;
  let y = 0;
  const roll = () => {
    mark = tile(pick(rand, DESIGNS), pick(rand, NUMBERS), pick(rand, LEVELS));
  };
  const paint = () => {
    const box = side();
    const n = mark.size;
    ctx.clearRect(0, 0, wide(), tall());
    ctx.fillStyle = view.look().accent;
    for (let r = 0; r < n; r++) {
      const top = Math.round(y + (r * box) / n);
      const bottom = Math.round(y + ((r + 1) * box) / n);
      for (let c = 0; c < n; c++) {
        if (!mark.cells[r * n + c]) continue;
        const left = Math.round(x + (c * box) / n);
        const right = Math.round(x + ((c + 1) * box) / n);
        ctx.fillRect(left, top, right - left, bottom - top);
      }
    }
  };
  const size = () => {
    ctx.setTransform(view.dpr, 0, 0, view.dpr, 0, 0);
    x = Math.min(x, Math.max(0, wide() - side()));
    y = Math.min(y, Math.max(0, tall() - side()));
  };
  const draw = () => {
    const box = side();
    const hx = bounce(x + vx, vx, Math.max(0, wide() - box));
    const hy = bounce(y + vy, vy, Math.max(0, tall() - box));
    x = hx.p;
    vx = hx.v;
    y = hy.p;
    vy = hy.v;
    if (hx.hit || hy.hit) roll();
    paint();
  };
  roll();
  size();
  x = rand() * Math.max(0, wide() - side());
  y = rand() * Math.max(0, tall() - side());
  return { draw, size, theme: paint };
}
