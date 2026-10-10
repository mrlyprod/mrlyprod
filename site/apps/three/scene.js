import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb } from '../../lib/scene.js';
import { canvas as pen, frame as box, svg as sheet } from '../../../pkgs/mrlyjs/view/index.js';
import { mesh } from '../designs/mesh.js';
import { paint } from './cuts.js';
import { CUT, DEN, GROW, SHAPES, study } from './engine.js';
import { space } from './stage.js';

const named = (word) => word[0].toUpperCase() + word.slice(1);

export const SPEC = [
  ...GROW,
  ...CUT,
  { key: 'crop', label: 'Shape', kind: 'pick', def: '', options: [['', 'None'], ...SHAPES.map((shape) => [shape, named(shape)])], group: 'Crop' },
  { key: 'radius', label: 'Radius', kind: 'slider', def: 16, min: 1, max: DEN, step: 1, unit: `/${DEN}`, group: 'Crop' },
  { key: 'spin', label: 'Spin', kind: 'toggle', def: 1, group: 'Motion' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  gestures: true,
  record: true,
  keys: [
    { key: 'Enter', label: 'Random', act: 'random', button: true },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'Plane', act: 'plane' },
  ],
  actions: {
    random: (scene) => scene.random?.(),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
    plane: (scene, e) => scene.slide?.(e.key === 'ArrowLeft' ? -1 : 1),
  },
};

const TAU = Math.PI * 2;
const REST = 1000;
const PAD = 0.06;
const FAINT = 0.07;

/* STAGES */

function solid(canvas, view, three, math, plan, start, spin) {
  const extra = { facts: plan.facts, obj: () => math.three.to_obj(plan.cell) };
  const blank = { draw: () => {}, ...extra };
  if (!three) return blank;
  try {
    return { ...space(three, canvas, view, mesh(math.three.quads(plan.cell)), { start, spin }), every: spin ? undefined : REST, ...extra };
  } catch {
    return blank;
  }
}

function flat(canvas, view, plan) {
  const ctx = canvas.getContext('2d');
  let dirty = true;
  const ink = () => {
    const [r, g, b] = rgb(view.look().accent);
    return { fill: [r, g, b, 255], faint: [r, g, b, Math.round(255 * FAINT)] };
  };
  const area = (w, h) => {
    const pad = Math.min(w, h) * PAD;
    return box(pad, pad, w - 2 * pad, h - 2 * pad);
  };
  const draw = () => {
    if (!dirty) return;
    dirty = false;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    paint(pen(ctx, view.w, view.h), area(view.w, view.h), plan.cut, ink());
  };
  const touch = () => {
    dirty = true;
    draw();
  };
  const svg = () => {
    const w = view.w / view.dpr;
    const h = view.h / view.dpr;
    const page = sheet(w, h);
    paint(page, area(w, h), plan.cut, ink());
    return page.text();
  };
  return { draw, every: REST, size: touch, theme: touch, svg, facts: plan.facts, cut: plan.cut.facts };
}

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) throw new Error('three: the scene wants mrlyjs/math as opts.math');
  const value = { ...tidy(SPEC, opts), base: opts.base, code: opts.code };
  const plan = study(math, value, view.rand);
  const start = view.rand() * TAU;
  return value.view === 'solid' ? solid(canvas, view, opts.three, math, plan, start, value.spin) : flat(canvas, view, plan);
}
