import { tidy } from '../../lib/knobs.js';
import { TINTS } from '../../lib/scene.js';
import { DESIGN } from '../designs/scene.js';
import { CELLS, EDGE, FAINT, FILL, GROWTHS, NUMBERS, SOFT, TOP, TRAIL, camera, cameras, fade, head, index, phase, picture, study } from './engine.js';

export const SPEC = [
  { key: 'number', label: 'Number', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: 'Winding' },
  { key: 'top', label: 'Numbers', kind: 'slider', def: 300, min: 2, max: TOP, step: 1, group: 'Winding' },
  { key: 'growth', label: 'Growth', kind: 'segment', def: 'Every', options: GROWTHS, group: 'Winding' },
  { key: 'cell', label: 'Cell', kind: 'pick', def: 0, options: CELLS, group: 'Look' },
  { key: 'path', label: 'Path', kind: 'toggle', def: 1, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [{ key: 'Enter', label: 'Again', act: 'again', button: true }],
  actions: { again: (scene) => scene.again() },
};

export const units = () => import('./unit.js');

const TAU = Math.PI * 2;
const DOT = 3;
const LINE = 1.5;
const HAIR = 4;

/* ART */

function stack(cell, lines, color) {
  const side = cell.shape[1];
  const page = new OffscreenCanvas(side, side);
  const pen = page.getContext('2d');
  pen.fillStyle = color;
  pen.beginPath();
  for (const [row, col, len] of lines) pen.rect(col, row, len, 1);
  pen.fill();
  const out = [page];
  for (let s = side; s > 1; ) {
    const next = Math.ceil(s / 2);
    const small = new OffscreenCanvas(next, next);
    small.getContext('2d').drawImage(out[out.length - 1], 0, 0, next, next);
    out.push(small);
    s = next;
  }
  return out;
}

export function mip(stack, d) {
  let i = stack.length - 1;
  while (i > 0 && stack[i].width < d) i--;
  return stack[i];
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const { math, num } = opts;
  if (!math || !num) throw new Error('snail: the scene wants mrlyjs/math and mrlyjs/num as opts.math and opts.num');
  const ctx = canvas.getContext('2d');
  const value = { ...tidy(DESIGN, opts), ...tidy(SPEC, opts) };
  const plan = study(math, num, value, view.rand);
  const { shell, times } = plan;
  const { tiles } = shell;
  let origin = 0;
  let lens = null;
  let mips = [];
  let accent = '#000';
  let last = '';
  let told = 0;
  let gone = false;
  const press = () => {
    accent = view.look().accent;
    mips = plan.art.map((cell, level) => (cell ? stack(cell, plan.lines[level], accent) : null));
    last = '';
  };
  const fit = () => {
    lens = cameras(shell, view.w, view.h, value.cell, view.dpr);
    last = '';
  };
  const tell = (whole) => {
    const n = Math.min(tiles.length, whole + 1);
    if (n === told || gone) return;
    told = n;
    opts.onTile?.(tiles[n - 1]);
  };
  const paint = (b, [cx, cy, ls], shade) => {
    const k = Math.exp(ls);
    const dpr = view.dpr;
    const ox = view.w / 2 - cx * k;
    const oy = view.h / 2 + cy * k;
    const whole = Math.floor(b);
    const frac = b - whole;
    const shown = Math.min(tiles.length, whole + 1);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.fillStyle = accent;
    ctx.strokeStyle = accent;
    ctx.lineWidth = dpr;
    for (let i = 0; i < shown; i++) {
      const tile = tiles[i];
      const d = tile.side * k;
      const sx = ox + tile.x * k;
      const sy = oy - (tile.y + tile.side) * k;
      if (sx > view.w || sy > view.h || sx + d < 0 || sy + d < 0) continue;
      const a = (tile.prime ? 1 : FAINT) * (i < whole ? 1 : frac) * shade;
      if (!tile.level || d < 1) {
        ctx.globalAlpha = a;
        ctx.fillRect(sx, sy, Math.max(1, d), Math.max(1, d));
        continue;
      }
      ctx.globalAlpha = a * FILL;
      ctx.fillRect(sx, sy, d, d);
      ctx.globalAlpha = a;
      const small = d < mips[tile.level][0].width;
      const page = mip(mips[tile.level], d);
      ctx.imageSmoothingEnabled = small;
      for (let pass = small ? SOFT : 1; pass > 0; pass--) ctx.drawImage(page, sx, sy, d, d);
      if (d < HAIR * dpr) continue;
      ctx.globalAlpha = a * EDGE;
      ctx.strokeRect(sx + dpr / 2, sy + dpr / 2, d - dpr, d - dpr);
    }
    const [hx, hy] = head(tiles, b);
    if (value.path && whole) {
      ctx.globalAlpha = TRAIL * shade;
      ctx.lineWidth = LINE * dpr;
      ctx.lineJoin = 'round';
      ctx.lineCap = 'round';
      ctx.beginPath();
      for (let i = 0; i < Math.min(whole, tiles.length); i++) {
        const x = ox + (tiles[i].x + tiles[i].side / 2) * k;
        const y = oy - (tiles[i].y + tiles[i].side / 2) * k;
        if (i) ctx.lineTo(x, y);
        else ctx.moveTo(x, y);
      }
      if (whole < tiles.length) ctx.lineTo(ox + hx * k, oy - hy * k);
      ctx.stroke();
    }
    ctx.globalAlpha = Math.min(1, b) * shade;
    ctx.beginPath();
    ctx.arc(ox + hx * k, oy - hy * k, DOT * dpr, 0, TAU);
    ctx.fill();
    tell(whole);
  };
  const draw = () => {
    const now = phase(view.t - origin, times.length, view.still);
    const b = index(times, now);
    const cam = camera(times, lens, now, view.still);
    const shade = fade(now, times.length);
    const key = `${b} ${shade} ${cam.join(' ')}`;
    if (key === last) return;
    last = key;
    paint(b, cam, shade);
  };
  const again = () => {
    origin = view.t;
    last = '';
    if (view.still) view.wake?.();
    else draw();
  };
  const stop = () => {
    gone = true;
  };
  press();
  fit();
  return { draw, size: fit, theme: press, again, stop, svg: () => picture(plan, accent, Boolean(value.path)), facts: plan.facts };
}
