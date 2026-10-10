import { tidy } from '../../lib/knobs.js';
import { board } from '../../lib/scene.js';
import { cap, census, dimension, facts, grow, named, resolve, title, total, word } from './engine.js';
import { mesh } from './mesh.js';
import { space } from './stage.js';
import { rects } from '../../lib/svg.js';
import { flat } from './thumb.js';

export const DESIGN = [
  { key: 'dim', label: 'Dim', kind: 'segment', def: 2, options: [[2, '2D'], [3, '3D']], group: 'Design' },
  { key: 'base', label: 'Base', kind: 'segment', def: 2, options: [[2, '2'], [3, '3']], group: 'Design' },
  { key: 'code', label: 'Code', kind: 'text', def: '', group: 'Design' },
];

export const SPEC = [...DESIGN, { key: 'level', label: 'Level', kind: 'slider', def: 3, min: 1, max: (value) => cap(value.dim), step: 1, group: 'Design' }];

export const PAGE = {
  spec: SPEC,
  gestures: true,
  keys: [
    { key: 'Enter', label: 'Random', act: 'random', button: true },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Design', act: 'design' },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
  ],
  actions: {
    random: (scene) => scene.random?.(),
    design: (scene, e) => scene.step?.(e.key === 'ArrowLeft' ? -1 : 1),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
  },
};

const TAU = Math.PI * 2;
const FIT = 0.86;
const UNIT = 10;
const REST = 1000;

/* STAGES */

function plane(canvas, view, cell, info) {
  const ctx = canvas.getContext('2d');
  const n = cell.shape[0];
  let dirty = true;
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    const { x, y, w } = board(view, n, n, FIT);
    flat(ctx, cell, x, y, w, view.look().accent);
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  const svg = () => {
    const { paper, accent } = view.look();
    return rects(cell, { fill: accent, ground: paper, unit: UNIT });
  };
  return { draw, every: REST, size: touch, theme: touch, svg, info };
}

function solid(canvas, view, three, quads, info) {
  const start = view.rand() * TAU;
  const blank = { draw: () => {}, info };
  if (!three) return blank;
  try {
    return { ...space(three, canvas, view, mesh(quads), start), info };
  } catch {
    return blank;
  }
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  const value = tidy(SPEC, opts);
  const { dim, base, level } = value;
  const code = String(resolve(value.code, total(math, dim, base), view.rand));
  const cell = grow(math, dim, base, code, level);
  const info = { dim, base, code, level, side: cell.shape[0], name: word(named(math, dim, base), code), title: title(math, dim, base, code), ...facts(math, dim, base, code), ...census(math, dim, cell), dimension: dimension(math, dim, base, code) };
  return dim === 3 ? solid(canvas, view, opts.three, math.three.quads(cell), info) : plane(canvas, view, cell, info);
}
