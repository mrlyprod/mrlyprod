import { tidy } from '../../lib/knobs.js';
import { TINTS, rgb } from '../../lib/scene.js';
import { DESIGN } from '../designs/scene.js';
import { COPIES, DIM, NUMBERS, ORIENTS, PROJECTIONS, RINGS, SCALE, cap, fit, kinds, mesh, study, turned, wrap } from './engine.js';

export const SPEC = [
  { key: 'number', label: 'Number', kind: 'pick', def: 3, options: NUMBERS.map((n) => [n, String(n)]), group: 'Shape' },
  { key: 'level', label: 'Level', kind: 'slider', def: 2, min: 1, max: (value) => cap(value.number, COPIES[value.radius] ?? 1), step: 1, group: 'Shape' },
  { key: 'projection', label: 'View', kind: 'segment', def: 'iso', options: PROJECTIONS, group: 'View' },
  { key: 'orient', label: 'Point', kind: 'segment', def: 'pointy', options: ORIENTS, group: 'View' },
  { key: 'radius', label: 'Rings', kind: 'slider', def: 0, min: 0, max: RINGS, step: 1, group: 'View' },
  { key: 'invert', label: 'Invert', kind: 'toggle', def: 0, group: 'View' },
  { key: 'tint', label: 'Tint', kind: 'pick', def: '', options: TINTS, group: 'Look' },
];

export const PAGE = {
  spec: SPEC,
  keys: [
    { key: 'Enter', label: 'Random', act: 'random', button: true },
    { key: ['ArrowLeft', 'ArrowRight'], label: 'View', act: 'view' },
    { key: ['ArrowUp', 'ArrowDown'], label: 'Level', act: 'level' },
  ],
  actions: {
    random: (scene) => scene.random?.(),
    view: (scene, e) => scene.view?.(e.key === 'ArrowLeft' ? -1 : 1),
    level: (scene, e) => scene.deeper?.(e.key === 'ArrowDown' ? -1 : 1),
  },
};

const REST = 1000;
const PAD = 24;
const VOID = 0.1;
const LEFT = 0.72;
const RIGHT = 0.5;

/* COLOUR */

export function tones(look) {
  const a = rgb(look.accent);
  const p = rgb(look.paper);
  const toward = (k) => a.map((v, i) => Math.round(p[i] + (v - p[i]) * k));
  return { FILL: [...a, 255], VOID: [...a, Math.round(255 * VOID)], UP: [...a, 255], LEFT: [...toward(LEFT), 255], RIGHT: [...toward(RIGHT), 255] };
}

const css = ([r, g, b, a]) => `rgba(${r}, ${g}, ${b}, ${Math.round((a / 255) * 1000) / 1000})`;

/* SCENE */

export function make(canvas, view, opts = {}) {
  const math = opts.math;
  if (!math) throw new Error('six: the scene wants mrlyjs/math as opts.math');
  const ctx = canvas.getContext('2d');
  const value = { ...tidy(SPEC, opts), ...tidy(DESIGN, { ...opts, dim: DIM }) };
  const { one, sheet, facts } = study(math, value, view.rand);
  const turn = turned(sheet.orientation, value.orient);
  const built = mesh(math, sheet, turn);
  const codes = kinds(math);
  let dirty = true;
  const paint = () => {
    const look = tones(view.look());
    const { k, ox, oy } = fit(built.box, view.w, view.h, PAD * view.dpr);
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    for (const { name, points } of built.groups) {
      ctx.fillStyle = css(look[name]);
      ctx.beginPath();
      for (let i = 0; i < points.length; i += 6) {
        ctx.moveTo(ox + points[i] * k, oy + points[i + 1] * k);
        ctx.lineTo(ox + points[i + 2] * k, oy + points[i + 3] * k);
        ctx.lineTo(ox + points[i + 4] * k, oy + points[i + 5] * k);
        ctx.closePath();
      }
      ctx.fill();
    }
    dirty = false;
  };
  const draw = () => {
    if (dirty) paint();
  };
  const touch = () => {
    dirty = true;
    paint();
  };
  const svg = () => {
    const look = tones(view.look());
    const custom = Object.fromEntries(Object.entries(codes).map(([name, type]) => [String(type), [look[name] ?? [0, 0, 0, 0]]]));
    return wrap(math.six.rect_svg(math.six.paint(one, custom), SCALE), one.orientation === 'Horizontal', turn);
  };
  return { draw, every: REST, size: touch, theme: touch, svg, facts };
}
