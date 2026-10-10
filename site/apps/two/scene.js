import { tidy } from '../../lib/knobs.js';
import { TINTS } from '../../lib/scene.js';
import { rects } from '../../lib/svg.js';
import { DESIGN } from '../designs/scene.js';
import { DEN, REACH, SHAPES, cap, lay, study } from './engine.js';

export const SIZES = [512, 1024, 2048, 4096];

export const SPEC = [
  { key: 'number', label: 'Side', kind: 'pick', def: 3, options: [[3, '3'], [5, '5'], [7, '7']], group: 'Tile' },
  { key: 'level', label: 'Level', kind: 'slider', def: 3, min: 1, max: (value) => cap(value.number, value.x * value.y), step: 1, group: 'Tile' },
  { key: 'x', label: 'Across', kind: 'slider', def: 5, min: 1, max: 12, step: 1, group: 'Sheet' },
  { key: 'y', label: 'Down', kind: 'slider', def: 5, min: 1, max: 12, step: 1, group: 'Sheet' },
  { key: 'crop', label: 'Shape', kind: 'pick', def: 'ball', options: SHAPES, group: 'Crop' },
  { key: 'radius', label: 'Radius', kind: 'slider', def: 16, min: 1, max: (value) => REACH[value.crop] ?? DEN, step: 1, unit: `/${DEN}`, group: 'Crop' },
  { key: 'touch', label: 'Touching', kind: 'toggle', def: 0, group: 'Crop' },
  { key: 'invert', label: 'Invert', kind: 'toggle', def: 0, group: 'Look' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  keys: [
    { key: 'Enter', label: 'Random', act: 'random', button: true },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
  ],
  actions: {
    random: (scene) => scene.random?.(),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
  },
};

const FIT = 0.9;
const UNIT = 10;
const REST = 1000;
const MIME = { png: 'image/png', webp: 'image/webp' };

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) throw new Error('two: the scene wants mrlyjs/math as opts.math');
  const ctx = canvas.getContext('2d');
  const plan = study(math, { ...tidy(DESIGN, opts), ...tidy(SPEC, opts) }, view.rand);
  const { cols, rows, runs } = plan;
  let dirty = true;
  const stamp = (pen, box, color) => {
    pen.fillStyle = color;
    pen.beginPath();
    for (const [row, col, len] of runs) {
      const left = Math.round(box.x + col * box.px);
      const top = Math.round(box.y + row * box.px);
      pen.rect(left, top, Math.round(box.x + (col + len) * box.px) - left, Math.round(box.y + (row + 1) * box.px) - top);
    }
    pen.fill();
  };
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    stamp(ctx, lay(cols, rows, view.w, view.h, FIT), view.look().accent);
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  const svg = () => rects(plan.drawn, { fill: view.look().accent, unit: UNIT });
  const shot = (size, kind = 'png') => {
    const long = Math.max(cols, rows);
    const w = Math.round((size * cols) / long);
    const h = Math.round((size * rows) / long);
    const page = new OffscreenCanvas(w, h);
    const pen = page.getContext('2d');
    const { paper, accent } = view.look();
    pen.fillStyle = paper;
    pen.fillRect(0, 0, w, h);
    stamp(pen, { px: size / long, x: 0, y: 0 }, accent);
    return page.convertToBlob({ type: MIME[kind] ?? MIME.png });
  };
  return { draw, every: REST, size: touch, theme: touch, svg, shot, facts: plan.facts };
}
