import { tidy } from '../../lib/knobs.js';
import { TINTS } from '../../lib/scene.js';
import { GAP, MOST, PAD, SHAPES, SMALL, cap, corners, ease, lattice, lay, phase, picture, reach, study } from './engine.js';

export const SPEC = [
  { key: 'shape', label: 'Shape', kind: 'pick', def: '', options: SHAPES, group: 'Shape' },
  { key: 'level', label: 'Level', kind: 'slider', def: MOST, min: 1, max: (value) => cap(value), step: 1, group: 'Shape', when: (value) => value.shape !== '' },
  { key: 'grow', label: 'Grow', kind: 'toggle', def: 1, group: 'Motion' },
  { key: 'cell', label: 'Cell', kind: 'slider', def: 0, min: 0, max: 32, step: 1, unit: 'px', zero: 'Fit', group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  record: true,
  keys: [
    { key: 'Enter', label: 'Again', act: 'again', button: true, when: (value) => value.grow === 1 },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Shape', act: 'shape' },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
  ],
  actions: {
    again: (scene) => scene.again(),
    shape: (scene, e) => scene.turn?.(e.key === 'ArrowLeft' ? -1 : 1),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
  },
};

export const units = () => import('./unit.js');

const REST = 1000;
const CHUNK = 8192;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const { math, num } = opts;
  if (!math || !num) throw new Error('radix: the scene wants mrlyjs/math and mrlyjs/num as opts.math and opts.num');
  const ctx = canvas.getContext('2d');
  const value = { ...tidy(SPEC, opts), code: String(opts.code ?? '').trim() };
  const plan = study(num, math, value, view.rand);
  const { top, sides, norm, angle, turn, sign } = plan;
  const held = new Map();
  const spare = [];
  let box = null;
  let accent = '#000';
  let origin = 0;
  let last = '';
  let told = 0;
  let gone = false;
  const paint = () => {
    accent = view.look().accent;
  };
  const fit = () => {
    const pinned = value.cell > 0 ? (value.cell * view.dpr) / lattice(norm, sides, top).spacing : 0;
    box = lay(plan.stage, view.w, view.h, PAD * view.dpr, pinned);
  };
  const lays = (page, level) => {
    const pen = page.getContext('2d');
    const pts = plan.levels[level - 1];
    const n = pts.length / 2;
    const { spacing, radius } = lattice(norm, sides, level);
    const [m11, m12, m21, m22] = box.quarter ? [0, -box.k, box.k, 0] : [box.k, 0, 0, box.k];
    const { cx, cy, ox, oy } = box;
    pen.setTransform(1, 0, 0, 1, 0, 0);
    pen.clearRect(0, 0, view.w, view.h);
    pen.fillStyle = accent;
    pen.beginPath();
    let open = 0;
    const flush = () => {
      pen.fill();
      pen.beginPath();
      open = 0;
    };
    if (2 * radius * box.k < SMALL * view.dpr) {
      const s = Math.max(spacing * reach(sides, angle, level, turn) * box.k, view.dpr);
      const h = s / 2;
      for (let i = 0; i < n; i++) {
        const x = pts[2 * i] - cx;
        const y = pts[2 * i + 1] - cy;
        pen.rect(ox + m11 * x + m12 * y - h, oy + m21 * x + m22 * y - h, s, s);
        if (++open >= CHUNK) flush();
      }
    } else {
      const ring = corners(sides, angle, level, turn, radius * box.k * (1 - GAP));
      for (let j = 0; j < sides; j++) {
        const vx = ring[2 * j];
        const vy = ring[2 * j + 1] * sign;
        ring[2 * j] = box.quarter ? -vy : vx;
        ring[2 * j + 1] = box.quarter ? vx : vy;
      }
      for (let i = 0; i < n; i++) {
        const dx = pts[2 * i] - cx;
        const dy = pts[2 * i + 1] - cy;
        const x = ox + m11 * dx + m12 * dy;
        const y = oy + m21 * dx + m22 * dy;
        pen.moveTo(x + ring[0], y + ring[1]);
        for (let j = 1; j < sides; j++) pen.lineTo(x + ring[2 * j], y + ring[2 * j + 1]);
        pen.closePath();
        if (++open >= CHUNK) flush();
      }
    }
    pen.fill();
  };
  const want = (levels) => {
    for (const level of [...held.keys()]) {
      if (levels.includes(level)) continue;
      spare.push(held.get(level));
      held.delete(level);
    }
    for (const level of levels) {
      if (held.has(level)) continue;
      const page = spare.pop() ?? new OffscreenCanvas(view.w, view.h);
      lays(page, level);
      held.set(level, page);
    }
  };
  const tell = (level) => {
    if (level === told || gone) return;
    told = level;
    opts.onLevel?.({ level, fill: plan.fill(level), distinct: plan.distinct(level) });
  };
  const draw = () => {
    const { level, k, from } = value.grow ? phase(view.t - origin, top, view.still) : { level: top, k: 1, from: 0 };
    const a = ease(k);
    const prior = a < 1 ? from : 0;
    const key = `${level} ${prior} ${a.toFixed(3)}`;
    if (key === last) return;
    last = key;
    want(prior ? [prior, level] : [level]);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    if (prior) {
      ctx.globalAlpha = 1 - a * a;
      ctx.drawImage(held.get(prior), 0, 0);
    }
    ctx.globalAlpha = a;
    ctx.drawImage(held.get(level), 0, 0);
    ctx.globalAlpha = 1;
    tell(level);
  };
  const release = () => {
    spare.push(...held.values());
    held.clear();
    last = '';
  };
  const size = () => {
    fit();
    held.clear();
    spare.length = 0;
    last = '';
    if (!value.grow) draw();
  };
  const theme = () => {
    paint();
    release();
    if (!value.grow) draw();
  };
  const again = () => {
    if (!value.grow) return;
    origin = view.t;
    last = '';
    if (view.still) view.wake?.();
    else draw();
  };
  const stop = () => {
    gone = true;
  };
  paint();
  fit();
  return { draw, every: value.grow ? undefined : REST, size, theme, again, stop, svg: () => picture(plan, accent), facts: plan.facts };
}
